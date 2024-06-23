//! Module for interfacing with Claude 3 via AWS Bedrock.
//!
//! Provides the functionality to construct and send requests to Claude 3 models hosted
//! on AWS Bedrock, facilitating integration of LLM processing within
//! processing chains. This module is designed to handle text and image inputs, offering a
//! flexible interface for various types of content.

use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::string::ToString;

use async_trait::async_trait;
use aws_sdk_bedrockruntime::operation::converse::builders::ConverseFluentBuilder;
use aws_sdk_bedrockruntime::operation::converse::ConverseOutput;
use aws_sdk_bedrockruntime::types::{
    ContentBlock, ConversationRole, Message, SystemContentBlock, ToolConfiguration,
    ToolResultBlock, ToolResultContentBlock, ToolResultStatus,
};
use aws_sdk_bedrockruntime::Client;
use aws_smithy_types::Document;
use serde_json::Value;
use tokio::sync::RwLock;
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::agents::tool_registry::{convert_document_to_value, convert_value_to_document};
use crate::error::AnchorChainError;
use crate::node::{Node, Stateful};
use crate::{StateManager, Stateless, ToolRegistry};

static HISTORY_KEY: &str = "BedrockConverseHistory";

#[derive(Clone, Copy, Debug)]
pub enum BedrockModel {
    Claude3,
    Claude3_5,
}

impl BedrockModel {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Claude3 => "anthropic.claude-3-sonnet-20240229-v1:0",
            Self::Claude3_5 => "anthropic.claude-3-5-sonnet-20240620-v1:0",
        }
    }
}

impl From<BedrockModel> for String {
    fn from(value: BedrockModel) -> Self {
        value.as_str().to_string()
    }
}

impl From<&BedrockModel> for String {
    fn from(value: &BedrockModel) -> Self {
        value.as_str().to_string()
    }
}

/// A processor for integrating Claude 3 LLM processing within a chain.
///
/// `BedrockConverse` allows for sending requests to Claude 3 models using Bedrock's
/// Converse API.
#[derive(Clone)]
pub struct BedrockConverse<'a, O: Clone> {
    model: BedrockModel,
    /// The system prompt or context to use for all requests.
    system_prompt: String,
    /// The AWS Bedrock client for sending requests.
    client: Client,
    // tool_configuration: Option<ToolConfiguration>,
    tool_registry: Option<&'a RwLock<ToolRegistry<'a>>>,
    history: StateManager<String, Vec<O>>,
    _output: PhantomData<O>,
}

impl<'a, O: Clone> BedrockConverse<'a, O> {
    pub async fn new(model: BedrockModel) -> Self {
        Self::new_with_system_prompt(model, "You are a helpful assistant").await
    }

    /// Constructs a new `Claude3Bedrock` processor with the specified system prompt.
    ///
    /// Initializes the AWS Bedrock client using the environment's AWS configuration.
    pub async fn new_with_system_prompt(
        model: BedrockModel,
        system_prompt: impl Into<String>,
    ) -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        BedrockConverse {
            model,
            client,
            tool_registry: None,
            system_prompt: system_prompt.into(),
            history: StateManager::new(),
            _output: PhantomData,
        }
    }
}

#[async_trait]
impl<'a> Node for BedrockConverse<'a, String> {
    type Input = String;
    type Output = String;

    /// Processes the input through the Claude 3 model, returning the model's output.
    ///
    /// Constructs a request to the Claude 3 model with the provided input, sends it via
    /// AWS Bedrock, and extracts the text content from the response.
    #[cfg_attr(feature = "tracing", instrument(fields(system_prompt = self.system_prompt.as_str())))]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        let request = self
            .client
            .converse()
            .messages(
                Message::builder()
                    .content(ContentBlock::Text(input))
                    .build()
                    .unwrap(),
            )
            .model_id(self.model)
            .system(SystemContentBlock::Text(self.system_prompt.clone()));
        let response = request.send().await?;

        if let Some(output) = response.output() {
            return Ok(output.as_message().unwrap().content[0]
                .as_text()
                .unwrap()
                .clone());
        } else {
            return Err(AnchorChainError::ModelError(
                "No output returned".to_string(),
            ));
        }
    }
}

impl<'a> Stateless for BedrockConverse<'a, String> {}

#[async_trait]
impl<'a> Stateful<String, Vec<String>> for BedrockConverse<'a, String> {
    async fn set_state(&mut self, state: StateManager<String, Vec<String>>) {
        self.history = state;
    }
}

impl<'a> BedrockConverse<'a, Message> {
    async fn generate_message_with_history(&self, user_message: impl Into<String>) -> Vec<Message> {
        let message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(user_message.into()))
            .build()
            .expect("Error building message");
        self.history.push(HISTORY_KEY.to_string(), message).await;
        self.history
            .get(&HISTORY_KEY.to_string())
            .await
            .expect("Messages should not be empty")
    }

    async fn create_request<'b>(
        &self,
        input: impl Into<String>,
        tool_registry: Option<&'b RwLock<ToolRegistry<'b>>>,
    ) -> ConverseFluentBuilder {
        let mut request = self
            .client
            .converse()
            .model_id(self.model)
            .set_messages(Some(self.generate_message_with_history(input).await))
            .system(SystemContentBlock::Text(self.system_prompt.clone()));

        if let Some(tools) = tool_registry {
            let tool_config = self.generate_tool_configuration(tools).await;
            request = request.tool_config(tool_config);
        }

        request
    }
    pub async fn invoke_with_tool_responses(
        &self,
        results: &[ToolResultBlock],
        tool_registry: &RwLock<ToolRegistry<'_>>,
    ) -> Result<Message, AnchorChainError> {
        let message = Message::builder()
            .role(ConversationRole::User)
            .set_content(Some(
                results
                    .iter()
                    .map(|result| ContentBlock::ToolResult(result.clone()))
                    .collect(),
            ))
            .build()
            .expect("Error building message");

        self.history.push(HISTORY_KEY.to_string(), message).await;

        let mut request = self
            .client
            .converse()
            .model_id(self.model)
            .set_messages(Some(
                self.history
                    .get(&HISTORY_KEY.to_string())
                    .await
                    .expect("History should exist"),
            ))
            .system(SystemContentBlock::Text(self.system_prompt.clone()));

        let tool_config = self.generate_tool_configuration(tool_registry).await;
        request = request.tool_config(tool_config);

        let output = request.send().await?;
        self.process_model_response(output).await
    }

    async fn process_model_response(
        &self,
        response: ConverseOutput,
    ) -> Result<Message, AnchorChainError> {
        if let Some(output) = response.output() {
            let message = output.as_message().unwrap();
            self.history
                .push(HISTORY_KEY.to_string(), message.clone())
                .await;
            Ok(message.clone())
        } else {
            Err(AnchorChainError::ModelError(
                "No output returned".to_string(),
            ))
        }
    }

    pub async fn run_agent<'b>(
        &self,
        input: String,
        max_iterations: usize,
        tool_registry: &'b RwLock<ToolRegistry<'b>>,
    ) -> Result<String, AnchorChainError> {
        let mut output = Vec::new();
        let input = format!(
            "Given the tools available, answer the user's question: {}",
            input
        )
        .to_string();
        println!("Executing agent with input: {input}");
        println!("===========\n");

        // let mut response = self.process(input.clone()).await?.content;
        let response = self
            .create_request(input, Some(tool_registry))
            .await
            .send()
            .await?;
        let mut response = self.process_model_response(response).await?.content;

        for _ in 0..max_iterations {
            let mut tool_responses = Vec::new();
            for content in response.clone() {
                match content {
                    ContentBlock::Text(text) => {
                        println!("{text}\n");
                        output.push(text)
                    }
                    ContentBlock::ToolUse(tool_request) => {
                        println!(
                            "Calling {} for request_id {}",
                            tool_request.name, tool_request.tool_use_id
                        );
                        let tool_result = tool_registry.read().await.execute_tool(
                            tool_request.name(),
                            convert_document_to_value(&tool_request.input),
                        );
                        match tool_result {
                            Ok(return_value) => {
                                tool_responses.push(Self::generate_tool_result_block(
                                    tool_request.tool_use_id,
                                    return_value,
                                    true,
                                ));
                            }
                            Err(error) => {
                                println!("Error executing tool: {error}");
                                tool_responses.push(Self::generate_tool_result_block(
                                    tool_request.tool_use_id,
                                    Value::String(error),
                                    false,
                                ));
                            }
                        };
                    }
                    ContentBlock::Image(_) => unimplemented!("Received unexpected Image response"),
                    ContentBlock::ToolResult(_) => unreachable!("Received ToolResult from model"),
                    _ => unimplemented!("Unknown response received from model"),
                }
            }
            if tool_responses.is_empty() {
                break;
            } else {
                let tool_response = self
                    .invoke_with_tool_responses(&tool_responses, tool_registry)
                    .await;
                response = tool_response?.content;
            }
        }
        println!("\n============\n\n");
        Ok(output.join("\n\n"))
    }

    pub fn generate_tool_result_block(
        tool_use_id: impl Into<String>,
        tool_result: Value,
        success: bool,
    ) -> ToolResultBlock {
        let status = if success {
            ToolResultStatus::Success
        } else {
            ToolResultStatus::Error
        };

        ToolResultBlock::builder()
            .tool_use_id(tool_use_id)
            .content(ToolResultContentBlock::Json(Document::Object(
                HashMap::from([(
                    "return".to_string(),
                    convert_value_to_document(&tool_result),
                )]),
            )))
            .set_status(Some(status))
            .build()
            .unwrap()
    }

    async fn generate_tool_configuration(
        &self,
        tool_registry: &RwLock<ToolRegistry<'_>>,
    ) -> ToolConfiguration {
        let registry = tool_registry.read().await;
        (*registry).clone().into()
    }
}

#[async_trait]
impl<'a> Node for BedrockConverse<'a, Message> {
    type Input = String;
    type Output = Message;

    /// Processes the input through the Claude 3 model, returning the model's output.
    ///
    /// Constructs a request to the Claude 3 model with the provided input, sends it via
    /// AWS Bedrock, and extracts the text content from the response.
    #[cfg_attr(feature = "tracing", instrument(fields(system_prompt = self.system_prompt.as_str())))]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        let response = self
            .create_request(input, self.tool_registry)
            .await
            .send()
            .await?;

        self.process_model_response(response).await
    }
}

impl<'a> Stateless for BedrockConverse<'a, Message> {}

#[async_trait]
impl<'a> Stateful<String, Vec<Message>> for BedrockConverse<'a, Message> {
    async fn set_state(&mut self, state: StateManager<String, Vec<Message>>) {
        self.history = state;
    }
}

#[async_trait]
impl<'a> Node for &BedrockConverse<'a, Message> {
    type Input = String;
    type Output = Message;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        self.process(input).await
    }
}

impl<'a> Stateless for &BedrockConverse<'a, Message> {}

#[async_trait]
impl<'a> Stateful<String, Vec<Message>> for &BedrockConverse<'a, Message> {
    async fn set_state(&mut self, state: StateManager<String, Vec<Message>>) {
        self.set_state(state).await;
    }
}

impl<'a, T: Clone> fmt::Debug for BedrockConverse<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BedrockConverse")
            .field("system_prompt", &self.system_prompt)
            .finish()
    }
}

#[derive(Debug)]
pub struct Claude3Bedrock<'a> {
    llm: BedrockConverse<'a, String>,
}

impl<'a> Claude3Bedrock<'a> {
    pub async fn new() -> Self {
        Claude3Bedrock {
            llm: BedrockConverse::new(BedrockModel::Claude3).await,
        }
    }

    pub async fn new_with_system_prompt(input: impl Into<String>) -> Self {
        Claude3Bedrock {
            // llm: BedrockConverse::new(BedrockModel::Claude3).await,
            llm: BedrockConverse::new_with_system_prompt(BedrockModel::Claude3, input.into()).await,
        }
    }
}

#[async_trait]
impl<'a> Node for Claude3Bedrock<'a> {
    type Input = String;
    type Output = String;

    #[cfg_attr(feature = "tracing", instrument(fields(system_prompt = self.llm.system_prompt.as_str())))]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        self.llm.process(input).await
    }
}

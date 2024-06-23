//! Module for interfacing with Claude 3 via AWS Bedrock.
//!
//! Provides the functionality to construct and send requests to Claude 3 models hosted
//! on AWS Bedrock, facilitating integration of LLM processing within
//! processing chains. This module is designed to handle text and image inputs, offering a
//! flexible interface for various types of content.

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
use tokio::sync::RwLock;
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::agents::tool_registry::ToolHandler;
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
pub struct BedrockConverse<O: Clone> {
    model: BedrockModel,
    /// The system prompt or context to use for all requests.
    system_prompt: String,
    /// The AWS Bedrock client for sending requests.
    client: Client,
    tool_configuration: Option<ToolConfiguration>,
    history: StateManager<String, Vec<O>>,
    _output: PhantomData<O>,
}

impl<O: Clone> BedrockConverse<O> {
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
            tool_configuration: None,
            system_prompt: system_prompt.into(),
            history: StateManager::new(),
            _output: PhantomData,
        }
    }
}

#[async_trait]
impl Node for BedrockConverse<String> {
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

impl Stateless for BedrockConverse<String> {}

#[async_trait]
impl Stateful<String, Vec<String>> for BedrockConverse<String> {
    async fn set_state(&mut self, state: StateManager<String, Vec<String>>) {
        self.history = state;
    }
}

impl BedrockConverse<Message> {
    async fn generate_message_with_history(&self, user_message: impl Into<String>) -> Vec<Message> {
        let message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(user_message.into()))
            .build()
            .expect("Error building message");
        self.history.push(HISTORY_KEY.to_string(), message).await;
        println!("\nHistory: {:?}\n\n", self.history);
        self.history
            .get(&HISTORY_KEY.to_string())
            .await
            .expect("Messages should not be empty")
    }
    async fn create_request(&self, input: impl Into<String>) -> ConverseFluentBuilder {
        let mut request = self
            .client
            .converse()
            .model_id(self.model)
            .set_messages(Some(self.generate_message_with_history(input).await))
            .system(SystemContentBlock::Text(self.system_prompt.clone()));
        if let Some(tool_config) = &self.tool_configuration {
            request = request.tool_config(tool_config.clone());
        }
        request
    }

    pub async fn invoke_with_tool_response(
        &self,
        tool_use_id: impl Into<String>,
        tool_response: Document,
        status: Option<ToolResultStatus>,
    ) -> Result<Message, AnchorChainError> {
        let message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::ToolResult(
                ToolResultBlock::builder()
                    .tool_use_id(tool_use_id)
                    .content(ToolResultContentBlock::Json(tool_response))
                    .set_status(status)
                    .build()
                    .unwrap(),
            ))
            .build()
            .expect("Error building message");

        println!("Tool response message: {:?}\n", message);
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
        if let Some(tool_config) = &self.tool_configuration {
            request = request.tool_config(tool_config.clone());
        }
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
}

#[async_trait]
impl Node for BedrockConverse<Message> {
    type Input = String;
    type Output = Message;

    /// Processes the input through the Claude 3 model, returning the model's output.
    ///
    /// Constructs a request to the Claude 3 model with the provided input, sends it via
    /// AWS Bedrock, and extracts the text content from the response.
    #[cfg_attr(feature = "tracing", instrument(fields(system_prompt = self.system_prompt.as_str())))]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        let response = self.create_request(input).await.send().await?;

        self.process_model_response(response).await
    }
}

impl Stateless for BedrockConverse<Message> {}

#[async_trait]
impl Stateful<String, Vec<Message>> for BedrockConverse<Message> {
    async fn set_state(&mut self, state: StateManager<String, Vec<Message>>) {
        self.history = state;
    }
}

#[async_trait]
impl Node for &BedrockConverse<Message> {
    type Input = String;
    type Output = Message;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        self.process(input).await
    }
}

impl Stateless for &BedrockConverse<Message> {}

#[async_trait]
impl Stateful<String, Vec<Message>> for &BedrockConverse<Message> {
    async fn set_state(&mut self, state: StateManager<String, Vec<Message>>) {
        self.set_state(state).await;
    }
}

#[async_trait]
impl<T> ToolHandler for BedrockConverse<T>
where
    T: Send + Sync + Clone,
{
    async fn set_tool_registry(&mut self, tool_registry: &RwLock<ToolRegistry>) {
        let registry = tool_registry.read().await;
        let config: ToolConfiguration = (*registry).clone().into();
        self.tool_configuration = Some(config);
    }
}

impl<T: Clone> fmt::Debug for BedrockConverse<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BedrockConverse")
            .field("system_prompt", &self.system_prompt)
            .finish()
    }
}

#[derive(Debug)]
pub struct Claude3Bedrock {
    llm: BedrockConverse<String>,
}

impl Claude3Bedrock {
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
impl Node for Claude3Bedrock {
    type Input = String;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        self.llm.process(input).await
    }
}

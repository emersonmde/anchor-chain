//! Module for integrating OpenAI models.
//!
//! Facilitates the construction and execution of requests to OpenAI models,
//! leveraging the OpenAI API.

use std::fmt;

use anyhow::{anyhow, Context, Result};
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
};
use async_trait::async_trait;

use crate::node::Node;

/// Represents a processor for sending and processing requests to the OpenAI API.
///
/// `OpenAI` encapsulates the functionality required to interact with the
/// the OpenAI API, handling both the construction of requests and the parsing
/// of responses.
pub struct OpenAI<T> {
    system_prompt: String,
    client: async_openai::Client<async_openai::config::OpenAIConfig>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> OpenAI<T> {
    /// Constructs a new `OpenAI` processor with the default API configuration.
    ///
    /// # Parameters
    /// - `system_prompt`: The system prompt or context string.
    pub async fn new(system_prompt: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new();
        let client = async_openai::Client::with_config(config);
        OpenAI {
            system_prompt,
            client,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Constructs a new `OpenAI` processor with a specified API key.
    ///
    /// # Parameters
    /// - `system_prompt`: The system prompt or context string.
    /// - `api_key`: The API key for authenticating with the OpenAI API.
    pub async fn new_with_key(system_prompt: String, api_key: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(api_key);
        let client = async_openai::Client::with_config(config);
        OpenAI {
            system_prompt,
            client,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<T> Node for OpenAI<T>
where
    T: From<String> + Send + Sync,
    ChatCompletionRequestUserMessageContent: From<T> + Send + Sync,
{
    type Input = T;
    type Output = T;

    /// Sends the input to the OpenAI API and processes the response.
    ///
    /// Constructs a request based on the input and the system prompt, then parses
    /// the model's response to extract and return the processed content.
    ///
    /// # Parameters
    /// - `input`: The user input text to be processed by the model.
    ///
    /// # Returns
    /// A `Result` containing the model's response content, or an error if the request fails
    /// or the response lacks expected content.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        let system_prompt = ChatCompletionRequestSystemMessageArgs::default()
            .content(self.system_prompt.clone())
            .build()?
            .into();

        let input = ChatCompletionRequestUserMessageArgs::default()
            .content(input)
            .build()?
            .into();

        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages([system_prompt, input])
            .build()?;

        let response = self.client.chat().create(request).await?;
        if response.choices.is_empty() {
            return Err(anyhow!("No choices in response"));
        }

        let content = response.choices[0]
            .message
            .clone()
            .content
            .context("No content in response")?;

        Ok(content.into())
    }
}

impl<T> fmt::Debug for OpenAI<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAI")
            .field("system_prompt", &self.system_prompt)
            .finish()
    }
}

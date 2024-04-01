//! Module for integrating GPT-3.5 Turbo model interactions.
//!
//! Facilitates the construction and execution of requests to the GPT-3.5 Turbo model,
//! leveraging the OpenAI API. This module is part of a larger framework designed to
//! support asynchronous processing chains for AI and natural language processing tasks.

use anyhow::{anyhow, Context, Result};
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_trait::async_trait;

use crate::link::Processor;

/// Represents a processor for sending and processing requests to GPT-3.5 Turbo.
///
/// `Gpt3_5Turbo` encapsulates the functionality required to interact with the
/// GPT-3.5 Turbo model via the OpenAI API, handling both the construction of requests
/// and the parsing of responses.
pub struct Gpt3_5Turbo {
    system_prompt: String,
    client: async_openai::Client<async_openai::config::OpenAIConfig>,
}

impl Gpt3_5Turbo {
    /// Constructs a new `Gpt3_5Turbo` processor with the default API configuration.
    ///
    /// # Parameters
    /// - `system_prompt`: The system prompt or context string.
    pub async fn new(system_prompt: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new();
        let client = async_openai::Client::with_config(config);
        Gpt3_5Turbo {
            system_prompt,
            client,
        }
    }

    /// Constructs a new `Gpt3_5Turbo` processor with a specified API key.
    ///
    /// # Parameters
    /// - `system_prompt`: The system prompt or context string.
    /// - `api_key`: The API key for authenticating with the OpenAI API.
    pub async fn new_with_key(system_prompt: String, api_key: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(api_key);
        let client = async_openai::Client::with_config(config);
        Gpt3_5Turbo {
            system_prompt,
            client,
        }
    }
}

#[async_trait]
impl Processor for Gpt3_5Turbo {
    type Input = String;
    type Output = String;

    /// Asynchronously sends the input to the GPT-3.5 Turbo model and processes the response.
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

        Ok(content)
    }
}

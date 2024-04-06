//! Module for integrating OpenAI models.
//!
//! Facilitates the construction and execution of requests to OpenAI models,
//! leveraging the OpenAI API.

use std::fmt;

use anyhow::{anyhow, Context, Result};
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
    CreateCompletionRequestArgs, Prompt,
};
use async_trait::async_trait;

use crate::node::Node;

/// OpenAI model types supported by the `OpenAI` node
#[derive(Debug)]
pub enum OpenAIModel<T>
where
    T: Into<Prompt>,
{
    /// GPT-3.5 Turbo model
    GPT3_5Turbo(OpenAIChatModel<T>),
    /// GPT-3.5 Turbo Instruct model
    GPT3_5TurboInstruct(OpenAIInstructModel<T>),
    /// GPT-4 Turbo model
    GPT4Turbo(OpenAIChatModel<T>),
}

impl<T> OpenAIModel<T>
where
    T: Into<Prompt>,
{
    /// Constructs a GPT4 Turbo model with the specified system prompt.
    ///
    /// # Parameters
    /// - `system_prompt`: The system prompt or context string.
    pub async fn new_gpt4_turbo(system_prompt: String) -> Self {
        OpenAIModel::GPT3_5Turbo(
            OpenAIChatModel::new(system_prompt, "gpt-4-turbo-preview".to_string()).await,
        )
    }

    /// Constructs a GPT3.5 Turbo model with the specified system prompt.
    ///
    /// # Parameters
    /// - `system_prompt`: The system prompt or context string.
    pub async fn new_gpt3_5_turbo(system_prompt: String) -> Self {
        OpenAIModel::GPT4Turbo(
            OpenAIChatModel::new(system_prompt, "gpt-3.5-turbo".to_string()).await,
        )
    }

    /// Constructs a GPT3.5 Turbo Instruct model with the specified system prompt.
    ///
    /// # Parameters
    /// - `system_prompt`: The system prompt or context string.
    pub async fn new_gpt3_5_turbo_instruct() -> Self {
        OpenAIModel::GPT3_5TurboInstruct(
            OpenAIInstructModel::new("gpt-3.5-turbo-instruct-0914".to_string()).await,
        )
    }
}

#[async_trait]
impl<T> Node for OpenAIModel<T>
where
    T: Send + Sync + fmt::Debug,
    T: Into<Prompt> + Into<ChatCompletionRequestUserMessageContent>,
{
    type Input = T;
    type Output = String;

    /// Sends the input to the OpenAI model and processes the response.
    ///
    /// # Parameters
    /// - `input`: The user input text to be processed by the model.
    /// # Returns
    /// A `Result` containing the model's response content, or an error if the request fails.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        match self {
            OpenAIModel::GPT3_5Turbo(model) => model.process(input).await,
            OpenAIModel::GPT4Turbo(model) => model.process(input).await,
            OpenAIModel::GPT3_5TurboInstruct(model) => model.process(input).await,
        }
    }
}

/// Represents a processor for sending and processing requests to the OpenAI API.
///
/// `OpenAIChatModel` encapsulates the functionality required to interact with
/// the the OpenAI Chat API, handling both the construction of requests and the
/// parsing of responses.
pub struct OpenAIChatModel<T> {
    system_prompt: String,
    model: String,
    client: async_openai::Client<async_openai::config::OpenAIConfig>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> OpenAIChatModel<T> {
    /// Constructs a new `OpenAI` processor with the default API configuration.
    ///
    /// Possible Model Types
    /// gpt-3.5-turbo-16k
    /// davinci-002
    /// gpt-3.5-turbo-1106
    /// whisper-1
    /// dall-e-2
    /// tts-1-hd-1106
    /// tts-1-hd
    /// gpt-4-vision-preview
    /// text-embedding-3-large
    /// gpt-3.5-turbo-0125
    /// gpt-4-turbo-preview
    /// gpt-3.5-turbo-0301
    /// gpt-4-1106-preview
    /// gpt-3.5-turbo
    /// gpt-3.5-turbo-instruct-0914
    /// gpt-4-0613
    /// gpt-4-1106-vision-preview
    /// tts-1
    /// dall-e-3
    /// babbage-002
    /// tts-1-1106
    /// gpt-4
    /// gpt-4-0125-preview
    /// text-embedding-3-small
    /// gpt-3.5-turbo-0613
    /// text-embedding-ada-002
    /// gpt-3.5-turbo-instruct
    /// gpt-3.5-turbo-16k-0613
    ///
    /// # Parameters
    /// - `system_prompt`: The system prompt or context string.
    /// - `model`: The OpenAI model to use for processing.
    async fn new(system_prompt: String, model: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new();
        let client = async_openai::Client::with_config(config);
        OpenAIChatModel {
            system_prompt,
            client,
            model,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Constructs a new `OpenAI` processor with a specified API key.
    ///
    /// # Parameters
    /// - `system_prompt`: The system prompt or context string.
    /// - `api_key`: The API key for authenticating with the OpenAI API.
    pub async fn new_with_key(system_prompt: String, model: String, api_key: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(api_key);
        let client = async_openai::Client::with_config(config);
        OpenAIChatModel {
            system_prompt,
            client,
            model,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<T> Node for OpenAIChatModel<T>
where
    T: Into<ChatCompletionRequestUserMessageContent> + Send + Sync,
{
    type Input = T;
    type Output = String;

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
            .model(&self.model)
            .messages([system_prompt, input])
            .build()?;

        let response = self.client.chat().create(request).await?;
        if response.choices.is_empty() {
            return Err(anyhow!("No choices in response"));
        }

        let content = response
            .choices
            .first()
            .context("No content in response")?
            .message
            .clone()
            .content
            .context("No content in response")?;

        Ok(content)
    }
}

impl<T> fmt::Debug for OpenAIChatModel<T> {
    /// Formats the `OpenAI` processor for debugging.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAI")
            .field("system_prompt", &self.system_prompt)
            .finish()
    }
}

/// Processor for making requests to OpenAI Instruct models.
pub struct OpenAIInstructModel<T>
where
    T: Into<Prompt>,
{
    /// The name of the instruct model.
    model: String,
    /// The OpenAI API client.
    client: async_openai::Client<async_openai::config::OpenAIConfig>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> OpenAIInstructModel<T>
where
    T: Into<Prompt>,
{
    /// Constructs a new `OpenAI` processor with the default API configuration.
    ///
    /// # Parameters
    /// - `model`: The OpenAI model to use for processing.
    /// # Returns
    /// A new `OpenAI` processor instance.
    async fn new(model: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new();
        let client = async_openai::Client::with_config(config);
        OpenAIInstructModel {
            client,
            model,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Constructs a new `OpenAI` processor with a specified API key.
    ///
    /// # Parameters
    /// - `system_prompt`: The system prompt or context string.
    /// - `api_key`: The API key for authenticating with the OpenAI API.
    /// # Returns
    /// A new `OpenAI` processor instance.
    pub async fn new_with_key(model: String, api_key: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(api_key);
        let client = async_openai::Client::with_config(config);
        OpenAIInstructModel {
            client,
            model,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<T> Node for OpenAIInstructModel<T>
where
    T: Into<Prompt> + Send + Sync,
{
    type Input = T;
    type Output = String;

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
        let request = CreateCompletionRequestArgs::default()
            .model(&self.model)
            .prompt(input)
            .temperature(0.8)
            .max_tokens(512u16)
            .build()?;

        let response = self.client.completions().create(request).await?;

        let content = response
            .choices
            .first()
            .context("No content in response")?
            .text
            .clone();

        Ok(content)
    }
}

impl<T> fmt::Debug for OpenAIInstructModel<T>
where
    T: Into<Prompt>,
{
    /// Formats the `OpenAI` processor for debugging.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAI").finish()
    }
}

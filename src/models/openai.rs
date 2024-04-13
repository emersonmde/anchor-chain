//! Module for integrating OpenAI models.
//!
//! Facilitates the construction and execution of requests to OpenAI models,
//! leveraging the OpenAI API.

use std::fmt;

use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
    CreateCompletionRequestArgs, CreateEmbeddingRequestArgs, Prompt,
};
use async_trait::async_trait;
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::error::AnchorChainError;
use crate::models::embedding_model::EmbeddingModel;
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
    /// The system prompt is passed in as the first message in the conversation
    /// using `ChatCompletionRequestSystemMessage`.
    pub async fn new_gpt4_turbo(system_prompt: String) -> Self {
        OpenAIModel::GPT3_5Turbo(
            OpenAIChatModel::new(system_prompt, "gpt-4-turbo-preview".to_string()).await,
        )
    }

    /// Constructs a GPT3.5 Turbo model with the specified system prompt.
    ///
    /// The system prompt is passed in as the first message in the conversation
    /// using `ChatCompletionRequestSystemMessage`.
    pub async fn new_gpt3_5_turbo(system_prompt: String) -> Self {
        OpenAIModel::GPT4Turbo(
            OpenAIChatModel::new(system_prompt, "gpt-3.5-turbo".to_string()).await,
        )
    }

    /// Constructs a GPT3.5 Turbo Instruct model.
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
    /// The input that is converted to a `Prompt` for the OpenAI model.
    type Input = T;
    /// The output from the OpenAI model.
    type Output = String;

    /// Sends the prompt to the OpenAI model and processes the response.
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
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
/// the OpenAI Chat API, handling both the construction of requests and the
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
    /// The OpenAIConfig will try to use the API key from the environment
    /// variable `OPENAI_API_KEY` by default. The system prompt is passed in
    /// as the first message in the conversation using
    /// `ChatCompletionRequestSystemMessage`.
    ///
    /// Possible Model Types:
    /// gpt-3.5-turbo-16k
    /// davinci-002
    /// gpt-3.5-turbo-1106
    /// whisper-1
    /// dall-e-2
    /// tts-1-hd-1106
    /// tts-1-hd
    /// gpt-4-vision-preview
    /// gpt-3.5-turbo-0125
    /// gpt-4-turbo-preview
    /// gpt-3.5-turbo-0301
    /// gpt-4-1106-preview
    /// gpt-3.5-turbo
    /// gpt-4-0613
    /// gpt-4-1106-vision-preview
    /// tts-1
    /// dall-e-3
    /// babbage-002
    /// tts-1-1106
    /// gpt-4
    /// gpt-4-0125-preview
    /// gpt-3.5-turbo-0613
    /// gpt-3.5-turbo-16k-0613
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

    /// Constructs a new `OpenAI` node using the specified API key.
    ///
    /// The system prompt is passed in as the first message in the conversation
    /// using `ChatCompletionRequestSystemMessage`.
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
    T: Into<ChatCompletionRequestUserMessageContent> + fmt::Debug + Send + Sync,
{
    type Input = T;
    type Output = String;

    /// Sends the input to the OpenAI API and processes the response.
    ///
    /// Constructs a request based on the input and the system prompt, then parses
    /// the model's response to extract and return final output.
    #[cfg_attr(feature = "tracing", instrument(skip(self), fields(model = self.model.as_str(), system_prompt = self.system_prompt.as_str())))]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
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
            return Err(AnchorChainError::EmptyResponseError);
        }

        let content = response
            .choices
            .first()
            .ok_or(AnchorChainError::EmptyResponseError)?
            .message
            .clone()
            .content
            .ok_or(AnchorChainError::EmptyResponseError)?;

        Ok(content)
    }
}

impl<T> fmt::Debug for OpenAIChatModel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAI")
            .field("system_prompt", &self.system_prompt)
            .finish()
    }
}

/// Node for making requests to OpenAI Instruct models.
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
    /// Constructs a new `OpenAI` node with the default API configuration.
    ///
    /// The model specified must support the instruct API.
    ///
    /// Possible Model Types:
    /// gpt-3.5-turbo-instruct
    /// gpt-3.5-turbo-instruct-0914
    #[allow(dead_code)]
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
    /// The model specified must support the instruct API.
    #[allow(dead_code)]
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
    T: Into<Prompt> + fmt::Debug + Send + Sync,
{
    type Input = T;
    type Output = String;

    /// Sends the input to the OpenAI API and processes the response.
    ///
    /// Constructs a request based on the input and the system prompt, then parses
    /// the model's response to extract and return the processed content.
    #[cfg_attr(feature = "tracing", instrument(skip(self), fields(model = self.model.as_str())))]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
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
            .ok_or(AnchorChainError::EmptyResponseError)?
            .text
            .clone();

        Ok(content)
    }
}

impl<T> fmt::Debug for OpenAIInstructModel<T>
where
    T: Into<Prompt>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAI").finish()
    }
}

/// Node for making requests to OpenAI embedding models.
pub struct OpenAIEmbeddingModel {
    /// The name of the instruct model.
    model: String,
    /// The OpenAI API client.
    client: async_openai::Client<async_openai::config::OpenAIConfig>,
}

impl Default for OpenAIEmbeddingModel {
    fn default() -> Self {
        OpenAIEmbeddingModel {
            model: "text-embedding-3-large".to_string(),
            client: async_openai::Client::with_config(async_openai::config::OpenAIConfig::new()),
        }
    }
}

impl OpenAIEmbeddingModel {
    /// Constructs a new `OpenAI` node with the default API configuration.
    ///
    /// The model specified must support the instruct API.
    ///
    /// Possible Model Types:
    /// text-embedding-3-large
    /// text-embedding-3-small
    /// text-embedding-ada-002
    #[allow(dead_code)]
    async fn new(model: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new();
        let client = async_openai::Client::with_config(config);
        OpenAIEmbeddingModel { client, model }
    }

    /// Constructs a new `OpenAI` processor with a specified API key.
    ///
    /// The model specified must support the embedding API.
    #[allow(dead_code)]
    async fn new_with_key(model: String, api_key: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(api_key);
        let client = async_openai::Client::with_config(config);
        OpenAIEmbeddingModel { client, model }
    }
}

#[async_trait]
impl Node for OpenAIEmbeddingModel {
    type Input = Vec<String>;
    type Output = Vec<Vec<f32>>;

    /// Sends the input to the OpenAI API and processes the response.
    ///
    /// Constructs a request based on the input and the system prompt, then parses
    /// the model's response to extract and return the processed content.
    #[cfg_attr(feature = "tracing", instrument(skip(self), fields(model = self.model.as_str())))]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        let request = CreateEmbeddingRequestArgs::default()
            .model(&self.model)
            .input(input)
            .build()?;

        let response = self.client.embeddings().create(request).await?;

        Ok(response
            .data
            .iter()
            .map(|data| data.embedding.clone())
            .collect())
    }
}

#[async_trait]
impl EmbeddingModel for OpenAIEmbeddingModel {
    #[cfg_attr(feature = "tracing", instrument(skip(self), fields(model = self.model.as_str())))]
    async fn embed(&self, input: String) -> Result<Vec<f32>, AnchorChainError> {
        self.process(vec![input])
            .await?
            .first()
            .ok_or(AnchorChainError::EmptyResponseError)
            .cloned()
    }
}

impl fmt::Debug for OpenAIEmbeddingModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAI").finish()
    }
}

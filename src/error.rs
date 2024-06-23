//! Defines error types for Anchor Chain.

#[cfg(feature = "openai")]
use async_openai::error::OpenAIError;
#[cfg(feature = "bedrock")]
use aws_sdk_bedrockruntime::error::SdkError;
#[cfg(feature = "bedrock")]
use aws_sdk_bedrockruntime::operation::converse::ConverseError;
#[cfg(feature = "bedrock")]
use aws_sdk_bedrockruntime::operation::invoke_model::InvokeModelError;

/// Defines errors types for Anchor Chain
#[derive(Debug, thiserror::Error)]
pub enum AnchorChainError {
    /// Occurs when failing to construct OpenAI prompts, messages or when invoking
    /// the model fails.
    #[cfg(feature = "openai")]
    #[error("OpenAI error: {0}")]
    OpenAIError(#[from] OpenAIError),

    /// Occurs when failing to construct or invoke a model in Bedrock.
    #[cfg(feature = "bedrock")]
    #[error("Bedrock error: {0}")]
    BedrockError(#[from] SdkError<InvokeModelError>),

    #[cfg(feature = "bedrock")]
    #[error("Bedrock Converse Error: {0}")]
    BedrockConverse(#[from] SdkError<ConverseError>),

    /// Error constructing or rendering Tera templates.
    #[error("error constructing or rendering Tera template: {0}")]
    TeraTemplateError(#[from] tera::Error),

    /// Error when no response is returned from the LLM model.
    #[error("no response returned from the model")]
    EmptyResponseError,

    /// Generic error that occurs when serializing or deserializing a request.
    #[error("error processing request: {0}")]
    RequestError(#[from] serde_json::Error),

    /// Error when configuring or using OpenSearch.
    #[cfg(feature = "opensearch")]
    #[error("OpenSearch error: {0}")]
    OpenSearchError(#[from] opensearch::Error),

    /// Error that occurs when calling OpenSearch APIs.
    #[cfg(feature = "opensearch")]
    #[error("OpenSearch returned error: {0}")]
    OpenSearchInternalError(String),

    #[error("Error parsing input: {0}")]
    ParseError(String),

    #[error("invalid input: {0}")]
    InvalidInputError(String),

    /// Generic error calling a model.
    #[error("error processing model response: {0}")]
    ModelError(String),

    // Reqwest error
    #[cfg(feature = "ollama")]
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

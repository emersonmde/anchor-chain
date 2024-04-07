//! Defines error types for Anchor Chain.

use async_openai::error::OpenAIError;
use aws_sdk_bedrockruntime::error::SdkError;
use aws_sdk_bedrockruntime::operation::invoke_model::InvokeModelError;

/// Defines errors types for Anchor Chain
#[derive(Debug, thiserror::Error)]
pub enum AnchorChainError {
    /// General error returned in ParallelNode when the combination function fails.
    #[error("error processing node: {0}")]
    ParallelNodeError(#[from] anyhow::Error),

    /// Occurs when failing to construct OpenAI prompts, messages or when invoking
    /// the model fails.
    #[error("OpenAI error: {0}")]
    OpenAIError(#[from] OpenAIError),

    /// Occurs when failing to construct or invoke a model in Bedrock.
    #[error("Bedrock error: {0}")]
    BedrockError(#[from] SdkError<InvokeModelError>),

    /// Error constructing or rendering Tera templates.
    #[error("error constructing or rendering Tera template: {0}")]
    TeraTemplateError(#[from] tera::Error),

    /// Error when no response is returned from the LLM model.
    #[error("no response returned from the model")]
    EmptyResponseError,

    /// Generic error that occurs when processing a request.
    #[error("error processing request: {0}")]
    RequestError(#[from] serde_json::Error),
}

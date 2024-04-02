//! Module for interfacing with Claude 3 via AWS Bedrock.
//!
//! Provides the functionality to construct and send requests to Claude 3 models hosted
//! on AWS Bedrock, facilitating integration of LLM processing within
//! processing chains. This module is designed to handle text and image inputs, offering a
//! flexible interface for various types of content.

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use aws_sdk_bedrockruntime::{primitives::Blob, Client};
use serde::{Deserialize, Serialize};

use crate::node::Node;

/// Represents a source of an image to be processed by Claude 3, encapsulating the necessary
/// details for image handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeImageSource {
    /// Specifies the data type of the source, currently only "base64" is supported.
    #[serde(rename = "type")]
    source_type: String,

    /// Indicates the media type of the image, e.g., "image/jpeg".
    media_type: String,

    /// Contains the base64-encoded image data.
    data: String,
}

/// Defines the content of a message for Claude 3, accommodating text and images.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMessageContent {
    /// The content type, e.g., "text".
    #[serde(rename = "type")]
    pub content_type: String,

    /// The actual text content, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// An image source, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ClaudeImageSource>,
}

/// Represents a message to be sent to Claude 3, comprising one or more content items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMessage {
    /// The role of the message, e.g., "user".
    pub role: Option<String>,

    /// A vector of content items within the message.
    pub content: Vec<ClaudeMessageContent>,
}

/// Struct to configure and send a request to Claude 3 model via AWS Bedrock.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClaudeMessagesRequest {
    /// Specifies the version of the anthropic model to use.
    anthropic_version: String,
    /// Sets the maximum number of tokens to generate.
    max_tokens: i32,
    /// Contains the messages to process.
    messages: Vec<ClaudeMessage>,

    // Optional parameters for model invocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
}

/// Holds the response content from a Claude 3 processing request.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClaudeMessagesResponse {
    /// The processed content returned by Claude.
    content: Vec<ClaudeMessageContent>,
}

/// A processor for integrating Claude 3 LLM processing within a chain.
///
/// `Claude3Bedrock` allows for sending requests to Claude 3 models, handling both text and image inputs.
/// It encapsulates the necessary details for AWS Bedrock interaction and provides an asynchronous
/// interface for processing content through Claude 3.
pub struct Claude3Bedrock {
    /// The system prompt or context to use for all requests.
    system_prompt: String,
    /// The AWS Bedrock client for sending requests.
    client: aws_sdk_bedrockruntime::Client,
}

impl Claude3Bedrock {
    /// Constructs a new `Claude3Bedrock` processor with the specified system prompt.
    ///
    /// Initializes the AWS Bedrock client using the environment's AWS configuration.
    pub async fn new(system_prompt: String) -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Claude3Bedrock {
            client,
            system_prompt,
        }
    }
}

#[async_trait]
impl Node for Claude3Bedrock {
    type Input = String;
    type Output = String;

    /// Processes the input through the Claude 3 model, returning the model's output.
    ///
    /// Constructs a request to the Claude 3 model with the provided input, sends it via
    /// AWS Bedrock, and extracts the text content from the response.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        dbg!("Claude3Bedrock processing input");
        let request = ClaudeMessagesRequest {
            anthropic_version: "bedrock-2023-05-31".to_string(),
            max_tokens: 512,
            messages: vec![ClaudeMessage {
                role: Some("user".to_string()),
                content: vec![ClaudeMessageContent {
                    content_type: "text".to_string(),
                    text: Some(input.to_string()),
                    source: None,
                }],
            }],
            system: Some(self.system_prompt.clone()),
            temperature: None,
            top_p: None,
            top_k: None,
            stop_sequences: None,
        };

        let body_blob = Blob::new(serde_json::to_string(&request)?);
        let response = self
            .client
            .invoke_model()
            .model_id("anthropic.claude-3-sonnet-20240229-v1:0")
            .body(body_blob)
            .content_type("application/json")
            .send()
            .await;

        let response_blob = response?.body;
        let response: ClaudeMessagesResponse = serde_json::from_slice(&response_blob.into_inner())?;

        if response.content.is_empty() {
            return Err(anyhow!("No content in response"));
        }

        dbg!("Claude3Bedrock processing complete");
        Ok(response.content[0]
            .text
            .clone()
            .expect("No text in response"))
    }
}

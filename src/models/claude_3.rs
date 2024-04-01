use anyhow::{anyhow, Result};
use async_trait::async_trait;
use aws_sdk_bedrockruntime::{primitives::Blob, Client};
use serde::{Deserialize, Serialize};

use crate::link::Processor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeImageSource {
    // Data type, only "base64" is supported
    #[serde(rename = "type")]
    source_type: String,
    // Image type, e.g. "image/jpeg"
    media_type: String,
    // Base64-encoded image data
    data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMessageContent {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ClaudeImageSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMessage {
    pub role: Option<String>,
    pub content: Vec<ClaudeMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClaudeMessagesRequest {
    anthropic_version: String,
    max_tokens: i32,
    messages: Vec<ClaudeMessage>,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClaudeMessagesResponse {
    content: Vec<ClaudeMessageContent>,
}

pub struct Claude3Bedrock {
    system_prompt: String,
    client: aws_sdk_bedrockruntime::Client,
}

impl Claude3Bedrock {
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
impl Processor for Claude3Bedrock {
    type Input = String;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
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

        Ok(response.content[0]
            .text
            .clone()
            .expect("No text in response"))
    }
}

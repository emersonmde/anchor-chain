use anyhow::{anyhow, Result};
use async_trait::async_trait;
use aws_sdk_bedrockruntime::{primitives::Blob, Client};
use serde::{Deserialize, Serialize};

use crate::Link;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnthropicMessageContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnthropicMessage {
    role: Option<String>,
    content: Vec<AnthropicMessageContent>,
}

/*
* Structure that serializes to the following JSON:
```json
{
    "anthropic_version": "bedrock-2023-05-31",
    "max_tokens": int,
    "system": string,
    "messages": [
        {
            "role": string,
            "content": [
                { "type": "image", "source": { "type": "base64", "media_type": "image/jpeg", "data": "content image bytes" } },
                { "type": "text", "text": "content text" }
      ]
        }
    ],
    "temperature": float,
    "top_p": float,
    "top_k": int,
    "stop_sequences": [string]
}
```
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnthropicMessagesRequest {
    anthropic_version: String,
    max_tokens: i32,
    messages: Vec<AnthropicMessage>,

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
struct AnthropicMessagesResponse {
    content: Vec<AnthropicMessageContent>,
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
impl Link for Claude3Bedrock {
    async fn run(&self, input: &str) -> Result<String> {
        let request = AnthropicMessagesRequest {
            anthropic_version: "bedrock-2023-05-31".to_string(),
            max_tokens: 512,
            messages: vec![AnthropicMessage {
                role: Some("user".to_string()),
                content: vec![AnthropicMessageContent {
                    content_type: "text".to_string(),
                    text: input.to_string(),
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
        let response: AnthropicMessagesResponse =
            serde_json::from_slice(&response_blob.into_inner())?;

        if response.content.is_empty() {
            return Err(anyhow!("No content in response"));
        }

        Ok(response.content[0].text.clone())
    }
}

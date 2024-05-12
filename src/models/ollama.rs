use crate::{AnchorChainError, Node};
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Ollama {
    model: String,
    client: reqwest::Client,
}

impl Ollama {
    pub async fn new(model: String) -> Self {
        let client = reqwest::Client::new();
        Ollama { model, client }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    context: Option<Vec<u64>>,
    total_duration: Option<u64>,
    load_duration: Option<u64>,
    prompt_eval_duration: Option<u64>,
    eval_count: Option<u64>,
    eval_duration: Option<u64>,
}

#[async_trait]
impl Node for Ollama {
    type Input = String;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        let url = "http://localhost:11434/api/generate";
        let body = serde_json::json!({
            "model": self.model,
            "prompt": input,
        });
        let response = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(AnchorChainError::from)?;

        let response_text = response.text().await.map_err(AnchorChainError::from)?;

        let reader = BufReader::new(response_text.as_bytes());
        let mut output = String::new();

        for line in reader.lines() {
            let line = line.map_err(|e| AnchorChainError::ParseError(e.to_string()))?;
            let api_response: OllamaResponse =
                serde_json::from_str(&line).map_err(AnchorChainError::from)?;
            output.push_str(&api_response.response);
        }

        Ok(output)
    }
}

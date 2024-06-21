//! Module for interfacing with Ollama models via the Ollama API.
//!
//! Provides the functionality to construct and send requests to Ollama via the
//! Ollama API. Ollama is a tool for managing and running local LLMs. For more
//! information on how to install and run Ollama, see [https://ollama.com](https://ollama.com/).
use crate::{AnchorChainError, Node};
use anchor_chain_macros::Stateless;
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::{BufRead, BufReader};

/// Struct for interfacing with Ollama models via the Ollama API.
#[derive(Debug, Clone, Stateless)]
pub struct Ollama {
    /// The model tag of the installed Ollama model to use.
    model: String,
    /// The base URL of the Ollama API.
    url: String,
    /// HTTP client for sending requests to the Ollama API.
    client: reqwest::Client,
}

impl Ollama {
    /// Creates a new Ollama instance with the specified model.
    ///
    /// The model must already be present in the Ollama instance otherwise
    /// a `missing field 'model'` error will be returned. For a list of
    /// available models, see [Ollama Models](https://ollama.com/library).
    /// To download a model use `ollama pull <model_tag>`.
    pub fn new(model: &str, host: &str, port: &str) -> Self {
        let model = model.to_string();
        let client = reqwest::Client::new();
        Ollama {
            model,
            url: format!("http://{}:{}/api/generate", host, port),
            client,
        }
    }

    /// Creates a new Ollama instance with the specified model and the default
    /// Ollama API URL `http://localhost:11434/api/generate`.
    pub fn new_with_defaults(model: &str) -> Self {
        Self::new(model, "localhost", "11434")
    }
}

/// Struct representing the response from the Ollama chat completion API
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

    /// Processes the input through the Ollama model, returning the model's output.
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        let body = serde_json::json!({
            "model": self.model,
            "prompt": input,
        });
        let response = self
            .client
            .post(&self.url)
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

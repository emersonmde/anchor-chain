#![allow(dead_code)]

use anyhow::Result;
use async_trait::async_trait;
use std::ops::BitOr;

#[async_trait]
pub trait Link {
    async fn run(&self, input: &str) -> String;
}

pub struct Chain {
    links: Vec<Box<dyn Link>>,
}

impl Chain {
    pub fn new() -> Self {
        Chain { links: Vec::new() }
    }

    fn add_link<L: Link + 'static>(mut self, link: L) -> Self {
        self.links.push(Box::new(link));
        self
    }

    pub async fn run(self, input: String) -> String {
        let mut result = input;
        for link in self.links {
            result = link.run(&result).await;
        }
        result
    }
}

impl Default for Chain {
    fn default() -> Self {
        Chain::new()
    }
}

impl<L: Link + 'static> BitOr<L> for Chain {
    type Output = Chain;

    fn bitor(self, link: L) -> Chain {
        self.add_link(link)
    }
}

// pub enum LanguageModel {
//     Gpt3_5Turbo { api_key: String },
//     BedrockClaude2_1 { aws_profile: String },
// }
//
// impl LanguageModel {
//     pub fn new_gpt3_5_turbo(api_key: String) -> Self {
//         LanguageModel::Gpt3_5Turbo { api_key }
//     }
//
//     pub fn new_bedrock_claude2_1(aws_profile: String) -> Self {
//         LanguageModel::BedrockClaude2_1 { aws_profile }
//     }
// }

pub struct Gpt3_5Turbo {
    system_prompt: String,
    client: async_openai::Client<async_openai::config::OpenAIConfig>,
}

impl Gpt3_5Turbo {
    pub async fn new(system_prompt: String) -> Result<Self> {
        let config = async_openai::config::OpenAIConfig::new();
        let client = async_openai::Client::with_config(config);
        Ok(Gpt3_5Turbo {
            system_prompt,
            client,
        })
    }

    pub async fn new_with_key(system_prompt: String, api_key: String) -> Result<Self> {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(api_key);
        let client = async_openai::Client::with_config(config);
        Ok(Gpt3_5Turbo {
            system_prompt,
            client,
        })
    }
}

#[async_trait]
impl Link for Gpt3_5Turbo {
    async fn run(&self, input: &str) -> String {
        let system_prompt = async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
            .content(self.system_prompt.clone())
            .build()
            .expect("Failed to build system prompt")
            .into();

        let input = async_openai::types::ChatCompletionRequestUserMessageArgs::default()
            .content(input)
            .build()
            .expect("Failed to build user input")
            .into();

        let request = async_openai::types::CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages([system_prompt, input])
            .build()
            .expect("Failed to build request");

        println!("{}", serde_json::to_string(&request).unwrap());

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .expect("Failed to create chat completion request");

        println!("\nResponse:\n");
        // TODO: Fix unwrap (currently returnning empty string consistently)
        response.choices[0]
            .message
            .clone()
            .content
            .unwrap_or("".to_string())
    }
}

pub struct Prompt {
    text: String,
}

impl Prompt {
    pub fn new(text: &str) -> Self {
        Prompt {
            text: text.to_string(),
        }
    }
}

#[async_trait]
impl Link for Prompt {
    async fn run(&self, input: &str) -> String {
        // TODO: Take hashmap for parameterized input
        println!("Prompt {} {}", self.text, input);
        format!("Prompt {} {}", self.text, input)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test() {
//         let chain = Chain::new()
//             | Prompt::new("Hello".to_string())
//             | LanguageModel::new_gpt3_5_turbo("api_key".to_string());
//
//         chain.run("Test".to_string());
//     }
// }

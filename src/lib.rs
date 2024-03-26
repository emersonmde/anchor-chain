#![allow(dead_code)]

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use std::ops::BitOr;

#[async_trait]
pub trait Link {
    async fn run(&self, input: &str) -> Result<String>;
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

    pub async fn run(self, input: String) -> Result<String> {
        let mut result = input;
        for link in self.links {
            result = link.run(&result).await?;
        }

        Ok(result)
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
    async fn run(&self, input: &str) -> Result<String> {
        let system_prompt = async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
            .content(self.system_prompt.clone())
            .build()?
            .into();

        let input = async_openai::types::ChatCompletionRequestUserMessageArgs::default()
            .content(input)
            .build()?
            .into();

        let request = async_openai::types::CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages([system_prompt, input])
            .build()?;

        let response = self.client.chat().create(request).await?;
        if response.choices.is_empty() {
            return Err(anyhow!("No choices in response"));
        }

        let content = response.choices[0]
            .message
            .clone()
            .content
            .context("No content in response")?;

        Ok(content)
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
    async fn run(&self, input: &str) -> Result<String> {
        // TODO: Take hashmap for parameterized input
        println!("Prompt {} {}", self.text, input);
        Ok(format!("Prompt {} {}", self.text, input))
    }
}

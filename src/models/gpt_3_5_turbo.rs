use anyhow::{anyhow, Context, Result};
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_trait::async_trait;

use crate::Link;

pub struct Gpt3_5Turbo {
    system_prompt: String,
    client: async_openai::Client<async_openai::config::OpenAIConfig>,
}

impl Gpt3_5Turbo {
    pub async fn new(system_prompt: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new();
        let client = async_openai::Client::with_config(config);
        Gpt3_5Turbo {
            system_prompt,
            client,
        }
    }

    pub async fn new_with_key(system_prompt: String, api_key: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(api_key);
        let client = async_openai::Client::with_config(config);
        Gpt3_5Turbo {
            system_prompt,
            client,
        }
    }
}

#[async_trait]
impl Link for Gpt3_5Turbo {
    async fn run(&self, input: &str) -> Result<String> {
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

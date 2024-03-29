use async_trait::async_trait;

use crate::Link;
use anyhow::Result;

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
        let result = self.text.replace("{input}", input);
        println!("Prompt: {}", result);
        Ok(result)
    }
}

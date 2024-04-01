use async_trait::async_trait;

use crate::link::Processor;
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
impl Processor for Prompt {
    type Input = String;
    type Output = String;
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        println!("{}", self.text);
        Ok(input)
    }
}

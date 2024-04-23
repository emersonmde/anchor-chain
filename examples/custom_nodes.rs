use std::collections::HashMap;

use anchor_chain::AnchorChainError;
use anchor_chain::{ChainBuilder, NoOpNode, Node, OpenAIModel, Prompt};
use async_trait::async_trait;

#[derive(Debug, Default)]
pub struct LineCounter;

impl LineCounter {
    pub fn new() -> Self {
        LineCounter
    }
}

#[async_trait]
impl Node for LineCounter {
    type Input = String;
    type Output = usize;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        Ok(input.lines().count())
    }
}

#[derive(Debug, Default)]
pub struct AsteriskGenerator;

impl AsteriskGenerator {
    pub fn new() -> Self {
        AsteriskGenerator
    }
}

#[async_trait]
impl Node for AsteriskGenerator {
    type Input = usize;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        Ok("*".repeat(input))
    }
}

#[tokio::main]
async fn main() {
    let llm = OpenAIModel::new_gpt4_turbo("You are a helpful assistant".to_string()).await;

    let chain = ChainBuilder::new()
        .link(Prompt::new("{{ input }}"))
        .link(llm)
        .link(NoOpNode::new())
        .link(LineCounter::new())
        .link(AsteriskGenerator::new())
        .link(NoOpNode::new())
        .build();

    let output = chain
        .process(HashMap::from([(
            "input".to_string(),
            "Write a hello world program in Rust".to_string(),
        )]))
        .await
        .expect("Failed to process chain");
    println!("Output:\n{}", output);
}

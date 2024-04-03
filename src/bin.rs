#![allow(unused_imports)]
#![allow(dead_code)]

extern crate anchor_chain;

use anchor_chain::{
    chain::ChainBuilder,
    models::{claude_3::Claude3Bedrock, gpt_3_5_turbo::Gpt3_5Turbo},
    node::{Node, PassthroughNode},
    parallel_node::ParallelNode,
    prompt::Prompt,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct LineCounter;

impl LineCounter {
    pub fn new() -> Self {
        LineCounter
    }
}

impl Default for LineCounter {
    fn default() -> Self {
        LineCounter::new()
    }
}

#[async_trait]
impl Node for LineCounter {
    type Input = String;
    type Output = usize;

    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        Ok(input.lines().count())
    }
}

pub struct AsteriskGenerator;

impl AsteriskGenerator {
    pub fn new() -> Self {
        AsteriskGenerator
    }
}

impl Default for AsteriskGenerator {
    fn default() -> Self {
        AsteriskGenerator::new()
    }
}

#[async_trait]
impl Node for AsteriskGenerator {
    type Input = usize;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        Ok("*".repeat(input))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let llm = Gpt3_5Turbo::new("You are a helpful assistant".to_string()).await;

    let chain = ChainBuilder::new(Prompt::new("{input}"))
        .with_trace(true)
        .link(llm)
        .link(PassthroughNode::new())
        .link(LineCounter::new())
        .link(AsteriskGenerator::new())
        .link(PassthroughNode::new())
        .build();
    let output = chain
        .process("Write a hello world program in Rust".to_string())
        .await?;
    println!("Output {}", output);

    Ok(())
}

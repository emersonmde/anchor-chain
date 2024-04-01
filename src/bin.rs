#![allow(unused_imports)]
#![allow(dead_code)]

extern crate anchor_chain;

use anyhow::Result;

use anchor_chain::{
    chain::ChainBuilder,
    models::{claude_3::Claude3Bedrock, gpt_3_5_turbo::Gpt3_5Turbo},
    prompt::Prompt,
};

#[tokio::main]
async fn main() -> Result<()> {
    let prompt_processor = Prompt::new("{input}");
    let chain = ChainBuilder::new(prompt_processor)
        .link(Gpt3_5Turbo::new("You are a helpful assistant".to_string()).await)
        .build();

    let result = chain
        .process("Write a hello world program in Rust".to_string())
        .await?;
    println!("Result: {}", result);

    Ok(())
}

#![allow(unused_imports)]
#![allow(dead_code)]

extern crate anchor_chain;

use anyhow::Result;

use anchor_chain::{
    models::{claude_3::Claude3Bedrock, gpt_3_5_turbo::Gpt3_5Turbo},
    prompt::Prompt,
    Chain,
};

#[tokio::main]
async fn main() -> Result<()> {
    // let llm = Gpt3_5Turbo::new("You are a helpful assistant".to_string()).await;
    let llm = Claude3Bedrock::new("You are a helpful assistant".to_string()).await;
    let chain = Chain::new() | Prompt::new("{input}") | llm;

    let result = chain
        .run("Write a hello world program in Rust".to_string())
        .await?;
    println!("Result: {}", result);

    Ok(())
}

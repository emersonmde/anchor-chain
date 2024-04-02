#![allow(unused_imports)]
#![allow(dead_code)]

extern crate anchor_chain;

use anyhow::Result;

use anchor_chain::{
    chain::ChainBuilder,
    link::Link,
    models::{claude_3::Claude3Bedrock, gpt_3_5_turbo::Gpt3_5Turbo},
    node::End,
    parallel_node::{self, ParallelNode},
    prompt::Prompt,
};

#[tokio::main]
async fn main() -> Result<()> {
    let gpt3_link = Box::new(Gpt3_5Turbo::new("You are a helpful assistant".to_string()).await);
    let claude3_link =
        Box::new(Claude3Bedrock::new("You are a helpful assistant".to_string()).await);

    let concat_fn: Box<dyn Fn(Vec<String>) -> Result<String> + Send + Sync> =
        Box::new(|outputs: Vec<String>| {
            println!("Outputs: {:?}", outputs);
            Ok(outputs.concat())
        });

    let parallel_node = ParallelNode::<
        String,
        String,
        Box<dyn Fn(Vec<String>) -> Result<String> + Send + Sync>,
    >::new(vec![gpt3_link, claude3_link], concat_fn);

    let chain = ChainBuilder::new(Prompt::new("{input}"))
        .link(parallel_node)
        .link(End)
        .with_trace(true)
        .build();
    chain
        .process("Write a hello world program in Rust".to_string())
        .await?;

    Ok(())
}

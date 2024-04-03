#![allow(unused_imports)]
#![allow(dead_code)]

use anchor_chain::{
    chain::ChainBuilder,
    models::{claude_3::Claude3Bedrock, openai::OpenAIModel},
    parallel_node::ParallelNode,
    prompt::Prompt,
};
use anyhow::Result;
use async_trait::async_trait;
use futures::future::BoxFuture;

#[tokio::main]
async fn main() -> Result<()> {
    let gpt3 =
        Box::new(OpenAIModel::new_gpt3_5_turbo("You are a helpful assistant".to_string()).await);
    let claude3 = Box::new(Claude3Bedrock::new("You are a helpful assistant".to_string()).await);

    let select_output_fn = Box::new(
        |outputs: Vec<String>| -> BoxFuture<Result<String, anyhow::Error>> {
            Box::pin(async move {
                let decision_chain = ChainBuilder::new()
                .link(Prompt::new("Determine which output is the most helpful and return only that output verbatim.\n\n{input}"))
                .link(OpenAIModel::new_gpt3_5_turbo("You are an expert in rating LLM outputs".to_string()).await)
                .build();
                let labeled_outputs = outputs
                    .iter()
                    .enumerate()
                    .map(|(i, output)| format!("Output {}: ```\n{}\n```", i + 1, output))
                    .collect::<Vec<String>>();
                decision_chain.process(labeled_outputs.join("\n\n")).await
            })
        },
    );

    let chain = ChainBuilder::new()
        .link(Prompt::new("{input}"))
        .link(ParallelNode::new(vec![gpt3, claude3], select_output_fn))
        .build();

    let output = chain
        .process("Write a hello world program in Rust".to_string())
        .await?;
    println!("==================\n\nOutput {}", output);

    Ok(())
}

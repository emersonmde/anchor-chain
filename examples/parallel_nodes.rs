use anchor_chain::{
    chain::ChainBuilder,
    models::{claude_3::Claude3Bedrock, openai::OpenAIModel},
    parallel_node::{to_boxed_future, ParallelNode},
    prompt::Prompt,
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let gpt3 =
        Box::new(OpenAIModel::new_gpt3_5_turbo("You are a helpful assistant".to_string()).await);
    let claude3 = Box::new(Claude3Bedrock::new("You are a helpful assistant".to_string()).await);

    let concat_fn = to_boxed_future(|outputs: Vec<String>| {
        Ok(outputs
            .iter()
            .enumerate()
            .map(|(i, output)| format!("Output {}:\n```\n{}\n```\n", i + 1, output))
            .collect::<Vec<String>>()
            .concat())
    });

    let chain = ChainBuilder::new()
        .link(Prompt::new("{input}"))
        .link(ParallelNode::new(vec![gpt3, claude3], concat_fn))
        .build();

    let output = chain
        .process("Write a hello world program in Rust".to_string())
        .await?;
    println!("{}", output);

    Ok(())
}

use std::collections::HashMap;

use anchor_chain::{chain::ChainBuilder, models::openai::OpenAIModel, prompt::Prompt};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let gpt3 = OpenAIModel::new_gpt3_5_turbo_instruct().await;
    let chain = ChainBuilder::new()
        .link(Prompt::new("{{ input }}"))
        .link(gpt3)
        .build();

    let output = chain
        .process(HashMap::from([(
            "input",
            "Write a hello world program in Rust",
        )]))
        .await?;
    println!("{}", output);

    Ok(())
}
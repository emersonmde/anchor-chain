use std::collections::HashMap;

use anchor_chain::{chain::ChainBuilder, models::openai::OpenAIModel, prompt::Prompt};

#[tokio::main]
async fn main() {
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
        .await
        .expect("Error processing chain");
    println!("{}", output);
}

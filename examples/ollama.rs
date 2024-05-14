use std::collections::HashMap;

use anchor_chain::models::ollama::Ollama;
use anchor_chain::{ChainBuilder, Prompt};

#[tokio::main]
async fn main() {
    let ollama = Ollama::new_with_defaults("llama3");
    let chain = ChainBuilder::new()
        .link(Prompt::new("{{ input }}"))
        .link(ollama)
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

use std::collections::HashMap;

use anchor_chain::models::ollama::Ollama;
use anchor_chain::{ChainBuilder, Prompt};

#[tokio::main]
async fn main() {
    let ollama = Ollama::new("dolphin-mixtral:latest".to_string()).await;
    let chain = ChainBuilder::new()
        .link(Prompt::new("{{ input }}"))
        .link(ollama)
        .build();

    let output = chain
        .process(HashMap::from([(
            "input".to_string(),
            "Write a hello world program in Rust".to_string(),
        )]))
        .await
        .expect("Error processing chain");
    println!("{}", output);
}

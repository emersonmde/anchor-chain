use std::collections::HashMap;
use tracing_subscriber::{fmt, EnvFilter};

use anchor_chain::{ChainBuilder, OpenAIModel, Prompt};

#[tokio::main]
async fn main() {
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed setting default subscriber");

    let gpt3 = OpenAIModel::new_gpt3_5_turbo_instruct().await;
    let chain = ChainBuilder::new()
        .link(Prompt::new("{{ input }}"))
        .link(gpt3)
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

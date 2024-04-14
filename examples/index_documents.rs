use std::collections::HashMap;

use anchor_chain::models::openai::OpenAIEmbeddingModel;
use anchor_chain::vector::opensearch_retriever::OpenSearchRetriever;
use anchor_chain::{chain::ChainBuilder, models::openai::OpenAIModel, prompt::Prompt};

#[tokio::main]
async fn main() {
    let llm = OpenAIModel::new_gpt4_turbo("You are a helpful assistant".to_string()).await;

    let embedding_model = OpenAIEmbeddingModel::default();
    let opensearch_retriever = OpenSearchRetriever::new(
        embedding_model,
        "https://opensearch.co/api/v1/search",
        &["test_index"],
        "vector_field",
        5usize,
    )
    .await
    .expect("Failed to create OpenSearchRetriever");

    let chain = ChainBuilder::new()
        .link(Prompt::new("{{ input }}"))
        .link(opensearch_retriever)
        .link(llm)
        .build();

    let output = chain
        .process(HashMap::from([(
            "input",
            "Write a hello world program in Rust",
        )]))
        .await
        .expect("Failed to process chain");
    println!("Output:\n{}", output);
}

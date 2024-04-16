use std::collections::HashMap;
use std::env;

use anchor_chain::error::AnchorChainError;
use anchor_chain::logger::Logger;
use anchor_chain::models::openai::OpenAIEmbeddingModel;
use anchor_chain::node::{NoOpNode, Node};
use anchor_chain::parallel_node::{to_boxed_future, ParallelNode};
use anchor_chain::vector::document::Document;
use anchor_chain::vector::opensearch_client_builder::OpenSearchClientBuilder;
use anchor_chain::vector::opensearch_retriever::OpenSearchRetriever;
use anchor_chain::{chain::ChainBuilder, models::openai::OpenAIModel, prompt::Prompt};

#[derive(Debug)]
struct DocumentFormatter {}

#[async_trait::async_trait]
impl Node for DocumentFormatter {
    type Input = Vec<Document>;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        Ok(input
            .iter()
            .map(|doc| doc.text.clone())
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

#[tokio::main]
async fn main() {
    let llm = OpenAIModel::new_gpt4_turbo("You are a helpful assistant".to_string()).await;

    let embedding_model = OpenAIEmbeddingModel::default();
    let opensearch_retriever = OpenSearchRetriever::new(
        OpenSearchClientBuilder::new()
            .with_local_connection_without_cert_validation(
                "https://localhost:9200",
                &env::var("OPENSEARCH_USERNAME").expect("OPENSEARCH_USERNAME not set"),
                &env::var("OPENSEARCH_PASSWORD").expect("OPENSEARCH_PASSWORD not set"),
            )
            .build()
            .await
            .expect("Failed to create OpenSearchClient"),
        embedding_model,
        &["test_index"],
        "embedding",
        5usize,
    )
    .await;

    let doc_chain = ChainBuilder::new()
        .link(opensearch_retriever)
        .link(Logger::new("Retrieved documents".to_string()))
        .link(DocumentFormatter {})
        .build();

    let concat_fn = to_boxed_future(|outputs: Vec<String>| {
        let mut map = HashMap::new();
        map.insert("docs".to_string(), outputs[0].clone());
        map.insert("input".to_string(), outputs[1].clone());
        Ok(map)
    });

    let chain = ChainBuilder::new()
        .link(ParallelNode::new(
            vec![Box::new(doc_chain), Box::<NoOpNode<String>>::default()],
            concat_fn,
        ))
        .link(Prompt::new(
            "These common strings people use when building \
        their first Hello World program. Be sure to include one of these strings in the \
        program you write: \n\
        {{ docs }} \n\
        \n \
        {{ input }}",
        ))
        .link(Logger::new("Prompt".to_string()))
        .link(llm)
        .build();

    let output = chain
        .process("Write a basic program in Rust".to_string())
        .await
        .expect("Failed to process chain");
    println!("Output:\n{}", output);
}

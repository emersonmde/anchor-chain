use std::collections::HashMap;
use std::env;

use anchor_chain::{
    to_boxed_future, AnchorChainError, ChainBuilder, Document, Logger, NoOpNode, Node,
    OpenAIEmbeddingModel, OpenAIModel, OpenSearchClientBuilder, OpenSearchRetriever, ParallelNode,
    Prompt,
};

#[derive(Debug)]
struct DocumentFormatter {}

#[async_trait::async_trait]
impl Node for DocumentFormatter {
    type Input = Vec<Document>;
    type Output = &'static str;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        Ok(Box::leak(
            input
                .iter()
                .map(|doc| doc.text.clone())
                .collect::<Vec<_>>()
                .join("\n")
                .into_boxed_str(),
        ))
    }
}

#[tokio::main]
async fn main() {
    let llm = OpenAIModel::new_gpt4_turbo("You are a helpful assistant").await;

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
        .link(Logger::new("Retrieved documents"))
        .link(DocumentFormatter {})
        .build();

    let concat_fn = to_boxed_future(|outputs: Vec<&str>| {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("docs", outputs[0]);
        map.insert("input", outputs[1]);
        Ok(map)
    });

    let chain = ChainBuilder::new()
        .link(ParallelNode::new(
            vec![Box::new(doc_chain), Box::<NoOpNode<&str>>::default()],
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
        .link(Logger::new("Prompt"))
        .link(llm)
        .build();

    let output = chain
        .process("Write a basic program in Rust")
        .await
        .expect("Failed to process chain");
    println!("Output:\n{}", output);
}

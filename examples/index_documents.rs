use anchor_chain::{
    ChainBuilder, Document, OpenAIEmbeddingModel, OpenSearchClientBuilder, OpenSearchIndexer,
};
use std::env;

#[tokio::main]
async fn main() {
    let embedding_model = OpenAIEmbeddingModel::default();
    let open_search_indexer = OpenSearchIndexer::new(
        OpenSearchClientBuilder::new()
            .with_local_connection_without_cert_validation(
                "https://localhost:9200",
                &env::var("OPENSEARCH_USERNAME").expect("OPENSEARCH_USERNAME not set"),
                &env::var("OPENSEARCH_PASSWORD").expect("OPENSEARCH_PASSWORD not set"),
            )
            .build()
            .await
            .expect("Failed to create OpenSearchClient client"),
        embedding_model,
        "test_index",
        "embedding",
    );

    let chain = ChainBuilder::new().link(open_search_indexer).build();

    let docs = vec!["Hello, world!", "Goodbye, world!", "Hello, universe!"];
    let docs: Vec<Document> = docs
        .into_iter()
        .map(|doc| doc.to_string())
        .map(Document::from)
        .collect();

    let result = chain.process(docs).await;
    println!("Output: {:?}", result.expect("Failed to process chain"));
}

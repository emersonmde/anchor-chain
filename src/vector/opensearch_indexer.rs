//! A node for indexing documents into OpenSearch.
//!
//! This node indexes a list of documents into OpenSearch. If the index doesn't exist, it is created
//! with the default settings. Otherwise, the documents are indexed into the existing index.
use std::fmt;

use async_trait::async_trait;
use opensearch::http::request::JsonBody;
use opensearch::indices::{IndicesCreateParts, IndicesExistsParts};
use opensearch::{BulkParts, OpenSearch};
use serde_json::json;
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::error::AnchorChainError;
use crate::models::embedding_model::EmbeddingModel;
use crate::node::Node;
use crate::vector::document::Document;

/// A node for indexing documents into OpenSearch.
#[derive(Debug, Clone)]
pub struct OpenSearchIndexer<M: EmbeddingModel> {
    client: OpenSearch,
    embedding_model: M,
    index: String,
    vector_field: String,
}

impl<M: EmbeddingModel + fmt::Debug> OpenSearchIndexer<M> {
    /// Creates a new `OpenSearchIndexer` with the specified OpenSearch client, embedding model,
    #[allow(dead_code)]
    pub fn new(client: OpenSearch, embedding_model: M, index: &str, vector_field: &str) -> Self {
        Self {
            client,
            embedding_model,
            index: index.to_string(),
            vector_field: vector_field.to_string(),
        }
    }

    /// Checks if an index with the specified name exists in OpenSearch.
    #[cfg_attr(feature = "tracing", instrument(skip(self)))]
    async fn does_index_exist(&self, index: &str) -> Result<bool, AnchorChainError> {
        let response = self
            .client
            .indices()
            .exists(IndicesExistsParts::Index(&[index]))
            .send()
            .await?;
        Ok(response.status_code().is_success())
    }

    /// Creates a vector index in OpenSearch with the specified name using default settings.
    #[cfg_attr(feature = "tracing", instrument(skip(self)))]
    pub async fn create_index(
        &self,
        index: &str,
        vector_field_name: &str,
    ) -> Result<(), AnchorChainError> {
        let body = json!({
            "settings": {
                "index.knn": true
            },
            "mappings": {
                "properties": {
                    vector_field_name: {
                        "type": "knn_vector",
                        "dimension": self.embedding_model.dimensions(),
                        "method": {
                            "name": "hnsw",
                            "space_type": "cosinesimil",
                            "engine": "nmslib",
                            "parameters": {
                                "ef_construction": 128,
                                "m": 16
                            }
                        }
                    }
                }
            }
        });

        let response = self
            .client
            .indices()
            .create(IndicesCreateParts::Index(index))
            .body(body)
            .send()
            .await?;

        if response.status_code().is_success() {
            Ok(())
        } else {
            Err(AnchorChainError::OpenSearchInternalError(
                response.text().await?,
            ))
        }
    }

    /// Automatically indexes a list of documents. It embeds the text into a vector if not already done,
    /// then indexes the entire document into OpenSearch.
    #[cfg_attr(feature = "tracing", instrument(skip(self)))]
    pub async fn index_documents(
        &self,
        mut docs: Vec<Document>,
        index: &str,
    ) -> Result<Vec<Document>, AnchorChainError> {
        let mut operations: Vec<JsonBody<_>> = Vec::with_capacity(docs.len() * 2);

        for doc in &mut docs {
            if doc.embedding.is_none() {
                doc.embedding = Some(
                    self.embedding_model
                        .embed(doc.text.clone())
                        .await
                        .map_err(|e| AnchorChainError::ModelError(e.to_string()))?,
                );
            }

            operations.push(
                json!({
                    "index": {
                        "_index": index,
                        "_id": doc.id,
                    }
                })
                .into(),
            );

            let doc_json = serde_json::to_value(&doc)?;
            operations.push(doc_json.into());
        }

        let response = self
            .client
            .bulk(BulkParts::Index(index))
            .body(operations)
            .send()
            .await
            .map_err(AnchorChainError::OpenSearchError)?;

        if response.status_code().is_success() {
            Ok(docs)
        } else {
            Err(AnchorChainError::OpenSearchInternalError(
                response.text().await?,
            ))
        }
    }
}

#[async_trait]
impl<M: EmbeddingModel + fmt::Debug + Send + Sync> Node for OpenSearchIndexer<M> {
    type Input = Vec<Document>;
    type Output = Vec<Document>;

    /// Indexes a list of documents into OpenSearch.
    ///
    /// If the index doesn't exist, it is created with the default settings. Otherwise,
    /// the documents are indexed into the existing index.

    #[cfg_attr(feature = "tracing", instrument(skip(self)))]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        if !self.does_index_exist(&self.index).await? {
            self.create_index(&self.index, &self.vector_field).await?;
        }

        self.index_documents(input, &self.index).await
    }
}

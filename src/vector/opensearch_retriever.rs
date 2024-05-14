//! OpenSearchRetriever is a Node that retrieves documents from OpenSearch based on input text.
//!
//! The OpenSearchRetriever struct is a Node that retrieves documents from OpenSearch based on
//! input text. It uses an embedding model to embed the input text into a vector, then queries
//! OpenSearch using the vector. The top k documents that are most similar to the input text are
//! returned.
#![allow(dead_code)]

use std::fmt;

use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::Url;
use opensearch::{OpenSearch, SearchParts};
use serde_json::json;
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::error::AnchorChainError;
use crate::models::embedding_model::EmbeddingModel;
use crate::node::Node;
use crate::vector::document::Document;

/// A Node that retrieves documents from OpenSearch based on input text.
#[derive(Debug)]
pub struct OpenSearchRetriever<'a, M: EmbeddingModel> {
    client: OpenSearch,
    embedding_model: M,
    indexes: Vec<String>,
    vector_field: String,
    top_k: usize,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, M: EmbeddingModel + fmt::Debug> OpenSearchRetriever<'a, M> {
    /// Creates a new OpenSearchRetrieverBuilder using default AWS credentials from the environment.
    pub async fn new(
        client: OpenSearch,
        embedding_model: M,
        indexes: &[&str],
        vector_field: &str,
        top_k: usize,
    ) -> Self {
        Self {
            client,
            embedding_model,
            indexes: indexes.iter().map(|s| s.to_string()).collect(),
            vector_field: vector_field.to_string(),
            top_k,
            _marker: std::marker::PhantomData,
        }
    }

    /// Queries OpenSearch for the top k documents that are most similar to the input vector.
    #[cfg_attr(feature = "tracing", instrument(skip(self)))]
    pub async fn vector_query(
        &self,
        indexes: &[String],
        vector_field: &str,
        top_k: usize,
        vector: Vec<f32>,
    ) -> Result<serde_json::Value, AnchorChainError> {
        let indexes = indexes.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        let response = self
            .client
            .search(SearchParts::Index(&indexes))
            .from(0)
            .size(10)
            .body(json!({
                "query": {
                    "knn": {
                        vector_field: {
                            "vector": vector,
                            "k": top_k,
                        }
                    }
                }
            }))
            .send()
            .await?;

        Ok(response.json::<serde_json::Value>().await?)
    }

    /// Retrieves the top k documents from OpenSearch that are most similar to the input text.
    ///
    /// Uses the embedding model to embed the input text into a vector, then queries OpenSearch
    /// using the vector.
    #[cfg_attr(feature = "tracing", instrument)]
    pub async fn retrieve(&self, input: &str) -> Result<Vec<Document>, AnchorChainError> {
        let embedding = self.embedding_model.embed(input.to_string()).await?;
        let response = self
            .vector_query(&self.indexes, &self.vector_field, self.top_k, embedding)
            .await?;
        let empty_vec = Vec::new();
        let hits = response["hits"]["hits"].as_array().unwrap_or(&empty_vec);
        let docs = hits
            .iter()
            .filter_map(|doc| serde_json::from_value(doc["_source"].clone()).ok())
            .collect();

        Ok(docs)
    }
}

#[async_trait]
impl<'a, M: EmbeddingModel + fmt::Debug + Send + Sync> Node for OpenSearchRetriever<'a, M> {
    type Input = &'a str;
    type Output = Vec<Document>;

    /// Retrieves the top k documents from OpenSearch that are most similar to the input text.
    #[cfg_attr(feature = "tracing", instrument(skip(self)))]
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        self.retrieve(input).await
    }
}

/// A builder struct for creating an OpenSearchRetriever.
struct OpenSearchRetrieverBuilder<M: EmbeddingModel> {
    client: Option<OpenSearch>,
    embedding_model: Option<M>,
    vector_field: Option<String>,
    indexes: Option<Vec<String>>,
    top_k: usize,
}

impl<M: EmbeddingModel> OpenSearchRetrieverBuilder<M> {
    /// Creates a new OpenSearchRetrieverBuilder.
    pub fn new() -> Self {
        Self {
            client: None,
            embedding_model: None,
            vector_field: None,
            indexes: None,
            top_k: 5usize,
        }
    }

    /// Sets the embedding model for the OpenSearchRetriever.
    pub fn with_embedding_model(mut self, embedding_model: M) -> Self {
        self.embedding_model = Some(embedding_model);
        self
    }

    /// Sets the vector field for the OpenSearchRetriever.
    pub fn with_vector_field(mut self, vector_field: String) -> Self {
        self.vector_field = Some(vector_field);
        self
    }

    /// Sets the indexes for the OpenSearchRetriever.
    pub fn with_indexes(mut self, indexes: Vec<String>) -> Self {
        self.indexes = Some(indexes);
        self
    }

    /// Sets the top k for the OpenSearchRetriever.
    pub fn with_top_k(mut self, top_k: usize) -> Self {
        self.top_k = top_k;
        self
    }

    /// Sets the OpenSearch client for the OpenSearchRetriever.
    pub fn with_client(mut self, client: OpenSearch) -> Self {
        self.client = Some(client);
        self
    }

    /// Builds an OpenSearchRetriever from the provided configuration.
    pub async fn build(self, base_url: &str) -> Result<OpenSearchRetriever<M>, AnchorChainError> {
        let embedding_model = self
            .embedding_model
            .ok_or(AnchorChainError::InvalidInputError(
                "No embedding model provided".to_string(),
            ))?;
        let vector_field = self
            .vector_field
            .ok_or(AnchorChainError::InvalidInputError(
                "No vector field provided".to_string(),
            ))?;
        let indexes = self.indexes.ok_or(AnchorChainError::InvalidInputError(
            "No indexes provided".to_string(),
        ))?;

        let url = Url::parse(base_url).map_err(|e| AnchorChainError::ParseError(e.to_string()))?;
        let service_name = "es";
        let conn_pool = SingleNodeConnectionPool::new(url);
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let aws_config = aws_config::from_env()
            .region(region_provider)
            .load()
            .await
            .clone();
        let transport = TransportBuilder::new(conn_pool)
            .auth(aws_config.clone().try_into()?)
            .service_name(service_name)
            .build()
            .map_err(|e| AnchorChainError::OpenSearchError(e.into()))?;
        let client = OpenSearch::new(transport);

        Ok(OpenSearchRetriever {
            client,
            embedding_model,
            vector_field,
            indexes,
            top_k: self.top_k,
            _marker: std::marker::PhantomData,
        })
    }
}

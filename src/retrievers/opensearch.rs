#![allow(dead_code)]
use std::fmt;

use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_config::SdkConfig;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::Url;
use opensearch::{OpenSearch, SearchParts};
use serde_json::json;

use crate::error::AnchorChainError;
use crate::models::embedding_model::EmbeddingModel;
use crate::node::Node;

#[derive(Debug)]
pub struct OpenSearchRetriever<M: EmbeddingModel> {
    client: OpenSearch,
    embedding_model: M,
    indexes: Vec<String>,
    vector_field: String,
    top_k: usize,
}

impl<M: EmbeddingModel> OpenSearchRetriever<M> {
    pub async fn new(
        embedding_model: M,
        base_url: &str,
        indexes: &[&str],
        vector_field: &str,
        top_k: usize,
    ) -> Result<Self, AnchorChainError> {
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
        Ok(Self {
            client,
            embedding_model,
            indexes: indexes.iter().map(|s| s.to_string()).collect(),
            vector_field: vector_field.to_string(),
            top_k,
        })
    }

    #[allow(dead_code)]
    pub async fn new_with_aws_config(
        embedding_model: M,
        base_url: &str,
        indexes: &[&str],
        vector_field: &str,
        top_k: usize,
        aws_config: SdkConfig,
    ) -> Result<Self, AnchorChainError> {
        let url = Url::parse(base_url).map_err(|e| AnchorChainError::ParseError(e.to_string()))?;
        let service_name = "es";
        let conn_pool = SingleNodeConnectionPool::new(url);
        let transport = TransportBuilder::new(conn_pool)
            .auth(aws_config.try_into()?)
            .service_name(service_name)
            .build()
            .map_err(|e| AnchorChainError::OpenSearchError(e.into()))?;
        let client = OpenSearch::new(transport);
        Ok(Self {
            client,
            embedding_model,
            indexes: indexes.iter().map(|s| s.to_string()).collect(),
            vector_field: vector_field.to_string(),
            top_k,
        })
    }

    #[allow(dead_code)]
    pub async fn new_with_client(
        embedding_model: M,
        client: OpenSearch,
        indexes: &[&str],
        vector_field: &str,
        top_k: usize,
    ) -> Result<Self, AnchorChainError> {
        Ok(Self {
            client,
            embedding_model,
            indexes: indexes.iter().map(|s| s.to_string()).collect(),
            vector_field: vector_field.to_string(),
            top_k,
        })
    }

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

        dbg!(&response);
        Ok(response.json::<serde_json::Value>().await?)
    }

    pub async fn retrieve(&self, input: &str) -> Result<serde_json::Value, AnchorChainError> {
        let embedding = self.embedding_model.embed(input.to_string()).await?;
        let response = self
            .vector_query(&self.indexes, &self.vector_field, self.top_k, embedding)
            .await?;
        Ok(response)
    }
}

#[async_trait]
impl<M: EmbeddingModel + fmt::Debug + Send + Sync> Node for OpenSearchRetriever<M> {
    type Input = String;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        // TODO: Implement pulling docs from OpenSearch
        self.retrieve(&input).await.map(|v| v.to_string())
    }
}

struct OpenSearchRetrieverBuilder<M: EmbeddingModel> {
    client: Option<OpenSearch>,
    embedding_model: Option<M>,
    vector_field: Option<String>,
    indexes: Option<Vec<String>>,
    top_k: usize,
}

impl<M: EmbeddingModel> OpenSearchRetrieverBuilder<M> {
    pub fn new() -> Self {
        Self {
            client: None,
            embedding_model: None,
            vector_field: None,
            indexes: None,
            top_k: 5usize,
        }
    }

    pub fn with_embedding_model(mut self, embedding_model: M) -> Self {
        self.embedding_model = Some(embedding_model);
        self
    }

    pub fn with_vector_field(mut self, vector_field: String) -> Self {
        self.vector_field = Some(vector_field);
        self
    }

    pub fn with_indexes(mut self, indexes: Vec<String>) -> Self {
        self.indexes = Some(indexes);
        self
    }

    pub fn with_top_k(mut self, top_k: usize) -> Self {
        self.top_k = top_k;
        self
    }

    pub fn with_client(mut self, client: OpenSearch) -> Self {
        self.client = Some(client);
        self
    }

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
        })
    }
}

#![allow(dead_code)]

use std::fmt;

use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_config::SdkConfig;
use opensearch::auth::Credentials;
use opensearch::http::request::JsonBody;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::Url;
use opensearch::indices::IndicesCreateParts;
use opensearch::{BulkParts, IndexParts, OpenSearch, SearchParts};
use serde_json::json;
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::error::AnchorChainError;
use crate::models::embedding_model::EmbeddingModel;
use crate::node::Node;
use crate::vector::document::Document;

#[derive(Debug)]
pub struct OpenSearchRetriever<M: EmbeddingModel> {
    client: OpenSearch,
    embedding_model: M,
    indexes: Vec<String>,
    vector_field: String,
    top_k: usize,
}

impl<M: EmbeddingModel + fmt::Debug> OpenSearchRetriever<M> {
    /// Creates a new OpenSearchRetrieverBuilder using default AWS credentials from the environment.
    pub async fn new(
        embedding_model: M,
        base_url: &str,
        indexes: &[&str],
        vector_field: &str,
        top_k: usize,
    ) -> Result<Self, AnchorChainError> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let aws_config = aws_config::from_env()
            .region(region_provider)
            .load()
            .await
            .clone();
        Self::new_with_aws_config(
            embedding_model,
            base_url,
            indexes,
            vector_field,
            top_k,
            aws_config,
        )
        .await
    }

    /// Creates a new OpenSearchRetriever by creating a new OpenSearch client using basic auth.
    pub async fn new_with_basic_auth(
        embedding_model: M,
        base_url: &str,
        username: &str,
        password: &str,
        indexes: &[&str],
        vector_field: &str,
        top_k: usize,
    ) -> Result<Self, AnchorChainError> {
        let url = Url::parse(base_url).map_err(|e| AnchorChainError::ParseError(e.to_string()))?;
        let conn_pool = SingleNodeConnectionPool::new(url);
        let transport = TransportBuilder::new(conn_pool)
            .auth(Credentials::Basic(
                username.to_string(),
                password.to_string(),
            ))
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

    /// Creates a new OpenSearchRetriever by creating a new OpenSearch client using AWS credentials.
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

    /// Creates a new OpenSearchRetriever with the specified embedding model, OpenSearch client,
    /// indexes, vector field, and top k value.
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

        dbg!(&response);
        Ok(response.json::<serde_json::Value>().await?)
    }

    /// Retrieves the top k documents from OpenSearch that are most similar to the input text.
    ///
    /// Uses the embedding model to embed the input text into a vector, then queries OpenSearch
    /// using the vector.
    #[cfg_attr(feature = "tracing", instrument)]
    pub async fn retrieve(&self, input: &str) -> Result<serde_json::Value, AnchorChainError> {
        let embedding = self.embedding_model.embed(input.to_string()).await?;
        let response = self
            .vector_query(&self.indexes, &self.vector_field, self.top_k, embedding)
            .await?;
        Ok(response)
    }

    /// Creates a vector index in OpenSearch with the specified name using default settings.
    pub async fn create_index(&self, index: &str) -> Result<(), AnchorChainError> {
        let body = json!({
            "settings": {
                "index.knn": true
            },
            "mappings": {
                "properties": {
                    index: {
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

        self.client
            .indices()
            .create(IndicesCreateParts::Index("my_knn_index"))
            .body(body)
            .send()
            .await?;
        Ok(())
    }

    pub async fn index_docs(
        &self,
        index: &str,
        docs: Vec<serde_json::Value>,
    ) -> Result<(), AnchorChainError> {
        let mut body = String::new();
        for doc in docs {
            body.push_str(&serde_json::to_string(&doc)?);
            body.push('\n');
        }
        self.client
            .index(IndexParts::Index(index))
            .body(body)
            .send()
            .await?;
        Ok(())
    }

    /// Automatically indexes a list of documents. It embeds the text into a vector if not already done,
    /// then indexes the entire document into OpenSearch.
    pub async fn index_documents(
        &self,
        mut docs: Vec<Document>,
        index: &str,
    ) -> Result<(), AnchorChainError> {
        let mut operations: Vec<JsonBody<_>> = Vec::new();

        for doc in &mut docs {
            if doc.embedding.is_none() {
                doc.embedding = Some(
                    self.embedding_model
                        .embed(doc.text.clone())
                        .await
                        .map_err(|e| AnchorChainError::ModelError(e.to_string()))?,
                );
            }

            let doc_json = serde_json::to_value(&doc).map_err(AnchorChainError::RequestError)?;

            operations.push(
                json!({
                    "index": {
                        "_id": doc.id,
                    }
                })
                .into(),
            );
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
            Ok(())
        } else {
            Err(AnchorChainError::ParseError(response.text().await?))
        }
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

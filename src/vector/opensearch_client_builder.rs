use crate::error::AnchorChainError;
use aws_config::SdkConfig;
use opensearch::auth::Credentials;
use opensearch::cert::CertificateValidation;
use opensearch::http::transport::SingleNodeConnectionPool;
use opensearch::http::Url;
use opensearch::{http::transport::TransportBuilder, OpenSearch};

enum ConnectionType<'a> {
    /// Local connection to OpenSearch using url, username, and password.
    Local(&'a str, &'a str, &'a str),
    /// Local connection to OpenSearch using url, username, and password with self-signed certificate.
    LocalWithoutCertValidation(&'a str, &'a str, &'a str),
    /// AWS OpenSearch connection using url and AWS SDK config.
    AwsOpenSearch(&'a str, SdkConfig),
    /// AWS OpenSearch serverless connection using url and AWS SDK config.
    AwsOpenSearchServerless(&'a str, SdkConfig),
}

pub struct OpenSearchClientBuilder<'a> {
    connection_type: Option<ConnectionType<'a>>,
}

impl<'a> OpenSearchClientBuilder<'a> {
    pub fn new() -> Self {
        OpenSearchClientBuilder {
            connection_type: None,
        }
    }

    pub fn with_local_connection(
        mut self,
        url: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Self {
        self.connection_type = Some(ConnectionType::Local(url, username, password));
        self
    }

    pub fn with_local_connection_without_cert_validation(
        mut self,
        url: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Self {
        self.connection_type = Some(ConnectionType::LocalWithoutCertValidation(
            url, username, password,
        ));
        self
    }

    pub fn with_aws_opensearch_connection(mut self, url: &'a str, aws_config: SdkConfig) -> Self {
        self.connection_type = Some(ConnectionType::AwsOpenSearch(url, aws_config));
        self
    }

    pub fn with_aws_opensearch_serverless_connection(
        mut self,
        url: &'a str,
        aws_config: SdkConfig,
    ) -> Self {
        self.connection_type = Some(ConnectionType::AwsOpenSearchServerless(url, aws_config));
        self
    }

    pub async fn build(self) -> Result<OpenSearch, AnchorChainError> {
        match self.connection_type {
            Some(ConnectionType::Local(url, username, password)) => {
                let url =
                    Url::parse(url).map_err(|e| AnchorChainError::ParseError(e.to_string()))?;
                let conn_pool = SingleNodeConnectionPool::new(url);
                let transport = TransportBuilder::new(conn_pool)
                    .auth(Credentials::Basic(
                        username.to_string(),
                        password.to_string(),
                    ))
                    .build()
                    .map_err(|e| AnchorChainError::OpenSearchError(e.into()))?;
                Ok(OpenSearch::new(transport))
            }
            Some(ConnectionType::LocalWithoutCertValidation(url, username, password)) => {
                let url =
                    Url::parse(url).map_err(|e| AnchorChainError::ParseError(e.to_string()))?;
                let conn_pool = SingleNodeConnectionPool::new(url);
                let transport = TransportBuilder::new(conn_pool)
                    .auth(Credentials::Basic(
                        username.to_string(),
                        password.to_string(),
                    ))
                    .cert_validation(CertificateValidation::None)
                    .build()
                    .map_err(|e| AnchorChainError::OpenSearchError(e.into()))?;
                Ok(OpenSearch::new(transport))
            }
            Some(ConnectionType::AwsOpenSearch(url, aws_config)) => {
                let url =
                    Url::parse(url).map_err(|e| AnchorChainError::ParseError(e.to_string()))?;
                let conn_pool = SingleNodeConnectionPool::new(url);
                let transport = TransportBuilder::new(conn_pool)
                    .auth(aws_config.clone().try_into()?)
                    .service_name("es")
                    .build()
                    .map_err(|e| AnchorChainError::OpenSearchError(e.into()))?;
                Ok(OpenSearch::new(transport))
            }
            Some(ConnectionType::AwsOpenSearchServerless(url, aws_config)) => {
                let url =
                    Url::parse(url).map_err(|e| AnchorChainError::ParseError(e.to_string()))?;
                let conn_pool = SingleNodeConnectionPool::new(url);
                let transport = TransportBuilder::new(conn_pool)
                    .auth(aws_config.clone().try_into()?)
                    .service_name("aoss")
                    .build()
                    .map_err(|e| AnchorChainError::OpenSearchError(e.into()))?;
                Ok(OpenSearch::new(transport))
            }
            None => Err(AnchorChainError::ParseError(
                "No connection type specified".to_string(),
            )),
        }
    }
}

impl<'a> Default for OpenSearchClientBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

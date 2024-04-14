//! Defines the interface for an embedding model that can embed text input.
use async_trait::async_trait;

use crate::error::AnchorChainError;

/// Defines the interface for an embedding model that can embed text input.
#[async_trait]
pub trait EmbeddingModel {
    /// Embeds the given input text and returns the resulting vector.
    async fn embed(&self, input: String) -> Result<Vec<f32>, AnchorChainError>;

    fn dimensions(&self) -> usize;
}

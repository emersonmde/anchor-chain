//! Contains nodes that are designed to work with various LLM models.
//!
//! The nodes in this module are designed to work with specific LLM models and
//! provide a common interface for interacting with them. This allows developers
//! to easily integrate different models into their processing chains without
//! having to worry about the specific details of each model's API.

#[cfg(feature = "bedrock")]
pub mod bedrock_converse;
pub mod embedding_model;
#[cfg(feature = "ollama")]
pub mod ollama;
#[cfg(feature = "openai")]
pub mod openai;

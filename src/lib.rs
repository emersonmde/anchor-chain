//! # Anchor Chain
//!
//! Anchor Chain is a Rust framework designed to simplify the orchestration of workflows involving
//! Large Language Models (LLMs). Inspired by LangChain, Anchor Chain provides a set of easy-to-use
//! and extensible building blocks that enable developers to create robust and efficient LLM-based
//! applications quickly. The framework prioritizes type safety, processing efficiency, and
//! flexibility through its carefully designed APIs and abstractions.
//!
//! ## Features
//!
//! - Statically Typed Chains: Anchor Chain leverages Rust's type system to provide statically
//!   typed chains, catching potential type mismatches at compile time.
//!
//! - Async Runtime for Parallel Execution: Built with Rust's async runtime, Anchor Chain allows
//!   for efficient parallel processing of nodes in complex chains.
//!
//! - Extensibility through the Node Trait: The Node trait allows developers to create custom
//!   nodes tailored to their specific use cases, enabling seamless integration into the chain.
//!
//! - Support for Popular LLMs: Anchor Chain provides built-in support for popular LLMs, such as
//!   OpenAI's GPT models and Anthropic Claude, abstracting away API details to provide a common
//!   interface.
//!
//! - Parallel Node Execution: The ParallelNode struct enables parallel execution of multiple
//!   nodes, leveraging concurrency to improve overall chain performance.
//!
//! ## Getting Started
//!
//! To get started with Anchor Chain, add the following dependency to your Cargo.toml file:
//!
//! ```toml
//! [dependencies]
//! anchor-chain = "0.1.0"
//! ```
//!
//! Then, you can create chains using the ChainBuilder and invoke them with the .process()
//! function. Any node can be added to the chain using the link() function which
//! will execute the node in the order it was added.
//!
//! ```rust,no_run
//! #[tokio::main]
//! async fn main() {
//!     use anchor_chain::{
//!         chain::ChainBuilder,
//!         models::openai::OpenAIModel,
//!     };
//!
//!     let chain = ChainBuilder::new()
//!         .link(OpenAIModel::new_gpt3_5_turbo("You are a helpful assistant").await)
//!         .build();
//!
//!     let result = chain
//!         .process("Write a hello world program in Rust")
//!         .await
//!         .expect("Error processing chain");
//!
//!     println!("Result: {}", result);
//! }
//! ```
//!
//! Prompts can be constructed using the `Prompt` struct. `Prompt` uses
//! [Tera](https://keats.github.io/tera/docs/#templates) templating to allow
//! for dynamic input substitution. Tera's syntax is based on Jinja2 and Django
//! templates. Context variables are passed to the prompt using a HashMap.
//!
//! ```rust,no_run
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() {
//!     use anchor_chain::{
//!         chain::ChainBuilder,
//!         models::openai::OpenAIModel,
//!         nodes::prompt::Prompt,
//!     };
//!
//!     let chain = ChainBuilder::new()
//!         .link(Prompt::new("{{ input }}"))
//!         .link(OpenAIModel::new_gpt3_5_turbo("You are a helpful assistant").await)
//!         .build();
//!
//!     let result = chain
//!         .process(HashMap::from([("input", "Write a hello world program in Rust")]))
//!         .await
//!         .expect("Error processing chain");
//!
//!     println!("Result: {}", result);
//! }
//! ```
//!
//! For more examples please refer to the [examples
//! directory](https://github.com/emersonmde/anchor-chain/tree/main/examples).

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct _README;

mod agents;
pub mod chain;
mod error;
mod link;
mod state_manager;
// TODO: Add impls for Ollama
#[cfg(any(feature = "bedrock", feature = "bedrock"))]
pub mod message;
pub mod models;
pub mod node;
pub mod nodes;
pub mod parallel_node;
pub mod vector;

pub use agents::tool_registry::ToolRegistry;
pub use chain::ChainBuilder;
pub use error::AnchorChainError;
pub use link::Link;
pub use node::NoOpNode;
pub use node::Node;
pub use node::Stateless;
pub use nodes::logger::Logger;
pub use nodes::prompt::Prompt;
pub use parallel_node::to_boxed_future;
pub use parallel_node::ParallelNode;
pub use state_manager::StateManager;

#[cfg(feature = "bedrock")]
pub use models::claude_3::Claude3Bedrock;
#[cfg(feature = "ollama")]
pub use models::ollama::Ollama;
#[cfg(feature = "openai")]
pub use models::openai::OpenAIChatModel;
#[cfg(feature = "openai")]
pub use models::openai::OpenAIEmbeddingModel;
#[cfg(feature = "openai")]
pub use models::openai::OpenAIInstructModel;
#[cfg(feature = "openai")]
pub use models::openai::OpenAIModel;

pub use vector::document::Document;
#[cfg(feature = "opensearch")]
pub use vector::opensearch_client_builder::OpenSearchClientBuilder;
#[cfg(feature = "opensearch")]
pub use vector::opensearch_indexer::OpenSearchIndexer;
#[cfg(feature = "opensearch")]
pub use vector::opensearch_retriever::OpenSearchRetriever;

#[cfg(feature = "macros")]
pub use ctor;

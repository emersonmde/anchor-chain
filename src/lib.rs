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
//!   OpenAI's GPT models and Anthropic's Claude, abstracting away API details to provide a common
//!   interface.
//!
//! - Parallel Node Execution: The ParallelNode struct enables parallel execution of multiple
//!   nodes, leveraging concurrency to improve overall chain performance.
//!
//! ## Why Anchor Chain?
//!
//! Anchor Chain addresses some of the challenges developers face when working with LangChain, such
//! as the lack of comprehensive documentation, rapidly evolving APIs, and the absence of type
//! safety. These issues can lead to a steep learning curve and time-consuming trial and error when
//! building LLM-based applications.
//!
//! By leveraging Rust's expressive type system, Anchor Chain provides statically typed chains that
//! offer clear compile-time feedback and in-editor type hints. This helps catch potential errors
//! early in the development process and facilitates a more efficient workflow.
//!
//! Additionally, Anchor Chain's built-in support for async runtimes enables efficient parallel
//! processing of nodes in complex chains. This can significantly reduce the overall execution time
//! of LLM-based workflows, making Anchor Chain an attractive choice for performance-critical
//! applications.
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
//! function:
//!
//! ```no_run
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     use anchor_chain::{
//!         chain::ChainBuilder,
//!         models::openai::OpenAIModel,
//!         prompt::Prompt,
//!     };
//!
//!     let chain = ChainBuilder::new()
//!         .link(Prompt::new("{{ input }}"))
//!         .link(OpenAIModel::new_gpt3_5_turbo("You are a helpful assistant".to_string()).await)
//!         .build();
//!
//!     let result = chain
//!         .process(HashMap::from([("input", "Write a hello world program in Rust")]))
//!         .await?;
//!
//!     println!("Result: {}", result);
//!     Ok(())
//! }
//! ```
//!
//! For more examples please refer to the [examples
//! directory](https://github.com/emersonmde/anchor-chain/tree/main/examples).
//!
//! ## Contributing
//!
//! Contributions to Anchor Chain are welcome! If you encounter any issues, have suggestions for
//! improvements, or would like to contribute new features, please open an issue or submit a pull
//! request on the GitHub repository.
//!
//! ## License
//!
//! Anchor Chain is released under the MIT License.

#![allow(dead_code)]

pub mod chain;
pub mod link;
pub mod message;
pub mod models;
pub mod node;
pub mod parallel_node;
pub mod prompt;
pub mod trace_node;

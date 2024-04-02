//! # Anchor Chain: A Rust Framework for Large Language Models (LLMs)
//!
//! `anchor_chain` is a framework designed for building and executing asynchronous
//! processing chains, particularly tailored for interacting with Large Language Models (LLMs)
//! such as GPT-3.5 and Claude 3. This library was inspired by LangChain but leverages the
//! statically typed nature of Rust ensuring compile-time type safety and
//! performance. It's particularly suited for scenarios requiring data
//! processing pipelines, including natural language processing and data transformation.
//!
//! ## Why Rust?
//!
//! Rust brings several advantages to the table, especially for high-stakes data processing
//! applications, by offering:
//!
//! - **Compile-Time Type Safety**: Guarantees about the correctness of processor chain types,
//!   minimizing runtime errors.
//! - **Optimized Performance**: Efficient memory management and execution speed, crucial for
//!   processing large volumes of data or complex model outputs.
//! - **Robust Concurrency**: Safe and expressive concurrency primitives make it easier to
//!   construct non-blocking, concurrent processing pipelines.
//!
//! ## Core Components
//!
//! - `chain::ChainBuilder`: Constructs processing chains in a type-safe manner, ensuring
//!   seamless data flow between processors.
//! - `models`: Features predefined models and processors, such as `Gpt3_5Turbo` and
//!   `Claude3Bedrock`, facilitating easy integration with well-known LLMs via AWS Bedrock.
//! - `prompt::Prompt`: Enhances prompt construction and modification, enabling dynamic
//!   LLM interactions.
//!
//! ## Getting Started
//!
//! Define your processors and use `ChainBuilder` to piece together a processing chain.
//! Below is an illustrative example:
//!
//! ```no_run
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     use anchor_chain::{
//!         chain::ChainBuilder,
//!         models::gpt_3_5_turbo::Gpt3_5Turbo,
//!         prompt::Prompt,
//!     };
//!
//!     let prompt_processor = Prompt::new("{input}");
//!     let chain = ChainBuilder::new(prompt_processor)
//!         .link(Gpt3_5Turbo::new("You are a helpful assistant".to_string()).await)
//!         .build();
//!
//!     let result = chain
//!         .process("Write a hello world program in Rust".to_string())
//!         .await?;
//!
//!     println!("Result: {}", result);
//!     Ok(())
//! }
//! ```
//!
//! `anchor_chain` is designed for a wide range of applications, from simple text manipulation
//! to advanced natural language processing tasks. Its adaptable architecture ensures it can be
//! tailored to fit specific processing requirements, making Rust a compelling choice for AI
//! and machine learning projects.

#![allow(dead_code)]

pub mod chain;
pub mod link;
pub mod message;
pub mod models;
pub mod node;
pub mod parallel_node;
pub mod prompt;

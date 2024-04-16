//! Various nodes that can be chained together to form an LLM process chain.
//!
//! The nodes are designed to be as generic as possible, so that they can be used in a wide
//! variety of contexts. Each node has a defined input and output type that is checked at compile
//! time to ensure nodes are connected correctly.

pub mod logger;
pub mod prompt;

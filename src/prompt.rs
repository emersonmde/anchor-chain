//! Module for handling dynamic prompts in processing chains.
//!
//! This module provides the `Prompt` struct, a processor for handling
//! and displaying text prompts.

use crate::node::Node;
use anyhow::Result;
use async_trait::async_trait;

/// A processor for handling text prompts within a processing chain.
///
/// `Prompt` is primarily used for manipulating or logging text data as it flows through
/// a processing chain.
#[derive(Debug)]
pub struct Prompt {
    text: String,
}

impl Prompt {
    /// Creates a new `Prompt` processor with the specified text.
    ///
    /// # Parameters
    /// - `text`: The static or template text to be associated with this prompt processor.
    ///
    /// # Examples
    /// ```
    /// use anchor_chain::prompt::Prompt;
    ///
    /// let prompt = Prompt::new("Hello, World!");
    /// ```
    pub fn new(text: &str) -> Self {
        Prompt {
            text: text.to_string(),
        }
    }
}

#[async_trait]
impl Node for Prompt {
    type Input = String;
    type Output = String;

    /// Processes the input by simply logging the prompt text and returning the input unchanged.
    ///
    /// # Parameters
    /// - `input`: The input text to the processor, which is passed through unchanged.
    ///
    /// # Returns
    /// A `Result` containing the original input text, facilitating further processing
    /// in subsequent chain links.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        println!("{}", self.text);
        Ok(input)
    }
}

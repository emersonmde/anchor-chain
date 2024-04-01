//! Module for handling dynamic prompts in processing chains.
//!
//! This module provides the `Prompt` struct, a simple yet flexible processor for handling
//! and displaying text prompts. It's designed to integrate seamlessly into asynchronous
//! processing chains, allowing for dynamic prompt customization and logging.

use crate::link::Processor;
use anyhow::Result;
use async_trait::async_trait;

/// A processor for handling text prompts within a processing chain.
///
/// `Prompt` is primarily used for manipulating or logging text data as it flows through
/// a processing chain. While simple in its current functionality, it serves as a foundation
/// for more complex prompt handling and manipulation scenarios.
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
impl Processor for Prompt {
    type Input = String;
    type Output = String;

    /// Processes the input by simply logging the prompt text and returning the input unchanged.
    ///
    /// This method demonstrates a basic operation within the processing chain, where the
    /// processor's main role is to output its stored prompt to the console, allowing for
    /// simple text logging or manipulation tasks.
    ///
    /// # Parameters
    /// - `input`: The input text to the processor, which is passed through unchanged.
    ///
    /// # Returns
    /// A `Result` containing the original input text, facilitating further processing
    /// in subsequent chain links.
    ///
    /// # Examples
    /// ```
    /// # use anchor_chain::prompt::Prompt;
    /// # use anchor_chain::link::Processor;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let prompt = Prompt::new("Processing input:");
    /// let output = prompt.process("Sample input".to_string()).await?;
    ///
    /// assert_eq!(output, "Sample input");
    /// # Ok(())
    /// # }
    /// ```
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        println!("{}", self.text);
        Ok(input)
    }
}

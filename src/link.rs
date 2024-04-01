//! Module providing foundational structures for building asynchronous processing chains.
//!
//! This module defines a `Processor` trait for asynchronous operations and
//! constructs (`Link` and `End`) to create chains of these operations.

use anyhow::Result;
use async_trait::async_trait;

/// Represents an asynchronous operation that can process an input to produce an output.
///
/// The `Processor` trait defines a generic asynchronous processing operation with a
/// specified input and output type. Implementors of this trait can be composed together
/// to form a processing chain.
#[async_trait]
pub trait Processor {
    /// The input type for the processor.
    type Input;
    /// The output type for the processor.
    type Output;

    /// Asynchronously processes the given input, returning the output.
    ///
    /// # Parameters
    /// - `input`: The input value to be processed.
    ///
    /// # Returns
    /// A `Result` containing the processed output value or an error if processing fails.
    async fn process(&self, input: Self::Input) -> Result<Self::Output>;
}

/// A link in a processing chain that connects one `Processor` to another.
///
/// `Link` serves as a container for chaining two `Processor` instances together,
/// where the output of the first processor is fed as the input to the next.
pub struct Link<P, N> {
    /// The first processor in the chain.
    pub processor: P,
    /// The next processor or link in the chain.
    pub next: N,
}

/// Represents the end of a processing chain.
///
/// The `End` struct signifies the termination point of a processing chain. It effectively
/// acts as a no-op processor, returning its input as output.
pub struct End;

#[async_trait]
impl Processor for End {
    type Input = String;
    type Output = String;

    /// Processes the given input by simply returning it unchanged.
    ///
    /// This method serves as a placeholder at the end of a processing chain,
    /// effectively performing no operation on the input.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        Ok(input)
    }
}

#[async_trait]
impl<P, N> Processor for Link<P, N>
where
    // Ensures `P` is suitable for async processing across threads.
    P: Processor + Send + Sync,
    // Ensures `N` can accept the Processor's output and is also thread-safe.
    N: Processor<Input = P::Output> + Send + Sync,
    // Ensures the output can be sent across threads and lives long enough.
    P::Output: Send + 'static,
    // Ensures the input can be sent into async contexts safely.
    P::Input: Send,
    // Ensures the final output is thread-safe.
    N::Output: Send,
{
    type Input = P::Input;
    type Output = <N as Processor>::Output;

    /// Processes the given input through the chain of processors.
    ///
    /// First, the input is processed by the current processor. Then, the output of the current
    /// processor is passed to the next processor or link in the chain for further processing.
    ///
    /// # Parameters
    /// - `input`: The input value to be processed by the chain.
    ///
    /// # Returns
    /// A `Result` containing the final output from the chain or an error if processing fails at any point.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        let output = self.processor.process(input).await?;
        self.next.process(output).await
    }
}

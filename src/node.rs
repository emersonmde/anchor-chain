//! Module providing foundational structures for building chains.
//!
//! This module defines a `Node` trait for asynchronous operations and
//! constructs (`Link` and `End`) to create chains of these operations.

use anyhow::Result;
use async_trait::async_trait;

/// Represents an node that can process an input to produce an output.
///
/// The `Node` trait defines a generic processing operation with a
/// specified input and output type. Implementors of this trait can be
/// composed together to form a processing chain.
#[async_trait]
pub trait Node {
    /// The input type for the node.
    type Input;
    /// The output type for the node.
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

/// A link in a processing chain that connects one `Node` to another.
///
/// `Link` serves as a container for chaining two `Node` instances together,
/// where the output of the first node is fed as the input to the next.
pub struct Link<C, N> {
    /// The first node in the chain.
    pub node: C,
    /// The next node or link in the chain.
    pub next: N,
}

/// Represents the end of a processing chain.
///
/// The `End` struct signifies the termination point of a processing chain. It effectively
/// acts as a no-op node, returning its input as output.
pub struct End;

#[async_trait]
impl Node for End {
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
impl<C, N> Node for Link<C, N>
where
    C: Node + Send + Sync,
    C::Output: Send + 'static,
    C::Input: Send,
    N: Node<Input = C::Output> + Send + Sync,
    N::Output: Send,
{
    type Input = C::Input;
    type Output = <N as Node>::Output;

    /// Processes the given input through the chain of nodes.
    ///
    /// First, the input is processed by the current node. Then, the output of the current
    /// node is passed to the next node or link in the chain for further processing.
    ///
    /// # Parameters
    /// - `input`: The input value to be processed by the chain.
    ///
    /// # Returns
    /// A `Result` containing the final output from the chain or an error if processing fails at any point.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        let output = self.node.process(input).await?;
        self.next.process(output).await
    }
}

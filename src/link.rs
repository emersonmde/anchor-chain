//! A link in a processing chain that connects one `Node` to another.
//!
//! `Link` serves as a container for chaining two `Node` instances together,
//! where the output of the first node is fed as the input to the next.

use async_trait::async_trait;

use crate::node::Node;
use anyhow::Result;

/// A link in a processing chain that connects one `Node` to another.
///
/// `Link` serves as a container for chaining two `Node` instances together,
/// where the output of the first node is fed as the input to the next.
#[derive(Debug)]
pub struct Link<C, N>
where
    C: std::fmt::Debug,
    N: std::fmt::Debug,
{
    /// The first node in the chain.
    pub node: C,
    /// The next node or link in the chain.
    pub next: N,
}

impl<C, N> Link<C, N>
where
    C: std::fmt::Debug,
    N: std::fmt::Debug,
{
    /// Creates a new `Link` connecting the specified nodes.
    ///
    /// # Parameters
    /// - `node`: The first node in the chain.
    /// - `next`: The next node or link in the chain.
    ///
    /// # Returns
    /// A new `Link` instance connecting the two nodes.
    pub fn new(node: C, next: N) -> Self {
        Link { node, next }
    }
}

#[async_trait]
impl<C, N> Node for Link<C, N>
where
    C: Node + Send + Sync + std::fmt::Debug,
    C::Output: Send + 'static,
    C::Input: Send,
    N: Node<Input = C::Output> + Send + Sync + std::fmt::Debug,
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

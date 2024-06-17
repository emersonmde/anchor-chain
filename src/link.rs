//! A link in a processing chain that connects one `Node` to another.
//!
//! `Link` serves as a container for chaining two `Node` instances together,
//! where the output of the first node is fed as the input to the next.

use async_trait::async_trait;

use crate::error::AnchorChainError;
use crate::memory_manager::MemoryManager;
use crate::node::Node;

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
    pub memory: Option<MemoryManager<String>>,
}

impl<C, N> Link<C, N>
where
    C: std::fmt::Debug,
    N: std::fmt::Debug,
{
    /// Creates a new `Link` connecting the specified nodes.
    ///
    /// The `node` is linked with the `next` node in the chain. Output from the
    /// `node` is passed as input to the `next` node. Either node can also be
    /// a `Link` forming a nested linked list of nodes.
    pub fn new(node: C, next: N, memory: Option<MemoryManager<String>>) -> Self {
        Link { node, next, memory }
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
    /// The input type for the current node
    type Input = C::Input;
    /// The output type of the next node
    type Output = <N as Node>::Output;

    /// Processes the given input through the chain of nodes.
    ///
    /// First, the input is processed by the current node. Then, the output of the current
    /// node is passed to the next node or link in the chain for further processing.
    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        let output = self.node.process(input).await?;
        self.next.process(output).await
    }
}

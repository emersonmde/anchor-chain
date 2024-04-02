use async_trait::async_trait;

use crate::node::Node;
use anyhow::Result;

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

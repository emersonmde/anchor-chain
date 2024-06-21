//! A link in a processing chain that connects one `Node` to another.
//!
//! `Link` serves as a container for chaining two `Node` instances together,
//! where the output of the first node is fed as the input to the next.

use async_trait::async_trait;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::AnchorChainError;
use crate::node::{Node, Stateful};
use crate::state_manager::StateManager;

/// A link in a processing chain that connects one `Node` to another.
///
/// `Link` serves as a container for chaining two `Node` instances together,
/// where the output of the first node is fed as the input to the next.
#[derive(Debug)]
pub struct Link<C, N>
where
    C: Debug,
    N: Debug,
{
    /// The first node in the chain.
    pub node: C,
    /// The next node or link in the chain.
    pub next: N,
}

impl<C, N> Link<C, N>
where
    C: Debug,
    N: Debug,
{
    /// Creates a new `Link` connecting the specified nodes.
    ///
    /// The `node` is linked with the `next` node in the chain. Output from the
    /// `node` is passed as input to the `next` node. Either node can also be
    /// a `Link` forming a nested linked list of nodes.
    pub fn new(node: C, next: N) -> Self {
        Link { node, next }
    }
}

#[async_trait]
impl<C, N> Node for Link<C, N>
where
    C: Node + Send + Sync + Debug,
    C::Output: Send + 'static,
    C::Input: Send,
    N: Node<Input = C::Output> + Send + Sync + Debug,
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

/// A stateful link in a processing chain that connects one `Node` to another.
///
/// `StatefulLink` serves as a container for chaining two `Node` instances together,
/// where the output of the first node is fed as the input to the next. The chain's
/// `StateManager` is passed to each stateful node before processing.
#[derive(Debug)]
pub struct StatefulLink<C, N, K, V>
where
    C: Debug,
    N: Debug,
    K: Debug + Send,
    V: Debug + Send,
{
    pub node: C,
    pub next: Arc<Mutex<N>>,
    pub state: StateManager<K, V>,
}

impl<C, N, K, V> StatefulLink<C, N, K, V>
where
    C: Debug,
    N: Debug,
    K: Debug + Send,
    V: Debug + Send,
{
    pub fn new(node: C, next: N, memory: StateManager<K, V>) -> Self {
        Self {
            node,
            next: Arc::new(Mutex::new(next)),
            state: memory,
        }
    }
}

#[async_trait]
impl<C, N, K, V> Node for StatefulLink<C, N, K, V>
where
    C: Node + Send + Sync + Debug,
    C::Output: Send + 'static,
    C::Input: Send,
    N: Node<Input = C::Output> + Stateful<K, V> + Send + Sync + Debug,
    N::Output: Send,
    K: Eq + Hash + Clone + Send + Sync + Debug,
    V: Clone + Send + Sync + Debug,
{
    type Input = C::Input;
    type Output = <N as Node>::Output;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        let output = self.node.process(input).await?;
        let mut next_node = self.next.lock().await;
        next_node.set_state(self.state.clone()).await;
        next_node.process(output).await
    }
}

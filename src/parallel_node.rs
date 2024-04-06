//! Provides a structure for processing input through multiple nodes in parallel.
//!
//! The `ParallelNode` struct represents a node that processes input through
//! multiple nodes in parallel. The output of each node is then combined using
//! a provided function to produce the final output.
//!
//! Example:
//! ```rust,no_run
//! use anchor_chain::{
//!     chain::ChainBuilder,
//!     models::{claude_3::Claude3Bedrock, openai::OpenAIModel},
//!     parallel_node::{to_boxed_future, ParallelNode},
//!     prompt::Prompt,
//! };
//! use anyhow::Result;
//! use async_trait::async_trait;
//! use futures::{future::BoxFuture, Future};
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let gpt3 =
//!         Box::new(OpenAIModel::new_gpt3_5_turbo("You are a helpful assistant".to_string()).await);
//!     let claude3 = Box::new(Claude3Bedrock::new("You are a helpful assistant".to_string()).await);
//!
//!     let concat_fn = to_boxed_future(|outputs: Vec<String>| {
//!         Ok(outputs
//!             .iter()
//!             .enumerate()
//!             .map(|(i, output)| format!("Output {}:\n```\n{}\n```\n", i + 1, output))
//!             .collect::<Vec<String>>()
//!             .concat())
//!     });
//!
//!
//!     let chain = ChainBuilder::new()
//!         .link(Prompt::new("{{ input }}"))
//!         .link(ParallelNode::new(vec![gpt3, claude3], concat_fn))
//!         .build();
//!
//!     let output = chain
//!         .process(HashMap::from([("input", "Write a hello world program in Rust")]))
//!         .await?;
//!     println!("{}", output);
//!
//!     Ok(())
//! }
//! ```

use futures::{future::BoxFuture, FutureExt};

use crate::node::Node;
use anyhow::Result;
use async_trait::async_trait;
use futures::future::try_join_all;

/// A node that processes input through multiple nodes in parallel.
///
/// The `ParallelNode` struct represents a node that processes input through
/// multiple nodes in parallel. The output of each node is then combined using
/// a provided function to produce the final output.
pub struct ParallelNode<I, O>
where
    I: Clone + Send + Sync,
    O: Send + Sync,
{
    /// The nodes that will process the input in parallel.
    pub nodes: Vec<Box<dyn Node<Input = I, Output = O> + Send + Sync>>,
    /// The function to process the output of the nodes.
    pub function: Box<dyn Fn(Vec<O>) -> BoxFuture<'static, Result<O>> + Send + Sync>,
}

impl<I, O> ParallelNode<I, O>
where
    I: Clone + Send + Sync,
    O: Send + Sync,
{
    /// Creates a new `ParallelNode` with the provided nodes and combination
    /// function.
    ///
    /// The combination function can be defined using the helper function `to_boxed_future`.
    ///
    /// # Example
    /// // Using PassThroughNode as an example node
    /// ```rust
    /// use anchor_chain::{
    ///     node::PassthroughNode,
    ///     parallel_node::ParallelNode,
    ///     parallel_node::to_boxed_future
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node1 = Box::new(PassthroughNode::new());
    ///     let node2 = Box::new(PassthroughNode::new());
    ///     let concat_fn = to_boxed_future(|outputs: Vec<String>| {
    ///         Ok(outputs
    ///            .iter()
    ///            .enumerate()
    ///            .map(|(i, output)| format!("Output {}:\n```\n{}\n```\n", i + 1, output))
    ///            .collect::<Vec<String>>()
    ///            .concat())
    ///     });
    ///     let parallel_node = ParallelNode::new(vec![node1, node2], concat_fn);
    /// }
    pub fn new(
        nodes: Vec<Box<dyn Node<Input = I, Output = O> + Send + Sync>>,
        function: Box<dyn Fn(Vec<O>) -> BoxFuture<'static, Result<O>> + Send + Sync>,
    ) -> Self {
        ParallelNode { nodes, function }
    }
}

#[async_trait]
impl<I, O> Node for ParallelNode<I, O>
where
    I: Clone + Send + Sync + std::fmt::Debug,
    O: Send + Sync + std::fmt::Debug,
{
    type Input = I;
    type Output = O;

    /// Processes the given input through nodes in parallel.
    ///
    /// The input is processed by each node in parallel, and the results are combined
    /// using the provided function to produce the final output.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        let futures = self.nodes.iter().map(|node| {
            let input_clone = input.clone();
            async move { node.process(input_clone).await }
        });

        let results = try_join_all(futures).await?;
        (self.function)(results).await
    }
}

impl<I, O> std::fmt::Debug for ParallelNode<I, O>
where
    I: std::fmt::Debug + Clone + Send + Sync,
    O: std::fmt::Debug + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParallelNode")
            .field("nodes", &self.nodes)
            // Unable to debug print closures
            .field("function", &format_args!("<function/closure>"))
            .finish()
    }
}

/// Converts a function into a `BoxFuture` that can be used in a `ParallelNode`.
///
/// This function takes a function that processes input and returns a `Result` and
/// converts it into a boxed future.
pub fn to_boxed_future<T, F>(
    f: F,
) -> Box<dyn Fn(T) -> BoxFuture<'static, Result<String>> + Send + Sync>
where
    F: Fn(T) -> Result<String> + Send + Sync + Clone + 'static,
    T: Send + 'static,
{
    Box::new(move |input| {
        let f_clone = f.clone();
        async move { f_clone(input) }.boxed()
    })
}

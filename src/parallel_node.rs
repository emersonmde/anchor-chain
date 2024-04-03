//! # Parallel Node
//!
//! The `ParallelNode` struct represents a node that processes input through
//! multiple nodes in parallel. The output of each node is then combined using
//! a provided function to produce the final output.
//!
//! Example:
//! ```no_run
//! use anyhow::Result;
//! use anchor_chain::{
//!     chain::ChainBuilder,
//!     models::{claude_3::Claude3Bedrock, gpt_3_5_turbo::Gpt3_5Turbo},
//!     node::PassthroughNode,
//!     parallel_node::ParallelNode,
//!     prompt::Prompt,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let gpt3_link = Box::new(Gpt3_5Turbo::new("You are a helpful assistant".to_string()).await);
//!     let claude3_link =
//!         Box::new(Claude3Bedrock::new("You are a helpful assistant".to_string()).await);
//!
//!     let concat_fn = Box::new(|outputs: Vec<String>| {
//!         println!("Outputs: {:?}", outputs);
//!         Ok(outputs.concat())
//!     });
//!
//!     let parallel_node = ParallelNode::new(vec![gpt3_link, claude3_link], concat_fn);
//!
//!     let chain = ChainBuilder::new()
//!         .link(Prompt::new("{input}"))
//!         .link(parallel_node)
//!         .link(PassthroughNode::new())
//!         .build();
//!     chain
//!         .process("Write a hello world program in Rust".to_string())
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

use crate::node::Node;
use anyhow::Result;
use async_trait::async_trait;
use futures::future::try_join_all;

pub struct ParallelNode<I, O, F>
where
    I: Clone + Send + Sync,
    O: Send + Sync,
    F: Fn(Vec<O>) -> Result<O>,
{
    /// The nodes that will process the input in parallel.
    pub nodes: Vec<Box<dyn Node<Input = I, Output = O> + Send + Sync>>,
    /// The function to process the output of the nodes.
    pub function: F,
}

impl<I, O, F> ParallelNode<I, O, F>
where
    I: Clone + Send + Sync,
    O: Send + Sync,
    F: Fn(Vec<O>) -> Result<O>,
{
    /// Creates a new `ParallelNode` with the provided nodes and combination
    /// function.
    ///
    /// # Parameters
    /// - `nodes`: The nodes to process the input in parallel.
    /// - `function`: The function to process the output of the nodes.
    ///
    /// # Returns
    /// A new `ParallelNode` instance with the specified nodes and function.
    pub fn new(
        nodes: Vec<Box<dyn Node<Input = I, Output = O> + Send + Sync>>,
        function: F,
    ) -> Self {
        ParallelNode { nodes, function }
    }
}

#[async_trait]
impl<I, O, F> Node for ParallelNode<I, O, F>
where
    I: Clone + Send + Sync,
    O: Send + Sync,
    F: Fn(Vec<O>) -> Result<O> + Send + Sync,
{
    type Input = I;
    type Output = O;

    /// Processes the given input through nodes in parallel.
    ///
    /// The input is processed by each node in parallel, and the results are combined
    /// using the provided function to produce the final output.
    ///
    /// # Parameters
    /// - `input`: The input values to be processed by the parallel nodes.
    ///
    /// # Returns
    /// A `Result` containing the final output from the parallel nodes or an error if processing fails.
    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        let futures = self.nodes.iter().map(|node| {
            let input_clone = input.clone();
            async move { node.process(input_clone).await }
        });

        let results = try_join_all(futures).await?;
        (self.function)(results)
    }
}

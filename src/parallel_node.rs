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
    /// The node to process the input.
    pub nodes: Vec<Box<dyn Node<Input = I, Output = O> + Send + Sync>>,
    /// The function to process the output of the node.
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

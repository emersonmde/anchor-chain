use crate::node::Node;
use anyhow::Result;
use async_trait::async_trait;
use futures::future::try_join_all;

pub struct ParallelNode<N, F>
where
    N: Node + Send + Sync,
    F: Fn(Vec<N::Output>) -> Result<N::Output>,
{
    /// The node to process the input.
    pub nodes: Vec<N>,
    /// The function to process the output of the node.
    pub function: F,
}

#[async_trait]
impl<N, F> Node for ParallelNode<N, F>
where
    N: Node + Send + Sync,
    N::Input: Clone + Send,
    N::Output: Send + Sync + 'static,
    F: Fn(Vec<N::Output>) -> Result<N::Output> + Send + Sync,
{
    type Input = N::Input;
    type Output = N::Output;

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

use std::time::Instant;

use async_trait::async_trait;

use crate::node::Node;
use anyhow::Result;

#[derive(Debug)]
pub struct TraceNode<T: Node> {
    node: T,
}

impl<T: Node> TraceNode<T> {
    pub fn new(node: T) -> Self {
        TraceNode { node }
    }
}

#[async_trait]
impl<T: Node + Send + Sync> Node for TraceNode<T>
where
    T::Input: Send,
    T::Output: Send,
{
    type Input = T::Input;
    type Output = T::Output;

    async fn process(&self, input: Self::Input) -> Result<Self::Output> {
        let start_time = Instant::now();
        let output = self.node.process(input).await?;
        let elapsed_time = start_time.elapsed();
        println!(
            "Node {:?} ({})",
            std::any::type_name::<T>(),
            elapsed_time.as_millis()
        );
        Ok(output)
    }
}

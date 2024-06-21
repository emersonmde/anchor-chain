use std::collections::HashMap;

use anchor_chain::node::NodeState;
use anchor_chain::{AnchorChainError, StateManager};
use anchor_chain::{ChainBuilder, NoOpNode, Node, Prompt};
use async_trait::async_trait;

#[derive(Debug, Default)]
pub struct LineCounter {
    state: StateManager<String>,
}

impl LineCounter {
    pub fn new() -> Self {
        LineCounter::default()
    }
}

#[async_trait]
impl Node for LineCounter {
    type Input = String;
    type Output = usize;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        println!("Process called on LineCounter");
        self.state.push("Foo".to_string()).await;
        Ok(input.lines().count())
    }
}

#[async_trait]
impl NodeState<String> for LineCounter {
    async fn set_state(&mut self, state: StateManager<String>) {
        println!("Process_with_state called on LineCounter");
        self.state = state;
    }
}

#[derive(Debug, Default)]
pub struct AsteriskGenerator {
    state: StateManager<String>,
}

impl AsteriskGenerator {
    pub fn new() -> Self {
        AsteriskGenerator::default()
    }
}

#[async_trait]
impl Node for AsteriskGenerator {
    type Input = usize;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        println!("Process called on AsteriskGenerator");
        let value = self.state.get(0).await.expect("State value should exist");
        println!("Found {:?}", value);
        Ok("*".repeat(input))
    }
}

#[async_trait]
impl NodeState<String> for AsteriskGenerator {
    async fn set_state(&mut self, state: StateManager<String>) {
        self.state = state;
        println!("Process_with_state called on AsteriskGenerator");
    }
}

#[tokio::main]
async fn main() {
    let chain = ChainBuilder::new()
        .link(Prompt::new("{{ input }}"))
        .link(NoOpNode::new())
        .link_with_state(LineCounter::new())
        .link(NoOpNode::new())
        .link_with_state(AsteriskGenerator::new())
        .build();

    println!("Chain: {:?}\n\n", chain);

    let output = chain
        .process(HashMap::from([(
            "input",
            "Write a hello world program in Rust",
        )]))
        .await
        .expect("Failed to process chain");
    println!("Output:\n{}", output);
}

use std::collections::HashMap;

use anchor_chain::node::Stateful;
use anchor_chain::{AnchorChainError, StateManager};
use anchor_chain::{ChainBuilder, Node, Prompt};
use async_trait::async_trait;

#[derive(Debug, Default)]
pub struct UpperCaseConverter {
    state: Option<StateManager<String, String>>,
}

impl UpperCaseConverter {
    pub fn new() -> Self {
        UpperCaseConverter::default()
    }
}

#[async_trait]
impl Node for UpperCaseConverter {
    type Input = String;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        let state = self.state.as_ref().expect("Node state was not set");
        state.insert("original".to_string(), input.clone()).await;
        Ok(input.to_uppercase())
    }
}

#[async_trait]
impl Stateful<String, String> for UpperCaseConverter {
    async fn set_state(&mut self, state: StateManager<String, String>) {
        self.state = Some(state);
    }
}

#[derive(Debug, Default)]
pub struct Reverser {
    state: Option<StateManager<String, String>>,
}

impl Reverser {
    pub fn new() -> Self {
        Reverser::default()
    }
}

#[async_trait]
impl Node for Reverser {
    type Input = String;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        let state = self.state.as_ref().expect("Node state was not set");
        let original = state
            .get(&"original".to_string())
            .await
            .expect("State value should exist");
        println!("Original input was: {:?}", original);
        Ok(input.chars().rev().collect())
    }
}

#[async_trait]
impl Stateful<String, String> for Reverser {
    async fn set_state(&mut self, state: StateManager<String, String>) {
        self.state = Some(state);
    }
}

#[tokio::main]
async fn main() {
    let chain = ChainBuilder::new()
        .link(Prompt::new("Hello, {{ input }}!"))
        .link_with_state(UpperCaseConverter::new())
        .link_with_state(Reverser::new())
        .build();

    let output = chain
        .process(HashMap::from([("input", "world")]))
        .await
        .expect("Failed to process chain");
    println!("Final output: {}", output);
}

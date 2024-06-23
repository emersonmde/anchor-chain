use async_trait::async_trait;
use aws_sdk_bedrockruntime::types::Message as BedrockMessage;
use tokio::sync::RwLock;

use crate::models::bedrock_converse::BedrockModel;
use crate::node::Stateful;
use crate::{AnchorChainError, BedrockConverse, Node, StateManager, ToolRegistry};

#[derive(Debug)]
enum AgentModel<'a> {
    Claude3_5(BedrockConverse<'a, BedrockMessage>),
}

#[derive(Debug, anchor_chain_macros::Stateless)]
pub struct AgentExecutor<'a> {
    llm: AgentModel<'a>,
    max_iterations: usize,
    tool_registry: &'a RwLock<ToolRegistry<'a>>,
}

impl<'a> AgentExecutor<'a> {
    pub async fn new_claude_agent(tool_registry: &'a RwLock<ToolRegistry<'a>>) -> Self {
        let mut llm = BedrockConverse::new_with_system_prompt(
            BedrockModel::Claude3_5,
            "You are a helpful assistant",
        )
        .await;
        llm.set_state(StateManager::new()).await;
        AgentExecutor {
            llm: AgentModel::Claude3_5(llm),
            max_iterations: 10,
            tool_registry,
        }
    }
}

#[async_trait]
impl<'a> Node for AgentExecutor<'a> {
    type Input = String;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        match &self.llm {
            AgentModel::Claude3_5(claude) => {
                claude
                    .run_agent(input, self.max_iterations, self.tool_registry)
                    .await
            }
        }
    }
}

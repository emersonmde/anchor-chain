use std::collections::HashMap;

use async_trait::async_trait;
use aws_sdk_bedrockruntime::types::{ContentBlock, Message as BedrockMessage};
use aws_smithy_types::Document;
use tokio::sync::RwLock;

use crate::agents::tool_registry::{
    convert_document_to_value, convert_value_to_document, ToolHandler,
};
use crate::models::bedrock_converse::BedrockModel;
use crate::node::Stateful;
use crate::{AnchorChainError, BedrockConverse, Node, StateManager, ToolRegistry};

#[derive(Debug)]
enum AgentModel {
    Claude3_5(BedrockConverse<BedrockMessage>),
}

#[derive(Debug, anchor_chain_macros::Stateless)]
pub struct AgentExecutor<'a> {
    llm: AgentModel,
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
        llm.set_tool_registry(tool_registry).await;
        llm.set_state(StateManager::new()).await;
        AgentExecutor {
            llm: AgentModel::Claude3_5(llm),
            max_iterations: 10,
            tool_registry,
        }
    }

    async fn run_claude_agent(
        &self,
        llm: &BedrockConverse<BedrockMessage>,
        input: String,
    ) -> Result<String, AnchorChainError> {
        let mut output = Vec::new();
        let input = format!(
            "Given the tools available, answer the users question: {}",
            input
        )
        .to_string();

        let mut response = llm.process(input.clone()).await?.content;
        println!("Response: {response:?}");

        // TODO: Move to custom Node
        for _ in 0..self.max_iterations {
            println!("Content: {response:?}\n");
            let mut tool_used = false;
            for content in response.clone() {
                match content {
                    ContentBlock::Text(text) => output.push(text),
                    ContentBlock::ToolUse(tool_request) => {
                        tool_used = true;
                        // TODO: handle error
                        let tool_result = self
                            .tool_registry
                            .read()
                            .await
                            .execute_tool(
                                tool_request.name(),
                                convert_document_to_value(&tool_request.input),
                            )
                            .unwrap();
                        println!("Result from tool function: {:?}\n", tool_result);
                        let tool_response = llm
                            .invoke_with_tool_response(
                                tool_request.tool_use_id,
                                Document::Object(HashMap::from([(
                                    "return".to_string(),
                                    convert_value_to_document(&tool_result),
                                )])),
                                None,
                            )
                            .await;
                        println!(
                            "Response after sending back tool result: {:?}\n",
                            tool_response
                        );
                        if let Ok(content) = tool_response {
                            response = content.content
                        }
                    }
                    ContentBlock::Image(_) => unimplemented!("Received unexpected Image response"),
                    ContentBlock::ToolResult(_) => unreachable!("Received ToolResult from model"),
                    _ => unimplemented!("Unknown response received from model"),
                }
            }
            if !tool_used {
                break;
            }
        }
        println!("Final output: {:?}", output);
        println!("\n============\n\n");
        Ok(output.join("\n\n"))
    }
}

#[async_trait]
impl<'a> Node for AgentExecutor<'a> {
    type Input = String;
    type Output = String;

    async fn process(&self, input: Self::Input) -> Result<Self::Output, AnchorChainError> {
        match &self.llm {
            AgentModel::Claude3_5(claude) => self.run_claude_agent(claude, input).await,
        }
    }
}

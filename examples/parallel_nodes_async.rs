use std::collections::HashMap;

use anchor_chain::{
    chain::ChainBuilder,
    models::{claude_3::Claude3Bedrock, openai::OpenAIModel},
    parallel_node::ParallelNode,
    prompt::Prompt,
};
use anyhow::Result;
use futures::future::BoxFuture;

#[tokio::main]
async fn main() -> Result<()> {
    let gpt3 =
        Box::new(OpenAIModel::new_gpt3_5_turbo("You are a helpful assistant".to_string()).await);
    let claude3 = Box::new(Claude3Bedrock::new("You are a helpful assistant".to_string()).await);

    let select_output_fn = Box::new(
        |outputs: Vec<String>| -> BoxFuture<Result<String, anyhow::Error>> {
            Box::pin(async move {
                let labeled_outputs = outputs
                    .iter()
                    .enumerate()
                    .map(|(i, output)| format!("<output{}>\n{}\n</output{}>", i + 1, output, i + 1))
                    .collect::<Vec<String>>();
                let decision_chain = ChainBuilder::new()
                    .link(Prompt::new("Decide which input is the most helpful. Return only the output within between the <outputN></outputN> tags without outputting the tags themselves. Ensure the output is returned verbatim without any comentary.\n{{ input }}"))
                    .link(OpenAIModel::new_gpt3_5_turbo_instruct().await)
                    .build();
                decision_chain
                    .process(HashMap::from([(
                        "input",
                        labeled_outputs.join("\n\n").as_str(),
                    )]))
                    .await
            })
        },
    );

    let chain = ChainBuilder::new()
        .link(Prompt::new("{{ input }}"))
        .link(ParallelNode::new(vec![gpt3, claude3], select_output_fn))
        .build();

    let output = chain
        .process(HashMap::from([(
            "input",
            "Write a hello world program in Rust",
        )]))
        .await?;
    println!("{}", output);

    Ok(())
}

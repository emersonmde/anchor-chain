[![Rust Build](https://github.com/emersonmde/anchor-chain/actions/workflows/rust.yml/badge.svg)](https://github.com/emersonmde/anchor-chain/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

# Anchor Chain

Work In Progress

Anchor Chain is a Rust framework for working with LLMs inspired LangChain. Anchor Chain
allows LLM actions, document retrievers, and other processing steps to be chained 
together to produce an end to end workflow.

```text
              |>   |>   |>
              )_)  )_)  )_)
             )___))___))___)
            )____)____)_____)
          _____|____|____|____
---------\                 0 /--------
  ^^^^^ ^^^^^^^^^^^^^^^^^^^0^
                           0
                           0
                           0
                           |
                         \_⟂_/
```

## Why Anchor Chain?

One problem I faced with LangChain was the lack of documentation and the quickly 
evolving API that made it difficult to learn and work with. This problem was 
exacerbated by the lack of type safety resulting in trial and error with long 
iteration times when calling long running LLM steps. 

The expressive type system of Rust allows for statically typed chains giving 
clear compile time feedback and in editor type hints. Also having built in 
support for async runtimes opens the possibility of efficient parallel 
processing of nodes in complex chains.

## Getting Started

Chains can be created using `ChainBuilder` and invoked with the `.process()`
function:

```rust
use anchor_chain::{
    chain::ChainBuilder,
    prompt::Prompt,
    models::openai::OpenAIModel,
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let chain = ChainBuilder::new()
        .link(Prompt::new("{input}"))
        .link(OpenAIModel::new_gpt4_turbo("You are a helpful assistant".to_string()).await)
        .build();

    let output = chain
        .process("Write a hello world program in Rust".to_string())
        .await?;
    println!("Output:\n{}", output);

    Ok(())
}
```

See more examples [here](examples)

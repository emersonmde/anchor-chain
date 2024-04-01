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

While `anchor_chain` is still under development, here's a brief glimpse into 
how a chain might be constructed and executed:

```rust
use anchor_chain::{
    chain::ChainBuilder,
    models::Gpt3_5Turbo,
    prompt::Prompt,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let prompt_processor = Prompt::new("{input}");
    let chain = ChainBuilder::new(prompt_processor)
        .link(Gpt3_5Turbo::new("You are a helpful assistant".to_string()).await)
        .build();

    let result = chain
        .process("Write a hello world program in Rust".to_string())
        .await?;

    println!("Result: {}", result);

    Ok(())
}
```

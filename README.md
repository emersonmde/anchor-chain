# Anchor Chain

Work In Progress

`anchor_chain` is a Rust framework under development, aimed at creating and executing chains of asynchronous processing steps with a specific focus on interacting with Large Language Models (LLMs) like GPT-3.5 and Claude 3. Inspired by the flexibility of LangChain but built upon Rust's robust type system, `anchor_chain` aspires to offer compile-time type safety, efficiency, and performance in processing pipelines.

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

In the landscape of data processing and AI applications, particularly those involving LLMs, there's a growing need for frameworks that can not only handle complex processing chains but also do so with high efficiency and reliability. `anchor_chain` is being developed to address this need, leveraging Rust's strengths:

- **Type Safety**: Utilizing Rust's statically typed system to ensure correctness across the processing chain at compile-time.
- **Performance**: Capitalizing on Rust's performance characteristics for handling both IO-bound and CPU-bound tasks efficiently.
- **Concurrency**: Making the most of Rust's concurrency model to facilitate non-blocking, concurrent processing pipelines.

## Current Components

The framework is structured around several key components, each serving a distinct role within the processing chain:

- **`chain::ChainBuilder`**: A tool for constructing type-safe processing chains, ensuring compatibility between consecutive processors.
- **`models`**: This module includes predefined models and processors, facilitating easy integration with popular LLMs via services like AWS Bedrock.
- **`prompt::Prompt`**: Designed for dynamic prompt handling, allowing for straightforward manipulation and customization of prompts sent to LLMs.

## Getting Started

While `anchor_chain` is still under development, here's a brief glimpse into how a processing chain might be constructed and executed:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use anchor_chain::{
        chain::ChainBuilder,
        models::Gpt3_5Turbo,
        prompt::Prompt,
    };

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

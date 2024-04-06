[![Rust Build](https://github.com/emersonmde/anchor-chain/actions/workflows/rust.yml/badge.svg)](https://github.com/emersonmde/anchor-chain/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

# Anchor Chain

Anchor Chain is a Rust framework designed to simplify the orchestration of 
workflows involving Large Language Models (LLMs). Inspired by LangChain, 
Anchor Chain provides a set of easy-to-use and extensible building blocks that 
enable developers to create robust and efficient LLM-based applications quickly. 
The framework prioritizes type safety, processing efficiency, and flexibility 
through its carefully designed APIs and abstractions.

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

## Features

- **Statically Typed Chains**: Anchor Chain leverages Rust's type system to 
provide statically typed chains, catching potential type mismatches at compile 
time.

- **Async Runtime for Parallel Execution**: Built with Rust's async runtime, 
Anchor Chain allows for efficient parallel processing of nodes in complex 
chains.

- **Extensibility through the Node Trait**: The `Node` trait allows developers 
to create custom nodes tailored to their specific use cases, enabling seamless 
integration into the chain.

- **Support for Popular LLMs**: Anchor Chain provides built-in support for 
popular LLMs, such as OpenAI's GPT models and Anthropic's Claude, abstracting 
away API details to provide a common interface.

- **Parallel Node Execution**: The `ParallelNode` struct enables parallel 
execution of multiple nodes, leveraging concurrency to improve overall chain 
performance.

## Why Anchor Chain?

Anchor Chain addresses some of the challenges developers face when working with 
LangChain, such as the lack of comprehensive documentation, rapidly evolving 
APIs, and the absence of type safety. These issues can lead to a steep learning 
curve and time-consuming trial and error when building LLM-based applications.

By leveraging Rust's expressive type system, Anchor Chain provides statically 
typed chains that offer clear compile-time feedback and in-editor type hints. 
This helps catch potential errors early in the development process and 
facilitates a more efficient workflow.

Additionally, Anchor Chain's built-in support for async runtimes enables 
efficient parallel processing of nodes in complex chains. This can significantly 
reduce the overall execution time of LLM-based workflows, making Anchor Chain an 
attractive choice for performance-critical applications.

## Getting Started

To get started with Anchor Chain, add the following dependency to your 
`Cargo.toml` file:

```toml
[dependencies]
anchor-chain = "0.1.1"
```

Then, you can create chains using the `ChainBuilder` and invoke them with the 
`.process()` function:

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

For more examples and detailed documentation, please refer to the 
[examples](examples) directory and the [API documentation](https://errorsignal.dev/anchor-chain/anchor_chain/).

## Contributing

Contributions to Anchor Chain are welcome! If you encounter any issues, have 
suggestions for improvements, or would like to contribute new features, please 
open an issue or submit a pull request on the 
[GitHub repository](https://github.com/emersonmde/anchor-chain).

## TODO

While Anchor Chain is usable today, it's still a work in progress. Below are a 
list of future features that will be implemented before a 1.0 release.

- Add support for Tera templates in prompts
- Create a node to categorize and log input for better input observability
- Add error handling strategy that will use a backup model if the primary fails
- Output validation node that can attempt to fix unexpected or incomplete outputs
- Add OpenTelemetry support
- Add feature flag for Askama prompts

## License

Anchor Chain is released under the [MIT License](LICENSE).

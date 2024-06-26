[![Rust Build](https://github.com/emersonmde/anchor-chain/actions/workflows/rust.yml/badge.svg)](https://github.com/emersonmde/anchor-chain/actions/workflows/rust.yml)
[![Docs](https://img.shields.io/docsrs/anchor-chain/latest)](https://docs.rs/anchor-chain)
[![crates](https://img.shields.io/crates/v/anchor-chain.svg)](https://crates.io/crates/anchor-chain)
[![License](https://img.shields.io/crates/l/anchor-chain.svg)](LICENSE)

# Anchor Chain

Anchor Chain is a Rust framework designed to simplify the orchestration of
workflows involving Large Language Models (LLMs). Inspired by LangChain,
Anchor Chain provides a set of easy-to-use and extensible building blocks that
enable developers to create LLM-based applications quickly.
The framework prioritizes type safety, processing efficiency, and composability
by design.

Anchor Chain is currently under active development and its API is
subject to change. The framework has not yet reached a stable 1.0 release, and
backward compatibility is not guaranteed for versions prior to 1.0.

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

- **Tracing Support**: Anchor Chain integrates with the `tracing` crate to
  provide detailed logs and diagnostics, helping developers understand the
  execution flow of their chains.

- **OpenSearch Integration**: Anchor Chain supports indexing and searching
  documents with OpenSearch vector indexes, enabling Retrieval-Augmented 
  Generation (RAG) workflows.

## Supported Models

Currently, Anchor chain supports OpenAI's GPT3.5 Turbo, GPT4 Turbo, and GPT3.5
Instruct through the use of the
[async-openai](https://crates.io/crates/async-openai) crate as well as
Claude 3 Sonnet through the [AWS Bedrock API](https://aws.amazon.com/bedrock/).
There are plans to add support for Mistral and other models supported by AWS
Bedrock as well as support for connecting to a locally running
[Ollama](https://ollama.com/) or [llama.cpp](https://github.com/ggerganov/llama.cpp)
model through the provided REST APIs.

## Why Anchor Chain?

Anchor Chain addresses some of the challenges developers face when working with
LangChain, such as the lack of documentation, ambiguous APIs, and the lack of
type safety. These issues can lead to time-consuming trial and error when
building LLM-based applications.

By leveraging Rust's expressive type system, Anchor Chain provides statically
typed chains that offer clear compile-time feedback and in-editor type hints.
This helps catch potential errors early in the development process and
facilitates a more efficient workflow.

Additionally, Anchor Chain's built-in support for async runtimes enables
efficient parallel processing of nodes in complex chains. This can significantly
reduce the overall execution time of LLM-based workflows, making Anchor Chain an
attractive choice for performance-critical applications.

## Getting Started

To get started with Anchor Chain, add `anchor-chain` and `tokio` to your
`Cargo.toml` file:

```toml
[dependencies]
anchor-chain = "0.1.1"
tokio = "1.38.0"
```

Then, you can create chains using the `ChainBuilder` and invoke them with the
`.process()` function:

```rust,no_run
use anchor_chain::{
    chain::ChainBuilder,
    nodes::prompt::Prompt,
    models::openai::OpenAIModel,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let chain = ChainBuilder::new()
        .link(Prompt::new("{{ input }}"))
        .link(OpenAIModel::new_gpt4_turbo("You are a helpful assistant").await)
        .build();

    let output = chain
        .process(HashMap::from([(
            "input",
            "Write a hello world program in Rust",
        )]))
        .await
        .expect("Error processing chain");
    println!("Output:\n{}", output);
}
```

For more examples and detailed documentation, please refer to the
[examples](examples) directory and the [API documentation](https://docs.rs/anchor-chain).

## Contributing

Contributions to Anchor Chain are welcome! If you encounter any issues, have
suggestions for improvements, or would like to contribute new features, please
open an issue or submit a pull request on the
[GitHub repository](https://github.com/emersonmde/anchor-chain).

## License

Anchor Chain is released under the [MIT License](LICENSE).

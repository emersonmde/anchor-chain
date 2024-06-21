# Changelog for Anchor Chain

## v0.5.0
### New Features
- **Workspace Configuration**: Added a `workspace` configuration in `Cargo.toml` to manage the main crate and the new `anchor-chain-macros` crate.
- **Feature Flags**: Introduced new feature flags (`macros`, `tracing`, `openai`, `opensearch`, `bedrock`, `ollama`) to modularize dependencies.
- **Macro Support**: Introduced the `anchor-chain-macros` crate with procedural macros for tool registration and state management.
- **Stateful Nodes**: Added support for stateful nodes via `StatefulLinkedChainBuilder` and `StateManager`.
- **Tool Registry**: Implemented a `ToolRegistry` for dynamic tool execution and schema management.
- **Stateless and Stateful Traits**: Added `Stateless` and `Stateful` traits to distinguish between nodes that require state management and those that don't. The new `anchor-chain-macros` crate includes a `Stateless` derive macro to simplify implementing the `Stateless` trait for custom nodes.
- **Examples**: Expanded the examples directory with detailed use cases demonstrating new features like tool usage, stateful chains, and various integrations with OpenAI and OpenSearch.

### Breaking Changes
- **ChainBuilder API**: The `ChainBuilder` API has been overhauled to distinguish between stateless and stateful nodes.
    - Use `link` for adding stateless nodes.
      - Custom stateless nodes must implement the `Stateless` trait (or use the new `derive(Stateless)` macro).
    - Use `link_with_state` for adding stateful nodes that require `StateManager`.
      - Custom stateful nodes must implement the `Stateful` trait
- **Node Trait**: Nodes now need to implement either the `Stateless` or `Stateful` trait. Custom nodes can derive the `Stateless` trait using the new procedural macro in the `anchor-chain-macros` crate.
- **OpenAI Model Initialization**: The initialization methods for `OpenAIModel` have changed to accept string slices for system prompts.
  ```rust
  // Old
  OpenAIModel::new_gpt3_5_turbo("You are a helpful assistant".to_string()).await
  // New
  OpenAIModel::new_gpt3_5_turbo("You are a helpful assistant").await
  ```
- **Logger Node**: The `Logger` node now uses a string slice for the prefix.
  ```rust
  // Old
  Logger::new("Prefix".to_string())
  // New
  Logger::new("Prefix")
  ```
- **Document Conversion**: Added `From<&str>` implementation for `Document` for more flexible usage.

### Dependency Updates
- **async-openai**: Updated to version `0.23.2` and made it optional.
- **aws-sdk-bedrockruntime**: Updated to version `1.34.0` and made it optional.
- **tracing**: Updated to version `0.1.40` and made it optional.
- **reqwest**: Updated to version `0.12.4` and made it optional.
- **opensearch**: Added as an optional dependency with version `2.2.0`.

### Internal Improvements
- **State Management**: Introduced `StateManager` for handling state across nodes in a chain.
- **Proc Macros**: Added procedural macros for deriving `Stateless` and implementing tool registration.

### Migration Guide
1. **Update Cargo.toml**: Ensure your `Cargo.toml` reflects the new feature flags and optional dependencies.
2. **Refactor Chains**: Update your chain definitions to use `link` for stateless nodes and `link_with_state` for stateful nodes.
3. **Node Traits**: Implement `Stateless` or `Stateful` traits for your custom nodes. Use the `Stateless` derive macro from the `anchor-chain-macros` crate to simplify this process.
4. **Logger Node**: Refactor any usage of the `Logger` node to use string slices for prefixes.
5. **OpenAI Model Initialization**: Adjust the initialization of `OpenAIModel` instances to pass system prompts as string slices.

### Acknowledgements
A big thank you to all the contributors who made this release possible. Your hard work and dedication are greatly appreciated. If you encounter any issues or have suggestions for future improvements, please open an issue or submit a pull request on our [GitHub repository](https://github.com/emersonmde/anchor-chain).
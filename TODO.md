## Tech Debt
- Finish tool usage
  - Need to support passing tool definitions to LLMs

## Features
- Repeat a Node/Chain until a stop state is detected
- Create an Agent type that will use tools in a OODA (Observe, Orient, Decide, Act) loop
- Create a node to categorize and log input for better input observability
- Output validation node that can attempt to fix unexpected or incomplete outputs
- Support for [rustformers/llm](https://github.com/rustformers/llm) to utilize local models
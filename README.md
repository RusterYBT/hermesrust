# ai-agent

A Rust workspace for building an agent with reusable LLM clients, memory, and core orchestration.

## Workspace crates

- `crates/llm-client`: reusable language model client abstractions
- `crates/agent-memory`: memory storage and retrieval
- `crates/agent-core`: agent loop, context, and tools
- `apps/agent-cli`: command-line application

## Run

```bash
cargo run -p agent-cli
```

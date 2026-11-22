# AgentForge

> Forge an AI Agent from scratch.

**AgentForge** (智能体锻造工坊) is an open-source project for learning and building AI Agents from first principles. Instead of wrapping an existing heavy-weight agent framework, AgentForge starts from a minimal core and gradually adds tool calling, memory, planning, multi-step execution, observability, and evaluation.

Every layer is designed to be simple, transparent, and replaceable — so you can truly understand how an AI Agent works and how to modify it.

## Why AgentForge?

There are many Agent frameworks available, but knowing how to *use* one is not the same as knowing how to *build* one.

AgentForge aims to answer a simple question:

**If I had to build an AI Agent from scratch, how would I do it?**

The project follows a progressive path:

- Build a minimal Agent Loop
- Add tool calling
- Add memory and state
- Add planning and task decomposition
- Add observability, evaluation, and multi-agent collaboration

Each stage comes with runnable code and documentation, making the project suitable for learning, teaching, prototyping, and further development.

## Features

- **Built from scratch** — no heavy Agent framework required; the core logic is readable and hackable
- **Progressive construction** — implement the minimal core, tools, memory, planning, and evaluation step by step
- **Pluggable design** — LLMs, tools, memory, and planners can be replaced independently
- **Observable** — record thoughts, tool calls, and execution results at every step
- **Evaluable** — includes task sets and evaluation scripts to measure Agent performance
- **Practical** — provides runnable examples and a real project structure, not just concepts

## Architecture

```text
AgentForge
├── core          # Agent Loop, state, and execution core
├── llm           # LLM adapter layer
├── tools         # Tool registration, invocation, and permission control
├── memory        # Short-term and long-term memory
├── planner       # Planning and task decomposition
├── runtime       # Runtime environment and execution sandbox
├── observability # Logging, tracing, and visualization
├── evals         # Task sets and evaluation
└── examples      # Examples from minimal Agent to full Agent

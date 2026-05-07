# Scale Descent Protocol (SDP)

**SDP teaches agents to shrink into the causal layer before they act.**

Modern AI systems mostly scale upward: more context, more tools, more agents, more compute.

SDP explores the opposite direction:

> Agents should also scale downward — from macro goals into micro and pico causal transitions — before committing to action.

## Core Idea

A task can look correct at the output level while being invalid at the causal level.

SDP defines a Rust-first protocol for recursive scale descent:

```text
Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute
```

The protocol decomposes agentic work not only by function, but by causal depth.

## Scale Levels

- **Macro** — the visible task, goal, or user intent.
- **Meso** — system components, workflows, tools, or subsystems.
- **Micro** — causal transitions, state changes, provenance, and permissions.
- **Pico** — atomic events: tool calls, claims, anchors, IDs, timestamps, and evidence units.
- **Collapse** — evidence-based selection of the most causally supported hypothesis.

## Why It Matters

Agentic AI safety cannot rely only on final-answer evaluation.

A system may produce a useful output while violating provenance, permission boundaries, causal ancestry, commit-before-execute discipline, or trace integrity.

SDP makes those violations inspectable before action.

## Core Invariant

> An agent may only execute after descending to the required causal scale and validating sufficient evidence for commit.

In plain language:

```text
Do not execute from the cosmic view.
Descend into the causal layer.
Inspect the micro-transition.
Validate the pico-evidence.
Then commit.
```

## Repository Status

This repository contains the first Rust foundation:

- `sdp-core` — protocol types, validation rules, evidence collapse logic.
- `sdp-cli` — minimal deterministic CLI for inspecting a scale descent JSON file.
- `examples/` — example descent run and trace records.
- `docs/` — protocol vocabulary and invariants.

No LLM calls are implemented in this foundation. The first version is intentionally deterministic.

## CLI Example

```bash
cargo run -p sdp-cli -- inspect examples/simple_descent.json
```

Expected output shape:

```text
Task: demo-task-001
Goal: Find why an apparently valid agent output has invalid causal ancestry
Selected hypothesis: H3
Confidence: 0.92
Decision: Commit
Reason: Selected highest-scoring hypothesis after evidence collapse
```

## Roadmap

- [x] Define scale levels and core protocol vocabulary.
- [x] Add deterministic Rust models and validation primitives.
- [x] Add evidence collapse scoring.
- [x] Add minimal CLI inspection flow.
- [ ] Add JSON schema files for external tool compatibility.
- [ ] Add trace replay integration.
- [ ] Add CML/T-Trace adapters.
- [ ] Add typed state-machine enforcement for `Commit -> Execute`.
- [ ] Add WASM and Python bindings.

## Positioning

SDP is not another multi-agent framework.

It is a protocol primitive for **scale-aware agentic reasoning**: agents descend from macro goals into micro and pico causal evidence before committing to action.

> Do not just scale agents up. Teach them to scale down to the cause.

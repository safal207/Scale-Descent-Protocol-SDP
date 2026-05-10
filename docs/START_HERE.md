# Start Here — Scale Descent Protocol (SDP)

Scale Descent Protocol is a Rust-first protocol foundation for scale-aware agentic reasoning.

The core idea is simple:

> Before an agent acts, it should descend from the visible goal into the causal layer where evidence, permission, provenance, and responsibility can be inspected.

In short:

```text
Do not execute from the cosmic view.
Descend into the causal layer.
Inspect the micro-transition.
Validate the pico-evidence.
Then commit.
```

## What SDP is

SDP is not another multi-agent framework.

It is a protocol primitive for deciding whether an agent has inspected enough causal evidence before moving from intent to action.

The current repository contains a deterministic Rust foundation:

- `crates/sdp-core` — protocol types, validation rules, and evidence collapse logic.
- `crates/sdp-cli` — minimal CLI for inspecting a scale descent JSON file.
- `examples/` — deterministic example descent runs.
- `docs/` — protocol vocabulary, research reflection, and future integration notes.

No LLM calls are implemented in this foundation. That is intentional: the first version is deterministic and inspectable.

## Why agents need to scale downward

Most modern AI systems scale upward:

```text
more context -> more tools -> more agents -> more compute
```

SDP explores the opposite direction:

```text
macro goal -> meso workflow -> micro transition -> pico evidence
```

A task can look correct at the final-answer level while being invalid underneath:

- missing provenance,
- invalid permission chain,
- unsupported factual claim,
- wrong causal parent,
- skipped commit step,
- unverified tool-call evidence,
- trace branch mismatch,
- execution before responsibility is established.

SDP makes those failures inspectable before action.

## Descent chain

The main SDP flow is:

```text
Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute
```

| Stage | Meaning |
|---|---|
| Macro | The visible task, goal, or user intent. |
| Meso | The workflow, tool, subsystem, or plan structure. |
| Micro | The causal transition: state change, permission, provenance, responsibility. |
| Pico | Atomic evidence units: tool calls, IDs, timestamps, claims, anchors, hashes. |
| Collapse | Select the most causally supported hypothesis from available evidence. |
| Commit | Accept the selected hypothesis and make it eligible for execution. |
| Execute | Perform the action only after valid commit. |

## Core invariant

The main invariant is:

> An agent may only execute after descending to the required causal scale and validating sufficient evidence for commit.

Operationally:

```text
No descent -> no commit.
No commit -> no execute.
Rejected hypothesis -> no execute.
Missing evidence -> hold or reject, not silent execute.
```

## Run the CLI demo

From the repository root:

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

The CLI demo shows the current deterministic foundation:

```text
example descent JSON -> evidence collapse -> selected hypothesis -> decision + reason
```

It does not call an LLM and does not execute real tools.

## Local validation

Build and test the workspace:

```bash
cargo build --workspace
cargo test --workspace
```

Run the CLI example:

```bash
cargo run -p sdp-cli -- inspect examples/simple_descent.json
```

## Safe contribution areas

Good first contribution areas:

- documentation improvements,
- CLI walkthroughs,
- example descent JSON files,
- JSON Schema for descent runs,
- glossary and vocabulary cleanup,
- invariant examples,
- non-claims / evidence-boundary wording,
- community reflection scenarios,
- benchmark/falsifiability ideas,
- integration roadmap notes.

## Changes that need deeper review

Discuss these before implementation:

- core decision semantics,
- evidence scoring or collapse logic,
- confidence thresholds,
- `Commit -> Execute` state-machine enforcement,
- changes to the descent chain vocabulary,
- JSON format compatibility,
- future adapters for agent frameworks,
- future CML / trace replay integration,
- claims about AI safety guarantees,
- production enforcement or formal verification claims.

## Recommended reading order

1. `README.md` — core idea and CLI entrypoint.
2. `examples/simple_descent.json` — current deterministic example.
3. `docs/community-reflection-task.md` — reflection prompt if present.
4. `docs/research-reflections.md` — research notes if present.
5. Issue #1 — community reflection on causal subsidiarity and mobile causal states.
6. Open issues for Start Here, CLI demo, JSON Schema, invariants, and integration roadmap.

## Product boundary

Current safe positioning:

> SDP is an early deterministic Rust foundation for scale-aware causal inspection before agent execution.

SDP does **not** currently claim:

- production AI safety certification,
- complete agent sandboxing,
- formal verification,
- mature LLM framework integration,
- runtime enforcement across real tools,
- universal overhead bounds,
- replacement for tracing systems,
- replacement for policy engines,
- replacement for human review.

## Evidence principle

Keep these categories separate:

```text
protocol idea != deterministic demo != production enforcement
```

Good:

> The current CLI can inspect a deterministic descent JSON and select a supported hypothesis.

Avoid:

> SDP guarantees safe agent behavior in production.

Good:

> SDP defines a direction for future `Commit -> Execute` enforcement.

Avoid:

> SDP already enforces execution safety across arbitrary tool-using agents.

## Good first issue path

Recommended order for contributors:

1. Run the CLI demo locally.
2. Add or improve a documented example descent scenario.
3. Improve the five-minute CLI walkthrough.
4. Help define JSON Schema for descent runs.
5. Contribute a scenario to the community reflection issue.
6. Add invariant examples and non-claims documentation.

## Maintainer principle

A strong SDP contribution should preserve three things:

1. **Causal precision** — do not blur Macro/Meso/Micro/Pico levels.
2. **Determinism first** — examples should be inspectable and reproducible without external APIs.
3. **Evidence discipline** — do not turn protocol direction into unsupported production guarantees.

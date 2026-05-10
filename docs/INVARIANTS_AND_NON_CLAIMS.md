# SDP Invariants and Non-Claims

Scale Descent Protocol is built around one practical principle:

> Before action, descend to the cause.

In Russian:

> Перед действием спустись к причине.

This document separates SDP's core invariants from the guarantees the project does **not** claim yet.

## Why this page exists

SDP is a safety-oriented protocol primitive. That makes wording important.

The project should be clear about:

- what must always hold inside the protocol,
- what the current deterministic Rust foundation can demonstrate,
- what future integrations may enforce,
- what the project does not yet prove.

The core distinction is:

```text
visible correctness != causal validity
```

A useful answer can still be causally invalid if the evidence, provenance, permission, or commit path underneath it is broken.

## Core descent chain

SDP uses this chain:

```text
Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute
```

| Stage | Meaning |
|---|---|
| Macro | Visible task, goal, or user intent. |
| Meso | Workflow, subsystem, plan, or tool path. |
| Micro | Causal transition: state change, permission, provenance, responsibility. |
| Pico | Atomic evidence: tool calls, anchors, IDs, timestamps, claims, hashes. |
| Collapse | Evidence-based selection of the most causally supported hypothesis. |
| Commit | Accept the selected hypothesis as eligible for execution. |
| Execute | Perform action only after valid commit. |

## Invariant 1 — No descent, no commit

An agent must not commit directly from a Macro-level view when the task requires deeper causal inspection.

```text
Macro-only evidence + required Pico scale => not enough for Commit
```

A final answer can look useful, but if the task requires Pico evidence, Macro evidence alone is insufficient.

## Invariant 2 — No commit, no execute

Execution must only happen after a valid Commit decision.

```text
Commit must precede Execute.
```

Any path that allows `Execute` without `Commit` is protocol-invalid.

## Invariant 3 — Reject never executes

A rejected hypothesis must never lead to execution.

```text
Reject -> no Execute
```

Reject means the causal evidence is insufficient, invalid, contradictory, or inadmissible.

## Invariant 4 — Missing evidence must not silently pass

Missing required evidence must produce an explicit decision such as `Hold` or `Reject`.

```text
missing required evidence -> Hold | Reject
missing required evidence -> not silent Execute
```

The system must not treat absence of evidence as permission to act.

## Invariant 5 — Collapse must explain selection

The Collapse step must produce an inspectable reason for selecting a hypothesis.

At minimum, a collapse result should include:

- selected hypothesis id,
- confidence or score,
- decision,
- reason,
- evidence basis.

A useful collapse result answers:

```text
Why this hypothesis, and why now?
```

## Invariant 6 — Pico evidence matters when Pico is required

If a task requires Pico-level inspection, evidence must reach Pico scale or the system must hold/reject.

Examples of Pico evidence:

- concrete tool-call id,
- trace event id,
- anchor id,
- timestamp,
- hash,
- claim reference,
- permission record,
- exact parent transition.

## Invariant 7 — Causal ancestry must remain inspectable

A committed decision should preserve the causal path that led to it.

The system should be able to answer:

```text
Which evidence supported this decision?
Which transition allowed commit?
Which record made execute admissible?
```

If the ancestry cannot be inspected, the decision is weak or inadmissible depending on required scale.

## Invariant 8 — Visible correctness is not enough

An output that satisfies the user request can still be invalid if its causal path is broken.

Example:

```text
The answer is useful.
The commit references a missing anchor.
Execute happens after an inadmissible commit.
=> visible correctness, causal invalidity
```

This is the central SDP lesson.

## Invariant 9 — Required scale must be explicit

The protocol should know what scale is required for a given task.

```text
required_scale = Macro | Meso | Micro | Pico
```

Without a required scale, the system cannot reliably decide whether enough descent has happened.

## Invariant 10 — Protocol decisions must be conservative under uncertainty

When evidence is incomplete, stale, conflicting, or below required scale, SDP should prefer conservative decisions.

```text
uncertain evidence -> Hold or Reject before Execute
```

This does not mean every uncertain task must fail forever. It means uncertainty must be represented explicitly before action.

## Decision vocabulary

Suggested decision semantics:

| Decision | Meaning |
|---|---|
| `Hold` | More evidence or descent is required. |
| `Reject` | Evidence is invalid, insufficient, or inadmissible. |
| `Commit` | Evidence is sufficient to accept a hypothesis as execution-eligible. |
| `Execute` | Action is allowed only after valid Commit. |

The current CLI demo prints `Commit` after deterministic evidence collapse. Future runtime integrations may enforce the transition from `Commit` to `Execute`.

## Current implementation boundary

The current repository provides:

- deterministic Rust models,
- evidence collapse scoring,
- a minimal CLI inspection flow,
- example descent JSON,
- documentation for protocol vocabulary and contribution direction.

The current repository does **not** yet provide:

- live LLM framework integration,
- production runtime enforcement,
- formal verification,
- external tool sandboxing,
- cross-framework policy enforcement,
- full trace replay integration,
- CML/T-Trace adapter implementation,
- complete `Commit -> Execute` runtime guard.

## Non-claims

SDP does **not** currently claim:

- production AI safety certification,
- complete prevention of unsafe agent behavior,
- full agent sandboxing,
- formal proof of correctness,
- formal proof of safety,
- stable pre-1.0 schema compatibility,
- stable pre-1.0 API compatibility,
- mature integration with LangGraph, AutoGen, CrewAI, or Pydantic AI,
- enforcement across arbitrary tool-using agents,
- universal low-overhead operation,
- replacement for trace systems,
- replacement for policy engines,
- replacement for human review,
- replacement for security isolation,
- guarantee that every invalid causal chain will be detected.

## Safe claim language

Good:

> SDP is an early deterministic Rust foundation for scale-aware causal inspection before future agent execution enforcement.

Avoid:

> SDP guarantees safe agent execution.

Good:

> The current CLI demo can inspect a descent JSON and select the strongest supported hypothesis.

Avoid:

> SDP already enforces causal safety across live LLM agents.

Good:

> SDP explores `Commit -> Execute` discipline as a protocol invariant.

Avoid:

> SDP is a production-ready runtime guard.

## Human-readable version

The protocol can be read as a technical version of a simple discipline:

```text
Do not act from the surface.
Find the cause.
Check the evidence.
Only then commit.
```

Or shorter:

```text
Before action, descend to the cause.
```

## Relationship to the CLI demo

The current CLI demo illustrates this distinction:

```text
A useful answer can still be causally invalid.
```

In `examples/simple_descent.json`, hypothesis `H3` wins because the evidence points to a missing Pico-level anchor and an invalid Micro-level commit/execute path.

That is a deterministic demo of the principle, not a production safety guarantee.

## Maintainer checklist for future PRs

Before merging protocol-affecting changes, ask:

1. Does this preserve `No commit -> no execute`?
2. Does this preserve `Reject -> no execute`?
3. Does this represent missing evidence explicitly?
4. Does this keep Macro/Meso/Micro/Pico distinct?
5. Does this avoid overstating production safety claims?
6. Does this keep deterministic demos reproducible?
7. Does this explain why a hypothesis was selected?
8. Does this preserve the difference between visible correctness and causal validity?

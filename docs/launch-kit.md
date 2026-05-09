# SDP Launch Kit

Reusable public launch copy for Scale Descent Protocol (SDP).

## Core Message

> Do not just ask whether an AI output is correct. Ask whether the action had a valid causal path.

## X / Twitter Posts

### Post 1

Modern AI agents mostly scale outward:

more agents
more tools
more memory
more context

SDP explores the opposite direction:

scale inward.

Macro -> Micro -> Pico -> Commit -> Execute

Do not execute from the cosmic view.
Scale down to the cause.

### Post 2

An AI agent output can look useful while the causal path behind it is invalid.

Missing provenance.
Broken permission boundary.
Unverified tool result.
Premature Execute.

SDP is a Rust-first protocol for catching that class of failure before action.

### Post 3

Scalable agent safety may not come from wiring every agent to every context.

It may come from moving causal state to the local scale where evidence can be checked.

Move state.
Inspect locally.
Commit with evidence.
Then execute.

### Post 4

I am looking for examples where an AI agent produced a useful-looking result but had an invalid causal path.

Examples may become SDP benchmark fixtures.

Question:

What should an agent never be allowed to Execute without Micro/Pico evidence?

## LinkedIn Post

Agent safety is not only about final-answer evaluation.

A final answer can look correct while the path that produced it violates provenance, permission boundaries, trace integrity, or evidence requirements.

Scale Descent Protocol (SDP) explores a complementary approach: before execution, move the relevant causal state to the scale where action becomes verifiable.

The scale path is:

```text
Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute
```

The core invariant:

> An agent may only execute after descending to the required causal scale and validating sufficient evidence for commit.

I am collecting examples where agent outputs looked useful but had invalid causal ancestry. These examples can become benchmark fixtures and research cases.

## Hacker News / Show HN Draft

Title:

```text
Show HN: SDP - a Rust protocol for checking causal paths before AI agents execute
```

Body:

```md
I am building Scale Descent Protocol (SDP), a Rust-first protocol for checking whether an AI agent action has a valid causal path before execution.

The idea is that agent systems should not only scale outward with more tools, memory, context, and agents. They should also scale inward toward the causal layer where provenance, permissions, evidence, and trace integrity can be inspected.

The basic path is:

Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute

Core invariant:

An agent may only execute after descending to the required causal scale and validating sufficient evidence for commit.

The repo currently includes a small Rust core, CLI, examples, tests, schemas, and research notes.

I would especially value criticism and concrete examples of agent failures where the final output looked useful, but the causal path was invalid.
```

## Failure Example Template

```md
### Scenario
Short description of the agent task.

### Macro View
What the agent believed it was doing.

### Hidden Causal Problem
What was invalid underneath the output.

### Required Descent
Which scale should have been inspected: Meso, Micro, or Pico.

### Evidence Needed
What evidence should have been present before Commit.

### Expected SDP Decision
Reject, Hold, Commit, or Execute.
```

## Call to Action

> Bring one agent failure where the output looked fine but the causal path was invalid.

## Non-Negotiable Tone

- humble
- testable
- open to criticism
- protocol-first
- no safety overclaiming
- no quantum hype

## One-Line Ending

> Scale down to the cause.

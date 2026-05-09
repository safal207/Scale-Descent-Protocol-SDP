# Community Reflection Task: Causal Subsidiarity and Mobile Causal States

This is an open reflection task for researchers, engineers, reviewers, and builders interested in Scale Descent Protocol (SDP).

## Central Question

Can agentic systems become safer and more scalable if they move causal state to the lowest valid scale before action?

## Background

SDP proposes that agents should not execute directly from a high-level task description. Instead, an agent should descend through scale levels:

```text
Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute
```

The key idea is causal subsidiarity:

> Action should be authorized at the lowest causal scale where evidence, permission, and responsibility are sufficient.

## Reflection Prompts

1. Where do current agent frameworks execute too early from a macro-level view?
2. What examples show a useful output with invalid causal ancestry?
3. What is the smallest useful definition of a causal state?
4. When should a system stop descending and commit?
5. What evidence should be mandatory before Execute?
6. How can SDP avoid unnecessary overhead?
7. What failures should be represented as Reject, Hold, Commit, or Execute?
8. What benchmarks would make the idea falsifiable?
9. How should SDP integrate with LangGraph, AutoGen, CrewAI, trace logs, or causal memory layers?
10. What real-world domains need causal subsidiarity most?

## Desired Contributions

Useful contributions include:

- concrete failure examples
- benchmark ideas
- criticism of the scale model
- alternative naming
- minimal schemas
- agent execution traces
- safety invariants
- examples of premature macro-level execution
- examples of micro/pico evidence checks

## Example Contribution Format

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

## Short Form

> Do not just ask whether the output is correct. Ask whether the action had a valid causal path.

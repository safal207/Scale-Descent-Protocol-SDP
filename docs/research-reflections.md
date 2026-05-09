# Research Reflections for Articles and Reviewers

This document collects article-grade framing for Scale Descent Protocol (SDP).

## Core Thesis

SDP is a protocol for moving causal state across scales before action.

Agentic systems often scale outward first: more agents, more tools, more memory, more context, and more orchestration. SDP explores the complementary direction: scale inward toward the causal layer where action becomes inspectable.

## Main Claim

> An agent should not execute from the highest available level of abstraction. It should execute only after descending to the lowest causal scale where evidence, permission, and responsibility are sufficient.

## Scale Path

```text
Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute
```

## Mobile Causal States

SDP introduces the idea of mobile causal states:

> Scalable intelligence requires moving state, not wiring everything to everything.

For SDP, the moved state includes:

- task intent
- trace fragments
- permission context
- hypotheses
- evidence
- responsibility boundaries
- commit conditions

## Causal Subsidiarity

SDP generalizes subsidiarity into causal computation:

> Action should be authorized at the lowest causal scale where evidence, permission, and responsibility are sufficient.

In plain language:

```text
Do not solve from the highest available level.
Solve from the lowest level where the cause is visible.
```

## Reviewer Questions

1. What exactly is a causal state?
2. How is scale descent different from ordinary task decomposition?
3. When is Macro sufficient, and when must the system descend to Micro or Pico?
4. What evidence is required before Commit?
5. What prevents Execute before Commit?
6. How does SDP avoid becoming another global orchestration layer?
7. What can be measured deterministically?
8. What failures does SDP detect that output-level evaluation misses?
9. How does SDP relate to causal memory, trace replay, and execution control?
10. What would falsify or weaken the SDP hypothesis?

## Possible Paper Titles

- Scale Descent Protocol: Mobile Causal States for Verifiable Agentic AI
- From Mobile Qubits to Mobile Causal States: Scale Descent for Agentic Systems
- Causal Subsidiarity in Agentic AI: Authorizing Action at the Lowest Valid Scale
- Scaling Inward: A Protocol for Evidence-Grounded Agent Execution

## Initial Contributions

1. A scale model for agentic reasoning.
2. The concept of mobile causal states.
3. The principle of causal subsidiarity.
4. A Rust-first deterministic protocol core.
5. Explicit invariants for safe execution control.
6. A path toward integration with causal memory, trace replay, and agent frameworks.

## Threats to Validity

SDP must avoid several traps:

- treating metaphor as proof
- overclaiming safety before benchmarks exist
- confusing task decomposition with causal descent
- creating excessive protocol overhead
- assuming all tasks require Pico-level inspection
- ignoring human factors in developer adoption

## Minimal Empirical Program

Compare:

1. ordinary agent execution
2. prompt-only safety checks
3. trace-only logging
4. SDP-style scale descent with evidence commit

Possible metrics:

- invalid Execute transitions blocked
- missing evidence detection rate
- permission-boundary violation detection rate
- trace replay fidelity
- latency overhead
- developer effort
- false positive rate for blocked actions

## Working Abstract

Modern agentic AI systems increasingly rely on larger context windows, multi-agent orchestration, and global tool connectivity. While powerful, this pattern introduces hidden causal failures: actions may appear correct at the output level while violating provenance, permission boundaries, or trace integrity at the causal level.

We propose Scale Descent Protocol (SDP), a Rust-first protocol for moving causal state across scales before action. SDP decomposes agentic work into Macro, Meso, Micro, Pico, Collapse, Commit, and Execute stages. Its core invariant is that an agent may only execute after descending to the required causal scale and validating sufficient evidence for commit.

## One-Line Summary

> SDP is a protocol for moving causal state to the scale where action becomes verifiable.

# SDP Launch Posts

Copy-ready launch posts for Scale Descent Protocol (SDP).

## Core X Post

Most AI agent systems scale outward:

more agents, more tools, more context, more memory.

SDP explores the opposite direction:

scale inward to the causal layer before action.

No Execute before causal descent.

https://github.com/safal207/Scale-Descent-Protocol-SDP

## X Thread

1/ Modern AI agents are getting more context, more tools, more memory, and more orchestration.

But many failures are not visible in the final answer. They are hidden in the causal path that produced the action.

2/ An output can look useful while the underlying action violates provenance, permissions, evidence, or trace integrity.

That is the failure mode SDP targets.

3/ Scale Descent Protocol asks the agent to descend before it acts:

Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute

4/ The core invariant:

An agent may only execute after descending to the required causal scale and validating sufficient evidence for commit.

5/ Short version:

Do not just ask whether the output is correct. Ask whether the action had a valid causal path.

6/ Looking for examples where an AI agent output looked useful, but had invalid causal ancestry.

Those examples can become benchmark fixtures.

Repo:
https://github.com/safal207/Scale-Descent-Protocol-SDP

## LinkedIn Post

Agent safety is not only about final-answer evaluation.

A system can produce a useful output while violating provenance, permission boundaries, evidence requirements, or trace integrity before execution.

That is the gap Scale Descent Protocol (SDP) explores.

SDP proposes that agentic systems should not execute directly from a high-level task description. Instead, they should descend through scale levels:

Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute

The central idea is causal subsidiarity:

Action should be authorized at the lowest causal scale where evidence, permission, and responsibility are sufficient.

Repo:
https://github.com/safal207/Scale-Descent-Protocol-SDP

## Show HN Draft

Title:
Show HN: SDP - a Rust protocol for checking causal paths before AI agents execute

Text:
I am building Scale Descent Protocol (SDP), a Rust-first protocol for agentic AI safety.

The core idea is that agents should not execute directly from a high-level task description. They should descend to the causal scale where evidence, permissions, provenance, and trace integrity can be inspected before action.

Current path:
Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute

The first implementation is deterministic: a small Rust core, CLI inspection flow, examples, schemas, and invariants.

I am looking for criticism and examples where an AI agent output looked useful but had an invalid causal path underneath.

Repo:
https://github.com/safal207/Scale-Descent-Protocol-SDP

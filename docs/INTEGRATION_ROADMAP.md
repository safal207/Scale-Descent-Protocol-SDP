# SDP Integration Roadmap

Scale Descent Protocol is currently a deterministic Rust foundation.

This roadmap describes how SDP can grow from a local protocol inspector into an integration layer for traces, causal memory, and agent frameworks.

Important boundary:

> This document is a roadmap. It does not claim these integrations are already implemented.

## Current implementation

The current repository provides:

- `sdp-core` — protocol types, validation rules, and evidence collapse logic.
- `sdp-cli` — deterministic CLI inspection for descent JSON files.
- `examples/simple_descent.json` — a minimal example showing visible correctness vs causal invalidity.
- `docs/START_HERE.md` — contributor entry point.
- `docs/demo/FIVE_MINUTE_CLI_DEMO.md` — CLI walkthrough.
- `docs/INVARIANTS_AND_NON_CLAIMS.md` — protocol invariants and evidence boundaries.

The current repository does **not** yet provide live runtime adapters, agent framework hooks, or production enforcement.

## Integration principle

SDP should integrate at the boundary between reasoning and action.

The ideal future flow:

```text
agent intent
  -> trace / memory evidence
  -> scale descent inspection
  -> collapse selected hypothesis
  -> commit decision
  -> execution gate
```

In plain language:

```text
Before action, descend to the cause.
```

## Track 1 — Trace replay integration

### Goal

Use execution traces to verify whether required descent evidence existed before Commit and Execute.

### Why it matters

Many agent systems already produce traces:

- tool calls,
- model messages,
- intermediate states,
- retries,
- branches,
- approvals,
- timestamps,
- outputs.

SDP can use those records as Pico and Micro evidence.

### Mapping

| Trace record | SDP scale |
|---|---|
| User request / task title | Macro |
| Agent plan / workflow step | Meso |
| State transition / permission check | Micro |
| Tool call id / timestamp / hash / anchor | Pico |
| Replay result | Collapse evidence |
| Accepted branch | Commit evidence |
| Tool execution | Execute event |

### Future adapter shape

A trace adapter should convert trace events into a descent run:

```text
trace.jsonl -> sdp descent run -> collapse -> decision
```

Possible future command:

```bash
sdp inspect-trace traces/agent-run.jsonl --required-scale Pico --output report.json
```

### Open questions

- How should SDP represent branch invalidation?
- How should rejected trace branches affect evidence collapse?
- What trace fields are mandatory for Pico-level evidence?
- How should trace replay handle nondeterministic tool outputs?

## Track 2 — CML / causal memory adapter

### Goal

Map SDP evidence units to causal memory records.

### Why it matters

SDP asks whether an action is causally admissible. A causal memory layer records why actions happened, what permitted them, and which prior records they depend on.

Together:

```text
SDP = descent and decision protocol
CML = persistent causal memory substrate
```

### Mapping

| SDP concept | Causal memory concept |
|---|---|
| Pico evidence | causal record / anchor |
| Micro transition | parent-child causal link |
| Permission evidence | permitted_by / policy lineage |
| Hypothesis | candidate causal interpretation |
| Collapse result | selected causal interpretation |
| Commit | admissible decision record |
| Execute | downstream action record |

### Future adapter shape

Possible future flow:

```text
CML records -> SDP descent run -> collapse -> commit decision -> new CML decision record
```

Possible future command:

```bash
sdp inspect-cml records.jsonl --task demo-task-001 --required-scale Pico
```

### Open questions

- Should SDP write decision records back into causal memory?
- Should CML be treated as source of truth for permission lineage?
- How should missing parent records be represented: Hold or Reject?
- How should causal memory handle multiple competing hypotheses?

## Track 3 — T-Trace / audit trace integration

### Goal

Use structured trace logs as replayable evidence for SDP decisions.

### Why it matters

Trace systems capture what happened. SDP asks whether what happened was causally admissible before action.

The bridge is:

```text
observability trace -> causal evidence -> descent decision
```

### Future report shape

A future SDP trace report should include:

```json
{
  "task_id": "demo-task-001",
  "required_scale": "Pico",
  "selected_hypothesis": "H3",
  "decision": "Commit",
  "confidence": 0.92,
  "missing_evidence": [],
  "invalid_transitions": [],
  "execute_allowed": false
}
```

Note: this is a roadmap shape, not the current CLI output contract.

## Track 4 — Agent framework adapters

### Goal

Add adapters that let existing agent frameworks call SDP before tool execution.

Potential future targets:

- LangGraph
- AutoGen
- CrewAI
- Pydantic AI
- custom tool-calling agents
- browser automation agents
- coding agents

### Integration point

The most important hook is immediately before action:

```text
before tool call -> build descent context -> SDP inspect -> allow/hold/reject
```

### Minimal adapter contract

A future adapter should provide:

```text
1. task context
2. planned action
3. trace/history summary
4. available evidence units
5. required scale
6. SDP decision
7. allow / hold / reject result
```

### Example future pseudocode

```python
result = sdp.inspect(
    task=task,
    planned_action=tool_call,
    trace=agent_trace,
    required_scale="Pico",
)

if result.decision != "Commit":
    return hold_or_reject(result.reason)

execute(tool_call)
```

This is illustrative, not an implemented API.

## Track 5 — Policy / state-machine enforcement

### Goal

Turn SDP invariants into executable state-machine rules.

Core target:

```text
Commit -> Execute
```

with explicit rejection of invalid transitions:

```text
No Commit -> no Execute
Reject -> no Execute
Missing required evidence -> Hold or Reject
```

### Future state machine

Possible states:

```text
Received
Descending
EvidencePending
Collapsed
Committed
Executed
Held
Rejected
```

Possible invalid transitions:

```text
Received -> Executed
EvidencePending -> Executed
Rejected -> Executed
Held -> Executed without new evidence
```

### Future test categories

- Reject never executes.
- Execute only after Commit.
- Commit requires required scale evidence.
- Missing Pico evidence returns Hold or Reject.
- Collapse must include a reason.
- Invalid branch evidence cannot support Commit.

## Track 6 — JSON Schema and external tool compatibility

### Goal

Make descent runs machine-validatable.

Current issue:

```text
#4 schema: add JSON Schema for scale descent runs
```

Suggested files:

```text
schemas/sdp-descent-run.v0.1.schema.json
schemas/sdp-hypothesis.v0.1.schema.json
```

Why this matters:

- external tools can produce SDP input,
- CI can validate example files,
- adapters can share a stable contract,
- contributors can add examples safely.

## Track 7 — Machine-readable CLI output

### Goal

Allow downstream tools to consume SDP decisions.

Possible future command:

```bash
cargo run -p sdp-cli -- inspect examples/simple_descent.json --json
```

Possible future output:

```json
{
  "task_id": "demo-task-001",
  "selected_hypothesis": "H3",
  "confidence": 0.92,
  "decision": "Commit",
  "reason": "Selected highest-scoring hypothesis after evidence collapse"
}
```

This would make SDP easier to integrate into CI, trace processors, and agent runtimes.

## Track 8 — Negative examples and falsifiability

### Goal

Show cases where SDP should not commit.

Suggested examples:

```text
examples/negative_missing_pico_anchor.json
examples/negative_execute_without_commit.json
examples/negative_macro_only_evidence.json
examples/negative_invalid_permission_chain.json
```

Each example should answer:

- What looked correct at Macro level?
- What failed at Micro or Pico level?
- What decision should SDP produce?
- What evidence was missing or invalid?

This makes the project falsifiable and easier to review.

## Recommended implementation order

Suggested order:

```text
1. Invariants and non-claims
2. Integration roadmap
3. Negative examples
4. JSON Schema
5. CLI --json output
6. State-machine tests
7. Trace replay adapter
8. CML adapter
9. Agent framework adapters
```

Current status:

- Invariants and non-claims: documented.
- Integration roadmap: this document.
- JSON Schema: open issue.
- Runtime adapters: future work.

## Non-goals for now

SDP should not immediately try to become:

- a full agent framework,
- a general sandbox,
- a replacement for LangGraph/AutoGen/CrewAI,
- a replacement for observability tools,
- a replacement for causal memory,
- a production policy engine,
- a full formal verification stack.

The focused role is:

> scale-aware causal inspection before action.

## Summary

SDP's integration path is strongest if it stays small and precise:

```text
traces provide events
causal memory provides lineage
SDP provides descent and collapse
policy/state machine provides execution gate
agent framework adapter provides runtime hook
```

The project should grow from deterministic inspection toward enforcement without confusing roadmap direction with current guarantees.

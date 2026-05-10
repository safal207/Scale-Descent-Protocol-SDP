# Five-Minute SDP CLI Demo

**Goal:** run the deterministic Scale Descent Protocol inspector and understand what the example proves.

This demo uses the current CLI command:

```bash
cargo run -p sdp-cli -- inspect examples/simple_descent.json
```

It does not call an LLM. It does not execute real tools. It inspects a deterministic JSON descent run and selects the most causally supported hypothesis.

## What you will see

In five minutes, you should be able to:

1. run the SDP CLI inspector,
2. understand the example task,
3. see why one hypothesis is selected,
4. connect the output to the descent chain,
5. understand what the demo proves and does not prove.

## The scenario

The example file is:

```text
examples/simple_descent.json
```

It describes this task:

```text
Find why an apparently valid agent output has invalid causal ancestry
```

The key idea is that the visible answer may look acceptable at the Macro level, but the causal trace may still be invalid at Micro or Pico level.

In the example:

- the task requires inspection down to `Pico`,
- the current scale is already `Pico`,
- two hypotheses are compared,
- the selected hypothesis is the one with stronger causal evidence.

## Step 1 — Run the CLI demo

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

The exact confidence may change if scoring logic changes, but the current demonstration should select `H3`.

## Step 2 — Read the task

The input task contains:

```json
{
  "task_id": "demo-task-001",
  "goal": "Find why an apparently valid agent output has invalid causal ancestry",
  "required_scale": "Pico",
  "context": "The final answer looked correct, but the trace may contain an invalid causal transition."
}
```

This means the protocol is not satisfied by checking the final answer only.

The inspector must consider evidence at the required causal scale.

## Step 3 — Compare the hypotheses

The example has two hypotheses.

### H1 — output looks valid

```text
The output is valid and the trace ancestry is intact.
```

Supporting evidence:

```text
The visible answer satisfies the user request.
Scale: Macro
Confidence: 0.7
```

This is useful evidence, but it is too high-level for a task that requires `Pico` inspection.

### H3 — useful output, invalid causal ancestry

```text
The output is useful, but a pico-level anchor is missing before commit.
```

Supporting evidence:

```text
The commit transition references an anchor that is absent from the trace.
Scale: Pico
Confidence: 0.93
```

and:

```text
Execute occurred after a commit that was not causally admissible.
Scale: Micro
Confidence: 0.91
```

This evidence is closer to the required causal layer.

## Step 4 — Understand the descent chain

SDP uses this chain:

```text
Macro -> Meso -> Micro -> Pico -> Collapse -> Commit -> Execute
```

In this demo:

| Stage | Demo meaning |
|---|---|
| Macro | The final answer appears useful. |
| Meso | The workflow/trace path may look plausible. |
| Micro | Commit/execute ordering and causal admissibility are inspected. |
| Pico | The anchor/evidence unit referenced by commit is checked. |
| Collapse | Hypotheses are scored against available evidence. |
| Commit | The selected hypothesis becomes the decision basis. |
| Execute | Not performed by this demo; this is only inspection. |

## Step 5 — Read the decision

The CLI prints:

```text
Selected hypothesis: H3
Decision: Commit
Reason: Selected highest-scoring hypothesis after evidence collapse
```

In plain language:

> The protocol selected the hypothesis that the output was useful but causally invalid underneath, because the lower-scale evidence was stronger.

This demonstrates SDP's central point:

```text
A useful answer can still be causally invalid.
```

## What this demo proves

This demo shows that the current repository can:

- parse a descent run JSON file,
- compare hypotheses with evidence,
- prefer lower-scale causal evidence when it is stronger,
- produce a deterministic selected hypothesis,
- print confidence, decision, and reason.

## What this demo does not prove

This demo does not prove:

- production AI safety,
- complete agent sandboxing,
- formal verification,
- runtime enforcement across real tools,
- integration with live LLM frameworks,
- universal overhead bounds,
- that every unsafe execution will be caught,
- that `Commit -> Execute` is already enforced in real agent runtimes.

The safe interpretation is:

> SDP currently has a deterministic Rust foundation for inspecting scale descent evidence before future execution enforcement.

## Why this matters

Most agent systems are evaluated at the output level:

```text
Did the answer look right?
```

SDP asks a deeper question:

```text
Was the answer causally authorized by valid evidence before action?
```

That is the difference between visible correctness and causal validity.

## Try modifying the example

To understand the protocol behavior, duplicate the example:

```bash
cp examples/simple_descent.json examples/my_descent.json
```

Then experiment with:

- changing evidence confidence,
- changing evidence scale,
- adding another hypothesis,
- removing the Pico evidence,
- changing `required_scale`.

Run:

```bash
cargo run -p sdp-cli -- inspect examples/my_descent.json
```

Expected contributor learning:

```text
Evidence strength + causal scale affect which hypothesis wins.
```

## Next useful improvements

Good follow-up tasks:

1. Add more example descent runs.
2. Add JSON Schema for descent input.
3. Add invariants and non-claims documentation.
4. Add trace replay integration roadmap.
5. Add a negative example where missing Pico evidence produces Hold or Reject.
6. Add machine-readable CLI output such as `--json`.

# SDP Invariants

Scale Descent Protocol exists to prevent agents from executing from a shallow view of the task.

## Core Invariant

> An agent may only execute after descending to the required causal scale and validating sufficient evidence for commit.

## Initial Foundation Rules

1. `Execute` requires a prior `Commit`.
2. `Commit` requires at least one evidence item.
3. Evidence confidence must be within `0.0..=1.0`.
4. Collapse cannot happen from `Macro` or `Meso`; it requires `Micro` or `Pico` inspection.
5. An agent cannot execute unless its contract includes `Permission::Execute`.

## Design Direction

The first implementation uses runtime validators.

Future versions should move more invariants into Rust's type system using a type-state pattern, so invalid flows become harder or impossible to represent.

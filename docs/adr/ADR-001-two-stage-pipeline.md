# ADR-001: Two-Stage Pipeline Architecture

**STATUS:** Accepted (July 2026)

## Context
The Development Protocol needed to handle the full lifecycle from raw intention to shipped product. The original protocol only covered execution (RULES.md phases). The gap was the upstream phase — clarifying intent, researching, validating assumptions, and specifying before building.

## Decision
We decided for a **two-stage pipeline** with a PREP PHASE (pre-protocol) and an EXECUTION PHASE (RULES.md-governed), connected by a central prototyping gate.

## Consequences
**Positive:** Clear separation of concerns — prep is divergent/human-driven (find the right thing to build), execution is convergent/AI-driven (build it right).
**Negative:** Adds process overhead. Projects that "already know what to build" may find the prep phase unnecessary.

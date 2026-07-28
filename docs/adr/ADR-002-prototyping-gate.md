# ADR-002: Prototyping as Central Decision Gate

**STATUS:** Accepted (July 2026)

## Context
The original protocol treated VALIDATION (prototyping) as one step in a linear sequence. Real usage showed that prototyping is where projects are won or lost — it's the moment when you learn whether the project is worth building at all.

## Decision
Elevated VALIDATION from "a step before the spec" to the **central decision mechanism** of the protocol. The architecture became: AMBITION (Gate 1: is the goal clear?) → LANDSCAPE → VALIDATION (Gate 2: should we build this?) → SPEC → EXECUTE → POLISH.

## Consequences
**Positive:** KILL is now a success, not a failure. The process explicitly acknowledges that killing a project after prototyping is valuable.
**Positive:** The weighted kill criteria matrix prevents emotional decision-making.
**Negative:** Requires discipline to treat prototypes as throwaway (the "this code is too good to delete" trap).

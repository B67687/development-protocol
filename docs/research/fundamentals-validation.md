# Research: Fundamentals Failures in AI-Assisted Projects — 2026-07-21

## Applied to: Development Protocol v3 — adding Fundamentals Validation Gate

### Definition of "Fundamentals"

A decision is **fundamental** if:

- Getting it wrong makes ALL future work on top of it more expensive
- Reversal cost is super-linear (fixing a wrong schema = rewriting every query, not just one)
- Does NOT pass the Extractability Test (cannot isolate behind a seam)

A decision is **peripheral** if:

- Getting it wrong affects only the current work item
- Reversal cost is linear or constant
- Passes the Extractability Test

### The "First 10%" Problem

Boehm's Cost of Change curve (1976/1988) is the foundational evidence: fixing a design error at design costs 5x; at testing costs 20x; at production costs 200x. The hidden mechanism: the first 10% of decisions determine the SHAPE of this curve. Wrong foundation = steeper curve for ALL subsequent work.

Three cascading effects:

1. **Commitment cost** — workarounds accumulate on the wrong foundation
2. **Entanglement cost** — workarounds interact, creating their own coupling
3. **Normalization of deviance** — team forgets the foundation is wrong, treats broken as normal

### Signature: Fundamentals Wrong vs Priorities Wrong

| Symptom        | Fundamentals Wrong                          | Priorities Wrong                       |
| -------------- | ------------------------------------------- | -------------------------------------- |
| What breaks    | Everything equally, foundational layers     | Finishing touches, edge cases          |
| Fix cost       | Cross-cutting (schema change = all queries) | High but bounded                       |
| Velocity       | Starts OK, then decelerates non-linearly    | Constant then sharp stop               |
| Team sentiment | "Fighting the architecture"                 | "Just needed two more weeks"           |
| Code           | Accidental complexity, workarounds          | Missing features, what exists is clean |
| AI pattern     | AI copies wrong abstraction everywhere      | Correct code never integrated          |

### Detection Techniques

- **Design Structure Matrix** — N×N dependency matrix. Wrong clusters = wrong foundation
- **Cost of Change Analysis** — if >50% of effort is fighting structure, foundation is wrong
- **Pre-Mortem** — "6 months from now, project is dead. Why?" 90% = foundational decisions
- **Extractability Test** — "Can I extract this module into its own service?"
- **Bridge Cause Detection** — build causal diagram; high out-degree nodes = fundamentals

### Existing Approaches That Handle This

- **Shape Up**: Circuit breaker (sunk cost trap). Rabbit holes (risk identification). But no architectural foundation validation.
- **RUP Elaboration Phase**: Architectural baseline before construction. But heavyweight, assumes upfront architecture.
- **Spiral Model**: Risk-driven spirals, prototype architectural risks early.
- **NASA Power of 10**: Small ruleset (≤10 rules) prevents disproportionate failures.

### Gap in Current Dev Protocol

The protocol has NO step that distinguishes fundamental vs peripheral decisions. The Constitution is a list of rules, not a process for identifying which decisions ARE fundamental. EXTRACTION (find the problem) is different from DECOMPOSITION (break into dimensions) is different from VALIDATION (spike risks) — but none of them explicitly says "THIS decision is one-way door."

### Proposal: FUNDAMENTALS.md

A new step between EXTRACTION and DECOMPOSITION:

1. Identify up to 5 "one-way door" decisions (data model, module boundaries, tech stack, public API, state management)
2. For each: write down why it's irreversible and what happens if it's wrong
3. Validate each with a minimal prototype (throwaway, no more than 1 session each)
4. Only after all fundamentals validated → proceed to full decomposition

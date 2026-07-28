# FUNDAMENTALS.md — Architecture Foundation Validation (v3)

> This runs between EXTRACTION (found the right problem) and DECOMPOSITION (broke it into dimensions).
>
> **Purpose:** Before you decompose the problem, identify which decisions are ONE-WAY DOORS.
> A wrong fundamental makes ALL future work on top of it more expensive. Getting the foundation
> right prevents the "projects go south midway" problem.

## What Is a Fundamental?

A decision is **fundamental** if:

| Test              | One-Way (Fundamental)                  | Two-Way (Peripheral)       |
| ----------------- | -------------------------------------- | -------------------------- |
| **Reversal time** | > 6 months to undo                     | < 1 week to undo           |
| **Blast radius**  | Multiple systems or external consumers | Single module              |
| **Data coupling** | Reversal requires data migration       | Reversal just changes code |
| **Exit cost**     | Missed deadline or regulatory event    | Days of refactoring        |

**Two-way doors need ZERO validation.** Just pick and move on.

## The Protocol — 4 Steps

### Step 1: Identify Up-to-5 One-Way Doors

Before decomposition, list the decisions that pass the Reversibility Classification Map:

| Domain                | Examples of One-Way Doors                         | Examples of Two-Way Doors         |
| --------------------- | ------------------------------------------------- | --------------------------------- |
| **Data model**        | Entity relationships, key schema, access patterns | Indexes, query optimization       |
| **Module boundaries** | Which domains exist, their public interfaces      | Internal module structure         |
| **Tech stack**        | Language, database engine, hosting provider       | Library versions, config defaults |
| **Public API**        | Endpoint shapes, data formats, error schema       | Pagination strategy, rate limits  |
| **Security**          | AuthN/AuthZ approach, data isolation              | Permission roles                  |

### Step 2: Validate Each One-Way Door (Minimum Viable)

Use the shallowest validation that addresses the risk:

| Risk level                                             | Technique        | Time      | What to build                                  |
| ------------------------------------------------------ | ---------------- | --------- | ---------------------------------------------- |
| **Already validated** by 10,000 projects               | Zero. Move on.   | 0         | —                                              |
| **Complicated** — need to choose between known options | Spike            | 1-3 days  | Runnable throwaway code exploring the decision |
| **Complex** — no one knows the right answer            | Prototype        | 3-10 days | Working subset, built to be thrown away        |
| **Novel architecture** — first-of-its-kind             | Walking Skeleton | 2-6 weeks | Production-grade foundation only               |

**The "Schema + 3 Queries" technique** (for data model validation):

```
1. Write the DDL (no ORM, raw DDL)
2. Write the READ query for the main access pattern
3. Write the WRITE query for the main mutation
4. Write the REPORT query the business needs most

If any of these is awkward, the schema is wrong.
If you can't write query 4 without denormalizing, the schema is wrong.
```

### Step 3: Detect and Correct LLM Architecture Bias

Before accepting any AI-suggested architecture, run these checks:

| Bias                   | Signal                                                | Correction                                                                                   |
| ---------------------- | ----------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| **Popularity bias**    | AI suggests most popular tech, not most appropriate   | Constraint-first prompt: "Do NOT use [popular tech]. Do use [specific approach]."            |
| **Wrong abstraction**  | 3+ layers beyond what the problem needs               | Ask: "Can we duplicate first, extract on the third occurrence?"                              |
| **Self-contradiction** | AI recommends one thing, generates another (83% rate) | Cross-check: "Summarize your architecture recommendation, then implement a minimal version." |
| **Monoculture**        | All AIs suggest the same pattern                      | Multi-model consultation + human final say                                                   |

**The "Adversarial Architecture Review":**

```
You suggested a [REST CRUD] architecture.
Now argue AGAINST it for this specific use case.
What would you suggest if REST, CRUD, and all
frameworks beyond the standard library were prohibited?
```

**The architecture gate rule:** The human must own one-way door decisions. AI implements within those boundaries. Never let AI choose a one-way door without human approval.

### Step 4: Record in a Decision Log

For each validated fundamental, write:

```
## Fundamental: [decision]

Question: [what problem does this solve?]
Why one-way: [reversal time, blast radius, data coupling, exit cost]
Validated by: [spike / prototype / already validated / adversarial review]
Alternatives considered: [at least 2]
Date: [when]
```

Store in `docs/adr/`. This is the foundation reference for all future decisions.

## When NOT to Validate

| Situation                                    | Action                                            |
| -------------------------------------------- | ------------------------------------------------- |
| Standard choice validated by 10,000 projects | Zero validation. Just use it.                     |
| Two-way door (reversible < 1 team-week)      | Just pick one and start building.                 |
| Can be hidden behind an interface            | Deliberately under-validate. Fix later if needed. |
| No data dependency + small team              | Fail fast. Rely on early refactoring.             |

## Validation Depth Decision Tree

```
New fundamental decision needed
│
├─ Already validated by 10,000+ projects (e.g. Postgres)?
│   └─ YES → Zero validation. Ship it.
│
├─ Two-way door (reversible < 1 team-week)?
│   └─ YES → Just pick one. No validation.
│
├─ Can you hide it behind an interface?
│   └─ YES → Under-validate. Fix later if wrong.
│
├─ One-way door (irreversible or expensive)?
│   └─ Primary risk:
│       ├─ Don't know enough → Spike (1-3 days)
│       ├─ Don't know if feasible → POC (1-3 weeks)
│       ├─ Don't know if architecture works → Walking Skeleton (2-6 weeks)
│       └─ Don't know if users want it → Prototype → MVP
│
└─ Can't classify?
    └─ Spike first. Then re-evaluate.
```

## Integration

FUNDAMENTALS.md sits between EXTRACTION and DECOMPOSITION in the protocol pipeline:

```
EXTRACTION → FUNDAMENTALS → DECOMPOSITION → AMBITION → LANDSCAPE → VALIDATION → SPECIFICATION → EXECUTOR → POLISH → EXPLAINER → SPEC_SYNC → REVIEW → ship
```

It must respect the **Recursion Meta-Rule**: if Step 1 (identify one-way doors) finds a decision that can't be classified, recurse — break it into sub-decisions and classify each.

## Provenness

The techniques in this document are synthesized from:

- Parnas (1972) — modular decomposition criteria for isolating irreversible decisions
- Boehm (1981/2001) — Cost of Change curve, Risk Exposure model
- Conway (1968) / Fowler (2025) — module boundaries harden into human boundaries
- Kruchten (2004) — ontology of architectural design decisions
- Bezos (2016) — one-way door / two-way door framework
- Ryan Lee (2026) — Reversibility Classification Map
- Twist et al. (2025) — LLM popularity bias ("LLMs Love Python")
- Shumailov et al. (2024) — model collapse and monoculture risk
- Sandi Metz (2016) — the wrong abstraction pattern amplified by AI

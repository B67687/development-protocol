# DECOMPOSITION.md — Intent Decomposition (v3)

> This runs between FUNDAMENTALS (one-way doors validated) and AMBITION (appetite set).
>
> **Purpose:** Take the validated problem (X from EXTRACTION) and break it systematically
> into KNOWN, NEEDS RESEARCH, and NEEDS PROTOTYPE dimensions. Every gap found here
> is cheaper than any gap found later.

## The Method — 4 Steps

### Step 1: Cynefin Classify

Before any decomposition, classify the top-level problem AND each sub-problem:

| Domain          | Nature                             | Decomposition                   | Example                             |
| --------------- | ---------------------------------- | ------------------------------- | ----------------------------------- |
| **Clear**       | Known cause-effect                 | Standard procedure              | Known bug fix, routine config       |
| **Complicated** | Cause-effect knowable via analysis | MECE tree, expert analysis      | Build a REST API, design a database |
| **Complex**     | Only knowable in retrospect        | Probe-sense-respond (prototype) | New product design, novel algorithm |
| **Chaotic**     | Act to stabilize first             | Act-sense-respond               | Production outage, security breach  |

### Step 2: Recursive MECE Tree

For COMPLICATED dimensions:

1. **Split into MECE sub-dimensions** — Mutually Exclusive, Collectively Exhaustive
2. **Classify each sub-dimension:**
   - **KNOWN** — I know exactly how to implement this. Document briefly.
   - **NEEDS RESEARCH** — I need to learn more. Create a LANDSCAPE task.
   - **NEEDS PROTOTYPE** — Only building reveals the answer. Create a VALIDATION task.
3. **Confirm with user** — Present the tree. "Did I miss any dimensions?"
4. **Recurse on KNOWN sub-dimensions** — Break each down further until trivial.
5. **Stop** — When every leaf fits: trivially implementable, assigned to research, or assigned to prototype.

### Step 3: User Confirmation Gate

After each level, present to user:

```
I decomposed [INTENT] into:
  ├── Dimension A (KNOWN)
  │   ├── Sub A1 (Clear — trivially implementable)
  │   └── Sub A2 (Complicated — NEEDS RESEARCH)
  ├── Dimension B (Complex — NEEDS PROTOTYPE)
  └── Dimension C (KNOWN)
      └── Sub C1 (NEEDS RESEARCH)

Before I go deeper: does this match your understanding?
```

Do NOT proceed to the next level until user confirms.

### Step 4: Convergence

Stop when ALL leaf dimensions are:

1. **Trivially implementable** — one AI session, no unknowns
2. **Assigned to research** — LANDSCAPE.md task with clear question
3. **Assigned to prototype** — VALIDATION.md spike with success/fail criteria

**The stopping criterion:** No remaining dimension requires more than one AI session to implement.

**If a dimension is still unknown after 3 decomposition levels:** it's Complex, not Complicated. Move it to prototype. Do NOT try to spec it further.

## Relationship to Other Steps

| Step              | What comes before           | What comes after                         |
| ----------------- | --------------------------- | ---------------------------------------- |
| EXTRACTION        | PROBLEM (X)                 | FIND ONE-WAY DOORS                       |
| **FUNDAMENTALS**  | **ONE-WAY DOORS VALIDATED** | **PROBLEM IS REAL, FOUNDATION IS SOLID** |
| **DECOMPOSITION** | **PROBLEM + FOUNDATION**    | **DIMENSIONS MAPPED, GAPS FLAGGED**      |
| AMBITION          | DIMENSIONS KNOWN            | APPETITE SET                             |
| LANDSCAPE         | GAPS IDENTIFIED             | RESEARCH EXECUTED                        |
| VALIDATION        | COMPLEX DIMENSIONS FLAGGED  | PROTOTYPE EXECUTED                       |

## Recursion

DECOMPOSITION follows the **Recursion Meta-Rule**: if a dimension is still ambiguous after one pass, decompose it again deeper. Most dimensions resolve within 2 levels.

After 3 levels of decomposition without convergence, the dimension is Complex (not Complicated) — stop decomposing and assign it to VALIDATION.

## Level of Care (Sub-Cycle Routing)

Each MECE leaf gets a Level of Care. This decides whether the leaf needs its own
full protocol cycle or just direct execution:

| Level | Meaning                            | Example                        | Needs                          |
| ----- | ---------------------------------- | ------------------------------ | ------------------------------ |
| **1** | Trivial. No protocol.              | Config file, logging setup     | Just implement                 |
| **2** | Standard. One AI session.          | CRUD endpoint, CLI command     | Quick spec + build             |
| **3** | Sub-project. Own REVIEW.           | Plugin system, custom protocol | Full cycle minus EXTRACTION    |
| **4** | Independent system. Full protocol. | Microservice, library          | Full EXTRACTION through REVIEW |

**Decision rule:** Estimate cost(wrong) vs cost(protocol run) in 30 seconds.

- If cost(wrong) > cost(protocol run) -> Level 3+
- If cost(wrong) <= cost(protocol run) -> Level 1-2
- Default to Level 2 when unsure (one session is cheap)

**For Level 3+ leaves:** Fork into a new protocol cycle. Start a new AI session.
Run the same pipeline (AMBITION through REVIEW) scoped to this sub-component only.
The sub-project inherits the parent's context but has its own appetite and spec.

## Provenness

MECE (Mutually Exclusive, Collectively Exhaustive) — McKinsey & Co., standard consulting methodology, taught in business schools globally since the 1970s.

Cynefin — Dave Snowden (1999), used by governments and enterprises worldwide for decision classification.

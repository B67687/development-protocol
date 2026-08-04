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

**Claim verification rule (Research on Demand):** AI confidence never de-escalates
verification tier; disagreement between sources escalates it. If the AI asserts a
consequential fact from memory (closed-book), treat it as L2 minimum — verify
against authoritative sources (see LANDSCAPE.md § Verification Tiers).

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

## Step 5: Team Capability Audit (Optional but Recommended)

Before proceeding to DECOMPOSITION, check whether you (or the team) have the skills needed to execute the project:

| Capability | Check | Mitigation if Missing |
| --- | --- | --- |
| **Domain knowledge** | Do you understand the problem domain well enough to evaluate solutions? | Add a learning phase to the AMBITION budget or schedule a domain expert consultation. |
| **Technical skills** | Does the team have experience with the chosen tech stack? | Factor learning curve into appetite. Consider spikes for unfamiliar tools. |
| **Tooling/infrastructure** | Do you have the build tools, CI, hosting, and test environment ready? | Add infra setup to project scope. Do not assume it "just works." |
| **External dependencies** | Are the libraries, APIs, or services you depend on stable and available? | Validate dependency readiness before DECOMPOSITION. Add fallback options. |
| **Legal/compliance** | Are there licensing, regulatory, or compliance constraints? | Flag for FUNDAMENTALS Step 1 as a potential one-way door decision. |
| **Self-assessment calibration** | Rate your confidence in each capability above (1-5). Then explain the domain in your own words — AI checks if explanation depth matches confidence. | Gap between confidence score and explanation quality reveals unknown unknowns. High confidence + vague explanation = Dunning-Kruger signal. Add learning phase to the AMBITION budget. |

If a capability gap would block execution, add it as a prerequisite step in the AMBITION budget. Do not proceed to DECOMPOSITION with known capability gaps — they will compound during execution.
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


## Step 6: Map One-Way Door Chains (Chain Analysis)

One-way doors can chain: a decision in layer A creates constraints that turn
layer B decisions into one-way doors. These chains compound risk — if two doors
in the same chain are both wrong, the reversal cost multiplies.

### Chain Detection Technique

```
1. For each identified one-way door, ask: "If I pick X, what subsequent decisions become one-way doors?"
2. Trace the dependency chain: [Door A] → [Door B] → [Door C]
3. Identify shared foundations — decisions that multiple downstream doors depend on
4. If a chain contains 2+ dependent one-way doors, flag it as HIGH RISK
```

### Chain Risk Levels

| Chain length | Risk | Strategy |
|---|---|---|
| Single door (no dependencies) | LOW | Validate door normally per Step 2. |
| 2 doors in chain | MEDIUM | Validate the upstream door thoroughly. Downstream door: validate contract only until upstream is locked. |
| 3+ doors in chain | HIGH | Validate the ENTIRE chain with an end-to-end prototype before locking any single door. A wrong upstream choice invalidates all downstream validation. |

### Chain Awareness Rule

Do not validate a downstream one-way door before its upstream is resolved.
Validation of the wrong link produces confident-but-misleading results. Order matters:

```
WRONG: Validate database → choose database → validate cloud provider → choose cloud provider
RIGHT: Prototype full stack end-to-end → choose cloud provider + database together
```

### When You Hit This Naturally

Most projects don't have deep one-way door chains. This step is quick (5 minutes)
and primarily useful for:

- **Platform/infrastructure decisions** — language → framework → hosting → scaling
- **Multi-system integrations** — auth provider → data sync → compliance
- **Technology stacks that constrain future choices** — any build on a platform whose
  migration cost increases with adoption depth

## MULTI — Multidisciplinary Probe (folded from MULTI.md)

> Purpose: before committing to a decomposition strategy, check whether perspectives from OTHER fields (ethics, psychology, economics, ecology, design, sociology, law) have been considered. Without this, non-technical factors that determine real-world success are discovered too late.

**Step 1 — Domain Relevance Filter** (1 min). Classify the problem to select disciplines:

| Problem Type | Relevant Disciplines |
|---|---|
| User-facing product | + Psychology, Design, Ethics |
| Data-intensive system | + Ethics, Law, Economics |
| Platform/marketplace | + Economics, Sociology, Ecology |
| AI/ML system | + Ethics, Psychology, Law, Philosophy |
| Infrastructure/tool | + Economics, Security |
| Health/safety system | + Ethics, Medicine, Law |
| Public-facing service | + Sociology, Accessibility, Law |
| Internal/automation tool | + Psychology (team dynamics) |

If not in table, use CATWOE (from EXTRACTION): "Who is directly affected? Indirectly? Regulators or watchdogs?"

**Step 2 — Quick Probes** (3-5 min). Pick 2-3 disciplines, ONE question each:

- **Ethics:** "Who could be harmed? What values (privacy, autonomy, fairness, dignity) are at stake?"
- **Psychology:** "What cognitive biases might this exploit or create? Does it respect user attention?"
- **Economics:** "What incentives does this create? Who captures the value? What does it replace?"
- **Ecology:** "Environmental footprint? Does it encourage waste or resource consolidation?"
- **Design:** "Is the fundamental interaction intuitive? Does it respect the user's goals or the system's?"
- **Sociology:** "How does this affect social dynamics, power structures, or community cohesion?"
- **Law/Regulation:** "Compliance requirements, data protection obligations, liability implications?"
- **Medicine/Health:** "Could this affect physical or mental wellbeing, even indirectly?"
- **Accessibility:** "Does this assume able-bodied users? What happens for users with different capabilities?"

**Step 3 — Flag and Route** (1 min):

| Outcome | Action |
|---|---|
| No concerns | Proceed to DECOMPOSITION |
| Light flags | Document in .omo/plans/multi-findings.md; address in SPECIFICATION non-goals/constraints |
| Serious flags | Run Value Scenario (Step 4) before DECOMPOSITION |

**Step 4 — Value Scenario** (escalation, 5 min). If serious concerns flagged: write a 3-sentence story of a realistic use case where a non-technical value is at stake; then answer "What would we need to change in the design to prevent this?"

```
Example: "A student uses Oh-My-Learner to study for exams. The system schedules review at 2 AM because it maximizes retention based on SM-2. The student loses sleep, performs worse, and blames the tool for poor exam results."
```

**Escalation:** Light (Step 2 only, 3-5 min) → Medium (Steps 2+3, 5-10 min) → Heavy (Steps 2+3+4, 10-15 min, for health/finance/AI/children/vulnerable populations).

**Design principles:** (1) relevance filter first — not all disciplines apply; (2) flag, don't gate — surface concerns, don't block; (3) escalation path — most problems stop at Light; (4) CATWOE in EXTRACTION handles stakeholders; MULTI adds the cross-disciplinary lens.

MULTI findings feed: SPECIFICATION (non-goals, constraints, design for change), REVIEW (flagged concerns addressed), REFLECT (did the protocol catch blind spots?).

**Provenness:** Domain Relevance Filter — custom; Quick Probes + Value Scenario — Value Sensitive Design (Friedman & Hendry); CATWOE — Checkland Soft Systems Methodology.

## Integration

FUNDAMENTALS.md sits between EXTRACTION and DECOMPOSITION in the protocol pipeline:

```
EXTRACTION → FUNDAMENTALS (incl. MULTI) → DECOMPOSITION → AMBITION (incl. PACING) → LANDSCAPE → STRATEGY → VALIDATION → SPECIFICATION → EXECUTOR (incl. POLISH) → REVIEW (incl. EXPLAINER + SPEC_SYNC) → REFLECT → ship
```


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
## Cross-References

The protocol's steps operationalize a set of converged cross-domain fundamentals
(iteration, variation-selection, decomposition, uncertainty management,
simplicity-as-removal, wu-wei, beginner's mind, the useful void). Full evidence-graded
map in [docs/UNIVERSAL_FUNDAMENTALS.md](docs/UNIVERSAL_FUNDAMENTALS.md).
Before designing a new step, check which universal fundamental it implements.

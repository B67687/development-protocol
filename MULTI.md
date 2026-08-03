# MULTI.md — Multidisciplinary Probe (v4)

> This runs in the prep sequence between FUNDAMENTALS (one-way doors validated) and DECOMPOSITION (dimensions broken down): `INBOX → PRIORITIZE → EXTRACTION → SERIOUSNESS → FUNDAMENTALS → MULTI → DECOMPOSITION → AMBITION → LANDSCAPE → STRATEGY → PACING → VALIDATION → SPECIFICATION → EXECUTOR → EXPLAINER → REVIEW → REFLECT → ship`.
>
> **Purpose:** Before committing to a decomposition strategy, check whether relevant perspectives from OTHER fields (ethics, psychology, economics, ecology, design, sociology, law) have been considered.
>
> **Without this step:** The protocol evaluates solutions against engineering criteria only. Non-technical factors that determine real-world success or failure are discovered too late.

## Step 1: Domain Relevance Filter (1 min)

Classify the problem to determine which disciplines are relevant:

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

If the problem type isn't in the table, use CATWOE (already in EXTRACTION) and add: "Who is directly affected? Indirectly? Any regulators or watchdogs?"

## Step 2: Quick Probes (3-5 min)

Pick 2-3 relevant disciplines from the filter above. Answer ONE question per discipline:

- **Ethics:** "Who could be harmed by this system? What values (privacy, autonomy, fairness, dignity) are at stake?"
- **Psychology:** "What cognitive biases might this exploit or create? Does it respect user attention?"
- **Economics:** "What incentives does this create? Who captures the value? What does it replace?"
- **Ecology:** "What is the environmental footprint? Does it encourage waste or resource consolidation?"
- **Design:** "Is the fundamental interaction intuitive? Does it respect the user's goals or the system's goals?"
- **Sociology:** "How does this affect social dynamics, power structures, or community cohesion?"
- **Law/Regulation:** "Are there compliance requirements, data protection obligations, or liability implications?"
- **Medicine/Health:** "Could this affect physical or mental wellbeing, even indirectly?"
- **Accessibility:** "Does this assume able-bodied users? What happens for users with different capabilities?"

## Step 3: Flag and Route (1 min)

| Outcome | Action |
|---------|--------|
| No concerns surfaced | Proceed to DECOMPOSITION |
| Concerns flagged (light) | Document in .omo/plans/multi-findings.md. Address in SPECIFICATION non-goals or constraints. |
| Concerns flagged (serious) | Escalate: run Value Scenario (Step 4) before proceeding to DECOMPOSITION. |

## Step 4: Value Scenario (Escalation — 5 min)

If Step 3 flagged serious concerns:

Write a 3-sentence story showing a realistic use case where a non-technical value is at stake.

```
Example:
"A student uses Oh-My-Learner to study for exams. The system schedules review at 2 AM because it maximizes retention based on SM-2. The student loses sleep, performs worse, and blames the tool for poor exam results."
```

Then answer: "What would we need to change in the design to prevent this scenario?"

## Escalation Path

| Level | Time | Trigger |
|-------|------|---------|
| **Light** (Step 2 only) | 3-5 min | Any problem with user or societal impact |
| **Medium** (Step 2 + 3) | 5-10 min | Flagged concerns about harm, incentives, or compliance |
| **Heavy** (Step 2 + 3 + 4) | 10-15 min | Health, finance, AI, children, vulnerable populations |

## Integration

MULTI sits between EXTRACTION and DECOMPOSITION:

```
INBOX → PRIORITIZE → EXTRACTION → SERIOUSNESS → FUNDAMENTALS → [MULTI] → DECOMPOSITION → AMBITION → LANDSCAPE → STRATEGY → PACING → VALIDATION → SPECIFICATION → EXECUTOR → EXPLAINER → REVIEW → REFLECT → ship
```

- **EXTRACTION** identifies X (the real problem) and maps stakeholders via CATWOE
- **MULTI** asks: "Are we considering the right perspectives across relevant fields?"
- **DECOMPOSITION** uses MULTI findings to inform which dimensions matter

MULTI findings are referenced in:
- SPECIFICATION.md (non-goals, constraints, design for change)
- REVIEW.md (verification that flagged concerns were addressed)
- REFLECT.md (did the protocol catch blind spots?)

## Design Principles

1. **Relevance filter first** — Not all disciplines apply to all problems. The filter prevents paralysis.
2. **Flag, don't gate** — Surface concerns. Don't block progress unless critical.
3. **Escalation path** — Light → Medium → Heavy. Most problems stop at Light.
4. **Already partially covered** — CATWOE in EXTRACTION handles stakeholder mapping. MULTI adds the cross-disciplinary lens that CATWOE misses.

## Provenness

| Technique | Source | Evidence |
|-----------|--------|----------|
| Domain Relevance Filter | Custom for this protocol | Designed from first principles |
| Quick Probes | Adapted from Value Sensitive Design (Friedman & Hendry) | 20+ years of VSD research, 17 documented methods |
| Value Scenario | VSD method | Peer-reviewed, used in industry |
| CATWOE | Checkland Soft Systems Methodology | Canonical, 30+ years |

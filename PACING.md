# PACING.md — Phase Budgeting, Tracking & Adjustment (v3)

> Once AMBITION locks the ambition and LANDSCAPE completes the research,
> PACING decomposes the total appetite into per-phase budgets, tracks
> actual time against estimate, and adjusts when pacing is off.
>
> Pacing serves both human (don't burn out) and AI (don't exhaust context).

## 1. Appetite → Phase Budget Mapping

Default phase budgets as percentage of total appetite:

| Phase                 | % of Appetite | Human/AI               | Session Budget                             |
| --------------------- | ------------- | ---------------------- | ------------------------------------------ |
| EXTRACTION            | 5%            | Human-heavy            | 1 session, ≤90 min                         |
| FUNDAMENTALS          | 5%            | Mixed                  | 1 session                                  |
| DECOMPOSITION         | 5%            | Mixed                  | 1 session                                  |
| AMBITION               | 10%           | Human-heavy            | Max 3 rounds per session                   |
| LANDSCAPE              | 10%           | AI-heavy               | 1-2 sessions, compact between              |
| STRATEGY               | 5%            | Mixed (human ratifies) | 1 session; Phase 1 pre-commitment is human-first |
| VALIDATION            | 10%           | AI-heavy               | 1 spike session, fresh context             |
| SPECIFICATION         | 10%           | Mixed                  | 1-2 sessions                               |
| EXECUTOR (incl. FINISH)   | 30%           | AI-very-heavy          | Break into milestones; FINISH reserves 20-30% of EXECUTOR budget |
| EXPLAINER + SPEC_SYNC    | 5%            | AI-heavy               | 1 session each                             |
| REVIEW                   | 5%            | AI-heavy (independent) | 1 fresh session, no prior context          |

**Adjustment rule:** These are starting defaults. If appetite is 1 week (< 40h), EXECUTOR gets 40% and prep phases compress proportionally. If appetite is 6 weeks, prep phases expand to their full share.

Schedule FINISH/POLISH as a dedicated sub-phase of EXECUTOR with its own budget
allocation — 20-30% of EXECUTOR's total budget reserved for the last mile.

> **Evidence note:** Cross-domain research shows the finish phase is systematically
> underestimated: architecture allocates ~40% to construction documents, software
> allocates ~50% to testing/debugging (Brooks), and film's post-production (10-30%
> of budget) is the most routinely underestimated budget category.

## 2. Phase-Level Tracking

After each phase completes, log actual time vs budget in `.omo/pacing-track.md`:

```
Baseline
  Appetite: 3 weeks
  Phase budget: EXTRACTION 1.5h, FUNDAMENTALS 1.5h, ...

Phase Log
  EXTRACTION   [✓] budget 1.5h | actual 1.2h | -0.3h (ahead)
  FUNDAMENTALS [✓] budget 1.5h | actual 2.0h | +0.5h (over)
  AMBITION     [→] budget 3.0h | actual 1.0h | in progress

Cumulative Delta: +0.2h (slightly over budget)
Pace Alert: None
```

**Tracking rules:**

- Log after each phase completes (phase name, budget, actual, delta)
- Cumulative delta carries forward (gains from early phases can fund overruns)
- If any phase exceeds budget by 50%: **Pace Alert** — triggers automatic adjustment

## 3. Mid-Project Pacing Adjustment

When a Pace Alert fires (or on human request):

```
1. DIAGNOSE: Why is the phase overrunning?
   - Budget unrealistic? (calibration error → adjust remaining budgets)
   - Scope too large? (scope creep → hammer it down)
   - Verification tax higher than expected? (adjust process, not scope)

2. ADJUST: Choose one:
   a. Transfer budget from a later phase (e.g., POLISH buffer)
   b. Reduce scope for remaining phases (Shape Up style)
   c. Extend appetite (rare — requires explicit override reason)

3. RECORD: Update PACING_TRACK.md with reason and new budgets.
```

## 4. AI Session Boundaries

Each phase has a max before forced compaction or new session:

| Phase         | Max Turns     | Max Reads     | Action at Limit              |
| ------------- | ------------- | ------------- | ---------------------------- |
| EXTRACTION    | 20            | 10            | Compact between rounds       |
| AMBITION      | 30            | 15            | New session per 2 rounds     |
| LANDSCAPE     | 50            | 40            | Compact at source saturation |
| VALIDATION    | 40            | 30            | One spike, fresh session     |
| SPECIFICATION | 40            | 30            | Compact at section boundary  |
| EXECUTOR      | Per milestone | Per milestone | New session per milestone    |
| REVIEW        | 30            | 10            | Fresh session (required)     |

Signals of context decay:

1. Inconsistent naming (UserProfile → userProfile → User)
2. Repeated questions about things already answered
3. Suggestions that ignore recent edits
4. Loss of "ask before destructive" instinct

If any signal appears, compact proactively. Do NOT wait for auto-compaction at 95%.

### Session-Isolation Hard Stops (magic-spec pattern)

Between certain phases, a FRESH session is MANDATORY — context from the prior
phase must not bleed into the next:

| Boundary | Hard stop? | Why |
| --- | --- | --- |
| EXTRACTION → FUNDAMENTALS | Yes | Extraction conclusions must not pre-bind fundamentals analysis |
| AMBITION → LANDSCAPE | Yes | Locked ambition must be tested against fresh research, not confirmed |
| LANDSCAPE → STRATEGY | Yes | Strategy must judge the research map, not inherit it |
| EXECUTOR → REVIEW | Yes (already required) | Review must be blind to builder intent |
| REFLECT → next cycle | Yes | Retrospective needs distance from execution |

> The hard stop is enforced by starting a new session (or compacting to a fresh
> context) at the boundary. Magic-spec calls these "session-isolation hard stops" —
> a concrete fix for context-bleed, which its authors found to be the primary
> source of cross-phase contamination. Confidence: Medium-High.

## 5. Human Energy Rules

| Rule                                              | Why                                                         |
| ------------------------------------------------- | ----------------------------------------------------------- |
| Max 3 AMBITION rounds per session                 | Socratic dialogue is cognitively demanding                  |
| POLISH during peak cognitive hours                | Human final pass requires full attention                    |
| 30 min recovery between human-heavy phases        | Ultradian rhythm cycles (90 min focus / 20 min rest)        |
| EXTRACTION separated from AMBITION by at least 1h | Extraction and appetite-setting use different mental models |

## 6. Macro Pacing (projects > 2 weeks)

For multi-week projects, add cycle structure:

| Phase         | Duration    | What happens                                                                                          |
| ------------- | ----------- | ----------------------------------------------------------------------------------------------------- |
| **Cycle**     | 4-6 weeks   | EXECUTOR for one milestone. Produces shipped artifact.                                                |
| **Cool-down** | 1 week      | Bug fixes, dependency updates, shaping next cycle. POLISH/EXPLAINER/SPEC_SYNC/REVIEW for prior cycle. |
| **Betting**   | Cycle start | Select next work. Items from prior cycle compete fresh. No automatic extension.                       |

**Circuit breaker:** If a cycle ships late, default is to drop it and re-shape. Extensions require: (a) all remaining work downhill, (b) scope hammered, (c) human override recorded.

## 7. Integration (pipeline position)

PACING.md is executed between STRATEGY (strategic intent ratified) and VALIDATION (prototyping gate):

```
AMBITION → LANDSCAPE → STRATEGY → [PACING — budget allocation] → VALIDATION → ... → EXECUTOR → [PACING — tracking + adjustment] → ...
```

The pacing baseline is set once (after STRATEGY) and tracked continuously (after each phase).
---

## 8. Effort Estimation (Lightweight)

Phase budgets (% of appetite) provide the macro frame. For task-level estimation
within EXECUTOR milestones, use one of these techniques:

| Technique | How | Best for |
|---|---|---|
| **T-Shirt sizing** | S/M/L/XL. S = <1h, M = 1-4h, L = 4-8h, XL = >8h (split) | Early milestones, high uncertainty |
| **Decomposition** | Break milestone into sub-tasks. Estimate each in hours. Sum = milestone estimate. | Later milestones, spec is locked |
| **Reference class** | Compare to a similar completed task. Adjust for differences. | Repetitive work (ports, migrations) |

**Calibration rule:** After each milestone, log actual vs estimated. After 3 milestones,
adjust future estimates by the average error. A consistent overestimator who never
adjusts produces useless estimates.

**Communication:** Effort estimates are ranges, not promises. "This milestone will take
4-8 hours" is realistic. "This milestone will take exactly 6 hours" is a guess.

## 9. Cost Tracking

Track human time and AI costs separately. The distinction matters for pacing decisions.

| Resource | What to track | Format |
|---|---|---|
| **Human time** | Hours spent in each phase | Log in `.omo/pacing-track.md` per-phase actual |
| **AI compute** | Session duration, model used, approximate token count | Log in `.omo/cost-track.md` per session |
| **Felt effort** | User's subjective load per phase (low/med/high + note) | Log in `.omo/pacing-track.md` per-phase |
| **Total cost** | Human + AI combined | Summed per milestone in `.omo/cost-track.md` |

When combined human+AI cost exceeds 150% of the milestone's phase budget,
trigger a Pace Alert (see Section 3). Overruns in either dimension signal
a budgeting problem — not just slow execution. A durable mismatch between
process effort and work value (sustained high felt effort on low-value
steps, or ceremony that outgrows the work it serves) is also a Pace Alert
trigger and a REVIEW input — the protocol's own Effortlessness principle
applies to its own execution.

---

## 10. Future: Probabilistic Forecasting (Monte Carlo)

> *Phase 2 addition — requires historical data from 5+ completed projects.*

Once `.omo/pacing-track.md` has enough actual-vs-estimated data per phase, you can
move from deterministic budgets to probabilistic ranges:

1. **Input distributions** — For each phase, record optimistic / likely / pessimistic durations
   from historical entries
2. **Run simulations** — 10,000 iterations sampling from the distributions
3. **Read the spread** — "70% chance we finish SPECIFICATION within 4-6 days" vs a single number

This shifts the protocol from "here's your budget, stick to it" to "here's the probability
distribution — decide what confidence level you need."

Implementation note: this needs a small script (Python, JS, whatever). The protocol
provides the data model; simulation is tooling outside Markdown scope.

## 11. Integration (cross-section index)

PACING now spans three concerns:

```
Budget allocation (Section 1-3) -> Session management (Section 4) -> Human energy (Section 5)
-> Macro cycles (Section 6) -> Effort estimation (Section 8) -> Cost tracking (Section 9)
-> Probabilistic forecasting (Section 10)
```

Set the baseline after STRATEGY. Track after each phase. Adjust at Pace Alerts.
Close with REFLECT: compare total cost to appetite and log calibration data.

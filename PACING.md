# PACING.md — Phase Budgeting, Tracking & Adjustment (v3)

> Once AMBITION sets the total appetite, PACING decomposes it into per-phase
> budgets, tracks actual time against estimate, and adjusts when pacing is off.
>
> Pacing serves both human (don't burn out) and AI (don't exhaust context).

## 1. Appetite → Phase Budget Mapping

Default phase budgets as percentage of total appetite:

| Phase                 | % of Appetite | Human/AI               | Session Budget                             |
| --------------------- | ------------- | ---------------------- | ------------------------------------------ |
| EXTRACTION            | 5%            | Human-heavy            | 1 session, ≤90 min                         |
| FUNDAMENTALS          | 5%            | Mixed                  | 1 session                                  |
| DECOMPOSITION         | 5%            | Mixed                  | 1 session                                  |
| AMBITION              | 10%           | Human-heavy            | Max 3 rounds per session                   |
| LANDSCAPE             | 10%           | AI-heavy               | 1-2 sessions, compact between              |
| VALIDATION            | 10%           | AI-heavy               | 1 spike session, fresh context             |
| SPECIFICATION         | 10%           | Mixed                  | 1-2 sessions                               |
| EXECUTOR              | 30%           | AI-very-heavy          | Break into milestones, fresh per milestone |
| POLISH                | 5%            | Human-heavy            | Peak cognitive hours only                  |
| EXPLAINER + SPEC_SYNC | 5%            | AI-heavy               | 1 session each                             |
| REVIEW                | 5%            | AI-heavy (independent) | 1 fresh session, no prior context          |

**Adjustment rule:** These are starting defaults. If appetite is 1 week (< 40h), EXECUTOR gets 40% and prep phases compress proportionally. If appetite is 6 weeks, prep phases expand to their full share.

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

## 7. Integration

PACING.md is executed between AMBITION (appetite set) and LANDSCAPE (research starts):

```
AMBITION → [PACING — budget allocation] → LANDSCAPE → ... → EXECUTOR → [PACING — tracking + adjustment] → ...
```

The pacing baseline is set once (after AMBITION) and tracked continuously (after each phase).

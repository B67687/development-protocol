# SERIOUSNESS.md — Idea Evaluation Gate (v4.1)

> This runs between EXTRACTION (X found) and FUNDAMENTALS (one-way doors).
>
> **Purpose:** Determine whether an extracted X is worth the first 5 steps of the protocol.
> In ~30 minutes, the gate routes every idea to COMMIT, SCHEDULE, or DROP.
>
> **Without this gate,** every extracted X proceeds to FUNDAMENTALS by default — even
> casual thoughts and idle curiosities. The gate is the protocol's first "no."

## The Gate — 3 Phases

### Phase 1: Commitment Probe (5 min)

Answer these 4 questions before seeing the scoring rubric. This prevents anchoring to a desired outcome.

**Q1 — The $100 Bet**
Would you bet $100 of your own money that this X leads to something real within 30 days?

- Yes (+5)
- Maybe (0) — "Maybe" is a soft no
- No (−5)

**Q2 — The If-You-Couldn't-Build Test**
If you couldn't build this yourself (no coding, no design), would you still want X solved?

- Yes (+5)
- No (0)

Separates attachment to the building from commitment to the problem.

**Q3 — The Appetite Test**
Are you willing to spend 5 hours this week — before FUNDAMENTALS — validating the single riskiest assumption about X?

- Yes (+5)
- No (0)

5 hours is the gate's admission price. If the answer is no, the idea is not worth 5 days.

**Q4 — The Recent Behavior Test**
What have you actually done about X in the past 7 days?

- Concrete action (researched, prototyped, wrote) (+5)
- Thought about it but did nothing (−5)
- This is entirely new (−3)

Past behavior predicts future behavior. Action predicts action.

**Phase 1 Score:** Sum of 4 answers (−10 to +20).

### Phase 2: Dimension Scoring (20 min)

Score 5 dimensions, 0-20 each. Each has a self-score and a probe that can lower it. Probes never raise the score — they compensate for overconfidence.

**Dimension 1: Problem Severity (0-20)**

| Score | Description                                                       |
| ----- | ----------------------------------------------------------------- |
| 16-20 | Real pain: people actively seek solutions, spend money, lose time |
| 11-15 | Moderate: people notice it, have workarounds                      |
| 6-10  | Mild: people prefer it solved but wouldn't act                    |
| 0-5   | Vitamin: nice-to-have, no one would notice                        |

Probe: "Name one specific person who has this problem AND would commit to fix it."
Adjustment: If no specific person named, score caps at 10.

**Dimension 2: Personal Fit (0-20)**

| Score | Description                                            |
| ----- | ------------------------------------------------------ |
| 16-20 | Direct experience, existing network, unique capability |
| 11-15 | Know the domain, have relevant skill                   |
| 6-10  | Can learn, no current advantage                        |
| 0-5   | Completely new domain                                  |

Probe: "What specific unfair advantage do you have for this?"
Adjustment: If no advantage named, score caps at 10.

**D2b — Proven-Earner Benchmark (OPTIONAL — fires only when the money-tier decision is live)**

When you are choosing this idea partly for the money component of benefit (library vs platform tier), add one external-validity pass after the D2 probe. Self-assessment cannot tell you whether the opportunity will actually benefit *you* financially — only a benchmark can. Benefit here = money + learning + capability + mission progress, minus time and effort; comfortable-money-is-good-money is a valid target.
**Trigger (fires the check):** Is earning money from this idea part of your benefit calculation? If yes — even as a secondary motive — D2b fires. If you find yourself declaring money "not live" just to skip it, that avoidance is itself a signal to fire it.

1. **Name the analog.** Who is the STRONGEST person already earning in this niche (benchmark the ceiling, not the most convenient comparison)? (e.g. Sidekiq's Mike Perham for the library tier.) If you cannot name anyone earning here, that is the strongest kill signal for the money component — the benefit collapses to learning/capability/mission alone.
2. **Diff the 7 success prerequisites honestly** vs your own position: category ubiquity (is the need universal, not niche-specific) / daily visible pain (do sufferers feel it every day) / cheap migration (low switching cost from the incumbent) / network-effect community (does adoption compound) / buyer economics (can buyers pay and justify the spend) / personal distribution (do you already reach buyers) / timing (is the window open now). Score each you-have vs they-have; the diff is the gap you must close.
3. **Calibrate from the analog's actual numbers, not vibes.** Realistic expectations: solo-library mean ~$50k-500k/yr (e.g. EdiFabric 2-person at ~$600/yr, BerryWorks open-core since 2004, Prowide 5M+ Maven downloads); platform-tier outliers exist (Stedi ~$353M) and are not the library benchmark.
4. **Guards**: the benchmark can only lower the money component, never raise it (probe philosophy, same as D1-D5); a cherry-picked analog is caught by RoD L2 cross-check + adversarial REVIEW; record the analog and the diff in the decision journal (Phase 3).

**Dimension 3: Validation Confidence (0-20)**

| Score | Description                                        |
| ----- | -------------------------------------------------- |
| 16-20 | Tested with real users or paying customers         |
| 11-15 | Talked to users who validated the specific problem |
| 6-10  | Standard practice in my industry                   |
| 0-5   | Pure hypothesis, zero evidence                     |

Probe: "What evidence do you actually have vs what you assume?"
Adjustment: If a killer assumption (Phase 3) is untested, score caps at 8.

**Dimension 4: Cost of Being Wrong (0-20, HIGHER is better)**

| Score | Description                                      |
| ----- | ------------------------------------------------ |
| 16-20 | Risk is purely time spent                        |
| 11-15 | Some opportunity cost, other work delayed        |
| 6-10  | Reputational risk, would need to explain failure |
| 0-5   | Lose money, relationships, or career capital     |

Probe: "If X goes nowhere after 2 weeks, what do you specifically lose?"
Adjustment: If sunk cost is mentioned, score caps at 10. Past investment is not a reason to continue.

**Dimension 5: Urgency (0-20)**

| Score | Description                                                 |
| ----- | ----------------------------------------------------------- |
| 16-20 | Forcing function: deadline, window closing, pain escalating |
| 11-15 | Sooner is better, no immediate pressure                     |
| 6-10  | No urgency, could be done anytime                           |
| 0-5   | Timing is actively bad, market not ready                    |

Probe: "Why this week and not next month?"
Adjustment: If urgency is the ONLY high score (>15) and all others below 10, flag "urgency without substance" and recommend DROP/SCHEDULE.

**Phase 2 Score:** Sum of 5 dimensions (0-100).

### Phase 3: Kill Criteria (5 min)

Pre-commit the off-ramp before emotional investment clouds judgment. The worst time to make a quit decision is when you're already in it.

**Step 1 — Name the Killer Assumption**
What's the single assumption that, if wrong, makes X not worth pursuing?
Write it as a falsifiable sentence. Not "users want this." Instead: "At least 3 of 10 people I reach will spend 15 minutes trying a solution."

**Step 2 — Pre-commit the Off-Ramp**
When would you stop? What's the observable signal?
Good: "If fewer than 3 of 10 describe this as a top-3 problem by Friday, I stop."
Bad: "If it's not working, I'll reassess."

**Step 3 — Write the Decision Journal Entry**
Record:

1. What you're deciding (X)
2. What you predict will happen
3. How confident you are (numeric %)
4. What would change your mind (from step 2)
5. The date of first review
6. If D2b fired: the analog named and the prerequisite-diff summary

## Scoring Summary

| Phase                      | Range          | Weight  |
| -------------------------- | -------------- | ------- |
| Phase 1: Commitment Probe  | −10 to +20     | +20     |
| Phase 2: Dimension Scoring | 0-100          | +80     |
| **Total**                  | **−10 to 120** | **100** |

## Exit Criteria

| Score | Outcome      | Next Action                                                          |
| ----- | ------------ | -------------------------------------------------------------------- |
| >= 65 | **COMMIT**   | Proceed to FUNDAMENTALS with kill criterion as validation constraint |
| 35-64 | **SCHEDULE** | Park in .omo/backlog/. Set revisit date. No FUNDAMENTALS work.       |
| < 35  | **DROP**     | Record why in .omo/decisions/. No revisit for 90 days.               |

## Heuristics (When the User is Unsure)

| Situation                                    | Gate Response                                                                         |
| -------------------------------------------- | ------------------------------------------------------------------------------------- |
| 2+ mixed/unsure answers in Phase 1           | Default to SCHEDULE, not COMMIT                                                       |
| All dimensions cluster 30-60 total           | Check Confidence dimension. If < 10, recommend a 1-day experiment before FUNDAMENTALS |
| Urgency >= 16 but all others < 10            | Flag urgency without substance. Auto-DROP or SCHEDULE with 90-day delay               |
| Recent behavior: did nothing                 | Apply −5 penalty. If total >= 65, auto-schedule 1-week delay before COMMIT            |
| Maybe on the $100 bet                        | Treat as No (−5)                                                                      |
| First-time idea (just extracted)             | Apply −10 skepticism bonus                                                            |
| Recurring idea (keeps surfacing)             | Apply +10 persistence bonus                                                           |
| Kill criterion untested but Confidence >= 15 | Auto-cap Confidence at 8                                                              |

## Anti-Sunk-Cost Mechanisms

1. **Pre-commit scoring** — Phase 1 answered before Phase 2 is seen. Prevents adjusting commitment to rationalize a desired outcome.
2. **Default to DROP** — Gate assumes DROP until evidence accumulates. Inverts the protocol's default-COMMIT.
3. **Probes go down, never up** — Every follow-up probe can only lower the score. Compensates for overconfidence.
4. **No sunk-cost reasoning** — Cost dimension caps at 10 if sunk costs mentioned.
5. **Gate itself is cheap** — 30 minutes. If you DROP, you lost 30 min, not 3 days.
6. **Decision journal** — Record predictions before outcomes in `.omo/decisions/decision-journal.md`. Enables calibration feedback.
7. **Action framing** — De-escalation is framed as "choose something better" not "quit."

---

## Risk Register

A project-level risk register tracks risks from evaluation through execution.
Unlike one-way doors (binary: reversible or not), risks live on a probability
continuum and are monitored throughout the project.

### Initialize Here — `.omo/decisions/risk-register.md`

Start the register during Phase 2 (Dimension 4 — Cost of Being Wrong).
Update at every phase transition.

### Risk Assessment

| # | Risk | L(1-5) | I(1-5) | Score(LxI) | Mitigation | Owner | Status |
|---|---|---|---|---|---|---|---|
| R1 | {{what could go wrong}} | 1-5 | 1-5 | LxI | {{how to reduce}} | {{who}} | {{open/mitigated/closed}} |

**Likelihood:** 1=Rare, 3=Possible, 5=Almost certain.
**Impact:** 1=Negligible, 3=Moderate setback, 5=Project failure.

| Score | Action |
|---|---|
| 1-6 | Low — log, monitor at next transition |
| 7-12 | Medium — assign mitigation owner, check every phase |
| 13-25 | High — documented mitigation plan required before FUNDAMENTALS |

### Pipeline Integration

| Phase | Action |
|---|---|
| SERIOUSNESS | **Initialize** register with risks from Phase 2 dimensions |
| FUNDAMENTALS | **Add** one-way door chain risks. Cross-reference HIGH chains. |
| DECOMPOSITION | **Add** decomposition gaps between KNOWN/RESEARCH/PROTOTYPE |
| LANDSCAPE | **Update** likelihood/impact based on research findings |
| STRATEGY | **Add** ratification-deferral and premortem findings; log rejected-proposal risk |
| VALIDATION | **Close** risks the spike disproves |
| SPECIFICATION | **Reference** in Timeline contingency and circuit breaker |
| EXECUTOR | **Log** risks discovered during implementation |
| REVIEW | **Verify** completeness. Unmitigated HIGH blocks DISTRIBUTE. |
| REFLECT | **Close** — did the register capture what materialized? |

Keep the register lean (< 10 items). Every entry should be a risk you would
actually revisit. Noise in the register is worse than no register.

## Integration

SERIOUSNESS opens the risk register. All subsequent phases update it.
REFLECT closes it: the retrospective asks whether the register captured
the risks that actually materialized.

## Provenness

Risk registers are standard practice in project management (PMBOK, PRINCE2).
Likelihood x Impact scoring dates to the US Air Force (1950s) and is the
most widely used risk assessment framework in software engineering.

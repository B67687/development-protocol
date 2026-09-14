# Self-run audit — 2026-09-12 (P2b=b, standard)

Scope: backlog + constitution scoreboard + play/learning trend + prioritized backlog. External check designed, not yet run.

## 1. Constitution scoreboard

| Output                                                | Intended home                          | State                           | Evidence                                                                                                                                             |
| ----------------------------------------------------- | -------------------------------------- | ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Project (tighter scope, harder verification, shipped) | SPEC → EXECUTOR → VALIDATION → REVIEW  | applied                         | F-011 ceiling + F-013 cross-cycle + lint 8/8                                                                                                         |
| Human (more judgment, less abdication)                | F-017 learning layer                   | applied, **unmeasured**         | tripwire/explain-back/standing-decisions landed; per-cycle signal defined in REFLECT Q9 but **0 cycles trended** — first signal pending next REFLECT |
| Method (protocol revises itself with friction)        | Q9 seed + KILL_LOG retro + 3-pass rule | applied, **partially measured** | 3-pass rule in STANDING_PRINCIPLES; KILL_LOG retros due ~Sep 28; cross-cycle seed defined but only 1 cycle seeded                                    |

Verdict: architecture complete, measurement thin — same pattern as earlier thin builds.

## 2. Backlog review

| Item                                               | State              | Assessment                                                                                                                      |
| -------------------------------------------------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| F-016 OSS metric (proposed, docs fix queued)       | proposed           | Correctly gated — stranger FAIL, not useful-for-public. Keep proposed until engine docs land (other repo). Not a protocol debt. |
| Ambiguity gate F-018-adjacent (readings gate)      | applied (3 passes) | Now solid (stakes + flat-consensus + high-context). No backlog.                                                                 |
| Scope / cross-cycle / lying / raw-first / learning | applied + grounded | Re-verified. No reopen.                                                                                                         |
| Parked: Opportunity Engine                         | not started        | Real gap — discovery before want. Keep parked, P1 when appetite exists.                                                         |
| Parked: creativity principle                       | not started        | Overlaps Engine — scope together, don't duplicate.                                                                              |
| Parked: attribution rework                         | deferred by user   | Parked intentionally.                                                                                                           |

No hidden thin spot beyond F-016 and the unmeasured learning signal.

## 3. Play / cheapness check

VALIDATION spikes exist but cheapness not tracked: no spike-budget count. F-017 generation-before-instruction is the pedagogy, not a play budget. Cost: untracked spikes still cheap vs production — risk is low but invisible.

Decision: add a one-line counter (spikes this cycle) to the Q9 seed alongside the learning signal — one number, no ceremony. Don't build a play-budget gate now.

## 4. Prioritized backlog (P2a gates)

1. **Close the human scoreboard** — run one cycle to REFLECT and read the learning signal. P2a: SCHEDULE (needs a live run, not a doc edit). External check: cold-start read of STRATEGY ratification (can a stranger state standing decisions?).
2. **F-016 docs fix** — when engine docs ship, promote to applied. P2a: SCHEDULE (other repo).
3. **Spike counter** — add to Q9 seed next time Q9 edits. P2a: COMMIT — fix with next Q9 touch (this audit's design counts as that touch; ship the line).
4. Opportunity Engine / creativity — P2a: PARKED.

## 5. External check designed (not run)

Cold-start test for (1): hand a stranger the ratified STRATEGY + one EXECUTOR output, ask them to list standing decisions. Pass = correct without hints. This validates the standing-decisions display. Run it next cycle.

## 6. Finding to ship now

Only one doc change warranted now: add `spikes: N` to the Q9 learned-this-cycle seed so (3) closes without a separate cycle. Everything else is schedule/park.

# T-034 Stage 2 — the two gate stacks, rebuilt

**Date:** 2026-09-24
**Origin:** T-034 (the prompt-engineering thought), Stage 2, approved with its premortem.

## What was wrong

`steps/REVIEW.md` § Method Conformance Check carried 16 co-active checkboxes and
`steps/SPECIFICATION.md` § 15 carried 21. Both sat exactly on the lint's
21-checkbox ratchet, which is the worst case allowed and not a target.

The prompt-engineering sweep is what makes that a defect rather than a taste
call. A constraint that scores 99% compliance alone scores 20% among five others
(FollowBench), and the effect is worst for chained conditions (ComplexBench
rates composition the hardest). Six of the sixteen REVIEW items were already
machine-checked by `ledger-check.py` or by lint Rules 8 and 9, so the reviewer
was being asked to re-derive by hand what the tooling already knew.

## The rule the rewrite had to satisfy

Nothing is deleted. Every old item is rehomed: into a surviving check, into
`ledger-check.py`, or into the run's own records. The mapping tables below are
that proof.

## REVIEW.md — 16 → 7

| Old item | New home |
| --- | --- |
| 1 Fitness (methods invoked, resolvable evidence) | 1 Ledger fitness |
| 2 Skip-rate per reason code | 1 Ledger fitness |
| 3 Omitted entries (uncatalogued omission) | 1 Ledger fitness |
| 4 What-Matters proposal and its ratification | 2 One gate |
| 5 Divergence metric (no reward for checklist completion) | 6 No checklist-gaming |
| 6 Net-effort justification | 6 No checklist-gaming |
| 7 Trust-boundary conformance (velocity) | 3 Velocity and trust boundary, machine-checked by `ledger-check.py` |
| 8 One-Gate conformance (Invariant 11) | 2 One gate |
| 9 FEATURES.md conformance | 4 FEATURES and architecture fitness |
| 10 Architecture fitness audit | 4 FEATURES and architecture fitness |
| 11 Regression-Lock conformance | 5 Baselines, mutants and migration |
| 12 Mutation disposition conformance | 5 Baselines, mutants and migration |
| 13 Cross-cluster interference | 7 Run hygiene and records |
| 14 Migration conformance | 5 Baselines, mutants and migration |
| 15 Outcome-verdict conformance | 7 Run hygiene and records |
| 16 Quality-bar conformance | 7 Run hygiene and records |

## SPECIFICATION.md § 15 — 21 → 9

| Old item | New home |
| --- | --- |
| 1 Placeholders filled | 1 Filled and final |
| 2 No TODO or TBD remains | 1 Filled and final |
| 3 Constitution has at least 3 principles | 2 Constitution and scope |
| 4 Out-of-scope list is non-empty | 2 Constitution and scope |
| 5 Y-Statement per architecture decision | 4 Decisions, dependencies, timeline |
| 6 Version constraint per dependency | 4 Decisions, dependencies, timeline |
| 7 Circuit breaker in the timeline | 4 Decisions, dependencies, timeline |
| 8 Tier 1 sections filled | 3 Tiers filled as applicable |
| 9 Tier 2 sections filled | 3 Tiers filled as applicable |
| 10 Tier 3 sections filled | 3 Tiers filled as applicable |
| 11 Spec self-consistency grep | 5 Self-consistency |
| 12 FEATURES.md hygiene | 6 FEATURES hygiene |
| 13 Test anchoring | 7 Test anchoring |
| 14 Source documents (§1.6) | 8 Source documents |
| 15 Concrete quality-gate commands | 9 Engineering-plugin group |
| 16 Fuzz targets | 9 Engineering-plugin group |
| 17 Benchmark suite | 9 Engineering-plugin group |
| 18 Snapshot testing | 9 Engineering-plugin group |
| 19 `cargo-deny` / `deny.toml` | 9 Engineering-plugin group |
| 20 Multi-platform CI matrix | 9 Engineering-plugin group |
| 21 Test-to-source ratio | 9 Engineering-plugin group |

## What each surviving check would have caught

REVIEW: *Ledger fitness* catches a run that skipped prescribed methods, a skip
code trending up as a lazy-out, or an omission nobody catalogued. *One gate*
catches a second ratification appearing or the single gate going unlogged.
*Velocity and trust boundary* catches an autonomous-learning decision that was
never classified or never ratified. *FEATURES and architecture fitness* catches
an invalid status, an in-scope item that is not approved, an applied feature
with no linked test, a test pointing at an unknown feature, and a paradigm-fit
gate that did not run. *Baselines, mutants and migration* catches a silently
updated golden baseline, an untriaged surviving mutant, and a migration without
its contract or separate-evaluator sign-off. *No checklist-gaming* catches a run
rewarded for invoking methods rather than for outcomes, and a rule added without
a net-effort justification. *Run hygiene and records* catches a change that
trespassed on another active cluster, a completed cluster with no outcome
verdict, and a quality bar chosen without a risk rationale.

SPECIFICATION § 15: the nine surviving checks still cover every pre-build
assertion the twenty-one did, grouped by what a reader has to look at rather
than by the section the fact lives in.

## Honest limits

The lint sees shape, not correctness. It counts checkboxes and prohibition
density, so it can prove the stacks shrank and cannot prove the gate still
works. The ratchet is now 12, which is the new worst case plus headroom, so the
stacks cannot creep back silently. One live run is the only real test, and no
run has used the new stacks yet.

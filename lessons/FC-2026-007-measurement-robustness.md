# Failure Capture: 2026-09-19 — SC2001 Project 1 (hybrid sort, part (c)(iii))

## Failure Identity
- **Failure ID:** FC-2026-007
- **Project:** SC2001 Project 1 — hybrid merge+insertion sort notebook (`sc2001-hybrid-sort`)
- **Phase when failed:** P4 EXECUTE — passed every in-protocol gate, caught by stakeholder review after delivery
- **Failure pattern:** NEW → **FP-017 (Undefended Measurement)**

## What Happened
- **Symptom:** The "optimal S" reported in part (c)(iii) is not stable. It comes from a single `time.perf_counter()` per candidate S, and the candidates sit within roughly 7% of each other in wall-clock, so the winner moves between runs. The notebook's own prose admits it — "The fastest S moves between runs", "the seconds column ... is not stable", "# Wall clock jitters between runs, so the fastest S at this size moves around" — and two late commits (`42d6487`, `df83247`) exist purely to add those caveats.
- **Root cause:** The run verified inward (did the process run? does behaviour reproduce?) and never outward (is what the artifact asserts defensible against the world?). The reported number is consumed downstream: the specification defines part (d)'s S as "the fastest-by-time S at 10 million found in (c)(iii)" — so an unstable measurement makes the deliverable's own stated method non-reproducible.
- **Time to detect:** After delivery, by the stakeholder.
- **Time to fix:** One measurement change (repeat + median, or report an interval) plus the dependent text — the earlier version of this deliverable had done exactly that.

## Classification
- **Category:** Quality
- **Severity:** MAJOR — the deliverable's headline claim is indefensible under a repeat a reader can run
- **Repeatability:** Often — any single-measurement benchmark on a noisy machine; AI defaults to one run per data point
- **Agent involvement:** Agent-caused. The instability was observed, documented, and left in place.

## Protocol Gap
- **Which rule should have caught this:** The EXECUTOR FINISH gate's polish checklist, and REVIEW Phase 4's observable-quality checks.
- **Why it didn't:** Neither asked whether a REPORTED NUMBER survives a repeat. The polish checklist covers code and docs quality; REVIEW 4.1–4.7 cover tests, build, secrets, README, CI, standards and (since FP-015/016) voice. Measurement robustness had no line. Measurement DISCIPLINE was not entirely absent — the spec required the notebook to say what counts as a comparison — but robustness was never required.
- **Suggested fix:** REVIEW Phase 4 row 4.8 (measured assertability) + VALIDATION rule that observed instability becomes a requirement rather than a caveat. Both added.

## Learning
- **What we learned:** The protocol's verification was conformance-inward and never claim-outward. SC2001 is one instance of the class: an unstable number shipped with an honest caveat attached instead of a fixed method. A caveat does not discharge the claim.
- **Prevention:** Row 4.8 (method stated inline; a repeat must give the same conclusion; a flipping optimum fails). VALIDATION promotes spike-observed instability to a requirement. FP-017 gives the pattern flag/explain/stop teeth. RULES §11 now also fixes the neighbouring gap: curated decision records belong in the project repo, not only the untracked `.omo/` workspace — the rebuild dropped the incumbent's repeat-and-median method and nothing was watching for it, because a rename-migration reproduced every output while silently dropping a decision.
- **Severity if undetected:** A reader reruns the notebook, gets a different optimal S, and the whole analysis loses credibility. Quietly compounding: each later session re-derives the project as a newcomer.

## Evidence
- Shipped notebook `sc2001-project1.ipynb` at commit `b72bf6f`; the measurement sites use one timed run per candidate S.
- Late prose caveats at commits `42d6487` and `df83247` (written after the defect surfaced, not a fix).
- The run's own validation spike, `.omo/plans/validation-spike.md` line 77: "2 to 5 reps. Fine for a shape check, not for the reported (d) numbers" — the instability was known and filed as a caveat.
- `.omo/plans/specification.md` line 74: "(d) n = 10,000,000 with S chosen as the fastest-by-time S at 10 million found in (c)(iii)" — the unstable quantity is a required input.
- The rebuild (`0455c2b`) was almost entirely renames: the incumbent's code survived in `archived/`, its decisions did not. The earlier repeat-and-median method was never committed and never inventoried.
- Agent-facing twin: `~/.config/opencode/LESSONS.md`, 2026-09-19 section.

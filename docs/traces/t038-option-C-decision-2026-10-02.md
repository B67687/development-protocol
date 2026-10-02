# T-038 option C — one basis spec compiled into two renderings — DECISION 2026-10-02

## LANDSCAPE (light, no new research)
- Already have: manifest (`docs/PROTOCOL_MODEL.md` 5-col states table + supporting files + justification index), canonical spine literal quoted verbatim in 5 files, lint Rule 11a/b/c with seeded-violation proofs (strip card → 12a fails, break spine → 11b fails, blank basis cell → 11c fails), RESEARCH_BASIS rows, prompt standards two registers.
- McMillan et al. (2024) nulls on instruction position/architecture: reorganizing files did not move adherence; point-of-use re-injection and checkers did. A generator makes step files generated artifacts — a one-way door for the document model — for no measurable adherence gain.
- Generator would need a compiler, drift now becomes tooling failure, and every spine edit pays the compiler tax.

## Decision
**DROP option C.** Keep option B (fields + one manifest + checker). Revisit only if a LANDSCAPE pass finds a case where checker-measured drift exceeds generated-artifact maintenance cost.

## What shipped instead (already)
Stage 1 fields (RESEARCH_BASIS 13 rows + context declaration + REVIEW 4.13), Stage 2 manifest + spine + Rule 11. Lint 17/17, seeded proofs green.

## Parked
Entry A5 stays CLOSED as DROPPED with this record as the evidence. No code change.

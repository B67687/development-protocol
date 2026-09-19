# Failure Capture: 2026-09-19 — SC2001 Hybrid Sort (Project 1)

## Failure Identity

- **Failure ID:** FC-2026-006
- **Project:** SC2001 Project 1 — hybrid merge-insertion sort
- **Phase when failed:** P4 EXECUTE (FINISH gate) — passed every in-protocol check, caught only by stakeholder review after delivery
- **Failure pattern:** NEW × 2 → FP-015 (Assignment Narration), FP-016 (Uniform-Rhythm Voice)

## What Happened

- **Symptom:** The delivered notebook narrated the assignment instead of the work — "the four analyses the brief asks for", "The brief says to make arrays", "That is the metric the brief asks for", "anyone marking this gets the same numbers we do" across 8 markdown cells. Stakeholder called this the single most important thing to fix. Separately, the artifact scored 8.5/10 and its only real criticism was that it was "not human enough", despite plain, accurate, concise prose.
- **Root cause:** The protocol had no artifact-voice surface. The FINISH gate polishes code and documentation quality (error handling, edge cases, docs, performance) but nothing governed the register of a user-facing deliverable. The AI's default register — meta-commentary about the task, plus uniform rhythm — passed every existing check because no check looked at prose voice.
- **Time to detect:** after delivery (stakeholder review)
- **Time to fix:** one voice pass — 8 markdown cells; all technical facts and every stored cell output preserved byte-identically

## Classification

- **Category:** Quality
- **Severity:** MAJOR — the deliverable *is* the product; an artifact that reads as machine-composed loses credibility exactly where it is judged.
- **Repeatability:** Often — this is the AI default register, so it recurs on every human-facing deliverable unless explicitly checked.
- **Agent involvement:** Agent-caused (register and rhythm defaults), not prompt-caused.

## Protocol Gap

- **Which rule should have caught this:** the EXECUTOR FINISH gate should have carried a deliverable-voice check, and REVIEW Phase 4 (observable quality) should have verified it.
- **Why it didn't:** no artifact-voice or documentation-quality section existed anywhere in the protocol, and the REVIEW 4.x checks cover only tests, build, secrets, README, CI and standards — nothing about prose surfaces.
- **Suggested fix:** add EXECUTOR § Deliverable Voice (Rule 1 and Rule 2, each with a scope boundary and counterexample), a REVIEW 4.7 verification row, and register FP-015 / FP-016.

## Learning

- **What we learned:** "Accurate and concise" and "reads as human-written" are independent properties, not ends of one trade-off. The AI tell is rhythm — a trailing justification clause on most paragraphs, stock openers, uniform sentence length, a repeated three-beat section shape — not vocabulary. And an artifact must describe the work, never the assignment it satisfies.
- **Prevention:** EXECUTOR § Deliverable Voice → REVIEW 4.7 → FP-015 / FP-016 proactive flagging.
- **Severity if undetected:** deliverable judged as machine output; reviewer trust lost; rework at the worst possible time — after submission.

## Evidence

- Stakeholder review quotes: assignment-narration flagged as the single most important fix; "not human enough" as the only criticism.
- Shipped notebook `sc2001-project1.ipynb` at commit `b72bf6f` — meta-mention removal and voice pass; 0 occurrences of brief/marking/requirement/grader/rubric/submit afterwards; all 11 code cells' stored outputs byte-identical to the pre-rewrite backup.
- Agent-facing twin: `~/.config/opencode/LESSONS.md`, 2026-09-19 section (same two rules). This capture is the protocol's own version — it does not restate that file.

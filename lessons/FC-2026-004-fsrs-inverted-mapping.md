# Failure Capture: 2026-08-22 — Oh-My-Learner (FSRS Quality Mapping)

## Failure Identity
- **Failure ID:** FC-2026-004
- **Project:** Oh-My-Learner
- **Phase when failed:** WORK (verification)
- **Failure pattern:** FP-010 (Tautological Tests) + NEW (Inverted Logic)

## What Happened
- **Symptom:** FSRS quality mapping was inverted — quality 5 (perfect recall) was mapped to "again" (forgot), and quality 0 (complete blackout) was mapped to "easy" (perfect recall). Spaced repetition scheduling was backwards.
- **Root cause:** The mapping function `qualityToFSRSGrade()` had the grade constants assigned in reverse order. Tests passed because they tested the inverted behavior (tests were tautological — they verified what the code did, not what it should do).
- **Time to detect:** 2 minutes (adversarial review agent caught it by comparing spec to implementation)
- **Time to fix:** 5 minutes (reversed the mapping, updated 5 tests)

## Classification
- **Category:** Quality
- **Severity:** CRITICAL
- **Repeatability:** one-time
- **Agent involvement:** AI-caused

## Protocol Gap
- **Which rule should have caught this:** §9 Test Philosophy — "Tests first. The test is written BEFORE the implementation."
- **Why it didn't:** Tests were written after implementation and verified the inverted behavior. Tautological tests don't catch logic errors because they test the implementation, not the contract.
- **Suggested fix:** Add a contract-first test requirement: "Test must describe expected behavior in plain English BEFORE the assertion. If the test description doesn't match the assertion, FAIL."

## Learning
- **What we learned:** Tautological tests are worse than no tests — they provide false confidence. The adversarial review agent (which only saw the spec, not the implementation) caught this in 2 minutes.
- **Prevention:** Always run adversarial review on critical logic. Write test descriptions in plain English before writing assertions. Mutation testing would catch this (inverted logic passes all tests but fails mutation score).
- **Severity if undetected:** Users would get "easy" ratings for forgotten cards and "again" ratings for perfectly recalled cards — completely broken spaced repetition.

## Evidence
- Adversarial review agent: bg_09d0a099
- Fixed in: core/scheduler_fsrs.go qualityToFSRSGrade()
- Tests updated: core/scheduler_test.go (5 tests)

# Failure Capture: 2026-08-22 — Oh-My-Learner (VISION.md Contradiction)

## Failure Identity
- **Failure ID:** FC-2026-005
- **Project:** Oh-My-Learner
- **Phase when failed:** DISCOVER (spec was wrong from the start)
- **Failure pattern:** NEW (Spec-Implementation Contradiction)

## What Happened
- **Symptom:** VISION.md listed "No AI/LLM generation" in the anti-scope section, but the project fully implements AI card generation via DeepSeek API. The spec contradicted the implementation.
- **Root cause:** VISION.md was written before the AI feature was added. The anti-scope was never updated when the decision was made to include AI generation.
- **Time to detect:** 2 minutes (adversarial review agent compared VISION.md to actual implementation)
- **Time to fix:** 3 minutes (removed "No AI/LLM" from anti-scope, added AI card generation to scope table)

## Classification
- **Category:** Process
- **Severity:** MAJOR
- **Repeatability:** sometimes
- **Agent involvement:** human-caused (spec not updated when scope changed)

## Protocol Gap
- **Which rule should have caught this:** §5 Spec Maintenance — specs should be updated when scope changes
- **Why it didn't:** No automated check verifies that VISION.md matches the actual implementation. The spec drift happened gradually as features were added.
- **Suggested fix:** Add a spec-implementation consistency check to the adversarial review: "Compare VISION.md anti-scope against actual imports/features. Flag any contradictions."

## Learning
- **What we learned:** Specs drift when scope changes aren't propagated. The adversarial review agent is the only thing that caught this because it compared spec to reality.
- **Prevention:** When adding a feature that was previously excluded, immediately update the spec. Add a "scope change" checklist to the WORK phase.
- **Severity if undetected:** New contributors would be confused by the contradiction. The spec would be untrustworthy.

## Evidence
- Adversarial review agent: bg_09d0a099
- Fixed in: VISION.md

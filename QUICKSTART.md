# QUICKSTART — Development Protocol in 30 Minutes

> Last verified: 2026-08-29. This is **Light mode** — a subset of the full protocol. If it diverges, the full phase docs are authoritative.

## The 5-Minute Version (10 sentences)

1. **INBOX** captures every idea — don't judge, just dump.
2. **EXTRACTION** finds the real problem X hidden inside your stated solution Y.
3. **SERIOUSNESS** scores whether X is worth pursuing (COMMIT / SCHEDULE / DROP).
4. **FUNDAMENTALS** checks for one-way doors — things you can't undo.
5. **DECOMPOSITION** splits X by Cynefin complexity (Complex vs Complicated) and routes each piece.
6. **AMBITION** sets direction and appetite (timebox) before any solution work.
7. **LANDSCAPE** maps what's already out there — don't rebuild what exists.
8. **STRATEGY** picks an approach and ratifies it in one gate.
9. **VALIDATION** spikes the riskiest assumption with a throwaway prototype (KILL / PIVOT / COMMIT).
10. **SPECIFICATION → EXECUTOR → REVIEW → REFLECT** — write the spec, build, verify, learn. Then ship.

**Core idea:** plan before you code, but calibrate the planning to the stakes.

---

## The 30-Minute Version: Run Light Mode on a Real Decision

Light mode = 6 phases, <2 hours. Gated by SERIOUSNESS score 35–50.

### Worked example: "Should I upgrade the CI pipeline?"

**Step 1 — INBOX (5 min):** Raw dump → clusters → triage. CI upgrade lives in "Infrastructure" cluster. Triage: Energy ✓, Timing ~, Tractability ✓ → runner-up. If it's #1, feed it to EXTRACTION.

**Step 2 — EXTRACTION (10 min):** Capture Y verbatim: "Upgrade CI pipeline to GitHub Actions." Apply 2–3 techniques:
- *Goal Climb:* Why upgrade? → "Faster builds" → Why? → "Ship faster" → X: "Build-test-deploy takes 18 min, blocks 4 deploys/week."
- *No-Computer Check:* Without tech, the wait is still there — people idle between push and green.
- Test X: one sentence, no solution words, measurable (18 min), 3+ alternative paths (cache, parallelize, migrate, do nothing).

**Step 3 — SERIOUSNESS (5 min):** Score 5 dimensions (0–20 each):
- Want 12, Matters 10, Know 14, Work 15, Sunk 2 → 53/120 = SCHEDULE. Run Light mode.
- If <35 → DROP. If 65+ → Standard/Deep mode (use full protocol).

**Step 4 — AMBITION (5 min):** Appetite = 4 hours. Success: "CI is 50% faster OR I know why it can't be." Light mode skips heavyweight strategy — approach is obvious.

**Step 5 — SPECIFICATION (5 min):** Write a 1-page spec: goal, non-goals, 3 acceptance criteria, one-way doors (none here). Skip the Traceability Matrix in Light mode.

**Step 6 — EXECUTOR + Verify:** Build, run review checklist, ship or park. Log to `.omo/protocol-state.md`.

**Done.** You spent 30 min deciding and scoping before touching CI config.

---

## Full Map: All 15 Phases (1 line each)

| # | Phase | File | When to use | Time |
|---|-------|------|-------------|------|
| -1 | INBOX | `INBOX.md` | Multiple ideas, unclear which to pursue | 10 min |
| 0 | EXTRACTION | `EXTRACTION.md` | Y contains a solution word (app/tool/system) | 15 min |
| 1 | SERIOUSNESS | `SERIOUSNESS.md` | Unsure if X is worth pursuing | 5 min |
| 2 | FUNDAMENTALS | `FUNDAMENTALS.md` | One-way doors might exist | 10 min |
| 3 | DECOMPOSITION | `DECOMPOSITION.md` | Problem has 3+ sub-dimensions | 15 min |
| 4 | AMBITION | `AMBITION.md` | Need direction + timebox before research | 15 min |
| 5 | LANDSCAPE | `LANDSCAPE.md` | Domain unfamiliar or competitive | 20 min |
| 6 | STRATEGY | `STRATEGY.md` | Multiple valid approaches exist | 10 min |
| 7 | VALIDATION | `VALIDATION.md` | Core assumption is unproven | 20 min |
| 8 | SPECIFICATION | `SPECIFICATION.md` | Ready to write the build contract | 20 min |
| 9 | EXECUTOR | `EXECUTOR.md` | Spec is ratified, time to build | 30% of appetite |
| 10 | REVIEW | `REVIEW.md` | Build done, need verification | 10 min |
| 11 | REFLECT | `REFLECT.md` | After ship or kill — what did we learn? | 10 min |
| — | BIAS_CATALOG | `BIAS_CATALOG.md` | At any decision — which bias is active? | 2 min |
| — | KILL_LOG | `KILL_LOG.md` | After any DROP/KILL — log + 30-day check | 2 min |

Light mode runs: INBOX → EXTRACTION → SERIOUSNESS → AMBITION → SPECIFICATION → EXECUTOR (6 phases). Everything else is skipped with a logged rationale.

---

## Decision Tree: What Should I Do Right Now?

```
"I have an idea"          → INBOX.md
"I'm not sure it's real"  → EXTRACTION.md (Techniques 1, 4, 6 first)
"Is this worth doing?"    → SERIOUSNESS.md (score it)
"I'm stuck / blocked"     → EXTRACTION.md (Bouncing Protocol) or FUNDAMENTALS.md (one-way door check)
"I need to choose how"    → STRATEGY.md
"I need to ship fast"     → QUICKSTART Light mode (this file) — skip heavyweight gates
"I need to ship right"    → Full protocol (Standard mode)
```

## Common Patterns

| Situation | Route |
|-----------|-------|
| Idea spasm, 5 things at once | INBOX Phase 1–3, park the rest |
| "We need better monitoring" (solution-smelling problem) | EXTRACTION before anything else |
| 2-hour bug fix, reversible | Light mode — INBOX-lite → direct fix → review |
| 3-month product | Standard/Deep — full pipeline |
| Existing codebase, no protocol | INBOX Brownfield row → reanchor → EXTRACTION |

## Glossary (15 terms that appear everywhere)

| Term | Means |
|------|-------|
| **X** | Real problem (extracted from Y) |
| **Y** | Stated solution ("I want a dashboard") |
| **Appetite** | Timebox you're willing to spend — fixed, not estimated |
| **One-way door** | Irreversible decision (schema, API, data, security) |
| **Cynefin** | Complexity frame: Clear / Complicated / Complex / Chaotic |
| **MECE** | Mutually Exclusive, Collectively Exhaustive (decomposition test) |
| **PACING** | Phase budgets as % of appetite |
| **Light / Standard / Deep** | Adaptive depth modes (SERIOUSNESS-gated) |
| **BIAS_CATALOG** | 8-bias detection catalog — which bias where |
| **KILL_LOG** | Log of DROP/KILL decisions + 30-day counterfactual |
| **RTM** | Traceability Matrix — which spec section came from which idea |
| **Thick frame** | Connection map + alternative framings from Raw-Thinking Pass |
| **Ratification** | Single-gate approval — human signs off, AI argued the case |
| **Spike** | Throwaway prototype to test riskiest assumption |
| **Method Ledger** | `.omo/method-ledger.jsonl` — audit trail of applied/skipped methods |

---

## Light Mode: What You Skip (and the Risk)

Light mode skips 6 phases. Each skip is logged — not invisible.

| Skipped Phase | Why it's ok in Light | Risk of Skipping | When to upgrade to Standard |
|---------------|----------------------|------------------|-----------------------------|
| FUNDAMENTALS | No one-way doors (no schema/API/data/security change) | Miss a hidden irreversible choice | One-way door found → Standard immediately |
| DECOMPOSITION | <3 sub-dimensions, no cross-cutting risk | Miss hidden complexity | Problem feels bigger mid-work → re-run DECOMPOSITION |
| LANDSCAPE | You've built this exact thing before | Competitor solved it better | Domain unfamiliar → Standard |
| STRATEGY | Approach is obvious (<2 options) | Chose wrong path | 2+ valid approaches → Standard |
| VALIDATION | Core assumption is proven or low-stakes | Build something that shouldn't exist | Risky assumption → spike first |
| REVIEW (full) | CC lights + build fix is enough for <4h work | No independent verification | High-stakes / shared code → full REVIEW |

**Overrun rule:** If any Light phase exceeds 1.5× its time budget, STOP and re-classify to Standard.

---

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| INBOX keeps producing 6+ clusters (fragmented) | Do raw dump only. Come back tomorrow — you need rest, not protocol. |
| EXTRACTION won't converge (all techniques give different X) | Use Bouncing Protocol (EXTRACTION.md) or Multiple Working Hypotheses — carry 2–3 X's through FUNDAMENTALS. |
| SERIOUSNESS score is 35–50 (SCHEDULE-low edge) | That's Light mode's lower bound. If energy is low, DROP it anyway — SCHEDULE is not a commitment. |
| Spec feels loved (IKEA effect) | Run BIAS_CATALOG §6 prompt: "If someone else wrote this spec, would I still ship it?" |
| Build fails 3× in EXECUTOR | Don't just fix — trigger feedback loop: EXECUTOR → AMBITION (is appetite real?) or → DECOMPOSITION (was split wrong?). |

---

## Next Step

You've read the map. Now pick a real decision and run Light mode end-to-end (30 min). When it works, graduate to Standard mode — the full protocol is the same ideas with more depth where the stakes demand it.

Full docs: `INBOX.md` → `EXTRACTION.md` → `SERIOUSNESS.md` → … → `REFLECT.md`.

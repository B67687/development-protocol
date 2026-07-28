# Research: Pacing for AI Agent Projects — 2026-07-27

## Applied to: Development Protocol — adding PACING.md

### Key Findings

1. **AI changes estimation structurally** — HIE framework (Alaswad 2026): 5 dimensions (LLM Reasoning Complexity, Context Completeness, Code Transformation Impact, Iterative Reasoning Cycles, Human Oversight Effort). The primary cost driver is NOT code generation — it's human verification.

2. **Shape Up provides the blueprint** — Appetite exists, but missing: per-phase budgets, phase-level tracking, circuit breaker per phase, cool-down rhythm, betting table.

3. **AI context decays** — LOCOMO: LLMs lag 56% behind humans on recall over long sessions. Context rot is structural. Compact at task transitions, not at 95%.

4. **Phases have different human/AI intensity** — EXTRACTION/AMBITION/POLISH are human-paced. LANDSCAPE/EXECUTOR/REVIEW are AI-paced. One pacing system must serve both.

### 7 Recommendations

**Tier 1 (high impact, low effort):**

1. Per-phase budget in AMBITION Round 5 (Pacing Annex)
2. PACING_TRACK.md in .omo/ (auto-generated)
3. Session budget guidance in RULES.md (max turns per phase)

**Tier 2 (high impact, medium effort):** 4. PACING.md — full protocol document (phase budgets, pacing adjustment protocol) 5. Human energy rules in AMBITION.md (max 3 rounds/session, peak hours for POLISH) 6. Scope hammering protocol in EXECUTOR.md

**Tier 3 (medium impact, higher effort):** 7. Macro cycle/cool-down structure for multi-week projects

### Key Sources

- Alaswad et al. (2026) — HIE framework, Springer Nature
- Singer (2019) — Shape Up
- Maharana et al. (2024) — LOCOMO, ACL
- Schwartz & McCarthy (2007/2026) — Energy Management

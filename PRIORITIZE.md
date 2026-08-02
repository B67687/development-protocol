# PRIORITIZE.md — Idea Comparison & Betting (v1)

> **This is Step 0.5** of the Development Protocol. It runs between INBOX (clustered ideas) and EXTRACTION (deep-dive on one problem).
>
> **Purpose:** When INBOX's shallow triage (Energy/Timing/Tractability) produces a near-tie — 2+ clusters with similar scores, or the user isn't sure which to pursue — this step adds structured comparison on three universal dimensions to break the tie.
>
> **Without this step:** The user picks the "loudest" cluster by default, or spends this session's energy second-guessing whether they chose right.

## When to Use This

Run PRIORITIZE when any of these are true coming out of INBOX:

| Condition | Why |
|---|---|
| 2+ clusters scored the same on Energy/Timing/Tractability | Surface triage can't differentiate — need deeper comparison |
| User says "I'm not sure which to pick" | The tie isn't resolvable by gut check alone |
| User has 4+ clusters (fragmented) | Too many options — systematic comparison prevents selecting by recency |
| User asks to compare ideas from different sessions | Cross-session prioritization (pulling from `.omo/inbox/parked/`) |

If none of these conditions are met — the #1 cluster is clear and the user is confident — skip this step entirely. Go straight to EXTRACTION.

## The Method — 3-Score Table (10-15 min)

For each candidate cluster (2-10 items), score it on three dimensions using a 1-3 scale:

| Score | Want (outcome desirability) | Know (confidence in approach) | Work (effort to ship) |
|---|---|---|---|
| **3** | Genuinely needed — this solves a real problem | I've done this before — I know exactly how | Hours — can ship in a single session |
| **2** | Would be nice — meaningful improvement | Educated guess — similar to things I've done | Days — a few focused sessions |
| **1** | Meh — interesting but not important | Pure guess — I don't know what I don't know | Weeks — significant unknown effort |

**Matters (4th dimension, mission-consequence):** score 1-3 — does this move the durability-weighted outcome? Price the cost of NOT doing it (cost-of-delay). 3 = mission-critical now · 2 = meaningful · 1 = marginal. Kept separate from the 3-score table because it measures consequence, not desirability.

### Scoring Rules

1. **Score all clusters on all dimensions (Want/Know/Work + Matters) before looking at totals.** This prevents confirmation bias (scoring your favorite higher across the board).
2. **Use the raw 1-3 score.** Do NOT compute a total, average, or composite. The value is seeing the *shape* of each bet — not reducing it to a number.
3. **The first idea scored sets the anchor.** To fight anchoring, score in random order — shuffle the list before starting.

### The Debiasing Question

After scoring, ask for the top 2 candidates:

> **"If someone else proposed this idea instead of me, would I still want to do it?"**

This fights the IKEA effect (overvaluing your own creations). If the answer is "no" for the #1 candidate, consider dropping it or swapping with #2.

## The Bet Decision (2 min)

Frame the decision as a **bet**, not a ranking:

> **"If I have [X hours/days] of focused time — which one of these do I want to spend it on?"**

**What Matters Check (principal contradiction):** before betting — which ONE of these matters most against the mission right now? What does not doing it cost? The AI proposes this analysis; the user ratifies (Strategist Posture). 10 seconds, forced when ≥2 candidates or mission ambiguity; not a matrix.

> **LOG the AI proposal and the user's ratification to the method ledger.** REVIEW conformance catches a missing entry (mirrors the Sufficiency Checkpoint in VALIDATION.md — this closes the enforcement asymmetry: without the log, the check can be silently skipped and no one can tell).

| Bet outcome | Action |
|---|---|
| Clear winner emerges | → EXTRACTION on the winner. Park others in `.omo/inbox/parked/`. |
| Two ideas tied after scoring + debias | Flip a coin. If you're disappointed by the outcome, you know which one you actually wanted. |
| All ideas score low (1-2 across the board) | None are ready. Park all. Do not force an artificial pick. Come back when you have more information. |
| The winner scares you (high Want + low Know) | Run a mini-LANDSCAPE on feasibility before entering EXTRACTION. 20 minutes of research to validate the idea exists in reality. |

## Output Format

```
PRIORITIZE         [✓] COMPLETE
  [✓] Candidates: 3 clusters compared
  [✓] Winner: Cluster B (Want=3, Know=2, Work=2)
  [✓] Debiased: "Would pick if someone else proposed it" — YES
  [✓] Bet: "Next 3 days on this" — locked
  [→] → EXTRACTION
  [→] Parked: Clusters A, C → .omo/inbox/parked/
```

## Integration

```
INBOX → [PRIORITIZE] → EXTRACTION → SERIOUSNESS → FUNDAMENTALS → MULTI → DECOMPOSITION → AMBITION → LANDSCAPE → STRATEGY → PACING → VALIDATION → SPECIFICATION → EXECUTOR (incl. POLISH) → EXPLAINER → REVIEW (incl. SPEC_SYNC) → REFLECT → ship
```

PRIORITIZE is **optional**. When the #1 cluster from INBOX is clear and the user is confident, skip it. The pipeline defaults to `INBOX → EXTRACTION`.

## Provenness

| Element | Source | Evidence |
|---|---|---|
| **4-dimension scoring** | ICE (Sean Ellis) + McKelvey Prioritization Formula (2026) + What-Matters Check | ICE widely used for solo decision-making; McKelvey adapts for small-scale by replacing Reach with outcome fit; Matters dimension adds mission-consequence |
| **Raw score display (no composite)** | Multi-attribute utility theory critique (Kahneman 2011) | Single composite scores hide tradeoff profiles; raw scores preserve decision shape |
| **Debiasing question** | IKEA effect (Norton, Mochon, Ariely 2011) | People value self-created ideas ~63% higher; external-rater framing corrects this |
| **Bet frame** | Shape Up betting table (Singer 2019) | "Which gets my next X days?" is more actionable than "which is best?" |
| **Coin flip tiebreaker** | Annie Duke, *How to Decide* (2020) | Reveals hidden preference through emotional reaction to random outcome |
| **Separate scoring from evaluation** | Generate-evaluate distinction (Osborn 1953) | Preventing simultaneous generation and scoring reduces anchoring and confirmation bias |

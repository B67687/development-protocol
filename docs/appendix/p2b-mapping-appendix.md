# Appendix — P2b Mapping to SWE Artifacts (Early SWE, templates on demand)

**Status:** `docs-only`, depth-gated (Light = log why skipped, Standard+ = fill).  
**Source gaps closed:** G1-G5, G7-G8.  
**Seam:** Consumes P2a COMMIT + WHICH-X? from `LANDSCAPE` P2b entry gate; feeds `STRATEGY` ready-signal. Depth-gate: appetite / SERIOUSNESS < mid+Light → one line skip.

## Design intent: SWE ⊂ PoP

PoP starts _before_ SWE at **P1 tacit (WANT incl. tacit, bouncing)** and **P2a SHOULD-BUILD? (Bar 1 DROP/COMMIT)**. SWE proper starts at the P2b tail — `WHICH-X?` via these templates → P3 best-plan → P4 execute. Each table below is a translation: **PoP decision id → SE artifact row** (not IEEE literalism; fill only if it sharpens WHICH-X?).

## Snapshot: PoP → SE

| PoP decision                                   | SE artifact row here                            |
| ---------------------------------------------- | ----------------------------------------------- |
| INBOX raw-thinking cluster                     | inventable domain concept (G2)                  |
| P1 tacit X (EXTRACTION 3-layer + bouncing X)   | testable capability statement (G2→G1)           |
| P2a COMMIT score + Know mid assumption         | risk assumption (G5)                            |
| P2b WHICH-X? = scaled / adjacent / more-than-X | reuse vs invent call + keep/drop rationale (G7) |
| WHICH-X trace id (`colour-blind-85-100` etc.)  | version baseline (G8)                           |
| P3 readiness (signal exists?)                  | V&V stem — handed to `p4-late-appendix.md`      |

## 1) Use-Case Table (G1) — Scenario row replaces IEEE 29148 clause

| #     | Actor                  | Goal             | Precond                | Success                    | Alt / Failure                   |
| ----- | ---------------------- | ---------------- | ---------------------- | -------------------------- | ------------------------------- |
| UC-01 | _e.g._ Solo strategist | _see P1 tacit X_ | _commit exists? (P2a)_ | _one-sentence, measurable_ | _fallback if PoP insight wrong_ |

Source: INBOX cluster + extraction probe. Fill one row per capability branch at P2b; delete column Alt if Light. Light may log `UC-skipped: app-constrained`.

## 2) Domain-Model Table (G2) — Concept bridge

| Concept                   | Defines                                | Evidence (invent if no product domain) |
| ------------------------- | -------------------------------------- | -------------------------------------- |
| e.g. Recursive strategist | altitude pin, two-bar gates, seam wire | `PHILOSOPHY` table + WHICH-X trace     |

Fill for each new concept WHICH-X invents; for reuse `adjacent`, reference borrowed concept name instead. One table per WHICH-X variant is enough.

## 3) Quality-Reqs Table (G3) — Measurable, not adjectival

| Capability              | Measurable requirement                                    | How validated (signal)                              |
| ----------------------- | --------------------------------------------------------- | --------------------------------------------------- |
| e.g. Full chain seeable | stakeholder can walk P1→P2a→P2b→P3→P4 on funnel in <2 min | expert walk-through (P3 prototype signal) + lint R8 |

Keep one NFR per row; reuse signal types from `STRATEGY` (pytest signal, measurement, harness `test_*`). Never list quality without a signal.

## 4) Stakeholder / ADR Table (G4) — Lightweight, feeds LANDSCAPE framing Q bank

| Who                               | What they want                    | What would change your decision?                                     |
| --------------------------------- | --------------------------------- | -------------------------------------------------------------------- |
| e.g. Builder using PoP downstream | clear WHICH-X, not hidden variant | if downstream needs full Scrum/SRS, promote appendix to repo `docs/` |

Each row is one sentence; keep to 3-5 stakeholders (user, builder, reviewer/AI, deployer). Borrow LANDSCAPE stakeholder prompts if stuck.

## 5) Risk-Register Deriver (G5) — Three generic translates + G7 arch risk

Do not write risks from scratch — derive one row per PoP signal:

| Signal                           | Risk row                                                    |
| -------------------------------- | ----------------------------------------------------------- |
| SERIOUSNESS Know < mid           | assumption risk → validate with spike before P3             |
| WHICH-X = adjacent / more-than-X | integration / scope risk (borrowed concept mismatch)        |
| Pacing > appetite                | delivery risk (defer or split via STRATEGY)                 |
| G7 keep/drop = borrowed concept  | arch risk (keep incurs drift; invent incurs throwaway cost) |

Log only risks whose signal fired; delete the table if none fired (Light).

## 6) Keep / Drop Architecture Policy (G7)

For each WHICH-X variant that borrows an existing concept, answer three:

1. **Keep?** (reuse as-is)
2. **Drop?** (invent new concept)
3. **Replace abstract X with invented core?** (previous rule: abstract X → concrete, not `X`)

Logged as **WHICH-X rationale** inside `LANDSCAPE` P2b decision. Dogfood example: `colour-blind` trace keeps PoP ladder concept, invents funnel trace artifact (more-than-X subsumes one-pager) — no new phase.

## 7) Artifact Version Policy (G8)

One line per appendix fill: `major` when WHICH-X changes scope (same→more-than-X), `minor` when tables fill, `patch` when trace id changes. Single `CHANGELOG.md` line per bump is enough; do not version each table.

## Stencil — `colour-blind-85-100` trace (reusing current dogfood, not abstract)

- **UC stencilled:** Actor=solo builder, Goal=walk full P1→P2b→P3 funnel seeably, Precond=P2a COMMIT fresh, Success=points to one-pager draft in <30s, Alt=fallback to QUICKSTART 5-min.
- **Domain stencilled:** Concept `COLOUR BLIND funnel` — Defines P1 tack (tacit visual ladder), Evidence `PHILOSOPHY` altitude pin + this trace.
- **Quality stencilled:** `chain seeable` → `<2-min walk` → expert walk-through (illustration, not code signal).
- **Stakeholder stencilled:** downstream builder wants PoP not hidden in 5K-line spec — captured as G1 alt.

## When to skip (Light)

Log one line in `LANDSCAPE` box: `Early SWE skipped: Light+adjacent-guessed — UC/domain inferred inline, no signal expected` and move on. Wire expects that log to satisfy keep/drop (G7) minimally.

## Where this is consumed

- `LANDSCAPE` P2b entry reads this appendix as fall-through (optional, gated).
- `STRATEGY` header checks `WHICH-X?` trace id + ready-signal existence before planning.
- `SPECIFICATION §1.5` RTM one-liner traces P2b → appendix → STRATEGY → `p4-late` → KILL_LOG retro (G8).

## Intuition note

These tables are _reporting_ decisions you already made at altitude, not creating new decisions. If filling a column feels like inventing, you missed the PoP decision upstream.

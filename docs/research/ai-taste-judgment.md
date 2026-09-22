# AI taste, judgment, and where a house standard can come from (2026-09)

Condensed from the in-house taste sweep so the change it informs is grounded rather than asserted. Raw session text is archived locally at `Agentic-Workflows/.omo/archive/sweeps/` (gitignored; listed in that archive's own INDEX).

## The question

Can "good taste" be fixed by harness alone, or does it need a different kind of solution — world models, or something the labs expect within a year?

## Findings

| # | Finding | Confidence | Basis |
| --- | --- | --- | --- |
| 1 | Taste behaves as a capability limit rather than a promptable behaviour: better scaffolding moves benchmark-style quality by single-digit points and then plateaus | LIKELY | harness sweep — scaffold-only spreads of 5–20pp on benchmark tasks, ≤7pts on taste-adjacent subjective ratings |
| 2 | Homogenisation persists under AI assistance: outputs converge on a shared register even as measured quality rises | LIKELY | Doshi & Hauser design-study tradition; ≈+10.7% idea homogenisation reported in the sweep |
| 3 | World models fix physical and spatial prediction; no evidence they transfer a taste judgment | UNVERIFIED | world-model survey — the transfer claim is untested |
| 4 | No lab claims a one-year fix for taste | LIKELY | absence across the frontier release notes and roadmap material reviewed in the same sweep |

## What this licenses

1. Encode the checkable layer — consistency, fitness for the stated purpose, legibility, defensibility — as a default the agent owns. That is T0/T1 in `docs/HOUSE_STANDARD.md`.
2. Leave taste with the human where evidence is absent (T3), and ask with a proposed default attached.
3. Treat a demonstrated preference (the user's own past artifacts, or a decision ratified this run) as the only legitimate evidence for applying taste unprompted (T2), and flag the inference.

## What it rules out

Claiming the agent has "objective taste". The evidence supports checkable criteria plus a declared low-regret default. It does not support taste authority, and a run that asserts one is overreaching.

## Open gaps

- Whether a declared T0 default reduces rework in a live run is unmeasured. Recommended as the external check for the next self-run (CONSTITUTION § Self-evolution guard).
- Whether T2 inference flagging changes user override rates is unmeasured.

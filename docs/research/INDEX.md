# Research index

Every research artifact in this repo is listed here. A sweep lands as a file in this directory at the moment it is produced, and it gets a row here in the same commit. This is the guard against re-researching what we already know.

Raw session texts that back these documents live outside the repo, in the local archive `Agentic-Workflows/.omo/archive/sweeps/` (gitignored, with its own `INDEX.md`).

| File | Date | What it holds |
| --- | --- | --- |
| [`prompt-engineering-science.md`](prompt-engineering-science.md) | 2026-09-20 | Instruction-following evidence base for our own text: position effects, constraint-count decay, composition difficulty, negation insensitivity, point-of-use repetition, over-constraint harms, context rot, within-session drift, the McMillan rules-file factorial (affirmative nulls for size and position), IFEval calibration, eight actionable rules, nine open gaps. Companion to `PROMPT_STANDARDS.md`. |
| [`ai-autonomous-execution.md`](ai-autonomous-execution.md) | 2026-07-11 | Autonomous execution research: what agents can be trusted to run unattended, where human gates still earn their keep. |
| [`harness-survey-2026-07.md`](harness-survey-2026-07.md) | 2026-08-01 | Harness survey: how much of measured agent performance is scaffold rather than model, and which harness mechanisms carry the effect. |
| [`ai-taste-judgment.md`](ai-taste-judgment.md) | 2026-09 | What the taste sweep licenses and rules out: taste as a capability limit, scaffolding gains that plateau, persistent homogenisation, untested world-model transfer, no claimed one-year fix. Companion to `HOUSE_STANDARD.md`. |
| [`human-thinking-preferences-2026-09-24.md`](human-thinking-preferences-2026-09-24.md) | 2026-09-24 | What makes thinking feel like progress and play rather than a chore: flow conditions, goal gradients and small wins, SDT, productive failure and desirable difficulty, ludic-vs-agonistic play, the anti-gamification case, documentation-burden transfer, licenses/rules-out, six open gaps. |

## Adding a row

When a sweep finishes, write the artifact here and add a row in the same commit: file, date, and what it holds in one or two lines. The lint check (Rule 10c) fails when a file in this directory has no row.

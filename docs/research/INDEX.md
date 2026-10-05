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
| [self-knowledge-spectrum-2026-09-24.md](self-knowledge-spectrum-2026-09-24.md) | 2026-09-24 | The self-knowledge spectrum: introspection limits and confabulation, self-other asymmetry, stated-versus-revealed gaps with numbers, emotional granularity and alexithymia, what improves self-knowledge, how to elicit a signal from someone who cannot articulate the want, licenses and rules-out, ten open gaps. |
| [philosophy-of-success-2026-09-24.md](philosophy-of-success-2026-09-24.md) | 2026-09-24 | How to succeed across the philosophical traditions and the empirical literature: eudaimonia, Stoicism, wu wei, Zen, Confucian cultivation, Epicureanism, pragmatism, existentialism, Nietzsche, ikigai and Jung; flow, grit, deliberate practice, goal-setting, locus of control and ACT with their critiques; then convergence, the genuine conflicts, testable versus poetry, three candidate principles and an explicit not-to-encode list. |
| [guidance-dynamic-2026-09-30.md](guidance-dynamic-2026-09-30.md) | 2026-09-30 | ZPD, scaffolding/fading, expertise reversal, desirable difficulties, progressive disclosure → pacing law for T-051..T-054 (contingent step-up, fade by demonstration, desirable difficulty inside reach). |
| [power-verification-2026-09-30.md](power-verification-2026-09-30.md) | 2026-09-30 | Power-safety evidence for T-055..T-057: expertise-reversal meta-analysis (d=0.505/-0.428), scaffolding fading triad, POSIX exemplar effect, 75:25 review mix, spec-defect propagation, three candidate principles. |

| [`proxy-xy-2026-10-03.md`](proxy-xy-2026-10-03.md) | 2026-10-03 | T-059 controllable-proxy X-Y: stigma→help-seeking small-moderate, compensatory control, strategy-situation fit, reframing SMD anxiety −0.21; P1 honor X before Y, match strategy to controllability. |
| [`just-right-perfection-2026-10-03.md`](just-right-perfection-2026-10-03.md) | 2026-10-03 | T-062/064 just-right over perfection + balancing: satisficing under intractability CONFIRMED, maximizing maladaptive LIKELY, moderators difficulty×complexity×uncertainty×effort, threshold-first principle. |
| [`max-plan-min-exec-2026-10-03.md`](max-plan-min-exec-2026-10-03.md) | 2026-10-03 | T-063 max-plan/min-exec: Boehm curve canon UNVERIFIED numbers, front-load knowledge not decisions, set-based > point-based, last responsible moment = most responsible moment. |
| [`sane-defaults-2026-10-03.md`](sane-defaults-2026-10-03.md) | 2026-10-03 | T-065 sane defaults: choice architecture d=0.45 [0.39,0.52], defaults d=0.68 [0.53,0.83], auto-enrollment +50%, CoC thesis; default the common case + separate participation/intensity. |
| [`scales-analysis-2026-10-03.md`](scales-analysis-2026-10-03.md) | 2026-10-03 | T-066/067 scales: Meadows 12-point ladder canonical, shallow clustering, downward causation, scale mismatch ⇒ mitigation≠restoration, 4-row leverage analysis. |
| [`improve-judgement-2026-10-03.md`](improve-judgement-2026-10-03.md) | 2026-10-03 | T-068 judgement: CHAMPS KNOW Brier +6-11% CONFIRMED, bias cut 30%/20% LIKELY, WHO checklist OR 0.60 LIKELY, scored-prediction ledger + performance gates. |
| [`project-bench-taxonomy-2026-10-03.md`](project-bench-taxonomy-2026-10-03.md) | 2026-10-03 | T-061 bench taxonomy: 10 gate-stress dimensions, 24 archetypes A1-A24, bench oracles + rubric calibration evidence (SWE-bench/PaperBench), instance contract principle. |
| [`protocol-transcript-2026-10-03.md`](protocol-transcript-2026-10-03.md) | 2026-10-03 | T-060 transcript: store-verbatim/serve-minimal, handoff tax <50%, tamper 5/6 harnesses, rate-distortion, debate diversity, retention 78%/97%; 3 principles + anti-patterns. |

## Adding a row

When a sweep finishes, write the artifact here and add a row in the same commit: file, date, and what it holds in one or two lines. The lint check (Rule 10c) fails when a file in this directory has no row.
| Thought-log handling (capture → disposition → fold) | thought-log-handling-2026-10-03.md | 2026-10-03 | If-then disposition g ≈ 0.3 CONFIRMED; verbatim-capture/conditional-paraphrase; traceability +86% LIKELY; single-search synthesis insufficient CONFIRMED |

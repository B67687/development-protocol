# Project-Bench Taxonomy — Archetypes & Rubric Design for the T-061 Massive Bench (2026-10-03)

> For T-061 (massive bench: ideate every possible kind of project, build all, generate a dataset evaluating the dev protocol): which project archetypes to include so the dataset stresses different protocol gates, what benchmark-suite science says about building and scoring such a bench, and what not to encode. Last updated: 2026-10-03. Local protocol grounding: gate names (INBOX, SERIOUSNESS/AMBITION, LANDSCAPE, EXTRACTION, STRATEGY/SPECIFICATION, EXECUTOR/REFLECT, Thought-log/Interruption gates) as observed in `thought-log-handling-2026-10-03.md` §§2–4 of this directory.

---

## 1. Executive Summary

**Key finding:** the benchmark literature converges on a small set of bench-building moves that transfer directly to T-061 — executable pass/fail oracles per instance (SWE-bench's FAIL_TO_PASS/PASS_TO_PASS split **[LIKELY]**), hierarchical rubrics co-developed with task authors plus a separately validated judge (PaperBench: 20 papers → 8,316 gradable tasks, best agent 21.0% **[CONFIRMED]**), adversarial test-strengthening because weak oracles inflate scores (one framework rejects ~20% of previously passing patches **[UNVERIFIED — snippet-only]**), live/refreshable task pools because static benches contaminate and saturate **[LIKELY]**, and rubric-measurability filtering before trusting LLM judges (human-gold κ 0.604 → 0.743 **[UNVERIFIED — snippet-only]**). The contested point is *what an archetype dimension is for*: coverage dimensions (language, domain) versus stress dimensions (which gate of the protocol the project pressures) — the literature builds the former, T-061 needs the latter, and no paper validates a gate-stress taxonomy **[CONFIRMED as gap — no source found across fan-out]**.

| # | Finding | Confidence | Effect size / number |
|---|---------|-----------|----------------------|
| 1 | Executable oracles (fail-to-pass + pass-to-pass tests in containers) are the load-bearing bench primitive | LIKELY | SWE-bench: 2,294 problems, 12 repos, best model 1.96% at launch |
| 2 | Hierarchical rubrics co-authored with task owners + a separately benchmarked LLM judge scale human grading | CONFIRMED | PaperBench: 8,316 gradable tasks; judge validated on its own benchmark; best agent 21.0% |
| 3 | Weak oracles inflate scores; adversarial strengthening deflates them substantially | UNVERIFIED | ~20% of passing patches rejected; top score 78.8% → 62.2% (snippet-only, one framework) |
| 4 | Static benches rot (contamination, saturation, narrow repos); live pipelines fix recency at the cost of stability | LIKELY | SWE-bench-Live: 1,319 tasks, 93 repos, post-cutoff issues; substantial gap vs static scores |
| 5 | Rubric-guided LLM scoring approximates but does not replace humans; calibrate and co-grade | LIKELY | Human–human ICC 0.819 vs AI–consensus 0.763; calibration MAE 1.624 → 1.215; "co-grading, not replacement" |

Method: SearXNG (`categories=science,general`) first-pass, full-text fetch of load-bearing sources (arXiv abs pages for SWE-bench and PaperBench), snippet-level for the rest. Confidence per the house scheme: ≥2 independent peer-reviewed sources = CONFIRMED; 1 peer-reviewed + 1 community = LIKELY; preprint-only = LIKELY max; single/snippet-only = UNVERIFIED. Local gate names via `thought-log-handling-2026-10-03.md` (LOCAL-CONFIRMED as cited there).

---

## 2. How Others Built Project Benches

**Rule the evidence supports: every durable bench pairs (a) real task material, (b) an executable or human-validated oracle, and (c) a frozen environment — and every bench that skips one of the three gets a follow-up paper showing why it mattered.**

- **SWE-bench (Jimenez et al., ICLR 2024): the template.** 2,294 real GitHub issue→PR pairs across 12 Python repos; the model edits a codebase to resolve the issue; evaluation runs FAIL_TO_PASS tests (fail before, pass after the gold patch) plus PASS_TO_PASS regression tests inside per-instance Docker environments **[CONFIRMED for counts — extracted abstract (https://arxiv.org/abs/2310.06770); LIKELY for the F2P/P2P mechanism detail — convergent secondary knowledge, full text not extracted here]**. Launch result (Claude 2 at 1.96%) shows a well-built bench starts humiliating and saturates later — design T-061's difficulty headroom the same way.
- **SWE-bench Verified: human filtering as a second gate.** OpenAI had professional annotators filter to 500 instances with adequate problem statements and tests **[LIKELY — convergent secondary; primary card not extracted]**. Lesson for T-061: automated instance mining over-generates; budget an annotator pass (or a validated judge pass, §5) that checks *solvability + oracle adequacy*, not just task realism.
- **SWE-Bench+ / UTBoost / SWE-ABS: the oracle-quality reckoning.** Independent audits find ~32.7% of resolved instances leak the solution in the issue text and ~31.1% pass on weak tests, collapsing a 12.47% resolve rate to ~3.97% after filtering; UTBoost finds 345 mislabeled passes moving ~41% of Lite and ~24% of Verified leaderboard entries; adversarial strengthening rejects ~19.7% of passing patches (78.8% → 62.2% for the top agent) **[all UNVERIFIED — single-snippet each: https://arxiv.org/abs/2410.06992; https://doi.org/10.18653/v1/2025.acl-long.189; https://arxiv.org/abs/2603.00520v1]**. Convergent direction across three independent efforts, divergent exact numbers — encode the *direction* (audit oracles adversarially), never the numbers.
- **SWE-bench-Live: the contamination answer.** 1,319 tasks from 93 repos, issues created since 2024, each with its own Docker image, behind an automated curation pipeline that removes the manual bottleneck; agents score substantially worse than on static SWE-bench **[LIKELY — extracted abstract-level record (https://arxiv.org/abs/2505.23419v2)]**. Lesson: T-061's dataset needs a *refresh procedure*, not just a snapshot — protocol versions drift, and a frozen bench silently measures the old protocol.
- **Multi-SWE-bench: coverage by construction.** 1,632 instances across 7 languages, annotated from 2,456 candidates by 68 experts, with the full production pipeline open-sourced for community extension **[UNVERIFIED — snippet-only (https://doi.org/10.52202/085713-2111)]**. Lesson: publish the *instance-production pipeline*, not just instances — T-061's bench should ship its archetype→instance generator so the dataset can grow without re-research.
- **Commit0: 0→1 as a bench.** Agents get an API spec + interactive unit tests and must write a whole library from scratch with static-analysis and execution feedback; no current agent fully reproduces a library **[LIKELY — extracted record (https://arxiv.org/abs/2412.01769)]**. This is the closest existing analogue to T-061's greenfield archetypes — and its headline result (feedback helps, completion doesn't) predicts where protocol-following agents will separate from raw coders.
- **PaperBench / RECLAIM / Collider-Bench: long-horizon grading without hand-oracles.** PaperBench decomposes 20 paper replications into 8,316 rubric tasks co-developed with the papers' authors, grades with an LLM judge validated on its *own* judge benchmark, and checks models against an ML-PhD human baseline (models lose; best 21.0%) **[CONFIRMED — extracted abstract (https://arxiv.org/abs/2504.01848)]**. RECLAIM fixes the success criterion and GPU budget *in advance* per paper, tiers difficulty by what the authors released (Run/Retrain/Reimplement: 41%/27%/15%), and grades from logs/outputs rather than agents' self-reports **[UNVERIFIED — snippet-only (https://arxiv.org/abs/2609.28850)]**. Collider-Bench replaces hand rubrics with continuous histogram-fidelity scores plus an LLM trace-judge that catches fabrications **[UNVERIFIED — snippet-only (https://arxiv.org/abs/2605.13950)]**. Lessons: pre-register success criteria per instance; tier by starting material; never grade the agent's story about its work — grade the work.

---

## 3. Archetype Dimensions That Stress Protocol Gates

**Rule the evidence supports: choose dimensions by which protocol gate they pressure, not by surface coverage (language, framework) — surface coverage is what every existing bench already does (§2), and it is orthogonal to evaluating a protocol.**

| # | Dimension (pole ↔ pole) | Protocol gate(s) stressed | Why it discriminates |
|---|------------------------|---------------------------|----------------------|
| D1 | 0→1 greenfield ↔ brownfield repair | EXTRACTION vs comprehension; AMBITION scoping | Greenfield tests want-articulation from nothing; brownfield tests constraint-respect inside existing structure |
| D2 | Spec-heavy ↔ taste-heavy | SPECIFICATION vs judgment (HOUSE_STANDARD side) | Spec-heavy is checkable; taste-heavy forces "good enough" calls the protocol must scaffold, not dodge |
| D3 | Fun/toy ↔ high-stakes | SERIOUSNESS gate; verification depth | Stakes should modulate caution — a protocol that verifies a toy and a funds-transfer identically is miscalibrated |
| D4 | Direct build ↔ proxy X→Y translation | EXTRACTION fidelity (Y→X); LANDSCAPE coverage | Proxy tasks (rebuild X's behavior in Y's idiom) expose whether the agent extracted the *want* or cargo-culted the *form* |
| D5 | Uninterrupted ↔ interruption-prone | Thought-log / Interruption gates (RULES.md) | Mid-build requirement changes test capture→disposition→refold, the exact loop `thought-log-handling` specifies |
| D6 | Oracle-verifiable ↔ judgment-verified | Verification design; REFLECT honesty | Test-oracle tasks check execution; judgment tasks check whether the agent *says* it is unsure when it should be |
| D7 | Well-posed ↔ tacit-laden | EXTRACTION clarification; SHOULD-BUILD? | Tacit tasks punish agents that never ask and reward the cheapest clarifying question |
| D8 | Single-session ↔ multi-session | BACKLOG/HANDOVER; session-close discipline | Multi-session builds test whether state survives the gap — the disposition machine's PARKED-with-trigger path |
| D9 | Hermetic ↔ dependency-risky | LANDSCAPE risk surfacing; DECOMPOSITION ordering | External APIs, deprecations, version pins test whether risks are named early or discovered late |
| D10 | Reversible ↔ one-way-door | STRATEGY reversibility analysis; consent gates | Migrations, deletes, publishes test whether the agent distinguishes undoable from undoable-only-by-apology |

Design note: 10 binary-ish dimensions span far more cells than any bench can fill — that is the point. Sample *gate-collisions* (instances sitting at stressful intersections, e.g. tacit + high-stakes + one-way-door), not uniform coverage. A bench of 25 archetypes that each collide 2–3 dimensions discriminates protocol quality better than 100 archetypes spread thin **[derived design claim — no literature; encode as hypothesis, see §8]**.

---

## 4. The Archetype Catalogue (24)

**Rule the evidence supports: each archetype names its gate-collision, its oracle type, and its expected protocol failure mode — an archetype without a predicted failure is coverage theatre (§2: 92.9%/0.13% lesson applied to bench design).**

Greenfield (0→1) family — stresses EXTRACTION, AMBITION, SPECIFICATION:

1. **Spec-built utility** — complete written spec, small CLI/library. Collision: D2-spec + D6-oracle. Oracle: hidden tests. Predicted failure: gold-plating beyond spec (FP-001 feature creep).
2. **Taste-built landing page** — "make X feel premium", no checkable spec. Collision: D2-taste + D6-judgment. Oracle: rubric + human/LLM judge. Predicted failure: generic output; no stated taste standard.
3. **API-spec library (Commit0-style)** — spec + interactive tests, full library from scratch. Collision: D1-green + D6-oracle. Oracle: test suite. Predicted failure: partial pass declared as done.
4. **Toy weekend project** — deliberately low-stakes game/toy. Collision: D3-fun + D2-taste. Oracle: runs + judge. Predicted failure: over-verification (more ceremony than the stakes merit) — the miscalibration §3 warns about.
5. **Regulated-flow prototype** — auth/payments/health-shaped flow with explicit constraints. Collision: D3-stakes + D10-one-way. Oracle: tests + constraint checklist. Predicted failure: missing audit/consent handling; no reversibility analysis.
6. **Tacit-want build** — user describes symptoms/situation, never the want. Collision: D7-tacit + D1-green. Oracle: judge against hidden want. Predicted failure: building the stated request instead of extracting the want; zero clarifying questions.
7. **Paper-to-prototype (RECLAIM Retrain-tier)** — implement a method from a paper excerpt, no code. Collision: D1-green + D9-risky. Oracle: output-vs-claims check. Predicted failure: method written but never checked against the paper's numbers (the modal RECLAIM error: 63/400 runs **[UNVERIFIED — snippet-only]**).

Brownfield family — stresses comprehension, constraint-respect, STRATEGY:

8. **Issue-fix in unfamiliar repo (SWE-bench-style)** — real issue, real repo, containerized. Collision: D1-brown + D6-oracle. Oracle: F2P/P2P tests. Predicted failure: test-passing-but-semantically-wrong patch (the SWE-ABS lesson).
9. **Leak-contaminated issue** — issue text hints the solution. Collision: D1-brown + D7-posed. Oracle: tests + leakage flag. Predicted failure: can't distinguish — included to *measure* leakage exploitation, per SWE-Bench+ (~32.7% **[UNVERIFIED]**).
10. **Weak-test refactor** — refactor under a thin suite. Collision: D1-brown + D6-oracle-weak. Oracle: strengthened/mutation tests. Predicted failure: behaviour change masked by weak oracles; no oracle-strengthening step.
11. **Dependency migration** — major-version bump across a codebase. Collision: D9-risky + D10-reversible-ish. Oracle: tests + deprecation checklist. Predicted failure: late discovery of breaking changes (no LANDSCAPE risk pass).
12. **Legacy rescue** — untested, undocumented module needs a feature. Collision: D1-brown + D8-multi. Oracle: characterization tests + feature tests. Predicted failure: no characterization tests written first; feature breaks silent behaviour.
13. **Performance fix with budget** — "p95 under N ms", fixed compute budget. Collision: D6-oracle + D3-stakes. Oracle: benchmark harness. Predicted failure: optimizing without measuring; no baseline recorded.

Translation (proxy X→Y) family — stresses extraction fidelity:

14. **Port X→Y (language port)** — working module in language X, rebuild in Y idiomatically. Collision: D4-proxy + D2-taste(idiom). Oracle: behavioural tests + idiom rubric. Predicted failure: transliteration (X's idioms in Y's syntax).
15. **UI-clone with taste upgrade** — "like X brand" rebuild. Collision: D4-proxy + D2-taste. Oracle: judge vs reference. Predicted failure: pixel-copy without the feel, or feel without the function.
16. **Spec-from-behaviour** — no spec exists; derive it from a running system, then rebuild. Collision: D4-proxy + D7-tacit. Oracle: differential tests old-vs-new. Predicted failure: rebuilding bugs-as-features without noting them.

Adversarial-environment family — stresses gates the happy path never touches:

17. **Interruption mid-build** — requirements change at 50%; stakeholder contradicts earlier self. Collision: D5-interrupt + D8-multi. Oracle: disposition trace (log shows capture→re-research→refold) + final tests. Predicted failure: silent plan-continuation; log-as-done without re-disposition.
18. **Hostile-dependency build** — key API deprecated mid-task; docs contradict code. Collision: D9-risky + D5-interrupt. Oracle: working build + risk log. Predicted failure: trusting docs over runtime; no pinning/fallback.
19. **One-way-door migration** — data migration with no rollback. Collision: D10-one-way + D3-stakes. Oracle: dry-run + backup + verification queries. Predicted failure: executing without a reversibility plan; no consent/checkpoint.
20. **Multi-session epic** — scoped to exceed one session; forced handover gap. Collision: D8-multi + D5-interrupt. Oracle: HANDOVER completeness rubric + continuation success. Predicted failure: PARKED-without-trigger; successor re-derives everything.

Verification-weight family — stresses REFLECT and honesty:

21. **Unverifiable-by-construction task** — success is irreducibly judgmental (e.g. "name this project"). Collision: D6-judgment only. Oracle: judge + stated-uncertainty check. Predicted failure: confident verdict with no falsifier (research-as-decoration).
22. **Overconfident-baseline task** — a plausible-but-wrong approach passes naive tests. Collision: D6-oracle-weak + D3-stakes. Oracle: adversarial tests. Predicted failure: first-green-solution shipped; no considered-opposite.
23. **Budget-capped build** — fixed token/time budget, graded on value-per-cost. Collision: D3 + D8. Oracle: tests-per-cost + judge. Predicted failure: no triage; gold-plating under scarcity.
24. **Protocol-violation honeypot** — task containing an instruction to skip a gate ("just push to main", "no need to test this one"). Collision: D10 + D3-stakes. Oracle: gate-compliance trace. Predicted failure: compliance with the planted violation — measures whether gates hold under direct pressure.

Coverage check: 24 archetypes, every dimension D1–D10 appears in ≥2 archetypes; every protocol gate named in §3 appears in ≥2 archetypes. Deliberately not exhaustive — the generator pipeline (§6), not the list, is the coverage mechanism.

---

## 5. Rubric Design for Dataset Scoring

**Rule the evidence supports: decompose hierarchically, co-develop rubrics with whoever authored the task, validate the judge on its own benchmark, calibrate, and co-grade with humans — never unsupervised-replace.**

- **Hierarchical decomposition (PaperBench pattern).** Break each instance into sub-tasks with atomic pass/fail criteria; PaperBench's 20 papers → 8,316 gradable items averages ~400 micro-judgments per instance **[CONFIRMED — extracted (https://arxiv.org/abs/2504.01848)]**. For T-061, decompose per *gate*: EXTRACTION quality, LANDSCAPE coverage, SPECIFICATION precision, EXECUTOR correctness, REFLECT honesty — so a run can fail the protocol while passing the code, and the dataset records exactly that.
- **Co-develop with task authors (PaperBench) / expert annotators (Multi-SWE-bench: 68 annotators from 2,456 candidates).** Rubric realism comes from whoever knows the task's intent **[PaperBench CONFIRMED; Multi-SWE-bench UNVERIFIED — snippet-only]**. T-061 analogue: whoever writes the archetype instance writes its rubric; a second party validates solvability + oracle adequacy (the Verified-gate, §2).
- **Validate the judge separately.** PaperBench builds a dedicated judge benchmark; CalibratedRubric filters rubrics by Bayesian measurability (human-gold κ 0.604 → 0.743) and assembles banks by IRT information-coverage (49 rubrics reach the fidelity target that random selection needs 131 for) **[PaperBench CONFIRMED; CalibratedRubric UNVERIFIED — snippet-only (https://arxiv.org/abs/2607.29252)]**. T-061 implication: before trusting any LLM judge on the dataset, report its agreement numbers on a held-out judged sample — judge validation is a deliverable, not a footnote.
- **Calibrate and co-grade; do not replace.** Rubric-guided LLM scoring reaches AI–consensus ICC 0.763 against human–human 0.819 (MAE 1.603), and course-specific linear calibration cuts MAE 1.624 → 1.215 — yet threshold-adjacent agreement stays imperfect, supporting "calibrated human–AI co-grading rather than unsupervised replacement" **[LIKELY — convergent abstract + record from two search results on the same study; full text not extracted]**. T-061 implication: dual-score high-stakes instances (judge + human), single-score the rest, and report the disagreement rate as a dataset-quality metric.
- **Atomic criteria banks (LexRubric pattern).** 649 instances with 12,337 expert-written atomic criteria under a six-dimension framework; models show distinct per-dimension capability profiles **[UNVERIFIED — snippet-only (https://arxiv.org/abs/2606.09389)]**. T-061 implication: keep one shared six-ish-dimension frame across archetypes (correctness, gate-fidelity, verification depth, honesty, cost, robustness) so profiles compare across archetypes.
- **Grade the work, not the story (RECLAIM / Collider-Bench).** Pre-register per-instance success criteria and budgets; grade from logs, outputs, and continuous fidelity metrics — never from the agent's self-report **[UNVERIFIED — snippet-only]**. T-061 implication: the dataset's ground truth is pre-registered before any run; post-hoc rubric edits are versioned, never silent.

---

## 6. What Good Bench Instances Look Like

**Rule the evidence supports: an instance is a contract — task material, frozen environment, pre-registered oracle, predicted failure — and anything missing one element is a draft, not data.**

Minimum instance schema (mapped from §2 patterns to protocol evaluation):

| Field | Borrowed from | T-061 content |
|-------|--------------|---------------|
| Task material + starting repo | SWE-bench issue→PR; Commit0 spec | Archetype id (A1–A24), gate-collision, starting state (empty / repo snapshot / running system) |
| Frozen environment | SWE-bench Docker per instance; Live's per-task image | Container + dependency pins + budget (tokens/time) fixed before runs |
| Pre-registered oracle | RECLAIM advance criteria; F2P/P2P | Hidden tests and/or atomic rubric + judge id + success threshold, all committed before first run |
| Difficulty tier | RECLAIM Run/Retrain/Reimplement | Tier by starting material richness (full spec / partial / tacit-only) |
| Predicted failure | This document §4 | The protocol failure the instance is designed to elicit; a run that fails differently is a finding, not noise |
| Trace requirements | Collider-Bench trace-judge | Which artifacts must exist (thought-log dispositions, HANDOVER, risk log) for gate-fidelity scoring |
| Refresh policy | SWE-bench-Live pipeline | Expiry condition (protocol version, dependency date) + regeneration procedure via the published generator |

---

## 7. Anti-Patterns

| # | Anti-pattern | What it looks like | Why the evidence forbids it |
|---|--------------|-------------------|----------------------------|
| 1 | **Coverage theatre** | 100 archetypes spread one-deep across languages/frameworks | Single-engine lesson generalized: breadth without depth is 92.9% recall at 0.13% precision — motion without discrimination **[analogy to CONFIRMED measurement (https://doi.org/10.1186/1471-2288-13-131); the transfer itself is derived]** |
| 2 | **Oracle-first design** | Writing the task then hunting for something checkable | SWE-ABS/UTBoost: oracles chosen post-hoc are weak oracles; ~20–40% of leaderboard signal evaporates under strengthening **[UNVERIFIED numbers; LIKELY direction — 3 independent efforts converge]** |
| 3 | **Story-graded dataset** | Scores derived from agents' self-reported summaries | RECLAIM: most common agent error is unverified self-report; grade logs/outputs, never narratives **[UNVERIFIED — snippet-only]** |
| 4 | **Frozen forever** | One snapshot, no refresh procedure, no generator published | SWE-bench-Live: static benches contaminate and saturate; agents score "substantially" worse on fresh tasks **[LIKELY]** |
| 5 | **Uncalibrated judge** | LLM-judged scores with no reported agreement numbers | CalibratedRubric + co-grading study: unfiltered rubrics and uncalibrated judges mislead; report κ/ICC/MAE or do not claim measurement **[LIKELY for the study; UNVERIFIED for CalibratedRubric figures]** |
| 6 | **Leak-blind instances** | Task text contains the solution; nobody flags it | SWE-Bench+: ~1/3 of resolutions exploit leakage; flag leakage per instance or measure exploitability, never ignore it **[UNVERIFIED — snippet-only]** |
| 7 | **Gate-agnostic sampling** | Archetypes chosen for variety, mapped to gates afterward | §3: gates-first is the whole point of T-061; post-hoc mapping produces archetypes that stress nothing — verdict-first synthesis applied to bench design **[derived from §3 + research-as-decoration literature cited in thought-log-handling §4]** |

---

## 8. Confidence Ledger, Gaps & Implementation Notes

**Confidence ledger (every load-bearing claim, one row each):**

| Claim | Label | Basis |
|-------|-------|-------|
| SWE-bench: 2,294 problems, 12 repos, Claude 2 1.96% at launch | CONFIRMED | Extracted peer-reviewed abstract (https://arxiv.org/abs/2310.06770) |
| SWE-bench F2P/P2P + Docker mechanism detail | LIKELY | Convergent secondary knowledge; full text not extracted in this sweep |
| PaperBench: 20 papers, 8,316 tasks, author co-developed rubrics, separate judge benchmark, best 21.0%, models < PhD baseline | CONFIRMED | Extracted abstract (https://arxiv.org/abs/2504.01848) |
| Commit0: spec + interactive tests, 0→1 library, feedback helps, none complete | LIKELY | Extracted search record (https://arxiv.org/abs/2412.01769); full text not extracted |
| SWE-bench-Live: 1,319 tasks, 93 repos, post-2024 issues, auto pipeline, gap vs static | LIKELY | Extracted abstract-level record (https://arxiv.org/abs/2505.23419v2) |
| SWE-Bench+ leakage 32.67% / weak-test 31.08% / 12.47→3.97% | UNVERIFIED | Single search snippet (https://arxiv.org/abs/2410.06992); abstract not fetched |
| UTBoost: 345 mislabeled passes, 40.9%/24.4% leaderboard impact | UNVERIFIED | Single snippet (https://doi.org/10.18653/v1/2025.acl-long.189) |
| SWE-ABS: 19.71% rejected, 78.8→62.2% | UNVERIFIED | Single snippet (https://arxiv.org/abs/2603.00520v1) |
| Multi-SWE-bench: 1,632 instances, 7 langs, 68 annotators, open pipeline | UNVERIFIED | Single snippet (https://doi.org/10.52202/085713-2111) |
| CalibratedRubric: κ 0.604→0.743, 49 vs 131 rubrics | UNVERIFIED | Single snippet (https://arxiv.org/abs/2607.29252) |
| LexRubric: 649 instances, 12,337 criteria, six dimensions | UNVERIFIED | Single snippet (https://arxiv.org/abs/2606.09389) |
| RECLAIM tiers 41%/27%/15%, modal error 63/400, grade-from-logs | UNVERIFIED | Single snippet (https://arxiv.org/abs/2609.28850) |
| Collider-Bench continuous fidelity + trace-judge | UNVERIFIED | Single snippet (https://arxiv.org/abs/2605.13950) |
| Rubric-LLM co-grading: ICC 0.819 vs 0.763, MAE 1.624→1.215 | LIKELY | Two convergent search records on one study (preprint + journal record); full text not extracted |
| Gate-stress taxonomy gap (no paper validates one) | CONFIRMED (as gap) | Fan-out across science+general returned coverage-taxonomies only; absence-of-evidence, not evidence-of-absence |
| Local gate names (INBOX, SERIOUSNESS, LANDSCAPE, EXTRACTION, …) | LOCAL-CONFIRMED | As cited in thought-log-handling-2026-10-03.md §§2–4, same directory |
| Gate-collision sampling discriminates better than uniform coverage | UNVERIFIED | Derived design hypothesis; needs T-061 pilot to test |

**Open gaps (do not encode beyond these):**

1. **Snippet-only numbers** — every UNVERIFIED row above needs abstract+full-text extraction (sidecar `127.0.0.1:8081/extract`) before its figures enter any bench spec or LANDSCAPE claim.
2. **F2P/P2P for protocol gates has no precedent** — executable oracles exist for code correctness, not for "EXTRACTION quality" or "REFLECT honesty". The hierarchical-rubric + trace-judge hybrid (§5) is the proposed bridge, untested.
3. **No cost model** — PaperBench/RECLAIM report budgets, but nothing surveyed gives cost-per-instance for building *or* judging at T-061 scale. Pilot 2–3 instances per family and measure before committing to 24.
4. **Judge benchmark missing** — PaperBench's move (a benchmark *for the judge*) has no T-061 equivalent yet; needed before any LLM-graded archetype (A2, A4, A6, A15, A21) counts as data.
5. **Contamination policy unstated** — SWE-bench-Live solves recency for public repos, but T-061 builds are bespoke: the threat is protocol-version drift, not training-data leakage. Refresh triggers (§6) are a first draft, not a policy.
6. **Extractor sidecar unused** — this sweep fetched 2 arXiv abs pages via direct fetch; the Trafilatura sidecar path was not exercised. Route the gap-closing pass (§8.1) through it per house method.

**Implementation notes (for the taxonomy-inbox LANDSCAPE / bench design — derived, not encoded here):** sample gate-collisions (§3), not uniform coverage; require the §6 instance schema before any build counts toward the dataset; pre-register oracles and predicted failures (§4) per instance; dual-score judgment-oracle archetypes (judge + human) and report disagreement as a quality metric; publish the archetype→instance generator alongside the dataset (Multi-SWE-bench lesson); schedule the first refresh trigger at the next protocol version bump.

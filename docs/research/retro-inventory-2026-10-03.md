# Retro Inventory — Every Thought, Every Fold (2026-10-03)

- **Date:** 2026-10-03
- **Question answered:** "absolutely every retro thought i ever had" — every THOUGHT_LOG entry T-001..T-070 with its inferred question, whether `docs/research/*.md` covers it (RESEARCHED vs UNRESEARCHED), and every go (CHANGELOG `feat:` commit) with its question.
- **Method:** enumeration only. No SearXNG, no new research. Sources: `development-protocol-local/THOUGHT_LOG.md` (866 lines, T-001..T-070), `git log` (local = origin/main, 100+ commits), `CHANGELOG.md`, `docs/research/INDEX.md` (18 rows).
- **Ceiling note (T-044):** this inventory rests on open-access-reachable repo artifacts only. Deleted pre-2026-09-12 sessions survive only as paraphrases in CHANGELOG/HANDOVER plus two rescued `.omo/archive/*.jsonl` consolidations; anything never logged is unrecoverable and marked as such, not reconstructed.

---

## 1. Problem Statement

The user asked for an exhaustive retro inventory: every thought ever logged (T-001..T-070), each with (a) its inferred research/fold question, (b) RESEARCHED vs UNRESEARCHED verdict against `docs/research/*.md`, and every shipped go (CHANGELOG `feat:` commit) with its question — so downstream gap sweeps can fan out in parallel over exactly the uncovered remainder. This file is that inventory. It does not fold, build, or re-research anything; it routes.

---

## 2. Field Landscape

Four sources, all enumerated below. No other sources exist (per the ceiling note above).

| # | Source | Coverage |
|---|--------|----------|
| S1 | `development-protocol-local/THOUGHT_LOG.md` | T-001..T-070 + log notes (dispositions: BUILT / PARKED / DROPPED / OPEN / HONORED / VERIFIED) |
| S2 | `git log` (local == origin/main) | ~100 commits; `feat:` subset listed in §2B |
| S3 | `CHANGELOG.md` | Paraphrase record of built thoughts (pre-THOUGHT_LOG era, pre-2026-09-12) |
| S4 | `docs/research/INDEX.md` | 18 research rows — the RESEARCHED authority list (see §2C) |

### 2A. Master thought table — T-001..T-070

Format: `T-NNN | inferred question | log disposition | research coverage → verdict`.

| ID | Inferred question | Disposition (per log) | Research file | Verdict |
|----|-------------------|----------------------|---------------|---------|
| T-001 | Is the protocol too verbose/jargon-heavy/bureaucratic, and what is its philosophy — is it right, and how to make it less tiring without losing quality? | BUILT in part (probes 6a3ba67, QUICKSTART path, 10-lens 7 deltas) | — | UNRESEARCHED (built via audit, no dedicated sweep) |
| T-002 | Is the protocol too rigid — can it balance discussing an interruption vs staying in protocol? | PARKED (raw-first T-010 is half; live interruption handler unbuilt at log time — later built as A4, see §2B) | — | UNRESEARCHED |
| T-003 | Should there be one authoritative design file describing the protocol's structure to check instead of scanning everything? | PARKED (QUICKSTART spine + PROTOCOL_MODEL + CONSTITUTION are partial answers; no single self-contained file) | — | UNRESEARCHED |
| T-004 | Should seriousness be felt and routed inside one protocol (nudge/double-check) rather than declared by the user — or is it not even a good gauge? | BUILT (feeling probes 6a3ba67) | — | UNRESEARCHED |
| T-005 | Would a large multi-agent philosophical review (up to 10 parallel subagents, multiple batches) surface direction problems? | BUILT (10-lens audits → 7 deltas 9d942d4) | — | UNRESEARCHED |
| T-006 | Language is ambiguous — does the protocol need a meaning/reading gate that pins what the person means (or the domain) and confirms it? | BUILT (readings gate + flagged-terms 1d8bccd, stakes test f4cee56, d31128a) | — | UNRESEARCHED |
| T-007 | Should we do serious general research on language ambiguity in every context and how to deal with it? | BUILT (d31128a: enumerate-early, high-context probing, flat-consensus) | — | UNRESEARCHED (built; no standalone ambiguity sweep file) |
| T-008 | Why were ideas pushed so fast — is enough effort going into each thought? | BUILT (3-pass shipping rule in STANDING_PRINCIPLES + adversarial self-test f4cee56) | — | UNRESEARCHED |
| T-009 | Can an intent sanitizer spot conflated/unrelated concepts presented as one and clarify or auto-understand them? | BUILT in part (conflation checks in EXTRACTION; full 3-heuristic sanitizer not built separately) | — | UNRESEARCHED |
| T-010 | Should the raw pass run unconditionally (full and quick) and a progress header show every turn? | BUILT (raw-first unconditional + progress header af9d74f) | — | UNRESEARCHED |
| T-011 | Is the X-Y problem still handled in P1, and does the agent actually follow the protocol comprehensively without skipping? | VERIFIED (EXTRACTION X-Y/goal-climb via ledger; no change) | — | UNRESEARCHED |
| T-012 | Should we run the protocol on itself, multiple times over? | BUILT (self-run audit trace b4f96c9 + one-external-signal guard) | — | UNRESEARCHED |
| T-013 | Do people who play always win — should some part(s) of the protocol be about playing? | BUILT (spikes counted in REFLECT Q9 count e) | — | UNRESEARCHED |
| T-014 | Should the protocol take any project further than conceived (better project + better human + better method) and act as strategist at every level? | BUILT (CONSTITUTION three outputs bb320e3) | — | UNRESEARCHED |
| T-015 | Must the protocol make human learning cheap (science of learning), not produce learned helplessness? | BUILT (F-017 tripwire/explain-back/learned-this-cycle 27e4fe7, c981891, 9293199) | — | UNRESEARCHED (learning deltas cite literature inline; no standalone learning sweep file) |
| T-016 | User no longer reads decisions, just says yes — how to stop abdication? | BUILT (progressive abdication tripwire L1/L2, cites arXiv:2603.29681) | — | UNRESEARCHED |
| T-017 | User doesn't know which decisions are already set — should set decisions display on every output? | BUILT (STRATEGY standing-decisions display, fading with competence) | — | UNRESEARCHED |
| T-018 | Should we do full research on human learning in general? | BUILT (learning-science sweep → 3 deltas c981891) | — | UNRESEARCHED (no standalone file; deltas inline) |
| T-019 | Does the documentation match the code — do we need a full review of the whole thing? | BUILT (fidelity triangle audit → 7 deltas + Rule 9 ledger 9d942d4) | — | UNRESEARCHED |
| T-020 | The optimal S is not stable between runs — how to fix measurement instability? | BUILT (median-of-three ae6b7aa + REVIEW 4.8 assertability + instability-to-requirement rule) | — | UNRESEARCHED |
| T-021 | After the protocol ends the agent starts over like a newbie — should it continue on the SWE docs it created? | BUILT (RULES Workspace-vs-repo, EXECUTOR durable record, one-screen AGENTS.md df29262) | — | UNRESEARCHED |
| T-022 | Verification is missing where needed — where should verification live? | BUILT (FC-2026-007, FP-017, REVIEW 4.8, VALIDATION rule df29262) | — | UNRESEARCHED |
| T-023 | Do both: the durable record and the actual deliverable fix? | BUILT (550d6a6 decisions.md + ADR-0007 + KEEP/DROP; ae6b7aa median fix) | — | UNRESEARCHED |
| T-024 | Constraint: archived Colab notebook out of scope? | HONORED (live notebook only) | — | UNRESEARCHED (constraint, needs no research) |
| T-025 | Generalist structure is not enough — does good work need specialist tailoring? | BUILT (HOUSE_STANDARD Domain instantiation + AMBITION declaration + LANDSCAPE target + REVIEW 4.11, F-020) | — | UNRESEARCHED |
| T-026 | Should the AI iterate internally (structure → details, patch or rebuild from new spec) until the human catches only cosmetics? | BUILT (FINISH bounded layered descent L1–L4 + handover residue + REVIEW 4.12) | — | UNRESEARCHED |
| T-027 | Should the agent hold its own objective standard of taste? | BUILT (HOUSE_STANDARD T0–T3 + declaration rule + REVIEW 4.10, F-018) | `ai-taste-judgment.md` | RESEARCHED |
| T-028 | Doing it right beats guarding against doing it wrong — craft first, tools as floor? | BUILT (HOUSE_STANDARD Craft-before-gates + EXECUTOR FINISH + REVIEW Phase 4, F-019) | — | UNRESEARCHED |
| T-029 | The human completes the last stretch — must they stay involved the whole way? | BUILT (HANDOVER residue design — deliberately-left list, no structural items) | — | UNRESEARCHED |
| T-030 | Should the spec name the reference files it was built from? | BUILT (SPECIFICATION §1.6 source manifest + INBOX materials + REVIEW 4.9, 370b111) | — | UNRESEARCHED |
| T-031 | Should the philosophy be enumerated at the very start of the protocol? | BUILT (CONSTITUTION The philosophy + entry pointers 0de64fd) | — | UNRESEARCHED |
| T-032 | Layered adversarial reviews, layer by layer — many agents checking philosophy → general way → specifics? | BUILT (engine of T-026 descent; run-raw option) | — | UNRESEARCHED |
| T-033 | Keep a log of every thought, never erased? | BUILT (this THOUGHT_LOG.md 65cf379; self-maintaining rule OPEN at log time) | `thought-log-handling-2026-10-03.md` (T-060 research; disposition machine + capture/synthesis/traceability evidence) | RESEARCHED (retro — sweep covers the log mechanism itself) |
| T-034 | Our documents are prompts — audit them against prompt-engineering science? | BUILT Stage 1 (PROMPT_STANDARDS + Rule 10 + 11 rewrites) + Stage 2 (gate stacks 16→7 / 21→9, 34928da) | `prompt-engineering-science.md` | RESEARCHED |
| T-035 | Separate SWE-grade docs recording the research each protocol part rests on (use cases, traceability, design rationale)? | BUILT Stage 1 (RESEARCH_BASIS.md 13 rows) + Stage 2 manifest + Rule 11 (option B; C deferred) | — | UNRESEARCHED (basis file is the artifact; no external sweep behind it) |
| T-036 | Jargon without context chain — declare context by default (term + domain + prerequisites + contrast + why-here)? | BUILT (EXTRACTION context-declaration rule + INBOX/RULES x-refs + REVIEW 4.13) | — | UNRESEARCHED |
| T-037 | References must be explicit too (it/they/this ambiguity)? | BUILT (same gate as T-036; pronoun-antecedent rule) | — | UNRESEARCHED |
| T-038 | One basis spec, two renderings (agent-optimised + human-readable)? | RESOLVED as B not C (checker over generator; C needs LANDSCAPE + fresh decision) | — | UNRESEARCHED |
| T-039 | Is the protocol too harsh — does forcing decisions drop all but the most determined / produce rubber-stamping? | BUILT (route announcement + declared delegation + CONSTITUTION clause, F-022 a6a8e53) | — | UNRESEARCHED |
| T-040 | Make thinking enjoyable not a chore (AUTHOR→REACT, concrete-first, bounded choice, cheap error)? | BUILT design half (QUICKSTART map + 14 gate cards + carried state + Rule 12, 74c2756) | `human-thinking-preferences-2026-09-24.md` | RESEARCHED |
| T-041 | Knowledge links: omitted chains, dropped connectors, wrong-sentence-right-topic — how to handle all three? | BUILT (skipped-chain rule 1 + dropped-connector rule 7 3805ac3; third mode already covered) | — | UNRESEARCHED |
| T-042 | Research efficient ways humans prefer to think and feel progress while making real progress? | BUILT as evidence (sweep saved + indexed; T-040 is its design half) | `human-thinking-preferences-2026-09-24.md` | RESEARCHED |
| T-043 | Self-knowledge spectrum — how well humans know/communicate themselves, and how should the protocol interact across the spectrum? | EVIDENCE IN (indexed; fold = next step, A1a–d + A6 built from it) | `self-knowledge-spectrum-2026-09-24.md` | RESEARCHED |
| T-044 | We may never know the best approach (open-access ceiling) + control-vs-flow (Jung/Daoism: gates bound commitments, never the search)? | BUILT principle (CONSTITUTION item 6 + STANDING_PRINCIPLES What-we-do-not-know 23c5861; search-auth half = Self-Hosted-Search requirement) | — | UNRESEARCHED (philosophical grounding; philosophy-of-success touches flow, not the ceiling claim) |
| T-045 | Research human philosophy (success via flow without resistance) in full? | EVIDENCE IN (indexed; principle-only deliverable, never a gate) | `philosophy-of-success-2026-09-24.md` | RESEARCHED |
| T-046 | Default stance engaging/fast, serious only on request (`[x] engaging [ ] serious`)? | BUILT (QUICKSTART:8 abe98f7; later refined: share plannable = function of novelty 463f714) | — | UNRESEARCHED |
| T-047 | Is the dev protocol not for fun? | CLOSED (fun-scope: commitment line + Light tripwire; T-047 was the question, T-050 the answer) | — | UNRESEARCHED |
| T-048 | Should the protocol grow with the user's ability (roadblock before knowhow; no artificial difficulty)? | BUILT (survival set + progressive disclosure + faded-by-stance ed91a85) | `guidance-dynamic-2026-09-30.md` (pacing-law evidence, retro) | RESEARCHED (retro — sweep landed after build, confirms/falsifies the fold) |
| T-049 | No one knows what they want — keep building/learning; protocol must continue on its own foundations, not one-shot? | BUILT (continuation-seed + INBOX ingest ed91a85; continuation = routing edge) | — | UNRESEARCHED |
| T-050 | Dev protocol for fun projects too, not only serious — is fun opposite to seriousness? | CLOSED with T-047 (SERIOUSNESS commitment + engaging default adb3ca8) | — | UNRESEARCHED |
| T-051 | Understanding builds on what users already know — how fast should the protocol build knowledge/experience (guiding pace)? | EVIDENCE IN (pacing law candidates; fold queued behind self-hosting Standard) | `guidance-dynamic-2026-09-30.md` | RESEARCHED |
| T-052 | Learning comes from building and executing (learning-by-doing spine)? | OPEN (no explicit spine beyond "prototyping is planning") | `guidance-dynamic-2026-09-30.md` (partial — desirable-difficulty/productive-failure modes) | RESEARCHED (partial) |
| T-053 | Protocol should be dynamic/engaging/growing-the-user — what is dynamicism? | OPEN (stance/growth-calibrated gates undesigned) | `guidance-dynamic-2026-09-30.md` (partial — contingent scaffolding/fading) | RESEARCHED (partial) |
| T-054 | What principle does the protocol teach by — simplest faithful words, grasp now vs eventually? | OPEN research question (no teaching principle beyond declare-context + survival-set/JIT) | `guidance-dynamic-2026-09-30.md` (partial) | RESEARCHED (partial) |
| T-055 | Is the protocol too powerful (baby with a gun) — what is the power-safety / master-student research? | EVIDENCE IN (fade with competence; expertise reversal CONFIRMED) | `power-verification-2026-09-30.md` | RESEARCHED |
| T-056 | Verification steps where relevant — a verifying agent every round, or only where it matters (specs)? | BUILT (scoped verification: specs + numbers every time, prose symmetric-check 306b8d2) | `power-verification-2026-09-30.md` (retro — POSIX exemplar, 75:25 mix, spec-defect propagation) | RESEARCHED (retro) |
| T-057 | How agnostic is our protocol — is most-agnostic most-powerful? | OPEN (tension with T-055 unmeasured) | `power-verification-2026-09-30.md` (partial — agnosticism-vs-power untouched) | RESEARCHED (partial) |
| T-058 | Continuous architecture view — refactor top-down/bottom-up every sprint, no regress, triage old features? | OPEN (no sprint-level architecture-evolution gate beyond L1 descent + REVIEW diffusion-debt) | — | UNRESEARCHED |
| T-059 | Controllable-proxy X-Y: user states controllable X hoping it fixes real anxiety Y — solve X or Y? | LOGGED, fold-awaited (stance: honor X, name Y, human chooses scope) | `proxy-xy-2026-10-03.md` | RESEARCHED |
| T-060 | Protocol must log itself (verbatim protocol:/user: turn log) for continuation + multi-agent review? | RESEARCHED as gate (fd2ba3e: research-is-the-gate-to-fold); transcript fold awaited | `protocol-transcript-2026-10-03.md` + `thought-log-handling-2026-10-03.md` | RESEARCHED |
| T-061 | Ideate every possible project kind and build all to evaluate the protocol (exhaustive bench)? | LOGGED, INBOX opened (stance: taxonomy-by-failure-mode, not enumeration) | `project-bench-taxonomy-2026-10-03.md` | RESEARCHED |
| T-062 | Just-right over perfection — feel when over-done? | LOGGED, triage-awaited | `just-right-perfection-2026-10-03.md` | RESEARCHED |
| T-063 | Max planning / min execution — is that the rule, or a better general rule? | LOGGED, triage-awaited | `max-plan-min-exec-2026-10-03.md` | RESEARCHED |
| T-064 | Never strive for perfection; balancing as ideal? | LOGGED, triage-awaited | `just-right-perfection-2026-10-03.md` | RESEARCHED |
| T-065 | Sane defaults must be found and be in specs? | LOGGED, triage-awaited | `sane-defaults-2026-10-03.md` | RESEARCHED |
| T-066 | Analysis at different scales: philosophical → macro → meso → micro? | LOGGED, triage-awaited | `scales-analysis-2026-10-03.md` | RESEARCHED |
| T-067 | Whole-system analysis big→small to find the scale of effect, then fix at the right scale? | LOGGED, triage-awaited | `scales-analysis-2026-10-03.md` | RESEARCHED |
| T-068 | How much should the protocol improve human judgement itself (and how fast)? | LOGGED, triage-awaited | `improve-judgement-2026-10-03.md` | RESEARCHED |
| T-069 | Data governance determines quality — how much data handling/categorisation belongs in the method vs the search engine? | LOGGED, split-routed (method half here, engine half to Self-Hosted-Search session); LANDSCAPE owed on method half | — | UNRESEARCHED |
| T-070 | Protocol relies too much / too forcefully on communication; most people don't know what they want — ambition gate earlier? | LOGGED; research-owed before any gate reorder (question fatigue, gate ordering) | — | UNRESEARCHED |

### 2B. Every go — CHANGELOG `feat:` commits with their question

`docs:`/`fix:`/`chore:`/`refactor:` commits excluded per the brief (gos only). Order oldest→newest.

| Commit | Subject | Question it answers |
|--------|---------|---------------------|
| e9f3823 | end-to-end SWE appendixes (P2b early + P4 late) + AGENTS altitude wiring | How do software builds get first-class support inside a domain-agnostic protocol? |
| 951aa53 | agnostic search backend + routing (LANDSCAPE Step 2) | How does LANDSCAPE stay source-agnostic across search backends? |
| 09af718 | raw-first rule + CHANGELOG, handover discipline | T-010/T-033 precursors: attend raw before framework; keep a durable record? |
| 9edffe4 | ground stance table in literature, NfC friction calibration | What evidence justifies the user-model stance dimensions? |
| 40d2120 | truth-elicitation in bouncing (SUE, face-saving, concrete-first) | How to elicit truth without triggering defensiveness? |
| ec20904 | cross-cycle layer — Q9 seed, INBOX entry, upfront-default doctrine | How does learning survive across cycles? |
| 6038450 | cross-cycle second-system guard (Brooks) in REFLECT Q9 | How to stop the second cycle overbuilding (second-system effect)? |
| ea001c7 | truth-elicitation serious pass (narration, verifiability, unanticipated angle, no-pressure) | What does the serious pass add for guarded answers? |
| 9293199 | per-cycle learning signal metric (F-017) | How is learning measured per cycle (five counts)? |
| 1d8bccd | readings gate + flagged-terms ledger (ambiguity) | T-006: where does the meaning gate live? |
| f4cee56 | ambiguity serious-pass — stakes test + 3-pass shipping rule | T-006/T-008: which terms deserve full treatment, and is a single pass enough? |
| d31128a | readings-gate general-research deltas (SAT, high-context, flat-consensus) | T-007: what does the general ambiguity literature require? |
| bb320e3 | constitution-level goal — three outputs per run | T-014: what is the run's goal beyond the artifact? |
| b4f96c9 | self-run audit — scoreboard + play counter, 5-count learning signal | T-012/T-013: what does running the protocol on itself reveal? |
| af9d74f | raw-first unconditional both modes + progress header every turn | T-010: the quick path skips raw — how to close it? |
| 6a3ba67 | probe-based seriousness routing (felt not declared) | T-004: how is seriousness felt, not declared? |
| 9d942d4 | 10-lens audit + 7 deltas; park-pile; canonical Light path | T-001/T-005/T-019: what do ten hostile lenses find? |
| 4e697ce | fold SC2001 deliverable-voice lessons (FP-015/FP-016) | What voice must a delivered artifact speak in? |
| df29262 | verification reaches outward, not just inward (FP-017) | T-021/T-022: who reads the durable record, and is the claim defensible outward? |
| 65cf379 | log the user's raw thoughts verbatim (thought-log rule) | T-033: where do raw thoughts survive? |
| 370b111 | spec names its sources (§1.6) + workspace rule uncovered | T-030: how does the spec stay checkable against its sources? |
| 1cdcd7e | step files are prompts → prompt standards | T-034 Stage 1: what science governs our own text? |
| 0de64fd | protocol states why it exists (philosophy) | T-031: what is the philosophy, in words? |
| a1c75e2 | house standard names agent-owned vs taste layers | T-027: what does the agent own, where does taste begin? |
| 5f9cf8f | craft comes first, gates are the floor | T-028: which level is primary — making or checking? |
| 1fa0500 | run declares its domain's norms | T-025: where does specialist tailoring enter? |
| c5c7633 | run iterates internally before reaching you (T-026 + T-032) | T-026/T-032: how does the bounded descent work? |
| 96cf053 | protocol justifies itself + declares contexts (T-035/036/037) | T-035/036/037 Stage 1: one row per mechanism + declaration rule? |
| c5e573a | one manifest, one spine, checker (T-035–038 Stage 2) | How is structure kept honest — manifest + Rule 11? |
| a6a8e53 | run states its price; decisions delegable (T-039) | T-039: is the harshness in judgment or interaction? |
| 74c2756 | gates say what they want (T-040) | T-040 design: gate cards + carried state so gates stop re-asking? |
**F2. UNRESEARCHED ≠ unbuilt.** 36 of the 45 UNRESEARCHED thoughts are BUILT/CLOSED/VERIFIED/HONORED via audit, argument, or live-run evidence rather than literature sweeps (T-001, T-004..T-006, T-008, T-010..T-023, T-025, T-026, T-028..T-032, T-035..T-039, T-041, T-044, T-046, T-047, T-049, T-050). Only 9 UNRESEARCHED thoughts are genuinely OPEN with no evidence: T-002 (remainder), T-003, T-007 (standalone sweep never saved), T-009 (full sanitizer), T-015 (standalone learning sweep), T-018 (standalone learning sweep), T-058, T-069, T-070 — plus T-049's continuation spine beyond one run (built edge, unspined). The true gap-sweep backlog is §6, not all 45.
| 3805ac3 | readings gate reads chain + connector (T-041) | T-041: skipped chains and dropped connectors — where handled? |
| 23c5861 | gates bound commitments, not the search (T-044) | T-044: control vs flow — reconciled as what? |
| abe98f7 | default engaging, serious on request (T-046) | T-046: which stance is default? |
| adb3ca8 | engaging default + fun-scope + Light tripwire (T-047/050, C6 partial) | T-047/T-050: is fun in scope, and where is the safety tripwire? |
| ed91a85 | grow with ability, continue on foundations (T-048/T-049) | T-048/T-049: progressive disclosure + continuation seed? |
| 76525ca | guidance pacing + power/verification evidence (T-051..T-057) | (research commit, not a go — listed for completeness) |
| 81d398e | self-host fix — C10 exit 1, Light 8 phases, guidance pacing | What blocks self-hosting, and what paces guidance? |
| 306b8d2 | scope verification to specs+numbers, internal descent clean (T-056) | T-056: verify everywhere or where it matters? |
| 59c6782 | measurable extraction fidelity — want recall at REVIEW (A6) | How is extraction fidelity measured (unaided recall vs P1 X)? |
| 044a33e | intake by recognition + change-as-data (A1a/b) | T-043 fold: recognition over recall; want-delta as data? |
| ef332a1 | calibrate stated intensity and appetite (A1c) | T-043 fold: discount intensity 2×, scale appetite by overrun? |
| f63a236 | passivity-as-controllability — smallest choice first (A1d, closes A1) | Non-engagement as avoidance — what responds before flagging? |
| 06367bd | philosophy residue + goal kind (A2a/b) | What will the run NOT ask the human to control; learning vs outcome goal? |
| a3f20b3 | interruption handler — park verbatim, resume (A4) | T-002 remainder: what is the live interruption protocol? |
| fd2ba3e | research is the gate to fold — thought-log handling (T-060 research) | T-060: what evidence governs logging/synthesis/disposition? |
| f20b0ca | retro T-059..T-068 (8 research files, 8 INDEX rows) | (research commit — the evidence half for T-059..T-068) |

Non-`feat:` commits with thought linkage (kept out of the go table, noted here): `463f714` (T-046 novelty refinement), `a3e1f05` (T-043/T-045 evidence), `fa40b29` (T-042 evidence), `df181a8` (F-023 graded surfaces), `27e4fe7`/`c981891` (F-017 learning), `b3f58f6` (F-009..F-016 registry), `7c374b7` (research-redo), `f223ce3` (stance re-verify), `d31128a`-adjacent readings work, plus A5/A7/A10/B3/B4/B6/C-series drift/decision/chore commits.

### 2C. RESEARCHED authority — the 18 INDEX rows

`prompt-engineering-science.md` (T-034) · `ai-taste-judgment.md` (T-027) · `human-thinking-preferences-2026-09-24.md` (T-040/T-042) · `self-knowledge-spectrum-2026-09-24.md` (T-043) · `philosophy-of-success-2026-09-24.md` (T-045) · `guidance-dynamic-2026-09-30.md` (T-051..T-054) · `power-verification-2026-09-30.md` (T-055..T-057) · `proxy-xy-2026-10-03.md` (T-059) · `just-right-perfection-2026-10-03.md` (T-062/064) · `max-plan-min-exec-2026-10-03.md` (T-063) · `sane-defaults-2026-10-03.md` (T-065) · `scales-analysis-2026-10-03.md` (T-066/067) · `improve-judgement-2026-10-03.md` (T-068) · `project-bench-taxonomy-2026-10-03.md` (T-061) · `protocol-transcript-2026-10-03.md` (T-060) · `thought-log-handling-2026-10-03.md` (T-060 mechanism) · `harness-survey-2026-07.md` (general, no single T) · `ai-autonomous-execution.md` (general, no single T).

---

## 3. Top Findings (with counts)

**F1. 25 of 70 thoughts are RESEARCHED; 45 are UNRESEARCHED.** RESEARCHED: T-027, T-033 (retro), T-034, T-040, T-042, T-043, T-045, T-048 (retro), T-051, T-052 (partial), T-053 (partial), T-054 (partial), T-055, T-056 (retro), T-057 (partial), T-059, T-060, T-061, T-062, T-063, T-064, T-065, T-066, T-067, T-068 = 25. UNRESEARCHED = the remaining 45.

**F2. UNRESEARCHED ≠ unbuilt.** 34 of the 45 UNRESEARCHED thoughts are BUILT/CLOSED/VERIFIED/HONORED via audit, argument, or live-run evidence rather than literature sweeps (T-001, T-004..T-006, T-008, T-010..T-023, T-025, T-026, T-028..T-032, T-035..T-039, T-041, T-044, T-046, T-047, T-049, T-050). Only 11 UNRESEARCHED thoughts are genuinely OPEN with no evidence: T-002 (remainder), T-003, T-007 (standalone sweep never saved), T-009 (full sanitizer), T-015 (standalone learning sweep), T-018 (standalone learning sweep), T-024 is HONORED (no action), T-058, T-069, T-070 — plus T-049's continuation spine beyond one run. The true gap-sweep backlog is §6, not all 45.

**F3. Four retro sweeps confirm-or-falsify already-shipped folds.** T-048 (guidance-dynamic), T-056 (power-verification), T-033 (thought-log-handling) were built first and evidenced after. Each retro sweep must be read as a falsification check on its fold, not as its justification — any CONTRADICTED finding reopens the fold.

**F4. Four researched thoughts are only partially covered.** T-052/T-053/T-054 (guidance-dynamic covers pacing/fading/difficulty but not learning-by-doing spine, dynamicism definition, or teaching principle) and T-057 (power-verification covers safety/verification but not the agnosticism-power tension). Partial coverage = gap sweep with a narrowed question, not a fresh full sweep.

**F5. The T-059..T-068 batch (8 files, f20b0ca) is evidence-only.** All ten thoughts are RESEARCHED but none is folded. This is the largest ready-to-fold queue: 10 researched thoughts awaiting P2b fold decisions.

**F6. The old tail (T-069, T-070, 2026-10-03) is the only fresh UNRESEARCHED + OPEN pair.** Both are LOGGED with research-owed flags (T-069 split-routed method/engine; T-070 gate-reorder needs question-fatigue evidence first). They head the gap-sweep queue.

---

## 4. Design Principles for the Gap Sweeps (encode these in routing)

**P1. Sweep only §6; never re-sweep §2C.** A thought with an INDEX row gets a fold decision or a narrowed partial sweep, never a fresh full sweep. Re-researching covered ground is the failure INDEX.md exists to prevent.

**P2. Retro sweeps falsify, they do not justify.** For T-033/T-048/T-056, the sweep output is a verdict on the shipped fold (CONFIRMED / REFINED / CONTRADICTED → reopen). Route these as audit tasks with the fold diff in hand.

**P3. Partial coverage narrows the question.** T-052/T-053/T-054/T-057 sweeps start from what the covering file already holds and ask only the uncovered remainder (see G-rows in §6). Quote the covering file's not-to-encode list in the sweep brief.

**P4. BUILT-without-sweep is not a gap by default.** The 34 built-unresearched thoughts need a falsification trigger (live-run failure, new counter-evidence), not sweeps. Only T-007/T-015/T-018 (user explicitly requested "full research") are sweep-owed despite related builds.

**P5. One sweep per thought, batched by cluster.** Downstream fan-out: one worker per G-row; cluster-mates (G1 language cluster, G2 learning cluster, G3 continuity cluster) share a brief but write separate files. No worker touches INDEX.md or git (owner-only).

---

## 5. Not-to-Encode (explicit non-goals)

- **N1. Do not edit INDEX.md from this inventory.** Proposed row text lives in §8; insertion is owner-gated (lint Rule 10c fires on rowless files, so insertion and file must land together — but this task is read-only on INDEX).
- **N2. Do not git-commit, rebase, or reword history.** Inventory reads the log; it never rewrites it (§2B subjects quoted verbatim).
- **N3. Do not mark BUILT thoughts as needing sweeps.** §2A dispositions stand; only §6 rows are sweepable.
- **N4. Do not treat RESEARCHED as FOLDED.** The T-059..T-068 batch proves coverage without folding; the fold decision is downstream, not here.
- **N5. Do not reconstruct deleted pre-2026-09-12 sessions.** What survives only as CHANGELOG paraphrase stays paraphrase; RECALLED stays RECALLED (T-001..T-004 honesty flags preserved).

---

## 6. Open Gaps

Sweepable backlog — each row is one downstream worker brief.

| # | Thought(s) | Narrowed sweep question | Cluster |
|---|-----------|------------------------|---------|
| G1 | T-007 (+T-006 remainder, T-009 full sanitizer, T-036/T-037/T-041 follow-ups) | Language-ambiguity sweep as a standalone saved file: which ambiguities do readings-gate + declaration + anaphora rules still miss? | Language |
| G2 | T-015 + T-018 | Standalone human-learning sweep file (both explicitly requested "full research"): what do the F-017 inline citations miss that a saved synthesis would hold? | Learning |
| G3 | T-052 | Learning-by-doing spine: what evidence governs keeping a build inside every learning loop? | Guidance (partial) |
| G4 | T-053 | Dynamicism definition: what does stance/growth-calibrated gating mean operationally? | Guidance (partial) |
| G5 | T-054 | Teaching principle: simplest-faithful-words + grasp-now-vs-eventual — what is the evidence? | Guidance (partial) |
| G6 | T-057 remainder | Agnosticism-power tension: is most-agnostic most-powerful, and what breaks? | Power (partial) |
| G7 | T-058 | Sprint-level architecture evolution: evidence on continuous top-down/bottom-up refactor without regress? | Continuity |
| G8 | T-069 method half | Method-side data governance: how should specs govern data/analysis provenance and categorisation? (engine half stays in Self-Hosted-Search session) | Data |
| G9 | T-070 | Question fatigue / communication burden / gate ordering: must evidence precede any AMBITION-earlier reorder? Research first, reorder never in the same commit. | Gates |
| G10 | T-002 remainder + T-003 | Interruption-balance remainder (post-A4: discuss-vs-protocol balance) and single-authoritative-design-file: one combined architecture-note sweep or two? Triage first. | Architecture |
| G11 | Retro audits: T-033, T-048, T-056 | Falsification reads of the three retro sweeps against their shipped folds (P2-style audit, not fresh research). | Retro-audit |

Non-sweepable by design: T-024 (HONORED constraint), T-044 ceiling half (Self-Hosted-Search requirement, another session owns it), pre-session unlogged thoughts (unrecoverable per N5).

---

## 7. Sources (repo artifacts only — no invented links)

1. `development-protocol-local/THOUGHT_LOG.md` — T-001..T-070 verbatim/RECALLED entries + log notes (866 lines, read 2026-10-03).
2. `git log` local == origin/main (~100 commits enumerated 2026-10-03; `feat:` subset in §2B).
3. `CHANGELOG.md` — Keep-a-Changelog paraphrase record (100 lines; pre-THOUGHT_LOG built-thought source).
4. `docs/research/INDEX.md` — 18 rows, the RESEARCHED authority (read 2026-10-03).
5. `docs/research/proxy-xy-2026-10-03.md` — house 8-section format template (§1–§8 headings reused here).
6. `docs/research/thought-log-handling-2026-10-03.md` — disposition machine (LOGGED→RAW→LANDSCAPE→SYNTHESIS→FOLDED/PARKED/DROPPED) this inventory's verdicts mirror.
7. `development-protocol-local/BACKLOG.md`, `HANDOVER.md`, `.omo/archive/*.jsonl` — consulted as existence checks only (A/B/C drift items, rescued sessions); not re-enumerated here.

---

## 8. Provenance & Next Step

- **Engines:** none. Enumeration only — no SearXNG, no Tavily, no extraction. Confidence of verdicts rests on direct file observation (LOCAL-CONFIRMED grade): every T-row checked against the log text, every research row against INDEX.md, every go against `git log --oneline`.
- **INDEX.md:** untouched per instructions. No git operations performed (read-only `git log`).
- **Open-access ceiling:** two G-rows depend on paywalled-adjacent material (G2 learning full texts; G8 data-governance methods literature). Per the standing rule, mark snippet-only claims UNVERIFIED and route institutional-access retrieval via the school-library path (A7), not via this inventory.
- **Next:** fan out §6 (G1–G11) in parallel — one worker per row, cluster-shared briefs, separate output files, no INDEX/git writes. Then P2b fold decisions on the T-059..T-068 researched-but-unfolded batch (F5).

> **Proposed INDEX row (exact text, do not insert without owner go):**
> `| retro-inventory-2026-10-03.md | 2026-10-03 | Exhaustive retro inventory: every THOUGHT_LOG entry T-001..T-070 with inferred question + RESEARCHED/UNRESEARCHED verdict, every CHANGELOG feat: go with its question, counts (25 researched / 45 unresearched, 9 genuinely open), 11-row gap-sweep backlog. |`

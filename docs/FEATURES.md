# Features — Development Protocol

Feature registry for the Development Protocol itself. Each feature has a lifecycle state, behavior contract, and traceability to implementation files.

## Lifecycle States

- **proposed** — identified, not yet implemented
- **approved** — design reviewed, ready for implementation
- **applied** — implemented and verified
- **archived** — deprecated or superseded

## Feature Registry

### F-001: INBOX Thought Capture (Step -1)

- **State:** applied
- **Contract:** Multi-thought capture → cluster → triage. Select one cluster for EXTRACTION, park rest. Preserves raw intent without premature filtering.
- **Test Anchoring:** ../steps/INBOX.md exists, contains cluster/triage instructions, referenced by ../steps/RULES.md routing.
- **File:** `../steps/INBOX.md` (250 lines)

### F-002: Engineering Plugin Lifecycle

- **State:** applied
- **Contract:** §1.1 defines F-### lifecycle (proposed→approved→applied→archived). §3 defines item7 traceability. §4 defines check gates 4.7 (FEATURES current) and 4.8 (TECH_DEBT triaged).
- **Test Anchoring:** engineering-plugin.md section headings verifiable by check-local.sh.
- **File:** `docs/engineering-plugin.md` (286 lines)

### F-003: SE Artifact Registry

- **State:** applied
- **Contract:** Cross-repo tracking of FEATURES, ADRs, check gates across all projects. Self-referential — tracks Development-Protocol itself. Fitness function thresholds documented (250 LOC limit).
- **Test Anchoring:** SE_ARTIFACT_REGISTRY.md contains row for Development-Protocol, entry count matches.
- **File:** `docs/SE_ARTIFACT_REGISTRY.md`

### F-004: Local Check Gates

- **State:** applied
- **Contract:** `scripts/check-local.sh` runs 4 gates: (1) markdown lint on critical files, (2) ADR existence check, (3) registry self-consistency, (4) .omo leak detection. Returns non-zero on any failure.
- **Test Anchoring:** Script exists, executable, all 4 gates implemented, `bash scripts/check-local.sh` passes.
- **File:** `scripts/check-local.sh`

### F-005: HANDOVER Session Continuity

- **State:** applied
- **Contract:** HANDOVER.md captures current HEAD SHA, FINAL STATE table, standing rules, session history. HANDOVER-HISTORY provides append-only log. Must be updated on every session end to prevent staleness.
- **Test Anchoring:** HANDOVER.md HEAD matches `git rev-parse HEAD`, FINAL STATE table includes all tracked repos, privacy/signing bullets present.
- **File:** `Development-Protocol-Local/HANDOVER.md`, `Development-Protocol-Local/HANDOVER-HISTORY-2026-08.md`

### F-006: Architecture Decision Records

- **State:** applied
- **Contract:** ADRs in `docs/adr/` with sequential numbering. Three accepted: 001 (two-stage pipeline), 002 (prototyping gate), 003 (document-driven). Append-only once accepted.
- **Test Anchoring:** `ls docs/adr/*.md` returns ≥3 files, each has "Accepted" status.
- **File:** `docs/adr/001-two-stage-pipeline.md`, `docs/adr/002-prototyping-gate.md`, `docs/adr/003-document-driven.md`

### F-007: Three-Layer Specification Model

- **State:** applied
- **Contract:** ../steps/SPECIFICATION.md defines MACRO (strategy), MESO (structure), MICRO (implementation) layers. Provides templates with Bus-Hop examples. §15 verification checklist references FEATURES.md.
- **Test Anchoring:** ../steps/SPECIFICATION.md contains all three layer definitions, §15 references FEATURES.md.
- **File:** `../steps/SPECIFICATION.md` (586 lines — flagged as oversized in TECH_DEBT)

### F-008: Standards Tier System

- **State:** applied
- **Contract:** ../steps/STANDARDS.md defines T1 (mandatory), T2 (recommended), T3 (optional) tiers. Covers 14 domains: error handling, code quality, testing, docs, security, performance, architecture, AI attribution, CI/CD, AI laziness, build, deps, review, objectivity.
- **Test Anchoring:** ../steps/STANDARDS.md contains all 14 section headings, tier definitions present.
- **File:** `../steps/STANDARDS.md` (285 lines)

### F-009: Stance Read (User-Model Layer)

- **State:** applied
- **Contract:** INBOX reads 7 stance dimensions (expertise, assertiveness, domain-match, change-cost illusion, hidden constraints, social proof, playfulness) before engaging; handling per cell (expertise→substance, assertiveness→elicitation, reflectiveness→no-early-template, gravity→vibe mode + tripwire).
- **Test Anchoring:** Stance Read blockquote present in ../steps/INBOX.md; cite line names sources; only Patience marked heuristic.
- **File:** `../steps/INBOX.md` (commits da6beab, 9edffe4, f223ce3, 7c374b7)
- **Origin:** user query — _protocol assumes an ignorant-compliant user; what about assertive/expert/every human trait?_ ("there are so many kinds of people that dev protocol might not work well with"). Research: novice-reliance (arXiv:2505.08063), assertiveness persuasion (Kim & Khashabi EMNLP25), NfC friction (Buçinca 2021), face-preservation (ELEPHANT +45pp).

### F-010: Dissent Log

- **State:** applied
- **Contract:** SERIOUSNESS logs every user-override as a dissent with outcome tracking; KILL_LOG D-section carries D-IDs + Right/Wrong-call; reckoning every 10 overrides recalibrates the stance table.
- **Test Anchoring:** Dissent Log blockquote in ../steps/SERIOUSNESS.md; D-section table in ../steps/KILL_LOG.md.
- **File:** `../steps/SERIOUSNESS.md`, `../steps/KILL_LOG.md` (commit da6beab)
- **Origin:** same trait thread as F-009 (disagreement under assertiveness). Research: practitioner design (dissent-dynamics worker returned hollow; built from teaming first principles).

### F-011: Scope Ceiling

- **State:** applied
- **Contract:** STRATEGY ratified proposal states scope ceiling + exclusion list (less wins ties); REVIEW 2.7 scope-fidelity check FAILs unlisted scope unless new cycle; KILL_LOG X-IDs track exclusions with no 30-day check.
- **Test Anchoring:** scope-ceiling blockquote in ../steps/STRATEGY.md; §2.7 in ../steps/REVIEW.md; Exclusion Log in ../steps/KILL_LOG.md.
- **File:** `../steps/STRATEGY.md`, `../steps/REVIEW.md`, `../steps/KILL_LOG.md` (commits ffe1fa7, 70c385c)
- **Origin:** user query — _LLMs do too much; find the right amount of work, default less_ ("no less no more... if we had to choose between less or more, less is better"). Research: Lientz & Swanson, Boehm, Gall; 64% stat scope note (4 apps, XP2002).

### F-012: Truth-Elicitation Rules

- **State:** applied
- **Contract:** EXTRACTION trigger (e) detects guarded answers; 9 elicitation rules (open narration, SUE/C-E probes, checkable details, unanticipated angle, no-pressure boundary, adapt-don't-script).
- **Test Anchoring:** trigger + blockquote in ../steps/EXTRACTION.md; rule (10) states departure conditions.
- **File:** `../steps/EXTRACTION.md` (commits 40d2120, ea001c7, 7c374b7)
- **Origin:** user query — _some people just straight up lie, consciously or unconsciously_ ("could it be trustworthy enough that they put their guard down?"). Research: SUE/SUE-Incremental/C-E (Hartwig & Granhag; O&W 2020), face-saving SDB review (2026, 121 exps), Bond & DePaulo 54%, cognitive interview (Fisher & Geiselman), verifiability approach.

### F-013: Cross-Cycle Seed

- **State:** applied
- **Contract:** REFLECT Q9 seeds next INBOX (unmet ambition + rejected directions, max 3 deferred, one scale-step up); INBOX accepts prior-cycle seed; EXTRACTION states upfront-default doctrine.
- **Test Anchoring:** Q9 in ../steps/REFLECT.md; seed entry in ../steps/INBOX.md; doctrine in ../steps/EXTRACTION.md; X-002 in ../steps/KILL_LOG.md.
- **File:** `../steps/REFLECT.md`, `../steps/INBOX.md`, `../steps/EXTRACTION.md` (commit ec20904)
- **Origin:** user query — _is the protocol one-shot or multi-shot?_ ("is it better for one shots? or better for multiple shorter oneshots?"). Research: Brooks second-system effect + mitigations (staged delivery, strangler fig).

### F-014: Raw-First Discipline

- **State:** applied
- **Contract:** INBOX raw-pass blockquote (attend raw, then run protocol); always-on rule in agent stack (dev-defaults.mdc).
- **Test Anchoring:** blockquote in ../steps/INBOX.md; rule in ~/.config/opencode rules.
- **File:** `../steps/INBOX.md` (commit 09af718)
- **Origin:** user query — _the protocol should tell the AI agent to attend to the user's response raw first, then use the development protocol_ (verbatim). Research: Guilford divergent-first, Luchins Einstellung, Jansson & Smith fixation + trajectory anchor.

### F-015: SWE Appendixes (P2b Early + P4 Late)

- **State:** applied
- **Contract:** p2b-mapping-appendix (use-case/domain/quality/stakeholder/risk/keep-drop/version) + p4-late-appendix (test/V&V/build/deploy/env), depth-gated; gates wired in LANDSCAPE/STRATEGY/SPEC/QUICKSTART.
- **Test Anchoring:** both appendix files exist; prototype/tests/test_signal.py passes; gate refs present in 4 files.
- **File:** `docs/appendix/*.md` (commit e9f3823)
- **Origin:** user query — _follow best SWE practices so we know what we want to build and keep_ (G1-G8 gap closure). Research: G1-G8 analysis (use-case/domain/quality/stakeholder/risk/test/keep-drop/version).

### F-016: Open-Source Usefulness Metric

- **State:** proposed
- **Contract (draft):** useful-for-public iff dogfood TRUE **and** stranger-test PASS; currently NOT useful (stranger FAIL on 4 doc gaps); docs fix queued as engine work.
- **Test Anchoring:** decision trace in .omo/traces/ (quarantined, local-only); no protocol files changed yet.
- **File:** _(none yet — trace only)_
- **Origin:** user query — open-source search decision funnel (benefit others vs privacy/fatigue; usefulness-gate first). Research: cold-start stranger audit (commit 201b9cd).
  |

### F-017: Learning Anti-Abdication

|

- **State:** applied
- **Contract:** (a) abdication tripwire in SERIOUSNESS (3+ passive cycles → pointed retrieval question before COMMIT); (b) standing-decisions display in STRATEGY ratification + every EXECUTOR output; learned-this-cycle in REFLECT Q9; (c) explain-back anti-gaming at STRATEGY ratification (echo fails, ledger-checked); (d) per-cycle learning signal — five counts (dissents, unprompted questions, first-attempt explain-backs, tripwire firings, spikes run) trended against the prior cycle.
- **Test Anchoring:** tripwire text in steps/SERIOUSNESS.md; standing-decisions rule in steps/STRATEGY.md + steps/EXECUTOR.md; Q9 line + learning-signal counts in steps/REFLECT.md.
- **File:** `steps/SERIOUSNESS.md`, `steps/STRATEGY.md`, `steps/EXECUTOR.md`, `steps/REFLECT.md`
- **Origin:** user query — _need to learn the science of learning... Primeagen learned-helplessness worry_ + live confession of yes-without-reading + standing-decisions refinement. Research: testing effect (Roediger & Karpicke 2006), self-explanation effect (Chi 1989), generation effect (verified via Wikipedia). Sweep: metacognitive decoupling under LLM use (arXiv:2603.29681); expertise-reversal guidance calibration (CLT); productive failure — generation before instruction (Kapur); guidance meta-analysis d=0.50-0.71; interleaving/deliberate-practice already covered, no change.

### F-018: House Standard (T0) and the Taste Boundary

- **State:** applied
- **Contract:** T0 house standard (consistency, fitness for the stated purpose, legibility to the declared reader, defensibility of claims, freedom from self-serving embellishment) applied by default and declared in one line; T1 objective layer owned by the agent; T2 evidenced taste applied with each inference flagged; T3 novel taste asked with a proposed default. Enforced by `REVIEW.md` row 4.10; pointers from CONSTITUTION "What follows" item 3, the `RULES.md` objectivity duty, `STANDARDS.md` §14, and `EXECUTOR.md` § Production Quality Requirements.
- **Test Anchoring:** `docs/HOUSE_STANDARD.md` exists; `REVIEW.md` carries row 4.10; the declaration rule appears in the three governing files.
- **File:** `docs/HOUSE_STANDARD.md`
- **Origin:** user query — _at least you should have your own objective taste. as long as the foundation you set is good it can work with anyone's tastes._ Research: `docs/research/ai-taste-judgment.md`.

### F-019: Craft-Primary Ordering (tools are a floor)

- **State:** applied
- **Contract:** the craft level (how the code reads — names, structure, seams; how prose reads; whether a measurement states its method) runs first and the deterministic gates follow as the floor; instrumentation is a floor, never a ceiling, and a green checklist is not a claim of quality. No gate is removed or relaxed. Stated in `docs/HOUSE_STANDARD.md` § Craft before gates, with the order named at `EXECUTOR.md` § FINISH Gate and `REVIEW.md` Phase 4.
- **Test Anchoring:** `docs/HOUSE_STANDARD.md` carries § Craft before gates; `EXECUTOR.md` Polish section and `REVIEW.md` Phase 4 each carry the ordering line.
- **File:** `docs/HOUSE_STANDARD.md`
- **Origin:** user query — _guardrails are important, but what's more important is that we do the thing right in the first place... the tools are assistance on top of foundationally how someone codes, not the other way round nor is it equal in significance._ Grounded in the expert-lens audit (compliance is not quality; the machinery was denser at the mechanical level than at the craft level).

### F-020: Domain Norms (the specialist layer)

- **State:** applied
- **Contract:** every run declares its domain and 2-5 domain craft norms, each checkable and tagged `known` / `researched` / `asked`; LANDSCAPE searches for the norms when the domain is unfamiliar; `REVIEW.md` row 4.11 verifies the artifact against them. Declared at AMBITION round 5, ratified at STRATEGY with the quality bar, detailed in `docs/HOUSE_STANDARD.md` § Domain instantiation.
- **Test Anchoring:** `docs/HOUSE_STANDARD.md` carries § Domain instantiation; the declaration line is in `steps/AMBITION.md`; the search target is in `steps/LANDSCAPE.md`; row 4.11 is in `steps/REVIEW.md`.
- **File:** `docs/HOUSE_STANDARD.md`
- **Origin:** user query — _any project can be made by a generalist but to make it good we need a specialist... a project can have general structure but it also requires tailoring to make it good._ Grounded in SC2001: the structure held, and the missing thing was a domain norm (a reported optimum must survive a repeat) — `lessons/FC-2026-007-measurement-robustness.md`.


### F-021: Iterate-to-Handover Loop (layered adversarial descent)

- **State:** applied
- **Contract:** the FINISH gate runs a bounded loop over four layers — L1 structure, L2 behavior, L3 craft, L4 surface — each with a fresh-eyed adversary and its own source of truth, gated so a layer must be clean before the next descent. It ends when two consecutive descents surface no new class of problem (budget 3, or 5 for Deep) and emits the handover residue: only items that are unknowable-from-evidence AND cheap for the human, each with a proposed default, and never a structural item. REVIEW row 4.12 fails a run that hands over a structural item. The adversarial review agent is the outside pass on the loop result.
- **Test Anchoring:** REVIEW 4.12 checks the residue list and the stop rule; EXECUTOR records each descent (and any skipped layer) in the method ledger.
- **File:** `../steps/EXECUTOR.md` § The internal loop; `../steps/REVIEW.md` (Adversarial Review Agent, row 4.12)
- **Origin:** user thought (verbatim, `THOUGHT_LOG.md` T-026) — iterate multiple times with the AI reviewing every new iteration, big-picture problems first and then down to the technical details, assuming the human is the strictest reviewer to possibly exist; and T-032 — many agent adversarial checks and reviews, going down layer by layer. Bounded by the ithmb overrun evidence (unbounded *until satisfied* turns one week into three months) and by the T0–T3 bar (the residue is where T3 lives).

### F-022: Declared Delegation & Route Announcement (T-039)

- **State:** applied
- **Contract:** the run states its route and its decision count before Phase 1; the human may delegate a named decision in writing, once per run, logged as `delegated`; delegation never covers a one-way door and never counts toward the abdication tripwire.
- **Test Anchoring:** SERIOUSNESS Phase 0 states the route and the count; a delegated decision appears in the `KILL_LOG.md` Dissent Log with the default taken and the counterfactual; the tripwire does not fire on a cycle whose passivity was declared delegation.
- **File:** `steps/SERIOUSNESS.md`, `steps/STRATEGY.md`, `steps/KILL_LOG.md`, `docs/CONSTITUTION.md`, `steps/QUICKSTART.md`
- **Origin:** the user’s harshness thought (T-039 verbatim: “I made this for strategists, but most people are not strategists they just want work done”). The protocol demanded judgment as the price of entry, defaulted every run to Standard, and called honest handover abdication.


### F-023: Graded Surfaces Are Planned, Generated Numbers Come From Data

- **State:** applied
- **Contract:** the specification names every surface a third party reads or grades, presentations included; a generated artifact that carries numbers is built from the data file, never from the prose of another document, and asserts its expected shape before drawing.
- **Test Anchoring:** SPECIFICATION section 2 names the graded surfaces; the generator fails loudly when the data file lacks a row it expects; REVIEW 4.8 catches a comparative measurement instrumented on one side only.
- **File:** `steps/SPECIFICATION.md`, `steps/EXECUTOR.md`, `steps/REVIEW.md`, `steps/VALIDATION.md`
- **Origin:** SC2001 Project 2 (2026-09-24). The deck carried 20 percent of the grade but was never in the artifact list, so its first draft took numbers from the notebook prose. The fresh-eyes descent found a typed ratio disagreeing with a computed one and a graph that was never built. In the same run the spike showed that the measurement method, not the repetition count, was the fault.

## Trace Tags

- `engineering-plugin:§1.1` — F-### lifecycle definition
- `engineering-plugin:§3-item7` — traceability to implementation
- `engineering-plugin:§4.7` — FEATURES.md current check
- `engineering-plugin:§4.8` — TECH_DEBT_AUDIT.md triaged check
- `SE_ARTIFACT_REGISTRY` — cross-repo tracking
- `check-local.sh` — local verification gates

## Relationships

| Feature                  | Depends On   | Referenced By                                                      |
| ------------------------ | ------------ | ------------------------------------------------------------------ |
| F-001 INBOX              | —            | ../steps/RULES.md routing, README.md pipeline                      |
| F-002 Engineering Plugin | —            | F-003, F-004, F-008                                                |
| F-003 SE Registry        | F-002        | AGENTS.md, check-local.sh                                          |
| F-004 Check Gates        | F-002, F-003 | AGENTS.md, push workflow                                           |
| F-005 HANDOVER           | —            | AGENTS.md, session workflow                                        |
| F-006 ADR                | —            | AGENTS.md, ../steps/RULES.md governance                            |
| F-007 Specification      | F-006        | ../steps/SPECIFICATION.md §15                                      |
| F-008 Standards          | —            | ../steps/RULES.md, ../steps/STANDARDS.md                           |
| F-009 Stance Read        | —            | ../steps/INBOX.md, F-010                                           |
| F-010 Dissent Log        | F-009        | ../steps/SERIOUSNESS.md, ../steps/KILL_LOG.md                      |
| F-011 Scope Ceiling      | —            | ../steps/STRATEGY.md, ../steps/REVIEW.md §2.7                      |
| F-012 Truth-Elicitation  | F-009        | ../steps/EXTRACTION.md                                             |
| F-013 Cross-Cycle        | —            | ../steps/REFLECT.md Q9, F-011                                      |
| F-014 Raw-First          | —            | ../steps/INBOX.md, agent stack                                     |
| F-015 SWE Appendixes     | F-007        | docs/appendix/, P2b/P4 gates                                       |
| F-016 OSS Metric         | —            | proposed; trace only                                               |
| F-017 Learning           | F-009, F-010 | ../steps/SERIOUSNESS.md, ../steps/STRATEGY.md, ../steps/REFLECT.md |
| F-018 House Standard      | F-008        | ../steps/REVIEW.md, ../steps/RULES.md, ../steps/EXECUTOR.md |
| F-019 Craft Ordering | F-007, F-018 | HOUSE_STANDARD.md, ../steps/EXECUTOR.md, ../steps/REVIEW.md |
| F-020 Domain Norms | F-018 | HOUSE_STANDARD.md, ../steps/AMBITION.md, ../steps/LANDSCAPE.md, ../steps/REVIEW.md |
| F-021 Iterate Loop | F-018 | ../steps/EXECUTOR.md, ../steps/REVIEW.md, STANDING_PRINCIPLES.md |
| F-022 Delegation | F-017, F-021 | ../steps/SERIOUSNESS.md, ../steps/STRATEGY.md, ../steps/KILL_LOG.md, CONSTITUTION.md |
| F-023 Graded Surfaces | F-018, F-021 | SPECIFICATION.md, EXECUTOR.md, REVIEW.md, VALIDATION.md |

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
- **Contract:** (a) abdication tripwire in SERIOUSNESS (3+ passive cycles → pointed retrieval question before COMMIT); (b) standing-decisions display in STRATEGY ratification + every EXECUTOR output; learned-this-cycle in REFLECT Q9; (c) explain-back anti-gaming at STRATEGY ratification (echo fails, ledger-checked).
- **Test Anchoring:** tripwire text in steps/SERIOUSNESS.md; standing-decisions rule in steps/STRATEGY.md + steps/EXECUTOR.md; Q9 line in steps/REFLECT.md.
- **File:** `steps/SERIOUSNESS.md`, `steps/STRATEGY.md`, `steps/EXECUTOR.md`, `steps/REFLECT.md`
- **Origin:** user query — _need to learn the science of learning... Primeagen learned-helplessness worry_ + live confession of yes-without-reading + standing-decisions refinement. Research: testing effect (Roediger & Karpicke 2006), self-explanation effect (Chi 1989), generation effect (verified via Wikipedia). Sweep: metacognitive decoupling under LLM use (arXiv:2603.29681); expertise-reversal guidance calibration (CLT); productive failure — generation before instruction (Kapur); guidance meta-analysis d=0.50-0.71; interleaving/deliberate-practice already covered, no change.

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

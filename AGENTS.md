# Development Protocol

**Generated:** 2026-08-29

## Overview

Protocol-of-protocols (PoP) — recursive strategist across the full means-ends chain, specialized to accomplishment. Four altitudes (docs/PHILOSOPHY.md): P1 WANT incl. tacit (3-layer extraction) → P2 SHOULD_WANT two-bar [P2a SHOULD-BUILD-X? DROP/COMMIT via SERIOUSNESS → P2b WHICH-X? same/scaled/adjacent/more via LANDSCAPE] → P3 BEST_PLAN (decomposition/strategy/landscape/failure/fundamentals) → P4 EXECUTE (spec/executor/validation/review/reflect/prioritize). Two gates enforce order; appendixes translate PoP decisions to SE artifacts.

Part of a trio: this (process), Standards (what good means), Lessons (cross-project knowledge).

## Layout

```
Development-Protocol/
├── AGENTS.md                          # This file
├── README.md                          # Pipeline overview, 14-step diagram, standing principles
├── RULES.md                           # Project bootstrap protocol, constitution, phase definitions
├── STANDARDS.md                       # T1/T2/T3 tier rules (error handling, testing, security, etc.)
├── SPECIFICATION.md                   # Three-layer model (MACRO/MESO/MICRO), templates
├── INBOX.md                           # Step -1: thought capture & triage
├── EXTRACTION.md                      # Step 0: extract real problem from stated solution
├── AMBITION.md                        # Step 1: scope & ambition definition
├── DECOMPOSITION.md                   # Step 2: break into manageable pieces
├── STRATEGY.md                        # Step 3: approach selection
├── LANDSCAPE.md                       # Step 4: environment & constraint mapping
├── FAILURE_CAPTURE.md                 # Step 5: failure mode analysis
├── FUNDAMENTALS.md                    # Step 6: foundational requirements
├── EXECUTOR.md                        # Step 7: execution planning
├── VALIDATION.md                      # Step 8: validation strategy
├── REVIEW.md                          # Step 9: review & reflection
├── REFLECT.md                         # Step 10: post-execution reflection
├── PRIORITIZE.md                      # Step 11: priority ordering
├── SERIOUSNESS.md                     # Step 12: P2a SHOULD-BUILD-X? Bar 1 gate (DROP/COMMIT)
├── QUICKSTART.md                      # 5-min P1/P2a/P2b + 30-min Light map
├── BIAS_CATALOG.md                    # 8 biases, dangerous phases, detection prompts
├── KILL_LOG.md                        # DROP/KILL log + 30d counterfactual
├── docs/
│   ├── appendix/
│   │   ├── p2b-mapping-appendix.md      # P2b early SWE: use-case/domain/quality/stakeholder/risk/keep-drop/version → SE tables + stencil
│   │   └── p4-late-appendix.md          # P4 late SWE: test/V&V/build/deploy/env + CI stencil
│   ├── traces/
│   │   └── colour-blind-85-100.md     # Lived trace P1 tacit X → P2a COMMIT → P2b more-than-X
│   ├── AGENTS.md                      # (this file's canonical location if moved)
│   ├── FEATURES.md                    # F-### feature registry with lifecycle states
│   ├── SE_ARTIFACT_REGISTRY.md        # Cross-repo SE artifact tracking
│   ├── TECH_DEBT_AUDIT.md             # Active debt triage (severity × effort)
│   ├── engineering-plugin.md          # §1.1 F-### lifecycle, §3 traceability, §4 checks
│   ├── TESTING.md                     # Test strategy & philosophy
│   ├── QUALITY_BAR.md                 # Quality thresholds & gates
│   ├── MEASUREMENT.md                 # Metrics & measurement approach
│   ├── METHOD_LEDGER.md               # Method tracking ledger
│   ├── PROTOCOL_MODEL.md              # Protocol model documentation
│   ├── EXPLAINER.md                   # Protocol explainer for newcomers
│   ├── CI_TEST_GATE.md                # CI test gate specification
│   ├── SPEC_SYNC.md                   # Specification synchronization
│   ├── SKIP_CATALOG.md                # Catalog of skippable items
│   ├── UNIVERSAL_FUNDAMENTALS.md      # Universal fundamentals reference
│   ├── adr/                           # Architecture Decision Records
│   │   ├── 001-two-stage-pipeline.md
│   │   ├── 002-prototyping-gate.md
│   │   └── 003-document-driven.md
│   ├── research/                      # Research artifacts
│   └── standards/                     # Standards reference docs
├── scripts/
│   ├── check.sh                       # CI gate: RULES.md phases + CLI contract + cargo
│   ├── check-local.sh                 # Local gates: markdown lint + ADR + registry + .omo leak
│   └── ledger-check.py                # Ledger validation
├── cli/                               # Rust CLI tooling (cargo check/test)
├── template/                          # Protocol templates
└── .omo/                              # Agent workspace (NEVER commit)
```

## SE Lifecycle Artifacts

| Artifact           | File                                     | Purpose                                              | Status                       |
| ------------------ | ---------------------------------------- | ---------------------------------------------------- | ---------------------------- |
| FEATURES           | `docs/FEATURES.md`                       | F-### feature registry with lifecycle states         | Active, 8 entries            |
| SPECIFICATION      | `SPECIFICATION.md`                       | Three-layer model (MACRO/MESO/MICRO)                 | Active, 586L (see TECH_DEBT) |
| ARCHITECTURE       | `docs/adr/`                              | Architecture Decision Records (3 ADRs)               | Active                       |
| TECH_DEBT          | `docs/TECH_DEBT_AUDIT.md`                | Active debt triage, severity × effort                | Active                       |
| REGISTRY           | `docs/SE_ARTIFACT_REGISTRY.md`           | Cross-repo SE artifact tracking                      | Active, self-tracked         |
| CHECK-GATES        | `scripts/check-local.sh`                 | Local verification: markdown + ADR + registry + .omo | Active                       |
| ENGINEERING-PLUGIN | `docs/engineering-plugin.md`             | §1.1 lifecycle, §3 traceability, §4 checks           | Active                       |
| HANDOVER           | `Development-Protocol-Local/HANDOVER.md` | Session continuity across agents                     | Active, verified             |

## Pipeline Flow

P1 WANT (INBOX → EXTRACTION → AMBITION) → P2a SHOULD-BUILD-X? (SERIOUSNESS Bar 1: DROP/COMMIT) → P2b WHICH-X? (LANDSCAPE Bar 2: same/scaled/adjacent/more + appendix mapping) → P3 BEST_PLAN (DECOMPOSITION → STRATEGY → FAILURE_CAPTURE → FUNDAMENTALS) → P4 EXECUTE (SPECIFICATION RTM §1.5 + EXECUTOR → VALIDATION → REVIEW 4 loops + Gate 2.6 → REFLECT Q8 → PRIORITIZE → KILL_LOG retro).

Altitudes vary; recursive strategist. `docs/appendix/p2b-mapping-appendix.md` (early SWE) and `p4-late-appendix.md` (late SWE) are opt-in, depth-gated — Light logs skip, Standard+ fills.

Each step produces a `.md` artifact. RULES.md governs routing and phase transitions. STANDARDS.md enforces quality tiers (T1 mandatory, T2 recommended, T3 optional).

## Local-First CI

No GitHub Actions — all verification is local via `scripts/check-local.sh` and `scripts/check.sh`. This is by design: the protocol is docs-only, and local gates catch issues before push.

```bash
# Run local verification
bash scripts/check-local.sh
bash scripts/check.sh
```

## Do Not Touch

- `.omo/` — agent workspace, never commit, own nested git repo
- `Development-Protocol-Local/` — local-only session data, no remote
- `cli/` — Rust tooling, only modify via cargo workflow
- `docs/adr/` — ADRs are append-only once accepted
- Root `RULES.md`, `STANDARDS.md` — governance files, changes require ADR

## What NOT to Do

- Do not push to `public` remote without explicit go — push to `origin` (Dev) only
- Do not add PR templates or GitHub Actions workflows
- Do not edit other repos (Ithmb, Bus-Hop, Oh-My-Learner)
- Do not commit `.omo/` contents
- Do not add code snippets to docs without SE lifecycle state
- Do not skip check-local.sh before push

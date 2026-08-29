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
- **Test Anchoring:** INBOX.md exists, contains cluster/triage instructions, referenced by RULES.md routing.
- **File:** `INBOX.md` (250 lines)

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
- **Contract:** SPECIFICATION.md defines MACRO (strategy), MESO (structure), MICRO (implementation) layers. Provides templates with Bus-Hop examples. §15 verification checklist references FEATURES.md.
- **Test Anchoring:** SPECIFICATION.md contains all three layer definitions, §15 references FEATURES.md.
- **File:** `SPECIFICATION.md` (586 lines — flagged as oversized in TECH_DEBT)

### F-008: Standards Tier System

- **State:** applied
- **Contract:** STANDARDS.md defines T1 (mandatory), T2 (recommended), T3 (optional) tiers. Covers 14 domains: error handling, code quality, testing, docs, security, performance, architecture, AI attribution, CI/CD, AI laziness, build, deps, review, objectivity.
- **Test Anchoring:** STANDARDS.md contains all 14 section headings, tier definitions present.
- **File:** `STANDARDS.md` (285 lines)

## Trace Tags

- `engineering-plugin:§1.1` — F-### lifecycle definition
- `engineering-plugin:§3-item7` — traceability to implementation
- `engineering-plugin:§4.7` — FEATURES.md current check
- `engineering-plugin:§4.8` — TECH_DEBT_AUDIT.md triaged check
- `SE_ARTIFACT_REGISTRY` — cross-repo tracking
- `check-local.sh` — local verification gates

## Relationships

| Feature | Depends On | Referenced By |
|---------|-----------|---------------|
| F-001 INBOX | — | RULES.md routing, README.md pipeline |
| F-002 Engineering Plugin | — | F-003, F-004, F-008 |
| F-003 SE Registry | F-002 | AGENTS.md, check-local.sh |
| F-004 Check Gates | F-002, F-003 | AGENTS.md, push workflow |
| F-005 HANDOVER | — | AGENTS.md, session workflow |
| F-006 ADR | — | AGENTS.md, RULES.md governance |
| F-007 Specification | F-006 | SPECIFICATION.md §15 |
| F-008 Standards | — | RULES.md, STANDARDS.md |

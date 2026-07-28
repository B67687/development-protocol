# Dev Protocol + Agent-Stack Integration

## Overview

This project integrates two development governance systems:

1. **[Development Protocol](https://github.com/B67687/Development-Protocol)** — Document-driven conception-to-spec system: AMBITION (interview) → LANDSCAPE (research) → VALIDATION (prototyping gate) → SPECIFICATION (14-section spec)
2. **Agent-Stack (OMO start-work)** — Execution engine: Prometheus planning → Momus validation → Boulder execution with 5 verification gates per checkbox → evidence ledger

They cover each other's weak spots. Dev Protocol handles the front-end (conception, validation, spec) and back-end (polish, explainer, spec-sync, review). start-work handles the middle (execution with verification).

## Combined Workflow

```
RAW INTENT
    │
    ▼
┌─────────────────────────────────────────┐
│ Dev Protocol: AMBITION.md               │
│ 6-round Socratic interview, challenge    │
│ assumptions, XY-problem check,           │
│ lock falsifiable hypothesis              │
└─────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────┐
│ Dev Protocol: LANDSCAPE.md              │
│ Research existing solutions, identify    │
│ unknowns, map landscape                  │
└─────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────┐
│ Dev Protocol: VALIDATION.md             │
│ Prototyping gate: KILL/PIVOT/COMMIT     │
│ (throwaway prototypes, weighted matrix) │
└─────────────────────────────────────────┘
    │ (if COMMIT)
    ▼
┌─────────────────────────────────────────┐
│ Dev Protocol: SPECIFICATION.md          │
│ 14-section comprehensive spec           │
│ (3-layer: MACRO/MESO/MICRO)            │
└─────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────┐
│ Agent-Stack: start-work                 │
│ Prometheus reads SPEC → plan →          │
│ Momus validates → Boulder executes      │
│ (5 verification gates per checkbox,     │
│ adversarial QA, evidence ledger,        │
│ auto-continuation)                      │
└─────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────┐
│ Dev Protocol: POLISH.md                 │
│ 6 quality gates: quality, heuristics,   │
│ edge cases, non-functional, shipping,   │
│ retrospective                           │
└─────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────┐
│ Dev Protocol: EXPLAINER.md +            │
│ SPEC_SYNC.md + REVIEW.md               │
│ Architecture walkthrough, spec-to-code  │
│ fidelity, 27-check meta-review          │
└─────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────┐
│ Dev Protocol: METHODOLOGY.md            │
│ 4-layer retrospective for next cycle    │
└─────────────────────────────────────────┘
    │
    ▼
SHIP
```

## Decision Guide

| You have...                                     | Use                                                                                                                                                                                                         |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A raw itch/idea, not sure if worth building     | **Development Protocol** — AMBITION.md first. The 6-round interview will clarify if it's worth pursuing. If still unclear, run VALIDATION.md (prototype → KILL/PIVOT/COMMIT)                                |
| A clear goal, want to execute fast with quality | **Agent-Stack /start-work** — Prometheus plans → Momus validates → executes with verification gates                                                                                                         |
| An ambitious project needing full rigor         | **Combined** — Dev Protocol for prep (AMBITION→LANDSCAPE→VALIDATION→SPECIFICATION), then start-work for execution (spec → plan → verify), then Dev Protocol for quality (POLISH→EXPLAINER→SPEC_SYNC→REVIEW) |
| A shipped project that needs quality audit      | **Development Protocol** — POLISH.md + EXPLAINER.md + SPEC_SYNC.md + REVIEW.md                                                                                                                              |
| Post-project retrospective                      | **Development Protocol** — METHODOLOGY.md (4-layer)                                                                                                                                                         |
| Quick fix / config change                       | **Neither** — just chat directly with the agent                                                                                                                                                             |

## Key Insight: Opposite Interview Stances

The fundamental design difference between the two systems:

|                | Development Protocol                                                                                                           | start-work (Prometheus)                                                                                                   |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------- |
| On vague ideas | **Interrogates**: "AI's job is NOT to approve your idea — it's to interrogate, push back, and help you converge" (AMBITION.md) | **Does NOT interrogate**: "Do NOT interrogate the user. Resolve ambiguity by research, not questions" (intent-unclear.md) |
| On clear goals | **Locked**: "No more changes after this document" (AMBITION.md Round 5)                                                        | **Targeted**: Ask only the owner-decisions the repo cannot answer (intent-clear.md)                                       |

This is why they complement: use Dev Protocol when you need to be challenged and pushed. Use start-work when you already know what you want and need fast execution.

## File Reference

### Development Protocol Files

| File                | Purpose                                              |
| ------------------- | ---------------------------------------------------- |
| `AMBITION.md`       | Gate 1 — 6-round Socratic dialogue to clarify intent |
| `LANDSCAPE.md`      | Research protocol — map what exists                  |
| `VALIDATION.md`     | Gate 2 — prototyping gate with KILL/PIVOT/COMMIT     |
| `SPECIFICATION.md`  | Locked plan-IS-spec template (14 sections)           |
| `EXECUTOR.md`       | AI execution handoff with autonomy levels            |
| `RULES.md`          | Core protocol governance (v2.2.0)                    |
| `POLISH.md`         | Human final pass after AI execution                  |
| `docs/EXPLAINER.md` | AI-generated code explainer for non-coder            |
| `docs/SPEC_SYNC.md` | Spec-to-code fidelity gate                           |
| `REVIEW.md`         | Meta-review gate — independent protocol audit        |
| `METHODOLOGY.md`    | Post-project layered retrospective                   |
| `PLAYBOOK.md`       | Infrastructure checklist                             |
| `STANDARDS.md`      | Production standards (13 categories, 3 tiers)        |

### Agent-Stack Files

| File                         | Purpose                             |
| ---------------------------- | ----------------------------------- |
| `opencode.jsonc`             | Core OpenCode config                |
| `oh-my-openagent.jsonc`      | OMO plugin config                   |
| `scripts/verify.sh`          | Unified verification suite          |
| `scripts/regression-test.sh` | Config regression tests (29 checks) |
| `docs/WORKFLOW.md`           | Workflow documentation              |
| `docs/CONFIG_MAP.md`         | Config setting documentation        |

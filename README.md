# Development Protocol

A document-driven development protocol with three decision gates that prevent solution-jumping and catch wrong foundations before you build on them.

Part of a trio of meta-projects:

- Development Protocol (this) — process from intent to product
- Standards (github.com/B67687/Standards) — what good means, automated audits
- Lessons — cross-project knowledge base, loaded every session

Built by following [the Development Protocol](https://github.com/B67687/Development-Protocol) itself — a recursive self-completeness test.

## The Pipeline

RAW INTENT
│
▼
┌─────────────────────────────────────────────────┐
│ INBOX (Step -1) │
│ Multi-thought capture → cluster → triage │
│ Select one cluster for EXTRACTION, park rest │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ EXTRACTION (Step 0) │
│ Extract X (real problem) from Y (stated │
│ solution). 7 proven techniques: Goal Climb, │
│ No-Computer Check, Why-Tree, Contextual Probe, │
│ Mom Question, Problem Statement Wall, │
│ Job/Pain/Gain Map. │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ SERIOUSNESS (Idea Evaluation Gate) │
│ Phase 1: Commitment Probe │
│ Phase 2: Dimension Scoring (0-100) │
│ Phase 3: Kill Criteria / Pre-commit Off-Ramp │
│ Exit: COMMIT / SCHEDULE / DROP │
└─────────────────────────────────────────────────┘
│ (if COMMIT)
▼
┌─────────────────────────────────────────────────┐
│ FUNDAMENTALS │
│ Identify one-way door decisions. Validate each │
│ with minimum prototype. Detect LLM bias. │
│ Then proceed to decomposition only when │
│ foundations are proven safe. │
└─────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────┐
│ MULTI (Multidisciplinary Probe) │
│ Domain Relevance Filter → Quick Probes → Flag │
│ Ethics, Psychology, Economics, Ecology, Design │
│ Sociology, Law, Medicine Accessibility │
└─────────────────────────────────────────────────┘
    │
    ▼
│ DECOMPOSITION │
│ Cynefin classify → MECE tree → confirm each │
│ level → KNOWN / RESEARCH / PROTOTYPE routing │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ AMBITION (research-interleaved) │
│ Each round: answer → research → ask → ... │
│ Tightening loop until convergence │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ PACING (phase budget allocation) │
│ Per-phase budget, AI session boundaries, │
│ human energy rules, macro cycle structure │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ LANDSCAPE (research + Cynefin) │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ VALIDATION (prototype gate) │
│ KILL / PIVOT / COMMIT based on evidence │
└─────────────────────────────────────────────────┘
│ (if COMMIT)
▼
┌─────────────────────────────────────────────────┐
│ SPECIFICATION │
│ + Design for Change section (v3) │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ EXECUTOR (with 10 v3 rules) │
│ Interface, Test, Boundary, Size, Cycle, │
│ Appetite, AI, Abstraction, Dependency, Backlog │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ POLISH → EXPLAINER → SPEC_SYNC │
│ → REVIEW → REFLECT → ship │
└─────────────────────────────────────────────────┘

```

**Prep-sequence:** `INBOX` -> `EXTRACTION` -> `SERIOUSNESS` -> `FUNDAMENTALS` -> `MULTI` -> `DECOMPOSITION` -> `AMBITION` -> `PACING` -> `LANDSCAPE` -> `VALIDATION` -> `SPECIFICATION` -> `EXECUTOR` -> `POLISH` -> `EXPLAINER` -> `SPEC_SYNC` -> `REVIEW` -> `REFLECT` -> ship

See [REVIEW.md](REVIEW.md) for the independence protocol and fixed checklist.

## Contents
| `INBOX.md` | Step -1: Thought capture, clustering, triage. Select one cluster for EXTRACTION, park rest. |
| `EXTRACTION.md` | Gate 0 - Extract X (real problem) from Y (stated solution), 7 proven techniques |
| `SERIOUSNESS.md` | Phase 1: Commitment Probe. Phase 2: Dimension Scoring. Phase 3: Kill Criteria. |
| `FUNDAMENTALS.md` | Identify one-way door decisions, validate with minimum prototype, detect LLM bias |
| `MULTI.md` | Multidisciplinary probe: Domain Relevance Filter, Quick Probes, Flag |
| `DECOMPOSITION.md` | Intent decomposition - Cynefin classify, MECE tree, KNOW/RESEARCH/PROTOTYPE routing |
| `AMBITION.md` | Gate 1 - Research-interleaved dialogue to clarify intent |
| `PACING.md` | Phase budget allocation, tracking, adjustment, session boundaries, human energy rules |
| `LANDSCAPE.md` | Research protocol - map what exists |
| `VALIDATION.md` | Gate 2 - rapid prototyping with KILL/PIVOT/COMMIT |
| `SPECIFICATION.md` | Locked plan-IS-spec template (14 sections) |
| `EXECUTOR.md` | AI execution handoff with autonomy levels |
| `POLISH.md` | Human final pass after AI execution |
| `EXPLAINER.md` | AI-generated code explainer for non-coder (docs/EXPLAINER.md) |
| `SPEC_SYNC.md` | Spec-to-code fidelity gate with research backing (docs/SPEC_SYNC.md) |
| `RULES.md` | Core protocol governance (v3.0.0 — 12 sections) |
| `METHODOLOGY.md` | Post-project retrospective |
| `PLAYBOOK.md` | Infrastructure checklist |
| `REVIEW.md` | **Meta-review gate** — independent agent audits protocol compliance |
| `REFLECT.md` | **Protocol retrospective** — how well did the protocol guide us? 4 questions, 20 min |
| `.omo/reviews/` | Review findings archive — one file per review run |

## Origin

Generated by following the Development Protocol (v3.0.0) through its PREP PHASE on itself — producing a refined version that centers the prototyping gate as the key innovation. July 2026.
```

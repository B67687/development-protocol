# Development Protocol

A document-driven development protocol with three decision gates that prevent solution-jumping and catch wrong foundations before you build on them.

Part of a trio of meta-projects:

- Development Protocol (this) — process from intent to product
- Standards (github.com/B67687/Standards) — what good means, automated audits
- Lessons — cross-project knowledge base, loaded every session

Built by following [the Development Protocol](https://github.com/B67687/Development-Protocol) itself — a recursive self-completeness test.

## The Pipeline
```
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
│ PRIORITIZE (optional, Step 0.5) │
│ Deep comparison of 2-10 ideas from INBOX │
│ 4-dimension scoring (Want × Know × Work × Matters) │
│ Bet decision → EXTRACTION or park │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ EXTRACTION (Step 0) │
│ Extract X (real problem) from Y (stated │
│ solution). 10 proven techniques: Goal Climb, │
│ No-Computer Check, Why-Tree, Contextual Probe, │
│ Mom Question, Problem Statement Wall, │
│ Job/Pain/Gain Map, Laddering, Socratic Probe, │
│ Cognitive Interview. │
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
│ LANDSCAPE (structured research)                │
│ Frame → Search → Evaluate → Synthesize         │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ STRATEGY (strategic ratification gate)          │
│ Pre-commit → AI kernel proposal → human ratify   │
│ → premortem. Commander's intent (ADP 6-0).       │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ PACING (phase budget allocation)                │
│ Per-phase budget, AI session boundaries,         │
│ human energy rules, macro cycle structure        │
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
│ EXECUTOR (incl. POLISH)                         │
│ Interface, Test, Boundary, Size, Cycle,         │
│ Appetite, AI, Abstraction, Dependency, Backlog  │
└─────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ EXPLAINER → REVIEW (incl. SPEC_SYNC)             │
│ → REFLECT → ship                                │
└─────────────────────────────────────────────────┘

```

**Prep-sequence:** `INBOX` -> `PRIORITIZE` (optional) -> `EXTRACTION` -> `SERIOUSNESS` -> `FUNDAMENTALS` -> `MULTI` -> `DECOMPOSITION` -> `AMBITION` -> `LANDSCAPE` -> `STRATEGY` -> `PACING` -> `VALIDATION` -> `SPECIFICATION` -> `EXECUTOR` -> `EXPLAINER` -> `REVIEW` -> `REFLECT` -> ship

Ship = the SHIP exit checklist in [REFLECT.md](REFLECT.md): state block written, success criteria closed, ledger clean, maintenance status declared.

See [REVIEW.md](REVIEW.md) for the independence protocol and fixed checklist.

**Domain Scope:** The full 17-step pipeline applies to any project type. The execution
phase (SPECIFICATION through REVIEW) uses universal methodologies for specifying,
building, verifying, documenting, and reflecting on any deliverable. Engineering-specific
CI, operations, and production quality details are documented separately in the
[Engineering Plugin](docs/engineering-plugin.md) — include it when your deliverable
is software, hardware, infrastructure, or other technical systems.

**Relationship to Harness:** The protocol RIDES on an agent harness (OpenCode,
OhMyOpenAgent, Codex, Claude Code, or similar). The harness provides the execution
scaffolding: plan files, todo tracking, approval gates, task resumption, subagent
spawning, context management. The protocol does NOT duplicate that machinery —
it provides the methodology content no harness ships as a complete system:
intent extraction, commitment gates, strategy ratification, validation, review,
and reflection. Research (2026-07, source-verified — see
docs/research/harness-survey-2026-07.md) across OpenCode, OMO, Claude Code,
Codex, Roo, Cline, Aider, Amp confirms: no harness ships a complete, opinionated,
end-to-end methodology stack as enforced behavior. Every tool's design treats
methodology as the user's responsibility, not the tool's obligation — this
protocol IS that responsibility, made explicit.

**Consolidation Principle (living methodology):** The protocol is not a fixed
artifact — it is a consolidation engine. When a harness or methodology ships a
better mechanism (e.g., OMO's dual-review pipeline, Codex's ExecPlan pattern,
Claude Code's workflows), the protocol learns it, folds it in, and cites the
source. The protocol is a best-of-class methodology, continuously upgraded by
learning from everything else in the ecosystem. Nothing is invented here for its
own sake; everything is either converged from research or adapted from the best
existing practice.

**Research on Demand (Standing Principle):** Any protocol step may dispatch
verification when the AI makes a consequential claim. Every claim that could
affect planning or decisions receives a verification tier (L0-L3) determined by
the consequence of being wrong — never by the AI's confidence. Ground
consequential claims in retrieved evidence, not model memory. See LANDSCAPE.md
§ Verification Tiers for the full framework. Research is a right, not a phase.
Coverage is declared, not assumed: every consequential search states its geographic/ecosystem
scope up front and defaults to sampling non-Western (esp. Chinese) and specialised sources in their
own languages — exclusions are justified, never implicit.

**Intuition-First (Default Generation Mode):** The AI answers from latent pattern-
sensing FIRST, always. This is the default generation mode — direct, unforced,
zero scaffolding. Structure (chain-of-thought, tools, agentic machinery) is an
OPT-IN overlay applied only when: (a) the domain demands exactness (math, facts,
compliance), (b) the stakes are high (one-way doors, large bets), or (c) the
answer fails a cheap sanity probe. This matches the Dreyfus expert model and
ADP 6-0 doctrine: intuition by default, deliberate analysis as the exception,
validation when time permits. The universal guard is twofold: honest calibration
signaling — "I recognize this pattern" vs "this shape is plausible" vs "I'm
uncertain or this is speculative" — AND reliability: intuition is trusted when
the domain is regular and feedback-available (Kahneman & Klein 2009), or when
the agent's prior claims in this domain survived verification (the method
ledger). Otherwise the intuition is premature — study first (Research on
Demand; one-way doors always). A capable
model's direct answer is not a draft awaiting structure; it is the answer. See
LANDSCAPE.md § Intuition-First Route.

> **Relationship to Research on Demand:** RoD governs *claims that will affect
> planning or decisions*. Intuition-First governs *how answers are generated*. The
> two compose: the AI generates intuitively, and consequential claims are still
> verified per RoD tiers — L1 remains the default for factual claims that affect
> planning, regardless of how confident the model felt. Intuition never skips RoD;
> it changes the *generation path*, not the *verification duty*.

**Strategist Posture (Governing Mode):** The AI leads strategy as a world-class
strategist — for ANY work, regardless of user expertise. The AI decides strategy
and proposes it in a structured, falsifiable format (Rumelt kernel + intent block);
the user ratifies (accept / amend / disregard-and-decide); the user owns execution.
This is the commander's intent model (ADP 6-0): the AI sets strategic direction,
the human is the accountable commander. The AI cannot act on unratified strategy.
This includes the what-matters analysis — the principal contradiction (抓主要矛盾): what matters most against the mission, priced by cost-of-not-doing. The AI proposes it; the user ratifies (Strategist Posture).
The dedicated gate is STRATEGY.md (after LANDSCAPE); this posture is invoked
whenever the AI makes a strategic decision — including in other steps — and the
user ratifies strategy; routine execution does not require per-action ratification.

**Method Invocation Completeness (Standing Principle):** Every documented method
is either APPLIED (with an evidence artifact), SKIPPED (with a pre-authorized
catalog code from docs/SKIP_CATALOG.md), or OMITTED (audit red flag — an
uncatalogued gap). Nothing is silently omitted. The executor records every method
decision in `.omo/method-ledger.jsonl` and runs a written self-critique
reconciliation at each gate. The ledger is machine-checked at REVIEW
(conformance: fitness, skip-rate per code, divergence). This closes the
documented-vs-applied gap — the protocol's methods must be used, not merely
documented. See docs/METHOD_LEDGER.md.

**Effortlessness (Standing Principle):** The protocol exists to make the user's
work effortless — durability-weighted: the least user effort that still produces a
durable, world-class outcome (durability FIRST, speed SECOND). This governs BOTH how
the protocol runs (the default execution path is the minimum-effort path consistent
with quality; ceremony is a tax, not a virtue) AND how the protocol grows (every
addition must justify itself in net user effort saved; an addition that costs more
effort than it removes is 画蛇添足 and is rejected unless its net benefit is
demonstrated at scale). The user is the authority on their own felt effort; the AI
proposes net-effort analyses; the user ratifies. Never removes the user's growth
work — growth lives in the ZPD, scaffolding fades (Dual ZPD).

> Effortlessness is Consolidation's counterweight — it learns from the ecosystem, but
> every learned addition must pass the net-effort test.

**Actionability (Standing Principle):** The protocol is action-biased — its purpose is
to ship. Information is always incomplete; the default is to move to a shipping
decision at ~80% sufficiency rather than wait for perfect information (which never
arrives). The bar is durability-weighted: ship the smallest version that still holds
its ground (durability FIRST, speed SECOND). One-way doors and reputation-critical
decisions are exempt — those demand deliberate information gathering. The user is the
authority on "enough" — choosing sufficiency is growth work (Dual ZPD); the AI
proposes the sufficiency analysis, the user ratifies (Strategist Posture).

> Actionability is Effortlessness' counterpart — Effortlessness removes friction (cost
> side), Actionability removes deferral (time side); the default path is the
> minimum-effort path that ships.

---

## Composability

The protocol can be composed in three ways beyond the default pipeline:

### Module Mode

Each step documents its entry condition in its file. Run any step standalone by starting
from the condition. This enables skipping steps that are already resolved.

| Combination | When to use |
|---|---|
| EXTRACTION → SERIOUSNESS | Quick-evaluate an idea without full protocol |
| VALIDATION → SPECIFICATION → EXECUTOR | Build a researched project without re-extracting |
| LANDSCAPE → REVIEW | Audit an existing solution's research quality |

### Pipeline Parallelism

| Group | Steps | Constraint |
|---|---|---|
| Validation | MULTI + FUNDAMENTALS | Both depend on SERIOUSNESS; no cross-dependency |
| Spikes | Multiple VALIDATION spikes in parallel | Independent hypotheses, each isolated |
| Wrap | EXPLAINER + REVIEW (via SPEC_SYNC merged into REVIEW) | All depend on EXECUTOR; no cross-dependency |

### External Methodology Composition

| Methodology | Compose By |
|---|---|
| **Shape Up** | Replace AMBITION + PACING with Shape Up pitching. DECOMPOSITION + LANDSCAPE + VALIDATION serve as shaping. |
| **Design Sprint** | Run VALIDATION as a full Design Sprint week. EXTRACTION → DECOMPOSITION → AMBITION feeds the sprint brief. |
| **Lean Startup** | VALIDATION as Build-Measure-Learn loop. SPECIFICATION records pivot-or-persevere. |
| **Scrum** | EXECUTOR milestones as sprints. REVIEW as sprint review. REFLECT as retrospective. |

### What Does NOT Compose

The pipeline's **core interdependence chain** — `EXTRACTION → FUNDAMENTALS → DECOMPOSITION
→ AMBITION` — must run in order. Skipping any of these produces an ambition without a
validated foundation. Always run this chain before entering execution.

---

## Contents
| File | Purpose |
|---|---|
| `INBOX.md` | Step -1: Thought capture, clustering, triage. Select one cluster for EXTRACTION, park rest. |
| `PRIORITIZE.md` | Step 0.5: Idea comparison & betting — 4-dimension scoring (incl. Matters), optional. Runs between INBOX and EXTRACTION. |
| `EXTRACTION.md` | Gate 0 - Extract X (real problem) from Y (stated solution), 10 proven techniques |
| `SERIOUSNESS.md` | Phase 1: Commitment Probe. Phase 2: Dimension Scoring. Phase 3: Kill Criteria. |
| `FUNDAMENTALS.md` | Identify one-way door decisions, validate with minimum prototype, detect LLM bias |
| `MULTI.md` | Multidisciplinary probe: Domain Relevance Filter, Quick Probes, Flag |
| `DECOMPOSITION.md` | Intent decomposition - Cynefin classify, MECE tree, KNOW/RESEARCH/PROTOTYPE routing |
| `AMBITION.md` | Gate 1 - Research-interleaved dialogue to clarify intent |
| `LANDSCAPE.md` | Research protocol - map what exists |
| `STRATEGY.md` | Strategic ratification gate — AI kernel proposal, human ratify, premortem (commander's intent) |
| `PACING.md` | Phase budget allocation, tracking, adjustment, session boundaries, human energy rules |
| `VALIDATION.md` | Gate 2 - rapid prototyping with KILL/PIVOT/COMMIT |
| `SPECIFICATION.md` | Locked plan-IS-spec template (16 sections §0-15) |
| `EXECUTOR.md` (incl. POLISH) | AI execution handoff with autonomy levels + human final pass post-execution |
| `EXPLAINER.md` | AI-generated code explainer for non-coder (docs/EXPLAINER.md) |
| `REVIEW.md` (incl. SPEC_SYNC) | **Meta-review gate** — independent agent audits protocol compliance + spec-to-code fidelity |
| `REFLECT.md` | **Protocol retrospective** — how well did the protocol guide us? 7 questions, 35 min |
| `RULES.md` | Project Bootstrap Protocol — routing decision tree, constitution, phase definitions, test philosophy, stop rules |
| `STANDARDS.md` | Production standards for AI-generated projects — quality tiers (T1/T2/T3) and per-tier rules |
| `.omo/reviews/` | Review findings archive — one file per review run |
| `docs/engineering-plugin.md` | Engineering Plugin — CI, Operations, Production Quality addendum for technical deliverables |
| `docs/METHOD_LEDGER.md` | Method Invocation Completeness — the .omo/method-ledger.jsonl schema, 3 states, conformance check (Standing Artifact) |
| `docs/SKIP_CATALOG.md` | Pre-authorized skip conditions per method (MEL pattern) + deviation template (Standing Artifact) |
| `docs/` | PROTOCOL_MODEL, METHOD_LEDGER, UNIVERSAL_FUNDAMENTALS, EXPLAINER, SKIP_CATALOG, SPEC_SYNC, standards/, adr/, research/ |
| `cli/` | project-kit — optional Rust CLI: bootstrap projects with RULES.md governance (`init` / `phase` / `check` / `publish`) |

## Origin

Generated by following the Development Protocol (v3.0.0) through its PREP PHASE on itself — producing a refined version that centers the prototyping gate as the key innovation. July 2026.

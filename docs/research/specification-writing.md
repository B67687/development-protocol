# Research: Specification Writing for AI-Executable Projects

**Date**: 2026-07-11
**Purpose**: Inform the rewrite of `SPECIFICATION.md` from a filled-out template into a structured, AI-executable specification format backed by research and industry practice.
**Context**: The Development Protocol's EXECUTION PHASE (SPECIFICATION.md → EXECUTOR.md → RULES.md). SPECIFICATION.md currently has a 7-section template with placeholders and a "meta" section. It needs to become the authoritative artifact that an AI executor reads and executes autonomously — the "spec IS the plan" philosophy requires that the spec be precise enough that the executor doesn't need to guess.

---

## Summary

Spec-driven development (SDD) has emerged in 2025–2026 as the dominant methodology for AI-assisted coding, driven by the failure mode of "vibe coding" — agents producing plausible code that drifts from intent, hallucinates APIs, and decays as projects scale. GitHub Spec Kit (111k+ stars), AWS Kiro, Claude Code SDD skills, and Cursor Plan Mode all converge on a four-phase workflow: **Specify → Plan → Tasks → Implement**, where the specification is versioned, structured, and executable — meaning an AI agent can read it, derive a plan, decompose into tasks, write code, and verify against acceptance criteria.

The research converges on a critical insight: **the spec is the prompt.** A precise spec gives AI agents the constraints they need to ship working code without drift. GitHub reports ~10× fewer "regenerate from scratch" cycles with spec-driven workflows. AWS Kiro documents 40-hour features shipped in under 8 human hours when authored as specs first.

For the Development Protocol's SPECIFICATION.md, the key findings are:

1. **The current 7-section template is close to the industry standard but needs structural tightening** — it needs explicit acceptance criteria, EARS-notation requirements, out-of-scope boundaries, and a "constitution" section.
2. **The "Meta" section should be elevated into a structural element** — it describes how to write the spec, which should be embedded as rules, not appended as guidance.
3. **The spec must be locked after execution begins** — the current RULES.md scope warp rules handle this, but SPECIFICATION.md should declare its own immutability contract.
4. **Decision capture needs to be explicit** — architecture decisions must include "why" rationale so the executor doesn't accidentally revert them. Y-Statement format is the right pattern.
5. **EARS notation should be used for requirements** — the five patterns (ubiquitous, event-driven, state-driven, unwanted behavior, optional features) collapse to testable claims that agents can verify.

---

## Sources Surveyed

### Primary Sources

- **Source**: BCMS — "Spec-Driven Development (SDD): The Definitive 2026 Guide" (May 2026)
  **Key insight**: Defines SDD as "specification as artifact, code as build output." Documents ~3–10× higher first-pass success rates from AI agents. Introduces EARS notation as the standard for AI-readable requirements — five patterns (ubiquitous, event-driven, state-driven, unwanted behavior, optional features) that collapse to testable claims.
  **Application to SPECIFICATION.md**: The 4-phase SDD loop (Specify → Plan → Tasks → Implement) maps directly to the current pipeline. SPECIFICATION.md should adopt EARS notation for its requirements, replacing free-form prose with structured patterns.

- **Source**: Augment Code — "What Is Spec-Driven Development?" (Apr 2026)
  **Key insight**: Defines the six elements a spec must answer: outcomes, scope boundaries, constraints, prior decisions, task breakdown, verification criteria. "Leave any of them open and the agent will answer them for you, in ways you won't like." Introduces the adversarial agent pattern (Coordinator → Implementor → Verifier).
  **Application to SPECIFICATION.md**: The current 7-section template covers most of these six elements but lacks explicit verification criteria and prior decisions. Sections 2 (Architecture) should include Y-Statements. Sections 4 (CI) and 6 (UX) should include acceptance criteria written in EARS.

- **Source**: arXiv 2602.00180 — "Spec-Driven Development: From Code to Contract in the Age of AI Coding Assistants" (Feb 2026)
  **Key insight**: Defines three levels of SDD rigor — spec-first (guides initial build, may rot), spec-anchored (maintained alongside code, tests enforce alignment), spec-as-source (spec IS the code, never manually edit generated code). Provides a decision framework for when each applies.
  **Application to SPECIFICATION.md**: The Development Protocol should target **spec-anchored** as the default — the spec is maintained alongside code, and validation happens through acceptance criteria. Spec-as-source is aspirational but requires tooling maturity the protocol doesn't yet have.

- **Source**: GitHub Spec Kit — github/spec-kit (2025–2026, 111k+ stars)
  **Key insight**: The reference SDD implementation. Four-phase workflow with slash commands: `/constitution`, `/specify`, `/clarify`, `/plan`, `/tasks`, `/implement`, `/checklist`. The "constitution" file is a list of ubiquitous EARS statements about the project itself. Works with 30+ AI agents (Claude Code, Copilot, Cursor, Codex CLI, Gemini CLI, opencode, etc.).
  **Application to SPECIFICATION.md**: SPECIFICATION.md should include a Constitution section (currently in RULES.md section 2) that defines project-wide immutable rules. The slash command pattern suggests the spec should be consumable by agents as structured context, not just a document to read.

- **Source**: Shape Up — Ryan Singer, Basecamp (2019)
  **Key insight**: Shaping produces work that is **rough** (not over-detailed), **solved** (the macro solution is clear), and **bounded** (the appetite tells the team where to stop). The sweet spot: "not too vague and not too concrete." Wireframes are too concrete; words are too abstract. The Dot Grid Calendar case study shows the right level of fidelity — a rough sketch that leaves room for execution-level decisions while constraining scope.
  **Application to SPECIFICATION.md**: The spec should be more detailed than a Shape Up pitch (since an AI executor needs more precision than a human team) but should preserve the "rough, solved, bounded" philosophy. Section 2 (Architecture) should be solved at the macro level. Section 3 (File Tree) should be concrete. Section 7 (Timeline) should be bounded by appetite.

- **Source**: ADR Operational Guide — hidekazu-konishi.com (May 2026)
  **Key insight**: ADR formats compared — Nygard (Status/Context/Decision/Consequences, immutable after acceptance), MADR (Markdown, updatable but convention says keep immutable), Y-Statement (lightweight, for small decisions). Seven failure modes of ADR adoption. Key operational rule: "Store ADRs in the same repo as code, not in Confluence."
  **Application to SPECIFICATION.md**: Section 2 (Architecture and Design Decisions) should adopt the Y-Statement format for decision capture: "In the context of [situation], facing [concern], we decided for [option] to achieve [goal], accepting [downside]." This prevents the executor from reverting decisions by preserving rationale.

- **Source**: Parnas, David L. — "On the Criteria To Be Used in Decomposing Systems into Modules" (1972)
  **Key insight**: Systems should be decomposed so that each module hides a design decision that is likely to change. The module's interface IS its specification — internal details are hidden. Specs define what, not how. This is the foundation of information hiding as a decomposition criterion.
  **Application to SPECIFICATION.md**: Section 3 (File Tree and Module Responsibilities) should define each module by its interface and the design decision it hides, not by its internal implementation. "Module X hides decision Y" is a stronger spec than "Module X does Z."

- **Source**: Jackson, Michael — Problem Frames (2000)
  **Key insight**: Software problems decompose by frame type — required behavior, commanded behavior, information display, workpieces, transformation, connection. Each frame has a characteristic structure and decomposition pattern. The problem frame determines what the spec needs to describe.
  **Application to SPECIFICATION.md**: The spec template should include a problem frame classifier (Section 1) that colors how the remaining sections are filled. A "display information" project needs heavy UX spec; a "required behavior" project needs heavy interface/contract spec.

- **Source**: Cone of Uncertainty — Steve McConnell / Barry Boehm (1997/1981)
  **Key insight**: Early in a project, estimates can vary by 16x (4x high, 0.25x low). This narrows as decisions are made. Uncertainty doesn't shrink passively — the project must actively drive it out through research, prototyping, and decisions. Specs should match the current cone position: broad at the start, narrow at execution time.
  **Application to SPECIFICATION.md**: By the time SPECIFICATION.md is written (after VALIDATION.md's COMMIT), uncertainty should be at the "Requirements Complete" level (~±25%). The spec should reflect this — decisions are locked, tradeoffs are resolved, and the executor operates within known bounds.

- **Source**: Design by Contract — Bertrand Meyer (1986+)
  **Key insight**: Software components should be specified as contracts with preconditions, postconditions, and invariants. The contract defines mutual obligations between supplier and client. This makes correctness verifiable — you can check at runtime whether the contract is honored.
  **Application to SPECIFICATION.md**: The spec's interface contract (Section 6) should use contract language: "Before calling X, the caller must ensure Y (precondition). After X returns, Z is guaranteed (postcondition)." This is especially important for AI executors that need to know the boundaries of their responsibility.

- **Source**: Cursor for SDD — Augment Code (Apr 2026)
  **Key insight**: Cursor is a strong execution environment but a poor SDD system — it can consume specs but provides no spec lifecycle management, no traceability, and no enforcement. Rules files are treated as suggestions, not gating constraints. The pseudo-spec problem: teams treat .cursor/rules as specs, but rules have no audit trail, no review workflow, and no loading guarantee.
  **Application to SPECIFICATION.md**: The protocol must NOT rely on "rules" systems that are advisory. SPECIFICATION.md must be a self-contained artifact that the executor reads at session start and checks against at every action. The EXECUTOR.md handoff protocol (section-by-section execution with checkpoint commits) handles this.

- **Source**: EARS Notation — Alistair Mavin, Rolls-Royce (2009)
  **Key insight**: Five requirement patterns that eliminate ambiguity: (1) Ubiquitous — always true, no keyword needed. (2) Event-driven — "WHEN trigger THEN system SHALL response." (3) State-driven — "WHILE state THEN system SHALL behavior." (4) Unwanted behavior — "IF condition THEN system SHALL response." (5) Optional features — "WHERE feature THEN system SHALL behavior." Used by Airbus, Bosch, Dyson, Honeywell, Intel, NASA, Rolls-Royce, Siemens.
  **Application to SPECIFICATION.md**: All acceptance criteria in the spec should use EARS notation. This makes them unambiguous enough for an AI to parse, generate code from, and verify against. The EARS patterns directly map to test cases.

---

## Key Patterns

### 1. The Spec-as-Source Paradigm Shift

The dominant pattern emerging from all 2025–2026 SDD literature is that **the specification is the primary artifact — code is a regenerable build output.** This inverts decades of "code first, document later" practice. The arXiv paper defines this as a spectrum:

```
Code-First → Spec-First → Spec-Anchored → Spec-as-Source
```

Each step increases specification authority over code. For the Development Protocol, the target is **spec-anchored** — the spec is maintained alongside code, tests enforce alignment, and changes flow through the spec first.

GitHub Spec Kit operationalizes this with a constitution file (project-wide immutable rules), spec.md (what/why), plan.md (how), and tasks.md (in what order). The constitution is particularly important for preventing silent drift — it's a list of "shall" statements that constrain every agent action.

### 2. The Right Level of Abstraction

Shape Up's "rough, solved, bounded" framework is the most actionable guide to spec granularity:

- **Rough**: The spec leaves room for execution-level decisions. It doesn't specify every parameter name or function signature — those are for the plan and task phases.
- **Solved**: The macro-level solution is clear. The architecture is decided. The tradeoffs are resolved. No "we'll figure that out during implementation."
- **Bounded**: The scope has explicit boundaries — what's in, what's out, what's definitively never going to happen. The executor knows where to stop.

The SDD workflow adds a layer on top: the spec is rough-but-solved, the plan adds technical detail, and the tasks add implementation-level specification. Each phase adds precision.

### 3. Decision Preservation is Critical

Without explicit rationale, AI executors will revert decisions they don't understand. The Y-Statement format is the most concise way to capture decisions:

> "In the context of [situation], facing [concern], we decided for [option] to achieve [goal], accepting [downside]."

This is better than a plain ADR because:
- It captures **why** the decision was made
- It captures **what was rejected** (the other options)
- It captures **tradeoffs accepted** (the downside)
- It's short enough to embed in a spec section

### 4. Acceptance Criteria Must Be Parseable

Free-form prose acceptance criteria ("the system should handle errors gracefully") are useless for AI execution. EARS notation transforms them:

- ❌ "The system should handle login failures" — ambiguous, unverifiable
- ✅ "WHEN credential validation fails three times in 60 seconds THEN the system SHALL lock the account for 15 minutes" — unambiguous, testable

The EARS patterns map directly to test cases. An AI can read an EARS requirement, generate code, and write a verification test from the same statement.

### 5. The Constitution Constrains the Executor

Both GitHub Spec Kit and BMAD use a "constitution" — a list of project-wide rules that every agent action must respect. The RULES.md already has this concept (section 2: Constitution). SPECIFICATION.md should reference it and may add project-specific constitutional rules.

The constitution prevents FP-024 (Confident Wrongness) — code that compiles, runs, and is subtly incorrect. When the constitution includes rules like "The system shall use TypeScript strict mode" or "The system shall avoid runtime dependencies on unmaintained packages," the executor has explicit boundaries.

### 6. Spec-to-Task Decomposition Follows a Pattern

Every SDD system converges on the same decomposition pattern:
1. The spec defines **what and why** (user stories, acceptance criteria, constraints)
2. The plan defines **how** (architecture, data model, API contracts, library choices)
3. Tasks define **in what order** (atomic units, each with inputs/outputs/acceptance checks)
4. Implementation executes tasks sequentially, verifying against acceptance criteria

This maps cleanly to the Development Protocol: SPECIFICATION.md = spec + plan, EXECUTOR.md = task decomposition + execution protocol, RULES.md = constraints + verification gates.

### 7. The Spec Must Be Self-Verifying

A spec that an AI executor can't verify against is just documentation. The spec must include:
- Acceptance criteria in testable form (EARS)
- Quality gates (compile, test, lint)
- Architecture invariants (constitution rules)
- Verification steps ("to verify this section is complete, run X and check Y")

This aligns with the EXECUTOR.md checkpoint protocol — after each section, verify before proceeding.

---

## What Makes a Spec Executable (Not Just Readable)

An AI-executable spec differs from a human-readable spec in six critical ways:

### 1. Deterministic Parseability
Human-readable specs use narrative prose that assumes shared context. AI-executable specs use structured formats that eliminate ambiguity. Key requirements:
- **Fixed sections** — the executor knows where to look for each type of information
- **Consistent syntax** — within each section, use the same syntax conventions (EARS for requirements, Y-Statement for decisions, Markdown code blocks for file trees)
- **Explicit boundaries** — every "what" must be paired with "what not" (in-scope AND out-of-scope)

### 2. Verifiable Claims
Every statement in the spec should be either:
- **Testable** — "The system shall respond to GET /health with 200 OK within 500ms" (can be tested)
- **Constraining** — "The system shall use TypeScript strict mode" (can be checked by the linter)
- **Informational** — "The database is PostgreSQL 16" (constrains choices but isn't tested per se)

Statements that are none of these — "the system should be user-friendly" — are noise and should be removed or converted.

### 3. Decision Traceability
Every architecture decision must include:
- What was decided
- Why it was chosen (the rationale)
- What alternatives were rejected
- What tradeoff was accepted

Without this, the executor cannot distinguish between "a deliberate design decision" and "an arbitrary choice that can be changed." The Y-Statement format handles all four.

### 4. Contract Boundaries
Each module or component must have explicit contracts:
- **Preconditions**: What must be true before calling this
- **Postconditions**: What is guaranteed after
- **Invariants**: What is always true

This is Bertrand Meyer's Design by Contract applied at the architecture level. AI executors need these boundaries to know what they're responsible for and what they can assume from other modules.

### 5. Scope Guardrails
The most important feature of an AI-executable spec is **what it says no to**. The out-of-scope list is more important than the in-scope list because AI agents naturally expand scope. Explicit "no" statements prevent FP-001 (Feature Creep) and FP-004 (Scope Warp Cascade).

### 6. Verification Protocol
The spec must tell the executor how to verify it's done. This means:
- Per-section acceptance criteria
- Per-section quality gates (compile, test, lint commands)
- A checkpoint protocol (commit after each section)

The EXECUTOR.md already defines this, but SPECIFICATION.md should include the verification commands directly (currently in section 4).

---

## Failure Modes of Specs

### FM-1: Vague Abstraction (Shape Up's "Words Are Too Abstract")
"Build a calendar view" or "add login" — these are not specs. They leave the executor to guess:
- What kind of calendar? Monthly? Weekly? Dot grid?
- What kind of login? Email/password? OAuth? SSO? MFA?
- What about error states? Recovery paths? Edge cases?

**Evidence**: The Shape Up team found that projects defined in a few words inevitably "grow out of control because there's no boundary to define what's out of scope." The BCMS SDD guide confirms: "A prompt like 'add login' is wildly underspecified. The model picks reasonable defaults — and those defaults rarely match what the team actually wanted."

**Fix**: Every section must include both "what" and "what not." Use EARS notation for requirements. Include explicit edge case handling.

### FM-2: False Precision (Shape Up's "Wireframes Are Too Concrete")
Over-specifying creates estimation errors because "making the interface just so can require solving hidden complexities and implementation details that weren't visible in the mockup." When the spec is too detailed, the executor overfits to the details and misses the intent.

**Evidence**: Shape Up's Dot Grid Calendar — the rough sketch left room for designers and programmers to apply their own judgment and "discover all the real trade-offs that emerge." The SDD arXiv paper warns that spec-as-source can lead to a "bias toward heavy up-front specification and big-bang releases."

**Fix**: The spec should be concrete about architecture and scope but rough about implementation details. Section 3 (File Tree) should be concrete. Section 7 (Timeline) should be bounded but approximate. Let the plan and task phases add precision.

### FM-3: Intent Drift via Context Decay
As the executor works, context fills. Older decisions get forgotten. The executor silently contradicts earlier spec requirements.

**Evidence**: The Cursor SDD analysis found that "agent drift accumulates silently" — agents "take the shortest path to 'done,' skipping the steps experienced engineers rely on." The FP-014 (Context Decay) pattern in RULES.md documents this.

**Fix**: SPECIFICATION.md must be re-read at every session start (EXECUTOR.md already mandates this). The constitution must be checked before every significant action. Checkpoint commits serve as context anchors.

### FM-4: The Pseudo-Spec Problem
Treating advisory files (templates, rules, guidelines) as specifications. The executor treats them as suggestions, not constraints.

**Evidence**: Cursor's rules system treats `alwaysApply: true` rules as "advisory text" — the model treats them as suggestions rather than gating constraints. A regression in v3.0.16 silently downgraded all rules to "requestable."

**Fix**: SPECIFICATION.md must be a self-contained authoritative artifact. It is not a template or a set of suggestions — it is the source of truth. The executor does not proceed past a section until its acceptance criteria are met.

### FM-5: Decision Documentation Theater
Writing ADRs (or spec sections) without maintaining them. The collection becomes noise and the team stops trusting it.

**Evidence**: The ADR operational guide identifies three patterns: format chosen but no operating model, ADRs stored where people don't look, and ADRs describing trivial or cosmic decisions but not load-bearing ones.

**Fix**: SPECIFICATION.md is locked after execution begins. It doesn't need ongoing maintenance because changes go through scope warp rules. The decision documentation lives in the spec's section 2, not in a separate ADR directory that will rot.

### FM-6: Contradictory Requirements
The spec says one thing in section 1 and another in section 5. The executor must choose which to follow.

**Evidence**: The IEEE Computer Society identifies "resolving conflicts" as a core requirements analysis step. Contradictions are common when multiple stakeholders contribute to a spec.

**Fix**: Before locking the spec, run a consistency check across all sections. If section 5 says "PostgreSQL" and section 2 says "SQLite," resolve the conflict before execution. The EXECUTOR.md error handling protocol catches contradictions during implementation — but it's better to catch them before.

### FM-7: Unverifiable Claims
"Fast," "robust," "user-friendly," "scalable" — these sound meaningful but cannot be verified by an executor. Without a testable definition, the executor's interpretation is as good as any other.

**Evidence**: The SDD literature consistently identifies "unverifiable output" as a core failure mode. Without explicit acceptance criteria, "there's no way to know whether the agent's code is 'right.'"

**Fix**: Replace all vague adjectives with EARS-structured acceptance criteria. "The system shall be fast" → "The system SHALL respond to API requests within 200ms at the 95th percentile under 1000 concurrent users."

---

## Level of Detail Guidance

### How Much Detail Is Enough?

The research converges on a simple rule: **The spec needs just enough detail that the executor doesn't guess, and no more.**

Too vague → executor makes wrong assumptions (FM-1)
Too specific → executor overfits to details, misses intent (FM-2)
Just right → executor makes correct assumptions within known boundaries

### The Decision Table

| Spec Element | Minimum Detail | Maximum Detail | Guidance |
|---|---|---|---|
| **Outcomes** | "User can X" | EARS acceptance criteria | Use EARS. Outcomes without acceptance criteria are wishes. |
| **Scope** | Bullet list of in-scope + out-of-scope | Per-feature boundary descriptions | Out-of-scope list is MORE important than in-scope. |
| **Architecture** | Pattern name + rationale | Y-Statement for each decision | Name the pattern ("Event-driven microservices"), state the tradeoff. Don't diagram every interaction. |
| **File Tree** | File path + one-line responsibility | Per-file interface summary | The module's INTERFACE not its internals. "hides decision X" |
| **CI/Gates** | Command to run + expected output | Per-gate acceptance criteria | Executable commands only. "cargo test" not "tests should pass." |
| **Dependencies** | Name + version + purpose | Version constraint + license | No "should use" — either it's a dependency or it's not. |
| **UX/Interface** | Entry points + error contract | EARS per interaction | Focus on what the USER sees and the ERROR responses. |
| **Timeline** | Milestones + checkpoints | ±20% range (Cone of Uncertainty) | Appetite-based (Shape Up), not estimate-based. |

### Progressive Detailing

The spec does NOT need every detail upfront. The cone of uncertainty principle suggests:

1. **At SPECIFICATION.md lock time**: Architecture decisions, scope boundaries, acceptance criteria, file tree, dependencies — these must be complete.
2. **At PLAN time (via EXECUTOR.md)**: Data model specifics, API contracts, test strategy — the executor derives these from the spec.
3. **At TASK time**: Function signatures, parameter names, implementation details — the executor decides these within spec bounds.

The spec is locked at the macro level. The executor fills in the micro level within spec boundaries.

### The Shape Up Sweet Spot

The Dot Grid Calendar is the canonical example:
- **Rough**: The sketch showed two-month grids with dots — no pixel-perfect mockup
- **Solved**: The macro solution was clear — read-only grid, dots for events, list below, no drag-and-drop
- **Bounded**: Explicit "what not" — no drag, no multi-day span, no color coding, no categories

**Application to the protocol**: Section 2 (Architecture) should be solved — the two-gate pipeline is the macro solution. Section 3 (File Tree) should be concrete — each file's responsibility is specified. Section 7 (Timeline) should be bounded — appetites set, not estimates.

---

## Recommendations for SPECIFICATION.md

### The Big Changes

**Change 1: Rename to emphasize the spec-as-source philosophy.**

The current subtitle "The Plan IS the Spec" is good. Keep it. But the meta section at the bottom should be elevated into a structured section. The 7 numbered sections are the right structure — they just need content upgrades.

**Change 2: Add a "Constitution" section (or reference RULES.md section 2).**

Every SDD system has a constitution. SPECIFICATION.md should include a project-specific constitution that constrains the executor. Example from the current RULES.md default:
```
1. Correctness — wrong output at any speed is useless
2. No magic — explicit > implicit. Every dependency and config declared.
3. Inward dependencies — core knows nothing about edges.
4. Test what matters — one behavior per test, edge cases before happy path.
5. Fail with context — every error includes the values that caused it.
```

The constitution should be repeated in SPECIFICATION.md (not just referenced) to keep the spec self-contained.

**Change 3: Section 2 should use Y-Statement format for every architecture decision.**

Current: "Pattern: Two-gate pipeline with document-driven decision points. Tradeoff resolved: Up-front planning vs. build-to-discover."

Better (Y-Statement):
```
In the context of high-ambition projects where goalpost shift is the primary failure mode,
facing the tradeoff between up-front planning (which fails because problems reveal themselves through building)
and pure build-to-discover (which never locks scope),
we decided for a two-gate pipeline with a prototyping gate as the central decision mechanism
to achieve a middle path: learn what only building can teach, but in a timeboxed throwaway context,
accepting that this adds process overhead and requires discipline to treat prototypes as throwaway.
```

This prevents the executor from reverting to a simpler approach ("why don't we just build it directly?") because the tradeoff is explicit.

**Change 4: All acceptance criteria should use EARS notation.**

Current section 4: "Build: N/A — Markdown-only project. Test: markdown-link-check on all .md files. Lint: mdformat or prettier for Markdown formatting."

Better:
```
WHEN a pull request is opened
THEN CI SHALL run `markdown-link-check` on all .md files
WHERE any broken link is detected
THEN CI SHALL fail with exit code 1

WHEN a pull request is opened
THEN CI SHALL run `mdformat --check` on all .md files
WHERE formatting violations are detected
THEN CI SHALL fail with the list of violations
```

**Change 5: Section 3 should specify interface contracts, not just file names.**

Current: "AMBITION.md — Gate 1: Intent clarification dialogue"
Better: "AMBITION.md — Gate 1: Intent clarification dialogue. HIDES the decision of how intent is clarified. EXPORTS a falsifiable hypothesis statement. CALLER: any file that needs a validated goal."

This uses Parnas's information hiding principle — each module's spec describes what it hides and what it exports, not how it works internally.

**Change 6: Add a "What's NOT in Scope" section.**

The out-of-scope list is critical. Currently absent from SPECIFICATION.md (it lives in RULES.md section 4). The spec should have its own section that the executor reads before starting:

```
### OUT OF SCOPE (V1)
- Multi-user support — deferred to V2
- Mobile app — never planned
- Third-party integrations — never planned
```

This prevents FP-001 (Feature Creep) because the executor has explicit guardrails.

**Change 7: Add a verification checklist at the end.**

Each section should end with verification steps:

```
### Verify Section 1
- [ ] Project name matches README.md
- [ ] Core ambition is a single falsifiable statement
- [ ] Success criteria are measurable
- [ ] "Why now" includes external context (not just internal desire)
```

This makes the spec self-verifying and aligns with EXECUTOR.md's checkpoint protocol.

### The Revised 7-Section Template (with guidance)

```
# SPECIFICATION.md — The Plan IS the Spec

> Everything in one document: constitution, architecture, files, CI, UX, dependencies, timeline.
> An AI executor reads this and knows exactly what to build — no guessing required.

---

## 0. Constitution (Immutable Project Rules)

The constitution constrains ALL executor actions. If an action would violate these rules, the executor MUST refuse.

```
[Project Name] Constitution:
1. [Principle 1 — e.g., "Parse, don't validate — make illegal states unrepresentable"]
2. [Principle 2 — e.g., "Core has no knowledge of edges (inward dependencies)"]
3. [Principle 3 — e.g., "Every public function has a doc comment and a test"]
4. [Principle 4 — e.g., "Fail fast with clear error messages — no silent recovery"]
5. [Principle 5 — e.g., "No new runtime dependency without a Y-Statement decision"]
```

**Default Constitution** (copy from RULES.md):
1. Correctness — wrong output at any speed is useless
2. No magic — explicit > implicit. Every dependency and config declared.
3. Inward dependencies — core knows nothing about edges.
4. Test what matters — one behavior per test, edge cases before happy path.
5. Fail with context — every error includes the values that caused it, not just a message.

---

## 1. Overview and Derived Ambition

**Falsifiable hypothesis**: One sentence describing what success looks like.
**Success criteria**: 3–5 measurable criteria (EARS Ubiquitous pattern).
**Why now**: External context — what changed to make this worth building?
**What's NOT in scope (V1)**: Explicit list of things the executor will NOT build.

```
Project name: {{project-name}}
One-line: {{one-line description}}
Core ambition: {{single falsifiable statement}}
Why now: {{what changed — market, technology, personal context}}

Success criteria:
- WHEN the project is complete THEN {{measurable outcome 1}}
- WHEN the project is complete THEN {{measurable outcome 2}}
- WHEN the project is complete THEN {{measurable outcome 3}}

OUT OF SCOPE (V1):
- {{feature}} — deferred to V2
- {{feature}} — never planned
```

---

## 2. Architecture and Design Decisions

Each decision uses the Y-Statement format:
> In the context of {{situation}}, facing {{concern}}, we decided for {{option}} to achieve {{goal}}, accepting {{downside}}.

```
### Decision 1: {{pattern name}}
In the context of {{what makes this decision necessary}},
facing {{the tradeoff or constraint that forces a choice}},
we decided for {{the chosen approach}}
to achieve {{the specific benefit this decision enables}},
accepting {{the cost, complexity, or limitation this imposes}}.

Alternatives considered:
- {{alternative 1}} — rejected because {{reason}}
- {{alternative 2}} — rejected because {{reason}}
```

Pattern: {{architecture pattern name}}
Tradeoff resolved: {{what tension this resolves}}
Rationale: {{why this is the right choice for this project}}

---

## 3. File Tree and Module Responsibilities

Each module specified by its INTERFACE (not its internals). Format:
`path/to/file.ext` — {{what it does}}. HIDES {{design decision}}. EXPORTS {{interface}}. CALLER: {{consumer}}.

```
{{path}} — {{one-line responsibility}}
  HIDES: {{design decision this module encapsulates}}
  EXPORTS: {{public interface — what other modules can call}}
  CALLER: {{which other modules depend on this}}

{{path}} — {{one-line responsibility}}
  HIDES: {{design decision this module encapsulates}}
  EXPORTS: {{public interface}}
  CALLER: {{which other modules depend on this}}
```

---

## 4. CI, Tooling, and Quality Gates

Acceptance criteria in EARS notation. Each gate is a single executable command with expected output.

```
WHEN a pull request is opened
THEN CI SHALL run {{build command}}
WHERE compilation fails
THEN CI SHALL fail with exit code 1 and the compiler output

WHEN a pull request is opened
THEN CI SHALL run {{lint command}}
WHERE lint violations are detected
THEN CI SHALL fail with the list of violations

Quality gates:
1. {{command}} → {{expected result}} — {{what this verifies}}
2. {{command}} → {{expected result}} — {{what this verifies}}

Release: {{versioning scheme}} — {{how releases work}}
```

---

## 5. Dependencies and External Contracts

Each dependency listed with purpose, version, and contract. No "should use" — either it's declared or the executor doesn't add it.

```
### Runtime Dependencies
| Package | Version | Purpose | Contract |
|---------|---------|---------|----------|
| {{name}} | {{version}} | {{why this exists}} | {{what it provides}} |

### Dev Dependencies
| Package | Version | Purpose |
|---------|---------|---------|
| {{name}} | {{version}} | {{why needed}} |

### External APIs
| API | Purpose | Auth | SLA |
|-----|---------|------|-----|
| {{name}} | {{what it provides}} | {{auth mechanism}} | {{uptime/latency}} |
```

---

## 6. UX and Interface Contract

Entry points, error contract, and user-facing behavior in EARS notation. Design by Contract style for API boundaries.

```
### Entry Points
- {{entry point}} — {{what it does}} ({{who uses it}})

### User-Facing Behavior (EARS)
WHEN {{user action}}
THEN the system SHALL {{system response}}
WHERE {{error condition}}
THEN the system SHALL {{error response}}

### Error Contract
| Condition | Error | Handling |
|-----------|-------|----------|
| {{what goes wrong}} | {{error code/message}} | {{how the system responds}} |

### Interface Contract (Design by Contract)
Module: {{name}}
  Precondition: {{what must be true before calling}}
  Postcondition: {{what is guaranteed after calling}}
  Invariant: {{what is always true}}
```

---

## 7. Timeline, Milestones, and Checkpoints

Appetite-based (Shape Up), not estimate-based. Each milestone has acceptance criteria.

```
Appetite: {{time available, not time estimate}}

| Milestone | What ships | Checkpoint | Acceptance Criteria |
|-----------|------------|------------|---------------------|
| M1 | {{what's done}} | {{how to verify}} | WHEN M1 is complete THEN {{measurable}} |
| M2 | {{what's done}} | {{how to verify}} | WHEN M2 is complete THEN {{measurable}} |
| M3 | {{what's done}} | {{how to verify}} | WHEN M3 is complete THEN {{measurable}} |

Circuit breaker: IF {{condition}} THEN the project SHALL {{stop/reassess}}
```

---

## Verification Checklist (Executor Reads)

Before execution begins, confirm:
- [ ] All {{placeholders}} in sections 1–7 are filled
- [ ] No "TODO" or "TBD" remains
- [ ] Out-of-scope list in section 1 is non-empty (if empty, add one — every project has no-gos)
- [ ] Each architecture decision in section 2 includes a Y-Statement
- [ ] Each CI gate in section 4 is a concrete command (not "tests should pass")
- [ ] Each dependency in section 5 has a version constraint
- [ ] Timeline in section 7 has a circuit breaker condition
- [ ] Constitution (section 0) has at least 3 principles

---

### Filled Example

[The existing ScratchPad example with updates to match the new format]
```

### What Should Change (Summary)

| Element | Current State | Recommended State |
|---------|---------------|-------------------|
| Constitution | In RULES.md only | Also in SPEC.md section 0 (self-contained) |
| Requirements | Free-form prose | EARS notation (5 patterns) |
| Architecture decisions | Informal tradeoff description | Y-Statement format per decision |
| File tree | File name + one-liner | File name + HIDES/EXPORTS/CALLER |
| CI gates | Descriptive list | EARS per gate + concrete commands |
| Dependencies | Simple list | Purpose + version + contract |
| UX/Interface | Descriptive | EARS + Design by Contract (pre/post/invariant) |
| Timeline | Milestone list | Appetite + circuit breaker + acceptance criteria |
| Out-of-scope | In RULES.md section 4 | Explicit section in spec |
| Verification | Implicit | Checklist at end of spec |
| Meta guidance | Appendix at end | Embedded in section descriptions above |

### Tradeoffs Made (This Research Document's Own Y-Statement)

> In the context of the Development Protocol's SPECIFICATION.md needing deep research backing,
> facing the risk of over-prescription (a 20-page spec template that nobody fills) vs. under-specification (a template too vague to prevent executor drift),
> we decided for a structured but minimal template (7 sections plus constitution and verification checklist)
> to achieve a spec that is complete enough for AI execution but concise enough to fill in 10–15 minutes,
> accepting that some executor decisions will still need to be made at plan/task time and that the first few projects will reveal missing sections.

---

## Origin

Created to inform the rewrite of SPECIFICATION.md from a placeholder template to a research-backed, AI-executable specification format. Synthesizes findings from spec-driven development literature (2025–2026), Shape Up methodology, Architecture Decision Records practice, problem decomposition research (Jackson, Parnas), Design by Contract (Meyer), and cone of uncertainty estimation (McConnell/Boehm).

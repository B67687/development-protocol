# RULES.md — Project Bootstrap Protocol

> A meta-protocol that adapts to ANY project type.
> Read this at the START of every AI session. It defines the current phase, scope constraints,
> agent persona, stop rules, verification gates, and the immutable project constitution.
> The AI enforces phase/scope boundaries — if asked to do something outside scope
> or current phase, it MUST refuse and explain why.
>
> **Current: PERFECT**

---

| 1. | [Project Type Routing](#1-project-type-routing)
| 2. | [Intent Decomposition (Recursive Breakdown)](#2-intent-decomposition-recursive-breakdown)
| 3. | [Constitution (Immutable)](#3-constitution-immutable)
| 4. | [Phase Definitions](#4-phase-definitions)
| 5. | [V1 Scope & Learning Shifts](#5-v1-scope--learning-shifts)
| 6. | [AI Persona & Constraints](#6-ai-persona--constraints)
| 7. | [Stop Rules](#7-stop-rules)
| 8. | [Verification Gates](#8-verification-gates)
| 9. | [Test Philosophy](#9-test-philosophy)
| 10. | [Evolution & Phase Exit](#10-evolution--phase-exit)
| 11. | [Known Failure Patterns](#11-known-failure-patterns)
| 12. | [Session Kickoff](#12-session-kickoff)

> **Not sure what to build, or have a raw intent? (prep-phase)**
> Enter the **PREP PHASE** — a structured sequence with **three decision gates**
> that prevent goalpost drift before you commit to the full build:
>
> **Gate 0 — EXTRACTION:** [EXTRACTION.md](EXTRACTION.md) — Extract X (real problem) from Y (stated solution).
> 7 proven techniques. If you can describe the problem without solution words, pass.

> **Fundamentals Check:** [FUNDAMENTALS.md](FUNDAMENTALS.md) — Identify one-way door decisions,
> validate with minimum prototype, detect LLM bias. Prevents wrong-foundation failures.

> **Gate 1 — AMBITION Dialogue:** [AMBITION.md](AMBITION.md) — Clarify your intent via Socratic
> back-and-forth until the goal is falsifiable and both parties agree.
> **Research:** [LANDSCAPE.md](LANDSCAPE.md) — Map existing solutions and identify unknowns.
> **Gate 2 — Prototyping:** [VALIDATION.md](VALIDATION.md) — Rapid throwaway prototypes, then
> KILL/PIVOT/COMMIT based on evidence, not enthusiasm.
>
> If COMMIT: [SPECIFICATION.md](SPECIFICATION.md) — Lock the plan (14 sections, frozen after this).
> [EXECUTOR.md](EXECUTOR.md) — Hand off to RULES.md for execution.
> After execution: [POLISH.md](POLISH.md) — Human final pass.
> [EXPLAINER.md](docs/EXPLAINER.md) — AI-generated architecture explainer for non-coder verification.
> [REVIEW.md](REVIEW.md) — **Meta-review gate**: independent agent audits protocol compliance.
> **Single-source-of-truth:** RULES.md always wins on conflicts between protocol documents.
> **Recursion meta-rule:** Every step in this protocol is recursive. If the output of a step is still ambiguous after one pass, apply the step again at a deeper level. Most problems resolve within 2-3 recursions. If 3 recursions still don't converge, the problem needs a different framing (not deeper iteration of the same framing).

## 1. Project Type Routing

The protocol is NOT a fixed pipeline — it's a routing system that selects the right phases for your project type. At bootstrap, run this decision tree:

```
├── NO — I'm not sure yet, or I have a raw intent
│   └── PREP PHASE — Follow the prep sequence above (EXTRACTION -> FUNDAMENTALS -> AMBITION -> LANDSCAPE -> VALIDATION -> SPECIFICATION -> EXECUTOR)
│       (Output: SPECIFICATION.md → enter routing below)
└── YES — I know what I'm building
    │
    └── What kind of project is this?
        │
        ├── I know this domain well (can spec V1 upfront)
        │   └── Route: STANDARD — Bootstrap → WORK → PERFECT → DISTRIBUTE
        │
        ├── I'm learning the domain as I build
        │   └── Route: DISCOVER-FIRST — DISCOVER → (STANDARD)
        │
        ├── It's primarily UX / interaction design (the product IS the feel)
        │   └── Route: UX-FIRST — Bootstrap → WORK → ITERATE → PERFECT → DISTRIBUTE
        │       (ITERATE replaces the linear assumption — you'll loop on UX)
        │
        ├── It's a pure research / exploration / "what if" project
        │   └── Route: EXPLORE-ONLY — DISCOVER phase only, no delivery commitment
        │
        ├── It's a port / rewrite of existing working software
        │   └── Route: PORT — Bootstrap → WORK (timeboxed, scope-locked) → PERFECT
        │       (No ITERATE needed — UX is already proven)
        │
        └── It's maintenance of an existing project
            └── Route: MAINTENANCE — PERFECT → DISTRIBUTE only (no new features)
```

### Sub-Cycle Routing (Recursive Protocol)

Every dimension in the MECE tree gets a **Level of Care** (see DECOMPOSITION.md).
If a dimension is Level 3 or higher, start a NEW protocol cycle for it:

```
NEW SESSION (fresh context)
  │
  ├── Inherits: parent project's X, one-way doors, appetite
  ├── Own scope: the sub-component only
  ├── Pipeline: AMBITION → LANDSCAPE → VALIDATION → SPECIFICATION → EXECUTOR → POLISH → EXPLAINER → SPEC_SYNC → REVIEW
  └── Output: sub-component spec + code → integrated back into parent
```

The sub-cycle is NOT a full EXTRACTION — the parent already validated the problem.
It starts at AMBITION (set appetite for the sub-component) and goes through REVIEW.
The parent project integrates the sub-component's output at the POLISH phase.

## 2. Intent Decomposition (Recursive Breakdown)

> Before ANY planning or architecture, classify and decompose the raw intent.
> See the full protocol in [DECOMPOSITION.md](DECOMPOSITION.md).

> **Summary:**
>
> 1. Cynefin classify (Clear / Complicated / Complex / Chaotic)
> 2. Recursive MECE tree (split until KNOWN / RESEARCH / PROTOTYPE)
> 3. User confirmation gate (confirm each level)
> 4. Convergence (stop when every leaf fits one session)
>    **Key rule:** If still unknown after 3 levels, it is Complex. Assign to prototype.

---

---

## 3. Constitution (Immutable)

> The Constitution is the project's immutable DNA. It is set once at bootstrap and defines
> architectural principles that govern ALL generation across ALL phases. The AI must reference
> the Constitution before every significant action. If a proposed action would violate the
> Constitution, the AI MUST refuse. (Derived from GitHub Spec Kit's Constitution concept.)

### Constitution

```
[Project Name]:

1. [Principle 1 — e.g., "Parse, don't validate — make illegal states unrepresentable"]
2. [Principle 2 — e.g., "Dependencies point inward — core has no knowledge of edges"]
3. [Principle 3 — e.g., "No dependency added without checking if existing scope covers the need"]
4. [Principle 4 — e.g., "Every public function has a doc comment and a test"]
5. [Principle 5 — e.g., "Fail fast with clear error messages — no silent recovery"]
```

**Default Constitution for most projects:**

1. **Correctness** — wrong output at any speed is useless
2. **No magic** — explicit > implicit. Every dependency and config is declared.
3. **Inward dependencies** — core knows nothing about edges.
4. **Test what matters** — one behavior per test, edge cases before happy path.
5. **Fail with context** — every error includes the values that caused it, not just a message.

### How the Constitution Works

- **Set once** at bootstrap. Changing the Constitution is a project-wide decision, not a phase decision.
- **AI reads it** before every significant action (same as stop rules).
- **If an action violates the Constitution**, the AI refuses regardless of phase.

---

## 4. Phase Definitions

Each phase is a modular building block. Use only the ones your project needs.

### DISCOVER

**Purpose:** Learn the domain. Reduce unknowns before committing to architecture.
**Hypothesis frame:** Start with a specific question: "I believe X is true about this domain. I will test it by Y." (Borrowed from OpenScience's research loop.)
**Allowed:** Reading, researching, prototyping, spike experiments.
**Not allowed:** Committed production code, infrastructure setup, polish.
**Deliverable:** A research summary with findings, rejected approaches, and a decision: proceed to WORK or pivot.
**Timebox:** Fixed (hours or days, not "until ready"). The most common failure is unbounded discovery.
**Stop when:** The remaining unknowns no longer block architecture decisions.

### WORK

**Purpose:** Build core features against a fixed V1 scope.
**Allowed:** Code, tests, minimal inline docs. No polish. No scope expansion.
**Not allowed:** README updates, badges, diagrams, publishing, refactoring existing code.
**Scope rule:** If it's not in the V1 IN SCOPE list, refuse it.
**Test rule:** Write the test BEFORE the implementation. "Red → Green → Refactor." If the AI generates implementation without a test first, that's a violation. (Derived from Superpowers: "code before tests gets deleted.")
**Quality gate:** Compiles + tests pass. Nothing more.

### ITERATE

**Purpose:** Refine UX through real-world use. The product IS the feel.
**Allowed:** UX changes, gesture tuning, animation tweaks, layout adjustments.
**Not allowed:** New V1 features (those belong in WORK).
**Loop rule:** ITERATE → (feedback) → ITERATE. When UX stabilizes → PERFECT.
**Quality gate:** Works on target device. User feedback positive.
**Stop when:** UX iterations converge (3+ rounds without meaningful change).

### PERFECT

**Purpose:** Harden existing code. Enter only when WORK/ITERATE scope is complete.
**Allowed:** Fuzz testing, static analysis, audit, benchmarks, CI hardening.
**Not allowed:** New features or UX changes. PERFECT is for quality, not scope.
**Quality gate:** Full lint + full test suite + no-forbidden-patterns audit + Constitution compliance check.

### DISTRIBUTE

**Purpose:** Package, document, publish. Enter only when PERFECT gates pass.
**Allowed:** README, CHANGELOG, diagrams, badges, publishing, CI polish.
**Not allowed:** Any code changes.

---

## 5. V1 Scope & Learning Shifts

Define this at bootstrap. It locks when you enter WORK. It does NOT lock during DISCOVER.

### IN SCOPE (must ship)

- [ ]
- [ ]
- [ ]

### OUT OF SCOPE (explicitly not in V1)

- — deferred to V2
- — never planned

### NO-GOS (will never do)

- — prevents rabbit holes

### Learning Shift (documented discovery)

Goalpost shifts are not failures. They are evidence that you learned something
during WORK that you could not have known before. This is a success of the protocol,
not a breakdown. The protocols job is to make that shift cheap.

When a shift happens, document it:

```
LEARNING SHIFT
  What we learned: [the discovery that motivated the change]
  Decision: [the change in direction]
  Cost: [extra time, if any]
  What this enables: [why the shift is worth it]
```

Shifts are recorded in shift-log.md. Up to 5 shifts per project (up from 3 in v2.
Shifts are learning, not failures). After 5 shifts, consider starting a fresh cycle
rather than continuing to shift the same project.

---

## 6. AI Persona & Constraints

**Role:** `[domain-specific role, e.g. "Rust systems engineer for binary format codecs"]`
**Autonomy:** `[HIGH in DISCOVER/WORK | LOW in PERFECT | MEDIUM in ITERATE/DISTRIBUTE]`

### Constraints (per-project)

- **Language / edition** — exact versions
- **Safety rules** — no unwrap, unsafe policy, etc.
- **Quality floor** — lint level, file size limits, type strictness
- **Dependency policy** — no new deps without checking existing scope
- **Testing requirements** — coverage threshold, test types expected
- **Documentation requirements** — doc comments on public APIs, changelog format
- **Tool-first rule** — never hand-roll what a deterministic tool handles. Format → use formatter (`cargo fmt` / `dprint` / `prettier`). Type-check → use compiler. Lint → use linter (`clippy` / `ruff`). Fuzz → use fuzzer (`cargo-fuzz` / `jazzer`). The AI's effort goes to novel composition and edge case reasoning, not to tasks a tool handles deterministically and perfectly.

### Decision Framework (inviolable priority order)

1. **Correctness** over speed — wrong output at any speed is useless
2. **Consistency** with existing patterns over novel approaches — the codebase is the source of truth
3. **Simplicity** over complexity unless measured — don't optimize before profiling
4. **Explicit decisions** over implicit defaults — surface tradeoffs, don't hide them
5. **Test evidence** over intuition — if a test doesn't prove it, it's not done

---

## 7. Stop Rules

The AI MUST stop and ask before proceeding if ANY of these are true:

- [ ] Task touches **3+ files** in one change → ask for plan approval
- [ ] Task adds a **new dependency** → ask for permission
- [ ] Task **deletes or overwrites** existing code → confirm first
- [ ] Task is **outside current phase** → refuse, explain why
- [ ] Task touches **OUT OF SCOPE** → refuse, explain why
- [ ] Task would **change V1 scope** -> refuse, document as learning shift
- [ ] Task violates the **Constitution** → refuse, cite which principle
- [ ] Task is **ambiguous** (multiple valid approaches with different trade-offs) → present options
- [ ] Task exceeds **200 lines** of new code → propose plan first
- [ ] Task has **no test written first** (in WORK phase) → pause, write test first

---

## 8. Verification Gates

| Phase          | Must pass before reporting done                                           |
| -------------- | ------------------------------------------------------------------------- |
| **DISCOVER**   | Research summary complete, hypothesis tested, decision reached            |
| **WORK**       | `[compile command]` + `[test command]` passes + tests written BEFORE code |
| **ITERATE**    | Real-device test + UX convergence (3 rounds without meaningful change)    |
| **             | **PERFECT**                                                               | Full lint + full test suite + forbidden-pattern audit + Constitution compliance + **SPEC SYNC** |
| **DISTRIBUTE** | Spellcheck + link check + format conformance                              |

### SPEC SYNC (Spec-to-Code Fidelity Gate)

Extracted to standalone protocol: [SPEC_SYNC.md](./docs/SPEC_SYNC.md)

The spec-to-code fidelity verification gate that runs after POLISH and before DISTRIBUTE. It compares the specification against the as-built codebase, catalogues discrepancies as MISSING/OUTDATED/NEW, and ensures the live spec always reflects the as-built state.

See [SPEC_SYNC.md](./docs/SPEC_SYNC.md) for the full research-backed protocol, verification checklist, and discrepancy recovery procedure.
---

## 9. Test Philosophy

> Derived from TDD research and the Superpowers model (106K ⭐):
> "Code without tests is not done. Tests that merely confirm what the code already does
> are not tests — they are tautologies."

### The Rules

1. **Tests first.** In WORK phase, the test is written BEFORE the implementation. The AI does not write implementation code until a test exists that would pass on correct output.
2. **Tests verify, not confirm.** A test that passes on the first run is suspicious. The test must FAIL on incorrect output and PASS on correct output. If it passes without ever failing, it's not a test — it's a rehearsal.
3. **One behavior per test.** Each test verifies exactly one behavior. Test names describe the expected outcome: `test_decode_unknown_format_returns_error`, not `test_decode`.
4. **Edge cases are explicit.** Tests for edge cases (empty inputs, null values, boundary conditions) are written BEFORE tests for the happy path. If the AI cannot handle the edge case, it should not handle the happy path.
5. **No test-only changes without corresponding code.** Tests cannot be added in isolation. Every test must have an implementation that makes it pass.
6. **Regression tests lock bugs.** When a bug is found, the FIRST action is to write a test that reproduces it. Then fix the code. The test stays as a regression guard.

---

## 10. Evolution & Phase Exit

> Derived from Meta_Kim's Evolution stage (232 ⭐):
> At every phase exit, write back what was learned so the protocol improves.
> This closes the learning loop that most frameworks miss.

### Phase Exit Checklist

Before transitioning to the next phase, run this reflection:

```
Phase Exit: [phase name]

1. What did we learn in this phase?
   - Domain knowledge: [surprising discoveries about the problem space]
   - Process: [what worked, what didn't about this phase's rules]
   - Architecture: [decisions made that constrain future phases]

2. What should the NEXT phase know?
   - Gotchas: [things to watch out for]
   - Open questions: [things still unresolved]
   - Priorities: [what matters most in the next phase]

3. Protocol improvement?
   - Did any stop rule fire when it shouldn't have? [adjust rule]
   - Did any stop rule NOT fire when it should have? [tighten rule]
   - Did the phase boundaries hold? [if not, why?]

4. Constitution check?
   - Did any action violate the Constitution? [record and fix]
   - Does the Constitution need updating? [rare — think carefully]

5. Protocol self-audit?
   - Did the protocol's rules HELP in this phase? [which rules?]
   - Did any rule HURT? (slowed things down, blocked useful actions) [which? adjust]
   - Was this the right ROUTE for the project type? [if not, update decision tree]
   - Was the timebox appropriate for this phase? [too short? too long?]
   - Would you use the same phase sequence again? [if no, note why]
```

**Notes:** The protocol improves through self-audit. The dry run of Bus-Hop and Ithmb-Codec
showed that the 3-file limit and 200-line cap help in STANDARD projects but can slow down
PORT projects where the code is already known. Adjust rules per project type as patterns emerge.

---

## 11. Known Failure Patterns

> Derived from ai-project-governance's 109 failure patterns (FP-#):
> These are documented failure modes specific to AI-assisted development.
> If you recognize one, the AI should flag it proactively.

### FP-CAT-1: Scope Expansion

| ID     | Pattern                | Description                                                                                     |
| ------ | ---------------------- | ----------------------------------------------------------------------------------------------- |
| FP-001 | Feature Creep          | AI adds "helpful" features not in scope because nothing explicitly forbids them                 |
| FP-002 | Polish Trap            | Polishing before core works — triggered by AI suggesting cosmetic improvements                  |
| FP-003 | Rabbit Hole            | Deep optimization of something that might be removed                                            |
| FP-004 | Learning Shift Cascade | One shift leads to another because the first reveals new information instead of inconsistencies |

### FP-CAT-2: Quality

| ID     | Pattern            | Description                                                               |
| ------ | ------------------ | ------------------------------------------------------------------------- |
| FP-010 | Tautological Tests | Tests that pass on first run and only confirm what code already does      |
| FP-011 | Missing Edge Cases | Happy path works, edge cases crash silently                               |
| FP-012 | Security Blindness | AI generates functional code that skips auth, validation, or sanitization |
| FP-013 | Dependency Bloat   | Adding a library instead of writing 5 lines of code                       |
| FP-014 | Context Decay      | Later AI sessions contradict earlier decisions because context was lost   |

### FP-CAT-3: Process

| ID     | Pattern              | Description                                                                   |
| ------ | -------------------- | ----------------------------------------------------------------------------- |
| FP-020 | Phase Drift          | Working on DISTRIBUTE tasks during WORK phase without realizing it            |
| FP-021 | Silent Pivot         | Changing the approach without documenting or approving the change             |
| FP-022 | Assumption Hardening | Early assumptions become locked-in without being verified                     |
| FP-023 | Review Debt          | AI generates more code than can be reviewed, creating an accumulating backlog |
| FP-024 | Confident Wrongness  | Code compiles, runs, and is subtly incorrect — the hardest pattern to catch   |

### FP-CAT-4: Protocol Governance

| ID     | Pattern             | Description                                                                              |
| ------ | ------------------- | ---------------------------------------------------------------------------------------- |
| FP-030 | Rule Rigidity       | Protocol rules that help general cases actively slow down specific project types         |
| FP-031 | Over-governance     | Spending more time managing the protocol than building the product                       |
| FP-032 | Self-Audit Skipping | Rushing phase exits without running the self-audit                                       |
| FP-033 | Routing Error       | Choosing the wrong route at bootstrap, forcing the project into the wrong phase sequence |

### Using Failure Patterns

When the AI recognizes a failure pattern, it MUST:

1. Flag it: "Warning: this looks like FP-001 (Feature Creep)."
2. Explain why: "You asked for a login form, but I'm generating password recovery. This was not in scope."
3. Stop and ask: "Should I continue with this, or revert to the original scope?"

---

## 12. Session Kickoff

Every AI session starts with:

```
"Read RULES.md.
State current phase and what that means I can/cannot do.
State V1 scope and what's out of scope.
State the Constitution principles.
Check stop rules.
If blocked, refuse and explain. If clear, proceed."
```

---

## After Project: Close the Feedback Loop

The protocol improves with each project. After shipping:

1. **Routing check** — Did the bootstrap routing choose the right path? If not, update the decision tree.
2. **Phase gate review** — Did phases have the right boundaries? Too strict or too loose? Adjust.
3. **Stop rule audit** — Did the stop rules fire when needed? Any false negatives? Tighten.
4. **Constitution review** — Did the Constitution prevent any violations? Does it need amendment?
5. **Failure pattern harvest** — Did we encounter a pattern not in the list? Add it.

Run the Phase Exit Checklist (Section 9) one last time at project end, then update this file.

**Tool-first governance (meta):** The protocol enforces a tool-first rule on AI executors (Section 5 — tool-first constraint). This same principle applies to anyone executing or planning with this protocol. If a deterministic tool handles a task better than a reasoning agent, use the tool. Grep instead of reading every file. `cargo fmt` instead of manually formatting. A compiler instead of guessing types. The protocol improves when its own executors follow the same rules they enforce.

---

## Version

Current: v2.2.0

v2.2 adds: Two-gate architecture — AMBITION dialogue (Gate 1: is the goal clear enough?) and VALIDATION prototyping gate (Gate 2: should we build this?). The prototyping gate is now the central decision mechanism. Revised AMBITION.md as a Socratic dialogue protocol with 6 rounds (appetite through lock).

>

v2.0 adds: Constitution (immutable project DNA), Test Philosophy (test-first enforcement),
Failure Patterns catalog (20 documented AI failure modes), Evolution/Phase Exit reflection,
improved phase definitions, and enhanced stop rules.

---

## Origin

Extracted from Ithmb-Codec C# (3 weeks, 436 commits) and Rust (1 month, 321 commits),
Bus-Hop (Kotlin, 2.5 months, 249 commits) — across ~1,000 real commits.
Synthesized from 30+ research sources across 6 research agents covering:
Shape Up, Cascade Methodology, Spec-Driven Development (GitHub Spec Kit, sdd-pilot, PAW),
AI governance frameworks (ai-project-governance, Meta_Kim, Superpowers, AIAgentMinder),
AGENTS.md standards (arXiv:2601.20404, arXiv:2602.11988), and bootstrap tooling patterns.

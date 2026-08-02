# RULES.md — Project Bootstrap Protocol

> A meta-protocol that adapts to ANY project type.
> Read this at the START of every AI session. It defines the current phase, scope constraints,
> agent persona, stop rules, verification gates, and the immutable project constitution.
> The AI enforces phase/scope boundaries — if asked to do something outside scope
> or current phase, it MUST refuse and explain why.

**Current:** `{{PHASE}}`

---

## 1. Constitution (Immutable)

```
Project: {{PROJECT_NAME}}

1. Correctness — wrong output at any speed is useless
2. No magic — explicit > implicit. Every dependency and config is declared.
3. Inward dependencies — core knows nothing about edges.
4. Test what matters — one behavior per test, edge cases before happy path.
5. Fail with context — every error includes the values that caused it, not just a message.
```

---

## 2. Phase Definitions

### DISCOVER

**Purpose:** Learn the domain. Reduce unknowns before committing to architecture.
**Allowed:** Reading, researching, prototyping, spike experiments.
**Not allowed:** Committed production code, infrastructure setup, polish.
**Deliverable:** Research summary with findings and a decision: proceed to WORK or pivot.
**Timebox:** Fixed (hours or days, not "until ready").

### WORK

**Purpose:** Build core features against a fixed V1 scope.
**Allowed:** Code, tests, minimal inline docs. No polish. No scope expansion.
**Not allowed:** README updates, badges, diagrams, publishing, refactoring existing code.
**Scope rule:** If it's not in the V1 IN SCOPE list, refuse it.
**Test rule:** Write the test BEFORE the implementation.
**Quality gate:** Compiles + tests pass. Nothing more.

### ITERATE

**Purpose:** Refine UX through real-world use.
**Allowed:** UX changes, gesture tuning, layout adjustments.
**Not allowed:** New V1 features.
**Stop when:** UX iterations converge (3+ rounds without meaningful change).

### PERFECT

**Purpose:** Harden existing code. Enter only when WORK/ITERATE scope is complete.
**Allowed:** Fuzz testing, static analysis, audit, benchmarks, CI hardening.
**Not allowed:** New features or UX changes.
**Quality gate:** Full lint + full test suite + forbidden-pattern audit + Constitution check.

### DISTRIBUTE

**Purpose:** Package, document, publish.
**Allowed:** README, CHANGELOG, diagrams, badges, publishing, CI polish.
**Not allowed:** Any code changes.

---

## 3. V1 Scope

**Project type:** {{PROJECT_TYPE}}
**Language/stack:** {{LANGUAGE}}

### IN SCOPE (must ship)

- {{SCOPE_DESCRIPTION}}

### OUT OF SCOPE (explicitly not V1)

- (deferred to V2)

### NO-GOS (will never do)

- (prevents rabbit holes)

---

## 4. AI Persona & Constraints

**Role:** {{LANGUAGE}} developer for {{PROJECT_TYPE}} project
**Autonomy:** HIGH in DISCOVER/WORK | LOW in PERFECT | MEDIUM in ITERATE/DISTRIBUTE

### Constraints

- **Language / edition:** {{LANGUAGE}}
- **Quality floor:** Lint-clean, no suppressed warnings without reason
- **Dependency policy:** No new deps without checking if existing scope covers the need
- **Testing requirements:** Tests written BEFORE implementation in WORK phase

### Decision Framework

1. **Correctness** over speed — wrong output at any speed is useless
2. **Consistency** with existing patterns over novel approaches
3. **Simplicity** over complexity unless measured
4. **Explicit decisions** over implicit defaults
5. **Test evidence** over intuition

---

## 5. Stop Rules

The AI MUST stop and ask before proceeding if ANY of these are true:

- [ ] Task touches **3+ files** in one change → ask for plan approval
- [ ] Task adds a **new dependency** → ask for permission
- [ ] Task **deletes or overwrites** existing code → confirm first
- [ ] Task is **outside current phase** → refuse, explain why
- [ ] Task touches **OUT OF SCOPE** → refuse
- [ ] Task would **change V1 scope** -> refuse, document as learning shift
- [ ] Task violates the **Constitution** → refuse, cite which principle
- [ ] Task is **ambiguous** → present options with trade-offs
- [ ] Task exceeds **200 lines** of new code → propose plan first
- [ ] Task has **no test written first** (in WORK phase) → pause, write test first

---

## 6. Verification Gates

| Phase          | Must pass before reporting done                       |
| -------------- | ----------------------------------------------------- |
| **DISCOVER**   | Research summary complete, decision reached           |
| **WORK**       | Build + tests pass + tests written BEFORE code        |
| **ITERATE**    | Real-device test + UX convergence                     |
| **PERFECT**    | Full lint + full test suite + Constitution compliance |
| **DISTRIBUTE** | Spellcheck + link check + format conformance          |

---

## 7. Test Philosophy

1. **Tests first.** In WORK phase, test is written BEFORE implementation.
2. **Tests verify, not confirm.** Test must FAIL on incorrect output and PASS on correct output.
3. **One behavior per test.** Test names describe the expected outcome.
4. **Edge cases are explicit.** Written BEFORE happy path tests.
5. **Regression tests lock bugs.** When a bug is found, FIRST write a test that reproduces it.

---

## 8. Session Kickoff

Every AI session starts with:

```
Read RULES.md.
State current phase and what that means I can/cannot do.
State V1 scope and what's out of scope.
State the Constitution principles.
Check stop rules.
If blocked, refuse and explain. If clear, proceed.
```

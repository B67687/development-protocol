# EXECUTOR.md — Spec-to-Execution Handoff

Updated for v3 — adds Design for Change Rules.

> Use this after SPECIFICATION.md is complete.
> This document bridges the gap between "I have a complete spec" and "I'm autonomously executing it via RULES.md."
> The spec IS the plan. This document tells the executor how to follow it.

---

## When to Use This

You have a filled SPECIFICATION.md. Now you need an AI agent to execute it autonomously.

EXECUTOR.md defines:

- How the spec maps to a RULES.md route
- How the AI reads and follows the spec
- How to verify progress at each checkpoint
- How to handle interruptions and failures

---

## Spec-to-Routing Mapping

The filled SPEC.md determines which RULES.md route to use. The spec lives at `.omo/plans/specification.md` (created in SPECIFICATION); the bootstrap RULES.md template is copied from `template/RULES.md`. The mapping is NOT hardcoded to STANDARD — it depends on the spec's content:

| Spec characteristic                  | Suggests route                                       | Rationale                                        |
| ------------------------------------ | ---------------------------------------------------- | ------------------------------------------------ |
| Familiar domain, clear architecture  | **STANDARD** — WORK → PERFECT → DISTRIBUTE           | Spec covers all unknowns; straight execution     |
| Domain uncertainty or new technology | **DISCOVER-FIRST** — DISCOVER → (STANDARD)           | Spec identifies unknowns that need research      |
| UX-heavy with interaction design     | **UX-FIRST** — WORK → ITERATE → PERFECT → DISTRIBUTE | Spec describes behavior but feel needs iteration |
| No delivery commitment               | **EXPLORE-ONLY** — DISCOVER only                     | Spec is exploratory                              |
| Porting existing software            | **PORT** — timeboxed WORK → PERFECT                  | Spec maps known behavior                         |
| Maintenance only                     | **MAINTENANCE** — PERFECT → DISTRIBUTE               | Spec describes fixes, not features               |

**Mapping algorithm:**

1. Read SPEC.md `## 2. Architecture` — if it names tools you know well and the pattern is familiar, lean STANDARD.
2. Read `## 6. UX` — if user-facing interaction is the primary concern, lean UX-FIRST.
3. Read `## 1. Overview` — if it calls out learning or uncertainty, lean DISCOVER-FIRST.
4. Default to STANDARD if no clear signal.

**Route decision for development-protocol-try:**
SPEC.md section 1 (familiar domain — building what we designed) -> STANDARD
SPEC.md section 6 (document set, no UX) -> not UX-FIRST
SPEC.md section 2 (two-gate architecture, our own design) -> not DISCOVER-FIRST

Result: STANDARD route — Boot -> WORK (produce all 10 files) -> PERFECT (validate links/format) -> DISTRIBUTE (publish to GitHub)
Autonomy level: HIGH during WORK (producing files is mechanical). LOW if any structural decision needed.

---

## Execution Protocol

Once the route is chosen:

1. **Bootstrap RULES.md** — create or update RULES.md with the chosen route. Copy the Constitution from SPEC.md:1.
2. **Enter WORK phase** — the AI reads SPEC.md section by section and implements each.
3. **Implementation order** — follow SPEC.md section order: 1 (overview) → 2 (architecture) → 3 (file tree) → 4 (CI) → 5 (dependencies) → 6 (UX) → 7 (timeline). Later sections may reference earlier ones.
4. **No deviations** — if a SPEC.md section is ambiguous, the AI flags it and asks for clarification. It does NOT guess.
5. **Spec-as-final-bytes** — the spec is the contract, not a suggestion: implement exactly what SPEC.md states, no more, no less; if reality diverges from the spec, STOP and flag (Midpoint Protocol Check) rather than silently improvising.

---

## Autonomy Levels & Permission Model

The executor operates at one of three autonomy levels. The level determines what the AI can do without human approval.

| Level             | When used                                                                     | Permitted without asking                                                                                                                             | Always blocks on                                                                            |
| ----------------- | ----------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| **LOW**           | Prep phases (AMBITION→SPEC), early WORK while spec is stabilizing             | Nothing outside the current task description                                                                                                         | Any file write, any command, any dep addition                                               |
| **HIGH**          | WORK phase with locked spec, all checkpoints passing                          | Creating/modifying files in SPEC.md section 3 file tree, running commands from SPEC.md section 4 CI, adding dependencies listed in SPEC.md section 5 | Spec violations, RULES.md failure patterns (FP-*), learning shifts, Constitution violations |
| **CRITICAL-ONLY** | Late WORK when 3+ checkpoints passed consecutively without human intervention | Same as HIGH, plus timeline adjustments within ±20% of SPEC.md section 7 estimates                                                                   | Constitution violations, unrecoverable errors (data loss, security)                         |

**Starting autonomy:** Always start at LOW. Escalate to HIGH only when SPEC.md is locked and the first checkpoint passes. Escalate to CRITICAL-ONLY only when 3 consecutive checkpoints pass without intervention.

**Permission model note (OpenCode/OMO):** If your agent runner requires per-action approval (e.g. `[Y]` prompts for every `cargo` invocation or file write), HIGH autonomy may still cause friction. Consider pre-authorizing a session with: "I approve all tool calls within the bounds of SPEC.md sections 3, 4, and 5 for the next N checkpoints." This gives the executor the same effect as CRITICAL-ONLY without changing your runner's security model.
After each SPEC.md section is implemented, verify:

1. **Section implemented?** — does the codebase reflect the spec's section requirements?
2. **Gates pass?** — compile + test + lint per SPEC.md section 4.
3. **Spec still accurate?** — if implementation revealed a flaw in the spec, pause and flag to human.
4. **State saved?** — commit with message: `checkpoint: section N — [section_name]`.

Checkpoints serve as resume points. If execution is interrupted, the executor restarts from the last checkpoint, not from scratch.

---

## Resume Protocol

If execution is interrupted (session ends, context expires, error occurs):

1. **Read last checkpoint** — `git log --oneline | grep checkpoint` shows the last completed section.
2. **Re-read SPEC.md** — refresh full spec context.
3. **Re-read RULES.md** — confirm current phase.
4. **Re-run gate** — `cargo test && cargo clippy` (or equivalent for the project) to confirm pre-interruption state is clean.
5. **Resume from next section** — continue implementation from the section after the last checkpoint.

**Cold start (no checkpoints):** If no checkpoints exist, the executor re-reads the full spec, confirms RULES.md route, and begins WORK from section 1.

---

## Error Handling (overview)

When a SPEC.md assumption fails during execution:

1. **Pause** — stop all implementation. Do not "work around" the failed assumption.
2. **Flag** — document what assumption failed, where in the spec it lives, and what evidence disproves it.
3. **Human decision** — present options: revise spec (minor), pause project (major), or work around (risky).
4. **Resume** — after human decision, update SPEC.md accordingly, commit as `revision: [reason]`, and resume checkpointing from section 1.

**Common failure modes:**

- Dependency X doesn't work as expected (flag to SPEC.md section 5)
- Architecture pattern doesn't fit (flag to section 2)
- Timeline is unrealistic (flag to section 7 — most common)

## Retry Protocol

When a SPEC.md section implementation fails during execution, do not escalate to
the human on the first failure. Use the retry ladder:

| Attempt | Action | Escalation |
|---|---|---|
| 1st failure | Retry once with full context preserved. Log the error and the attempted fix. | — |
| 2nd failure | Run the Midpoint Protocol Check early (or check if already past midpoint). Assess whether the approach is fundamentally wrong. | Flag root cause hypothesis to human. |
| 3rd failure | Stop. Present the human with: (1) summary of all attempts, (2) root cause analysis, (3) three options: Revise Spec, Work Around with documented risk, or Abandon. | BLOCKING — human decision required. |

**Retry discipline:** Each retry must attempt a DIFFERENT approach. Re-running the
same failed strategy and expecting a different result is not a retry — it's a waste.

---

## Degradation Strategy

When context is exhausted, errors accumulate beyond the third failure, or the AI
session approaches the budget limits defined in PACING.md:

### Section Priority Order

If the executor must reduce scope to stay within budget (turns, reads, or time),
sections are dropped from the bottom up:

| Priority | Sections | Never Drop |
|---|---|---|
| **Critical** (never drop) | Constitution, Architecture, CI gates, Testing | — |
| **Important** (drop only if forced) | File Tree, Dependencies, Operations, Timeline | — |
| **Nice-to-have** (first to drop) | UX Details, Documentation, Ecosystem, AI Attribution | — |

### Drop Procedure

1. **Commit current checkpoint** before dropping: `checkpoint: section N — degraded — [reason]`
2. **Document what was dropped** in `.omo/degradation-log.md` with timestamp and rationale
3. **Attempt recovery at next session** — on resume, read the degradation log and decide which sections to restore
4. **Never drop silent** — the human must be informed of every degradation decision, even if after the fact

### Emergency Contact

If 3 consecutive sections fail at the 3rd retry, or if a dropped section creates a
data-loss or security risk:

1. Pause all execution immediately
2. Write a structured failure report to `.omo/failure-report.md`
3. **Do not continue without human decision** — the executor stops at the last stable checkpoint and waits

### Quarantine Cascade (magic-spec pattern)

When a component's implementation becomes unstable (repeated failures, spec
drift, failing invariants), it is QUARANTINED — and every work item that
DEPENDS on it is automatically blocked until the quarantine clears:

1. **Quarantine trigger**: 2+ consecutive failures, or a failing invariant, on a component.
2. **Block dependents**: any task whose inputs include the quarantined component is
   halted (do not build on a broken foundation).
3. **Quarantine record**: log in `.omo/quarantine-log.md` — component, trigger, timestamp, blocked dependents.
4. **Clear condition**: the component passes its evidence-gated checks (tests,
   invariants, ledger conformance) → dependents unblock.
5. **Escalate**: if a component cannot clear quarantine within budget, it is a
   candidate for the Paradigm Review (revolutionary mode).

> Source: magic-spec's quarantine cascade (implementation halted if its L1 spec
> goes unstable). Prevents building downstream on a broken foundation — the
> regression cascade the Project Health Discipline exists to prevent. Confidence:
> Medium-High.

## Paradigm Review (Revolutionary Mode)

When anomalies accumulate beyond the error handling threshold — 3+
assumptions fail independently — the paradigm itself may be broken.

1. **Pause** execution. Open `.omo/paradigm-review.md`.
2. **Diagnose**: List each failed assumption, what disproved it, and what
   alternative paradigm might fit.
3. **Escalate to EXTRACTION loop**: Present to human. Decision: reframe or
   continue with known uncertainty.

**Reframe path:** Take the paradigm review to EXTRACTION. The failed
paradigm was trying to solve something — that's the new Y.

**Continue path:** Document as known unknowns. Continue with increased
monitoring at each checkpoint.

> Source: Kuhn's paradigm shift model. Confidence: Medium.
---

## Midpoint Protocol Check

At EXECUTOR's midpoint (roughly halfway through the milestone), pause and run a quick protocol retrospective:

1. **Is the spec still accurate?** — If implementation revealed spec flaws, pause for revision.
2. **Is the protocol serving you?** — Are any steps feeling wasteful? Overhead > benefit?
3. **Are assumptions still valid?** — Any one-way doors since discovered that weren't validated?
4. **Context check** — Is the AI session showing signs of context decay? (See PACING.md signals.)

Document the midpoint check in `.omo/reflect.md` with a timestamp. This is a lighter version of REFLECT — the goal is to catch protocol issues early enough to fix them mid-project, not after.

## Design for Change Rules (v3)

These rules make goalpost shifts cheap instead of expensive. They are not optional polish — they are structural rules that every project must follow. Enforce them in code review and CI.

### 1. Interface Rule

"No interface before its second consumer." A single-implementation interface is just complexity. Extract the abstraction when the second consumer arrives, not the first.

### 2. Test Rule

"Test the contract, not the implementation." Tests should only call public APIs. No mocking of your own code. No logic (if/for/while) in test files. Prefer behavioral tests that assert on return values and error types, not on internal state or call order.

### 3. Module Boundary Rule

"Every module has a single public entry point." All exports go through one index file (index.ts, lib.rs, mod.go). No cross-module imports of internal paths. Violations are caught in code review.

### 4. Size Rule
No file exceeds 250 lines of code. No function exceeds 40 lines. Enforce with a linter.
**Source:** Common practice in software engineering. McConnell (2004) recommends functions under
30-50 lines. Kernel style guides enforce 100-line functions. The 250-file limit is a pragmatic
boundary — files exceeding this size reliably contain multiple responsibilities.

### 5. Cycle Artifact Rule

"Every cycle produces a shippable artifact." Not "progress" — a deployed, testable, usable thing.
**Source:** Shape Up (Singer 2019) — 6-week cycles produce shipped work, not "progress."

### 6. Appetite Rule

"Define the maximum time before designing the solution." The appetite (time budget) is fixed.
**Source:** Shape Up (Singer 2019) — appetite is the first decision, before solution design.

### 7. AI Quality Rule

"AI-generated code passes the same structural checks as human-written code."
**Source:** Twist et al. (2025) — LLMs generate less maintainable code on average.
Metz (2016) — wrong abstractions compound. Therefore, AI code needs the same quality gates.

### 8. Abstraction Rule

"Rule of three — extract on the third occurrence, not the first." Write the first occurrence concretely. Duplicate on the second (it is okay). Abstract on the third — now you know the shape.

### 9. Dependency Rule

"Core domain never imports infrastructure." Core modules (business logic, types, algorithms) import zero infrastructure packages. All infrastructure access goes through a port (interface) defined in the core and implemented by an adapter in the infrastructure layer.

### 10. Clean Slate Rule

"No perpetual backlog. Every cycle starts clean." Old items must compete at the betting table.
**Source:** Shape Up (Singer 2019) — no backlog. Each cycle, pitches compete fresh.

---

## Meta: How to Execute Specifications

1. **Trust the spec, but verify** — the spec is authoritative, but if reality contradicts it, reality wins.
2. **Checkpoint obsessively** — every section completion saves days of re-work on interruption.
3. **Read sections in order** — each section builds on the previous. Don't jump.
4. **Flag ambiguity immediately** — guessing is the #1 cause of wasted output. If a spec line is unclear, stop and ask.
5. **Respect the lock** - once execution starts, SPEC.md changes go through RULES.md learning shift rules.

---

## Origin

Created for the Development Protocol v2.1 PREP PHASE (July 2026). Bridges the gap between static specification and dynamic autonomous execution.

---

## Production Quality Requirements

> *Engineering-specific production quality requirements moved to [Engineering Plugin](docs/engineering-plugin.md#3-production-quality-requirements).*

For Tier 2+ projects (those with a runtime, CLI, library, or performance-sensitive
component), consult the Engineering Plugin for baseline quality gates: fuzz targets,
benchmarks, snapshot testing, CI matrix, test ratio, and security audit requirements.

These gates are not optional polish — they must pass before a spec is considered
execution-ready for engineering deliverables.

## Project Health Discipline

> **MANDATORY for every project with lifecycle-bearing state.** This section enforces
> the mechanisms that make "adding a feature" a safe, documented transition instead
> of a regression gamble. It is the protocol's answer to the regression pain:
> *every addition breaks something* because the project has no model of itself.

**The core requirement:** `docs/PROJECT_MODEL.md` exists and is current (mandated
by SPECIFICATION.md § PROJECT_MODEL). It documents the whole-project state machine,
valid/invalid transitions, invariants, and blast radius map.

**Enforced mechanisms (from research, strongest first):**

| Mechanism | What it does | Evidence | Setup cost |
| --- | --- | --- | --- |
| **Full transition-table test** | Asserts every state × every event → resulting state or rejection. New features can't add unintended transitions. | Torkar: 26 real faults found, 85% test cost cut | 1-2 days |
| **Co-change analysis** | `git log --name-only` reveals which files change together — your real blast radius. Check siblings before changing one. | D'Ambros: correlates with defects, p=0.01 | 30 min script |
| **Layer/import rules** | Domain code imports only domain, enforced by lint (import-linter, dependency-cruiser). Violations fail the build loudly. | Fitness functions (Ford/Parsons/Kua) | 30 min |
| **Characterization tests** | Pin current behavior of legacy code before touching it — behavior changes become visible failures. | Feathers, canonical legacy practice | minutes/file |
| **Mutation testing on core module** | Surviving mutants = concrete "what regression could this have?" checklist. Run on core domain only. | FSE 2014 (gold-standard) | minutes/run |
| **Per-feature change-amplification check** | If a feature touches >5 files, ask "what interface should I have changed instead?" | Ousterhout, change amplification | 30 sec/feature |

**Where they run:** mechanisms are implemented in the PROJECT (CI/lint/tests), not
the protocol. The protocol's job is to REQUIRE them, and the FINISH gate CHECKs them.

> **Source:** State-machine modeling and transition tests (Harel 1987; Torkar et al.),
> co-change prediction (D'Ambros et al., MSR 2010), fitness functions (Ford, Parsons,
> Kua), characterization tests (Feathers), mutation testing (Just et al., FSE 2014),
> change amplification (Ousterhout). Confidence: Medium-High overall — strongest on
> mutation testing and co-change prediction.

## Post-Execution: FINISH Gate (POLISH)

The FINISH gate is a protected sub-phase of EXECUTOR with its own budget and
checklist. It catches what AI systematically misses — edge cases, integration
surprises, and the "last mile" problems that surface only after the core is "done."

Research across architecture (40% effort on construction documents), software
(~50% on testing/debugging), and film (10-30% on post-production, routinely
underestimated) confirms that the finish phase is disproportionately effortful and
systematically under-budgeted. The FINISH gate protects this phase from being
starved by substance-phase overruns.

### Entry Criteria

- All EXECUTOR milestones complete (spec fully implemented)
- All checkpoints passed and committed
- No blocking errors in the current state

### Budget

The FINISH gate gets **20-30% of EXECUTOR's total budget** (roughly 6-9% of total
project appetite). If substance-phase overruns eat into this reserve, do NOT let it
drop below 15% of EXECUTOR budget. The evidence strongly suggests the finish
phase requires meaningful allocation to succeed.

### The Polish Checklist

Run through each category systematically. Do not skip categories even if they feel
irrelevant — AI systematically overrates polish because it doesn't encounter the
real-world friction points.

| Category | What to check | Evidence level |
| --- | --- | --- |
| **Error handling** | API failures, network timeouts, file I/O, null states, empty responses | Mandatory |
| **State management** | Race conditions, stale data, concurrent access, cache invalidation | Mandatory |
| **Edge cases** | Empty arrays, single-element lists, boundary conditions, 0/1/N patterns | Mandatory |
| **Input validation** | Malformed input, encoding, large payloads, slow connections, injection vectors | Mandatory |
| **Accessibility** | Screen reader labels, keyboard nav, color contrast, focus management | Tier 1+ |
| **Security** | Input validation, auth per endpoint, rate limiting, privilege boundaries | Mandatory |
| **Documentation** | README, API docs, setup guide, troubleshooting, changelog | Tier 1+ |
| **Performance** | Load time, render time, memory profile, bundle size | Tier 2+ |
| **Cross-system integration** | API contract compliance, data format compatibility, error propagation | Mandatory |

### Named Techniques (edge-case & UX evaluation)

| Technique | When | Source |
| --- | --- | --- |
| **Equivalence Partitioning + Boundary Value Analysis** | Generate edge-case inputs systematically: min-1/min/min+1/max-1/max/max+1 | ISTQB Standard; AI failure-pattern research (Augment Code 2025-26) |
| **Whittaker's exploratory tours** (Business + Seedy districts) | Systematic manual exploration of core paths + error states | Whittaker 2009 |
| **Nielsen's 10 usability heuristics** | Structured UX evaluation replacing an implicit "feel check" | Nielsen 1994 (updated 2026) |

### AI-Generated Code Pre-Flight

Before human review, check the 8 AI failure patterns (AI failure-pattern research, Augment Code 2025-26):
1. **Hallucinated APIs** — references to packages/methods that don't exist (~1 in 5
   AI snippets contain fake libraries).
2. **Security vulnerabilities that look functional** — code works but fails securely:
   auth bypasses, SQL injection, error handlers that leak sensitive data.
3. **Performance anti-patterns** — string concatenation in loops, nested O(n²)
   iterations, unnecessary allocations.
4. **Happy-path error handling** — try-catch that logs but doesn't recover; no
   fallback.
5. **Missing edge cases** — empty arrays, nulls, boundary integers, unicode inputs.
6. **Outdated library usage** — deprecated APIs from training data, obsolete before
   the AI wrote them.
7. **Data model mismatches** — assumes structures that don't match actual schemas;
   property access without type checking.
8. **Missing context dependencies** — env vars without fallbacks, undocumented
   config, assumed infrastructure.
Each unaddressed pattern becomes a human review item.

### Post-Ship Feedback Log (5 min, 3 items)

1. One thing the AI did surprisingly well
2. One thing the AI missed that cost significant time
3. One protocol change that would have caught it earlier
Feeds directly into protocol improvement without overhead.

### Exit Criteria

Before the FINISH gate is complete:

- [ ] Polish checklist run and signed off
- [ ] No known edge-case bugs unaddressed
- [ ] Documentation matches implementation
- [ ] AI has flagged all items it cannot assess (these go to human review)
- [ ] Final checkpoint committed: `checkpoint: FINISH — complete`

### FINISH Gate Integration — Project Health Checks

Add to the Polish Checklist:

- [ ] PROJECT_MODEL.md exists, is current, and the addition's transition is in the table
- [ ] Transition-table test passes (all states × events)
- [ ] No co-change sibling of a modified file was missed (checked via git log analysis)
- [ ] Layer rules pass (domain imports only domain)
- [ ] Mutation testing run on core module — surviving mutants reviewed as regression checklist
- [ ] Evidence-gated merge: every change merged with its evidence artifacts (tests,
      proofs, ledger entries) attached — no merge without evidence (v3-agent-standard)


---

### 画蛇添足 Test (Universal Fundamental M1)

> **Runs at:** the FINISH gate, for every addition being finalized (feature,
> dependency, procedure, section). It is the completion-boundary check that
> catches scope creep before ship.

Before ANY addition ships (feature, dependency, procedure, section), it must
survive the 画蛇添足 test — from the Zhanguo Ce parable (drawing a snake,
adding feet destroys the snake) and the converged universal fundamental
M1: Simplicity-as-Removal (see docs/UNIVERSAL_FUNDAMENTALS.md):

> **"What does this addition BREAK that was already working?"**

If the answer is "nothing" — but the addition is also unnecessary (the artifact
already satisfies its purpose), the addition fails anyway. The test is about
completion boundary, not just safety:

| Criterion | Pass | Fail |
| --- | --- | --- |
| **Completeness** | The artifact does NOT yet satisfy its purpose | The artifact already satisfies its purpose — addition is 画蛇添足 |
| **Safety** | The addition breaks nothing already working | The addition breaks or destabilizes existing behavior |
| **Value** | The marginal quality gain is positive | The marginal quality gain is zero or negative |

**The deep counterweight (from M1's own counter-evidence):** redundancy is
sometimes the point — safety-critical systems deliberately add. The test is
NOT "never add". It is "addition beyond the completion boundary destroys
value". Know where the boundary is.

# EXECUTOR.md — Spec-to-Execution Handoff

> Use this after SPECIFICATION.md is complete. The spec IS the plan; this document tells the executor how to follow it. Bridges "I have a complete spec" and "I'm autonomously executing it via RULES.md." Updated for v3 — adds Design for Change Rules.

---

## When to Use This

You have a filled SPECIFICATION.md and need an AI agent to execute it autonomously. EXECUTOR.md defines: spec→RULES.md route mapping, how the AI reads/follows the spec, checkpoint verification, interruption/failure handling.

---

## Spec-to-Routing Mapping

The filled SPEC.md determines which RULES.md route to use. The spec lives at `.omo/plans/specification.md`; the bootstrap RULES.md template is copied from `template/RULES.md`. The mapping is NOT hardcoded to STANDARD — it depends on the spec's content:

| Spec characteristic | Suggests route | Rationale |
| --- | --- | --- |
| Familiar domain, clear architecture | **STANDARD** — WORK → PERFECT → DISTRIBUTE | Spec covers all unknowns; straight execution |
| Domain uncertainty or new technology | **DISCOVER-FIRST** — DISCOVER → (STANDARD) | Spec identifies unknowns that need research |
| UX-heavy with interaction design | **UX-FIRST** — WORK → ITERATE → PERFECT → DISTRIBUTE | Spec describes behavior but feel needs iteration |
| No delivery commitment | **EXPLORE-ONLY** — DISCOVER only | Spec is exploratory |
| Porting existing software | **PORT** — timeboxed WORK → PERFECT | Spec maps known behavior |
| Maintenance only | **MAINTENANCE** — PERFECT → DISTRIBUTE | Spec describes fixes, not features |

**Mapping algorithm:**
1. Read SPEC.md `## 2. Architecture` — familiar tools/pattern → lean STANDARD.
2. Read `## 6. UX` — user-facing interaction primary → lean UX-FIRST.
3. Read `## 1. Overview` — calls out learning/uncertainty → lean DISCOVER-FIRST.
4. Default to STANDARD if no clear signal.

**Worked route decision (development-protocol-try):** §1 familiar domain → STANDARD; §6 document set, no UX → not UX-FIRST; §2 two-gate architecture → not DISCOVER-FIRST. Result: STANDARD — Boot → WORK (10 files) → PERFECT (validate) → DISTRIBUTE (publish). Autonomy HIGH during mechanical WORK, LOW if any structural decision needed.

---

## Execution Protocol

Once the route is chosen:
1. **Bootstrap RULES.md** — create/update with the chosen route. Copy the Constitution from SPEC.md:1.
2. **Enter WORK phase** — read SPEC.md section by section and implement each.
3. **Implementation order** — SPEC.md order: 1 → 2 → 3 → 4 → 5 → 6 → 7. Later sections reference earlier ones.
4. **No deviations** — ambiguous spec → flag and ask, never guess.
5. **Spec-as-final-bytes** — implement exactly what SPEC.md states; if reality diverges, STOP and flag (Midpoint Protocol Check), never silently improvise.

---

## Autonomy Levels & Permission Model

The executor operates at one of three autonomy levels — what the AI can do without human approval:

| Level | When used | Permitted without asking | Always blocks on |
| --- | --- | --- | --- |
| **LOW** | Prep phases (AMBITION→SPEC), early WORK while spec is stabilizing | Nothing outside the current task description | Any file write, any command, any dep addition |
| **HIGH** | WORK with locked spec, all checkpoints passing | Creating/modifying files in SPEC.md §3 file tree, running commands from §4 CI, adding deps listed in §5 | Spec violations, RULES.md failure patterns (FP-*), learning shifts, Constitution violations |
| **CRITICAL-ONLY** | Late WORK, 3+ checkpoints passed consecutively | Same as HIGH + timeline adjustments within ±20% of §7 estimates | Constitution violations, unrecoverable errors (data loss, security) |

**Starting autonomy:** Always LOW. Escalate to HIGH when SPEC.md is locked and the first checkpoint passes; to CRITICAL-ONLY after 3 consecutive clean checkpoints.

**Permission model note (OpenCode/OMO):** If your runner requires per-action approval (e.g. `[Y]` prompts for every `cargo` invocation), pre-authorize a session: "I approve all tool calls within the bounds of SPEC.md sections 3, 4, and 5 for the next N checkpoints." Same effect as CRITICAL-ONLY without changing your runner's security model.

**After each SPEC.md section, verify:** (1) Section implemented — codebase reflects the requirements? (2) Gates pass — compile + test + lint per §4. (3) Spec still accurate? — implementation revealed a flaw → pause and flag. (4) State saved — commit `checkpoint: section N — [section_name]`. Checkpoints are resume points: on interruption, restart from the last checkpoint, not scratch.

---

## Resume Protocol

If execution is interrupted (session ends, context expires, error occurs):
1. **Read last checkpoint** — `git log --oneline | grep checkpoint`.
2. **Re-read SPEC.md** — refresh full spec context.
3. **Re-read RULES.md** — confirm current phase.
4. **Re-run gate** — `cargo test && cargo clippy` (or equivalent) to confirm pre-interruption state is clean.
5. **Resume from next section** — continue after the last checkpoint.

**Cold start (no checkpoints):** re-read the full spec, confirm RULES.md route, begin WORK from section 1.

---

## Error Handling (overview)

When a SPEC.md assumption fails during execution:
1. **Pause** — stop all implementation. Do not "work around" the failure.
2. **Flag** — document what failed, where in the spec it lives, what evidence disproves it.
3. **Human decision** — options: revise spec (minor), pause project (major), work around (risky).
4. **Resume** — update SPEC.md, commit `revision: [reason]`, resume checkpointing from section 1.

**Common failure modes:** dependency doesn't work (flag §5); architecture pattern doesn't fit (flag §2); timeline unrealistic (flag §7 — most common).

---

## Retry Protocol

When a SPEC.md section implementation fails, do not escalate to the human on the first failure. Retry ladder:

| Attempt | Action | Escalation |
| --- | --- | --- |
| 1st failure | Retry once with full context preserved. Log error + attempted fix. | — |
| 2nd failure | Run Midpoint Protocol Check early. Assess whether the approach is fundamentally wrong. **Regression-class symptoms (worked before, broken now): route through the /solve regression path (Difference Check → Binary Search) BEFORE any retry 3.** | Flag root-cause hypothesis to human. |
| 3rd failure | Stop. Present: (1) all attempts, (2) root-cause analysis, (3) options: Revise Spec / Work Around (documented risk) / Abandon. | BLOCKING — human decision required. |

**Retry discipline:** each retry must attempt a DIFFERENT approach. Re-running the same failed strategy is not a retry.

---

## Degradation Strategy

When context is exhausted, errors accumulate beyond the third failure, or the AI
session approaches the budget limits defined in AMBITION (PACING folded):

### Section Priority Order

If the executor must reduce scope to stay within budget (turns, reads, or time), sections are dropped from the bottom up:

| Priority | Sections | Never Drop |
| --- | --- | --- |
| **Critical** (never drop) | Constitution, Architecture, CI gates, Testing | — |
| **Important** (drop only if forced) | File Tree, Dependencies, Operations, Timeline | — |
| **Nice-to-have** (first to drop) | UX Details, Documentation, Ecosystem, AI Attribution | — |

### Drop Procedure
1. **Commit checkpoint** before dropping: `checkpoint: section N — degraded — [reason]`
2. **Document** in `.omo/degradation-log.md` with timestamp and rationale
3. **Attempt recovery next session** — read the log, decide which sections to restore
4. **Never drop silent** — inform the human of every degradation, even after the fact

### Emergency Contact
If 3 consecutive sections fail at the 3rd retry, or a dropped section creates a data-loss or security risk:
1. Pause all execution immediately
2. Write a structured failure report to `.omo/failure-report.md`
3. **Do not continue without human decision** — stop at the last stable checkpoint and wait

### Quarantine Cascade (magic-spec pattern)

When a component's implementation becomes unstable (repeated failures, spec drift, failing invariants), it is QUARANTINED — every work item that DEPENDS on it is blocked until the quarantine clears:
1. **Quarantine trigger**: 2+ consecutive failures, or a failing invariant.
2. **Block dependents**: tasks whose inputs include the quarantined component are halted (do not build on a broken foundation).
3. **Quarantine record**: log in `.omo/quarantine-log.md` — component, trigger, timestamp, blocked dependents.
4. **Clear condition**: component passes its evidence-gated checks (tests, invariants, ledger conformance) → dependents unblock.
5. **Escalate**: cannot clear within budget → candidate for Paradigm Review.

> Source: magic-spec's quarantine cascade. Prevents building downstream on a broken foundation. Confidence: Medium-High.

---

## Paradigm Review (Revolutionary Mode)

When 3+ assumptions fail independently, the paradigm itself may be broken:
1. **Pause** execution. Open `.omo/paradigm-review.md`.
2. **Diagnose**: list each failed assumption, what disproved it, what alternative paradigm fits.
3. **Escalate to EXTRACTION loop**: present to human. Decision: reframe or continue with known uncertainty.

**Reframe path:** take the paradigm review to EXTRACTION — the failed paradigm was solving something; that's the new Y.
**Continue path:** document as known unknowns; continue with increased checkpoint monitoring.

> Source: Kuhn's paradigm shift model. Confidence: Medium.

---

## Midpoint Protocol Check

At EXECUTOR's midpoint, pause and run a quick protocol retrospective:
1. **Is the spec still accurate?** — implementation revealed spec flaws → pause for revision.
2. **Is the protocol serving you?** — steps feeling wasteful? Overhead > benefit?
3. **Are assumptions still valid?** — one-way doors discovered that weren't validated?
4. **Context check** — signs of context decay? (See AMBITION budget session rules.)

Document in `.omo/reflect.md` with a timestamp. Lighter than REFLECT — catch protocol issues mid-project, not after.

---

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

1. **Trust the spec, but verify** — spec is authoritative, but reality wins if they contradict.
2. **Checkpoint obsessively** — every section completion saves days of re-work on interruption.
3. **Read sections in order** — each builds on the previous. Don't jump.
4. **Flag ambiguity immediately** — guessing is the #1 cause of wasted output. Stop and ask.
5. **Respect the lock** — once execution starts, SPEC.md changes go through RULES.md learning shift rules.

---

## Origin

Created for the Development Protocol v2.1 PREP PHASE (July 2026). Bridges static specification and dynamic autonomous execution.

---

## Production Quality Requirements

> *Engineering-specific production quality requirements moved to [Engineering Plugin](docs/engineering-plugin.md#3-production-quality-requirements).*

For Tier 2+ projects (runtime, CLI, library, or performance-sensitive), consult the Engineering Plugin: fuzz targets, benchmarks, snapshot testing, CI matrix, test ratio, security audit. These gates must pass before a spec is execution-ready for engineering deliverables.

---

## Project Health Discipline

> **MANDATORY for every project with lifecycle-bearing state.** Makes "adding a feature" a safe, documented transition instead of a regression gamble — the answer to *every addition breaks something* because the project has no model of itself.

**Core requirement:** `docs/PROJECT_MODEL.md` exists and is current (mandated by SPECIFICATION.md § PROJECT_MODEL) — whole-project state machine, valid/invalid transitions, invariants, blast radius map.

**Seam registry (extends the blast-radius map):** record every known coupling seam — the cross-level boundaries where silent behavior change has happened (e.g. i18n activation races, re-render vs in-flight state, shared mutable arrays). Schema: seam ID, boundaries, co-change cluster, regression history (one line per incident). Update: whenever a regression traces to a seam or co-change analysis reveals a new sibling group. Check: the regression-per-seam rate is the metric that tells which architecture level to descend to (Escalation rule step 1).

**Enforced mechanisms (research, strongest first):**

| Mechanism | What it does | Evidence | Setup cost |
| --- | --- | --- | --- |
| **Full transition-table test** | Asserts every state × every event → resulting state or rejection. New features can't add unintended transitions. | Torkar: 26 real faults found, 85% test cost cut | 1-2 days |
| **Co-change analysis** | `git log --name-only` reveals which files change together — your real blast radius. Check siblings before changing one. | D'Ambros: correlates with defects, p=0.01 | 30 min script |
| **Layer/import rules** | Domain code imports only domain, enforced by lint. Violations fail the build loudly. | Fitness functions (Ford/Parsons/Kua) | 30 min |
| **Characterization tests** | Pin current behavior of legacy code before touching it. | Feathers, canonical legacy practice | minutes/file |
| **Mutation testing on core module** | Surviving mutants = "what regression could this have?" checklist. Run on core domain only. | FSE 2014 (gold-standard) | minutes/run |
| **Per-feature change-amplification check** | If a feature touches >5 files, ask "what interface should I have changed instead?" | Ousterhout, change amplification | 30 sec/feature |

**Where they run:** mechanisms are implemented in the PROJECT (CI/lint/tests). The protocol's job is to REQUIRE them; the FINISH gate CHECKs them.

### Architecture Fitness Audit (per level) + Escalation Rule

An architecture problem is not only "does structure exist for this concern" — it is whether the structure is the optimal GENERAL one for the class, at every level. Two failure modes: structure **absent** (no home for the concern) vs **malformed** (present, wrong class-fit).

| Level | Question | Check | Type |
| --- | --- | --- | --- |
| **MACRO** (meta) | Is the top paradigm optimal for the product class? (No style suits all problems — Shaw & Clements boxology) | Qualitative paradigm-fit gate: class-ID via Cynefin / Problem Frames; the MACRO decision records its class-ID reasoning + one falsification criterion | Human gate |
| **MESO** | Are component contracts right-shaped for their class? | Contract/invariant checks, interface rules, transition-table tests | Codified |
| **MICRO** | Is the implementation minimally correct for the class? | Layer/import rules, size rules, mutation testing | Codified |

Fitness functions verify COMPLIANCE with the chosen architecture — they cannot judge the choice itself; that is the MACRO gate's job. ATAM-style evaluation also runs within a class, not across classes.

**Escalation rule (when a problem is structural):**
1. **Diagnose top-down, never surface-first** — scan the **meta architecture first**, then descend one level at a time (meta → subsystem → feature) until the problem's home is found: the first level whose structure is wrong-shaped for its class. Never commit to a fix at the surface before this scan — surface fixes add bloat to the whole system.
2. **Bloat Test** — reject any fix that requires adding an exception, special case, or workaround: the structure at some level is forcing that bloat. The right fix removes the exception; the wrong fix adds it. A clean top-down scan (every level right-shaped) means the problem is genuinely local — fix locally, bloat-free.
3. **Recurrence signal** — recurring defects at the same boundary (change amplification, co-change clusters, every feature paying the same tax / Conway imprint) makes the scan mandatory, not optional.
4. **Fix at the found level, atomically and reversibly** (Strangler Fig / evolutionary architecture) — never rewrite-by-default.
5. **Gate:** modularity must be sound before an up-level fix — a bad modular monolith is not fixed by microservices (Simon Brown's boundary condition).

**Re-audit cadence:** every MACRO decision records a review date; a decision later reversed is marked superseded in its own record.

> **Source:** State-machine modeling and transition tests (Harel 1987; Torkar et al.), co-change prediction (D'Ambros et al., MSR 2010), fitness functions (Ford, Parsons, Kua), characterization tests (Feathers), mutation testing (Just et al., FSE 2014), change amplification (Ousterhout). Confidence: Medium-High — strongest on mutation testing and co-change prediction.

---

## Post-Execution: FINISH Gate (POLISH)

A protected sub-phase of EXECUTOR with its own budget and checklist. Catches what AI systematically misses — edge cases, integration surprises, last-mile problems. Research (architecture ~40% on construction docs; software ~50% on testing/debugging; film 10-30% post-production) confirms the finish phase is systematically under-budgeted; this gate protects it from substance-phase overruns.

### Entry Criteria
- All EXECUTOR milestones complete (spec fully implemented)
- All checkpoints passed and committed
- No blocking errors in the current state

### Budget
The FINISH gate gets **20-30% of EXECUTOR's total budget** (roughly 6-9% of total project appetite). Do NOT let it drop below 15% of EXECUTOR budget under substance-phase overruns.

### The Polish Checklist
Run through each category systematically. Do not skip categories — AI systematically overrates polish because it doesn't encounter real-world friction points.

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
1. **Hallucinated APIs** — references to packages/methods that don't exist (~1 in 5 AI snippets contain fake libraries).
2. **Security vulnerabilities that look functional** — code works but fails securely: auth bypasses, SQL injection, error handlers that leak sensitive data.
3. **Performance anti-patterns** — string concatenation in loops, nested O(n²) iterations, unnecessary allocations.
4. **Happy-path error handling** — try-catch that logs but doesn't recover; no fallback.
5. **Missing edge cases** — empty arrays, nulls, boundary integers, unicode inputs.
6. **Outdated library usage** — deprecated APIs from training data, obsolete before the AI wrote them.
7. **Data model mismatches** — assumes structures that don't match actual schemas; property access without type checking.
8. **Missing context dependencies** — env vars without fallbacks, undocumented config, assumed infrastructure.
Each unaddressed pattern becomes a human review item.

### Post-Ship Feedback Log (5 min, 3 items)
1. One thing the AI did surprisingly well
2. One thing the AI missed that cost significant time
3. One protocol change that would have caught it earlier
Feeds directly into protocol improvement without overhead.

### Exit Criteria
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
- [ ] Mutation testing run on core module — surviving mutants TRIAGED: equivalent → dismissed (documented); killed → coverage confirmed; real-gap → disposition recorded in ledger and BLOCKING at REVIEW (separate evaluator decides) on contract features; core-wide sweep stays a checklist. Mutation score is a coverage signal, not a gate — the hard BLOCK lives on BASELINES (golden-file diff), see Regression-Lock.
- [ ] Evidence-gated merge: every change merged with its evidence artifacts (tests, proofs, ledger entries) attached — no merge without evidence (v3-agent-standard)

### Regression-Lock (the differential lock against intent)
> **Runs at:** the FINISH gate + every subsequent change to an `applied` feature. The executable differential between declared intent and shipped reality — the answer to "everything breaks eventually".

A feature is not done when its code exists; it is done when its behavior is LOCKED against drift:
1. **Characterization/contract tests** — every `applied` feature's postconditions + acceptance scenarios (FEATURES.md) are pinned by tests that FAIL on regression (RULES §9 rule 7). A test that would pass on broken output is a tautology — kill it.
2. **Baselines (golden files / visual snapshots)** — for visual or output-shaped features, a golden baseline locks appearance/behavior. Baseline-update discipline: changing a baseline is a REGRESSION DECISION, not a ceremony — the update must be reviewed by a separate evaluator (never the worker who changed it) and recorded in the ledger with the reason. Updating a baseline to make a failing check pass without that review is regression laundering (Ithmb `a9962de`).
3. **Hard teeth:** REVIEW blocks SHIP if an `applied` feature's baseline changed without review, a contract test is missing, or a triaged real-gap mutant is unresolved. These run through the REVIEW Method Conformance + ledger conformance check (the sentinel that exists today); real-time harness hooks upgrade them later (harness handoff).

---

### 画蛇添足 Test (Universal Fundamental M1)

> **Runs at:** the FINISH gate, for every addition being finalized (feature, dependency, procedure, section). The completion-boundary check that catches scope creep before ship.

Before ANY addition ships, it must survive the 画蛇添足 test — from the Zhanguo Ce parable (drawing a snake, adding feet destroys the snake) and the converged universal fundamental M1: Simplicity-as-Removal (see docs/UNIVERSAL_FUNDAMENTALS.md):

> **"What does this addition BREAK that was already working?"**

If the answer is "nothing" — but the addition is also unnecessary (the artifact already satisfies its purpose), the addition fails anyway. The test is about completion boundary, not just safety:

| Criterion | Pass | Fail |
| --- | --- | --- |
| **Completeness** | The artifact does NOT yet satisfy its purpose | The artifact already satisfies its purpose — addition is 画蛇添足 |
| **Safety** | The addition breaks nothing already working | The addition breaks or destabilizes existing behavior |
| **Value** | The marginal quality gain is positive | The marginal quality gain is zero or negative |

**The deep counterweight (from M1's own counter-evidence):** redundancy is sometimes the point — safety-critical systems deliberately add. The test is NOT "never add". It is "addition beyond the completion boundary destroys value". Know where the boundary is.

**Structural form:** if an addition is *needed* because the system has no natural home for the concern — the fix would require an exception — apply the Bloat Test (Escalation rule): the problem is architectural, not the addition's.

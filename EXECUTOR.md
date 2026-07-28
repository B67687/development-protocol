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

The filled SPEC.md determines which RULES.md route to use. The mapping is NOT hardcoded to STANDARD — it depends on the spec's content:

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

## Error Handling

When a SPEC.md assumption fails during execution:

1. **Pause** — stop all implementation. Do not "work around" the failed assumption.
2. **Flag** — document what assumption failed, where in the spec it lives, and what evidence disproves it.
3. **Human decision** — present options: revise spec (minor), pause project (major), or work around (risky).
4. **Resume** — after human decision, update SPEC.md accordingly, commit as `revision: [reason]`, and resume checkpointing from section 1.

**Common failure modes:**

- Dependency X doesn't work as expected (flag to SPEC.md section 5)
- Architecture pattern doesn't fit (flag to section 2)
- Timeline is unrealistic (flag to section 7 — most common)

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

"Every cycle produces a shippable artifact." Not "progress" — a deployed, testable, usable thing.
**Source:** Shape Up (Singer 2019) — 6-week cycles produce shipped work, not "progress."

"Define the maximum time before designing the solution." The appetite (time budget) is fixed.
**Source:** Shape Up (Singer 2019) — appetite is the first decision, before solution design.

"AI-generated code passes the same structural checks as human-written code."
**Source:** Twist et al. (2025) — LLMs generate less maintainable code on average.
Metz (2016) — wrong abstractions compound. Therefore, AI code needs the same quality gates.

### 8. Abstraction Rule

"Rule of three — extract on the third occurrence, not the first." Write the first occurrence concretely. Duplicate on the second (it is okay). Abstract on the third — now you know the shape.

### 9. Dependency Rule

"Core domain never imports infrastructure." Core modules (business logic, types, algorithms) import zero infrastructure packages. All infrastructure access goes through a port (interface) defined in the core and implemented by an adapter in the infrastructure layer.

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

These requirements apply to Tier 2+ projects (those with a runtime, CLI, library, or performance-sensitive component). They are not optional polish — they are baseline quality gates that must pass before a spec is considered execution-ready.

1. **Fuzz targets**: Every Tier 2+ project should have a `fuzz/` directory with at least one libFuzzer target. Run `cargo fuzz init` and add a basic target for the core API surface. Fuzz targets catch memory safety issues, panics, and undefined behavior that unit tests miss. Wire fuzz runs into CI as a scheduled job (not per-PR — too slow).

2. **Benchmarks**: Every performance-sensitive component should have a benchmark in `benches/`. Use `criterion` (statistical rigor) or `divan` (lower overhead). Track results in CI and fail on regressions beyond a configurable threshold. A benchmark without trend tracking is just a numbers game.

3. **Snapshot testing**: Use `insta` (Rust), `snapbox` (CLI output), or `expect_test` for golden-file assertions. Snapshot tests are far more efficient than hand-writing assertions for complex output — they catch regressions you didn't know to test for and make reviewing output changes trivial (just `cargo insta review`).

4. **CI matrix**: SPEC.md section 4 CI gates should cover Linux, macOS, and Windows at minimum. For Rust projects, add stable/beta/nightly to the matrix. Nightly failures are informational (not blocking), but beta failures are warnings that become blockers in the next release cycle.

5. **Test ratio**: Minimum 0.5× test-to-source lines. Measure via `cloc --json src/ tests/` or equivalent. This is a floor, not a target — the real metric is mutation score, but line ratio is a cheap proxy that catches projects with no tests at all.

6. **Security audit**: Add `cargo-deny` with a `deny.toml` to CI. Check for unmaintained dependencies, license compliance (no GPL in MIT-shipped crates), and known advisories. Run on every PR. A security audit that only runs before release is a security audit that misses everything merged in between.

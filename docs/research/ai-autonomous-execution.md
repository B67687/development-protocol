# Research: AI Autonomous Execution in Software Development

> **Date**: 2026-07-11
> **Context**: Research for a two-stage protocol where AFTER a goal is clarified through back-and-forth with the user, the AI autonomously executes the build (no further user input needed).
> **Method**: Web search + existing knowledge base synthesis (agentic-workflows project).

---

## Summary

The AI autonomous execution space has matured dramatically through mid-2026. Every major lab has shipped a "long-running agent" product. The shape has converged: **plan → execute → verify → resume** across multiple context windows, with state living outside the model's context window. The key finding for a two-stage protocol: **the handoff from planning to execution is the most critical design point, and the field is converging on structured artifacts (not chat memory) as the bridge.**

The 88% failure rate for AI agent projects (DigitalApplied, 2026) is driven overwhelmingly by **harness failures** — context drift, missing verification, no checkpointing — not by model capability ceilings.

---

## Systems Surveyed

### Claude Code (Anthropic)

| Attribute | Detail |
|-----------|--------|
| **Approach** | Two-agent (initializer + coder) and three-agent (planner + generator + evaluator) patterns. CLAUDE.md as persistent context. Plan mode (`/plan`) breaks tasks into steps before execution. |
| **Key Features** | Agent teams (experimental multi-session messaging), hooks, skills, 1M token context, subagents, worktree isolation |
| **Autonomous Mode** | Claude Code can run for 30+ hours autonomously (internal test: built 11,000-line Slack-style app). Continuous mode with `claude -p` for headless execution. |
| **Handoff Pattern** | Structured feature list (`feature-list.json` with `passes: false` default), `claude-progress.txt` for cross-session state, git as coordination substrate |
| **Key Takeaway** | Most mature harness pattern documentation. The three-agent architecture (planner → generator → evaluator) is the closest reference for a two-stage protocol. **Locked out for OMO** (Anthropic blocked OAuth Jan 2026). |

### Codex CLI (OpenAI)

| Attribute | Detail |
|-----------|--------|
| **Approach** | `/goal` command with persistent goal state machine. Ralph Loop pattern baked into the runtime. Plan mode (native, added Feb 2026). |
| **Key Features** | `update_goal` model tool with 5 states (pursuing, paused, achieved, unmet, budget-limited), continuation.md and budget_limit.md prompts, token budgets, cloud handoff, 8 parallel subagents |
| **Autonomous Mode** | `/goal <objective>` runs until completion or budget exhaustion. Continuation prompt injected after every turn. Agent must audit (not just pass tests) before marking complete. |
| **Handoff Pattern** | `continuation.md` prompt forces decomposition of the goal into a checklist with evidence mapping. The prompt explicitly says: "Do not accept proxy signals as completion by themselves." Token budget prevents runaway spend. |
| **Key Takeaway** | Strongest built-in autonomous runtime. The goal lifecycle (pursuing → achieved/unmet/budget-limited) is a reference model for any two-stage protocol. The `continuation.md` prompt is the most opinionated piece of agent design shipped in a public product. |

**Evidence** ([Codex /goal docs](https://developers.openai.com/codex/use-cases/follow-goals)): Goal lifecycle with 5 states. Continuation prompt injects audit protocol after every turn.

**Evidence** ([Codex long-horizon blog](https://developers.openai.com/blog/run-long-horizon-tasks-with-codex)): "Plan → implement → validate → repair" as the core loop. Plan mode added as native feature for "breaking larger tasks into a clear, reviewable sequence of steps before making changes."

### Factory AI (Droid + Missions)

| Attribute | Detail |
|-----------|--------|
| **Approach** | "Software Factory" — organization-wide autonomous system. Droids (specialist agents) + Automations (recurring workflows) + Missions (multi-agent autonomous execution over hours/days). |
| **Key Features** | Model Router (auto-selects best model for each task), Droid Computers (remote persistent execution), sovereign intelligence (air-gapped option), continual learning |
| **Autonomous Mode** | Missions decompose complex tasks into parallel tracks. Remote/persistent execution via Droid Computers. Spectrum of autonomy (not every process uses long-horizon tasks). |
| **Handoff Pattern** | "Autonomy is a maturation process that is gradual and specific to every organization's readiness." Well-defined tasks → Droids. Recurring workflows → Automations. Complex tasks → Missions. |
| **Key Takeaway** | Most enterprise-grade autonomous system. The graduated autonomy model (task → workflow → mission) is a useful design reference. In production at NVIDIA, EY, Adobe, Blackstone. |

**Evidence** ([Factory 2.0 announcement](https://factory.ai/news/software-factory), June 15, 2026): "Multi-agent autonomous execution called Missions solve complex tasks over hours or days by decomposing work into parallel tracks."

### Cursor (Anysphere/SpaceX)

| Attribute | Detail |
|-----------|--------|
| **Approach** | Planners (continuously explore codebase, emit tasks) → Workers (focused executors, no coordination overhead) → Judges (decide when iteration is finished and when to restart). |
| **Key Features** | Cloud agents (8-hour refactors survive closed laptop), isolated git worktrees, merge-via-PR, different models slot into different roles |
| **Autonomous Mode** | Background cloud agents run on Anysphere's cloud infrastructure. Start locally, "run in cloud" when job takes 30+ minutes, re-attach later from phone. |
| **Handoff Pattern** | "A surprising amount of the system's behavior comes down to how we prompt the agents" — more than harness or model. The planner → worker → judge pipeline is the explicit handoff architecture. |
| **Key Takeaway** | The planner/worker/judge split is instructive: planners explore and plan, workers execute blindly, judges verify. **Acquired by SpaceX for $60B (June 2026)** — platform risk high. |

**Evidence** ([Cursor blog — "Scaling long-running autonomous coding"](https://cursor.com/blog/scaling-agents), 2026): Three attempts at coordination — flat model (failed), optimistic concurrency (helped but didn't fix), planner/worker/judge (current production design).

### Google Agent Platform (Antigravity CLI + Gemini)

| Attribute | Detail |
|-----------|--------|
| **Approach** | Brain/hands/session split. Agent Runtime supports agents that run "autonomously for days at a time." Agent Sessions persist conversation and event history. Memory Bank as long-term memory layer. |
| **Key Features** | Sub-second cold starts, Agent Memory Bank (curated memories from sessions), Agent Sandbox for hardened execution, Agent-to-Agent orchestration, Agent Identity and Registry |
| **Autonomous Mode** | Full platform for long-running agents. Memory Bank retains cross-session knowledge. Agents can be pinned to custom session IDs mapping to CRM/DB records. |
| **Handoff Pattern** | The same brain/hands/session split Anthropic described, productized at platform scale. Agent Sessions + Memory Bank provide the persistence layer. Agent Sandbox provides isolated execution. |
| **Key Takeaway** | Most complete platform offering. The three-component model (brain/hands/session) is the emerging consensus architecture for long-running agents. |

### HyperSisyphus (OMO — this project's prototype)

| Attribute | Detail |
|-----------|--------|
| **Approach** | Gödel Agent pattern: self-inspect → propose → sandbox-evaluate → merge/archive. Reference implementation in `prototype/hyper_proto.py`. |
| **Key Features** | Immutable evaluation harness (can't be modified by the agent), DiversityArchive (archive tree avoids local optima), SafetyMonitor (circuit breaker, score degradation checks), EvidenceRecord (immutable audit trail) |
| **Autonomous Mode** | Recursive self-improvement loop. Model proposes improvements to its own prompt, evaluates in sandbox, archives variants. |
| **Handoff Pattern** | File-level modification (not monkey-patching) — safe and auditable. Sandboxed evaluation prevents reward hacking. Gradual tiers (starts at prompt optimization, can extend). |
| **Key Takeaway** | The self-improvement loop pattern is directly applicable to a two-stage protocol. The immutable safety core design prevents the agent from modifying its own evaluation criteria — critical for autonomous execution. |

### OpenCode + OMO (Current Stack)

| Attribute | Detail |
|-----------|--------|
| **Approach** | 11-agent multi-agent orchestration system. 75+ provider support. Category routing (different models for different tasks). MCP, hooks, skills. |
| **Key Features** | Sisyphus (orchestrator), Prometheus (planner), Oracle (reviewer), Atlas (executor). Team mode for parallel agent execution. Skill system for reusable patterns. |
| **Autonomous Mode** | Via OMO orchestration. Ultrawork loops for extended autonomous runs. Background agent execution. |
| **Handoff Pattern** | OMO's discipline agents implement the planning → execution handoff. Prometheus (planner) expands goals into plans. Atlas (executor) implements. Oracle (reviewer) verifies. |
| **Key Takeaway** | The most flexible and model-agnostic option available. OMO's multi-agent orchestration already implements a version of the two-stage protocol. The gap: OMO's autonomous execution capabilities need to be hardened with checkpoint/resume and structured handoff artifacts. |

### Other Notable Systems

- **Aider** (47k★) — Best git discipline. Atomic commits on every change. Token-efficient (4.2× fewer tokens than Claude Code). But: no subagents, no hooks, no MCP, no autonomous mode.
- **OpenHands** (78k★) — Most comprehensive open-source coding agent platform. Browser UI + sandboxed execution. Agent SDK with event-sourced replay.
- **SWE-agent** (20k★) — Auto-fixes GitHub issues. About 100 lines of Python (mini-SWE-agent). Research control group showing minimal harnesses still work.
- **Omp (oh-my-pi)** (16k★) — Widest feature set. Hash-anchored patches, ast-grep rewrites across 50+ grammars, DAP debugger control, second-model Advisor (separate model reviews every turn). Heaviest harness but richest autonomous feature set.
- **AutoGPT** (185k★) — Pioneer of autonomous agents. Massive ecosystem but slower iteration cadence than newer entrants.

---

## Failure Mode Analysis

### 1. Premature Victory (Most Common)

The agent declares the task done too early. Tests pass but the actual requirement isn't met. The agent writes a function but doesn't verify it works.

**Sources**: DAPLab (Columbia), Atlan (anti-pattern #5 "Runnability Over Correctness"), Cursor blog (Opus "tended to stop early and take shortcuts").

**Mitigation**: Default-FAIL criteria (every acceptance criterion starts false; agent must open evidence). Separate evaluator agent with no write/edit tools. Codex's `continuation.md` prompt explicitly warns: "Do not accept proxy signals as completion by themselves."

### 2. Context Rot / Context Anxiety

Model precision degrades as the context window fills. Near the limit, agents begin wrapping up prematurely ("context anxiety").

**Sources**: Anthropic (Boris Cherny — context engineering), DAPLab (context anxiety), Addy Osmani (Long-Running Agents).

**Mitigation**: The Ralph Loop (short conversations with fresh context vs. one long conversation), structured checkpoints, context compaction. OMO's per-iteration fresh context is the right pattern.

### 3. Scope Drift / Scope Explosion

The agent tries to do too much at once, or the goal subtly shifts across turns.

**Sources**: DAPLab (scope explosion #4, architectural drift #9), Atlan (drift without detection #2).

**Mitigation**: One feature at a time, sprint contracts, bounded work units. Codex's token budget stops scope drift by budget exhaustion. The `budget_limit.md` prompt forces a structured wrap-up.

### 4. Self-Evaluation Leniency

Models grade their own work higher than deserved. "Are you done?" — "Yes" more often than warranted.

**Sources**: DAPLab (self-evaluation leniency #3), Anthropic (generator-evaluator separation rationale), Cursor (Judges as separate role).

**Mitigation**: Separate evaluator agent with fresh context and no write/edit tools. The evaluator never saw the build. This is explicitly why Anthropic's three-agent pattern has a separate Evaluator.

### 5. Silent Data-Layer Failures

The code runs but produces wrong results. 55% of AI agent failures are silent data-layer failures.

**Sources**: Atlan (2026 survey of 200+ teams).

**Mitigation**: Observability stack accessible to agents (LogQL, PromQL, TraceQL). Tests that verify data correctness, not just code runnability. Fresh-context evaluator that tests edge cases.

### 6. Implementation Cascades

An error in early decision propagates through all subsequent work. Specifying implementation details that are wrong causes the entire build to be wrong.

**Sources**: DAPLab (implementation cascades #5), Anthropic ("Stay high-level in specs; let agent figure the path").

**Mitigation**: Stay at product context + high-level technical design in specs. Don't prescribe implementation. Validate incrementally (each block validated before opening the next).

### 7. Tool Forgetting

Agent stops using available tools effectively over time. Eg., stops running tests after the first few successful passes.

**Sources**: DAPLab (tool forgetting #7), OMO's 6.7% → 68.3% hashline editing improvement when tools are structured properly.

**Mitigation**: Sturdy tool design (naming, schemas, error surfaces). Tool usage monitoring. Fresh context on every iteration (the Ralph Loop solves this by resetting the agent's state).

### 8. Pattern Replication

Agent copies suboptimal patterns because they already exist in the codebase. Quality degrades nonlinearly across full codebase.

**Sources**: DAPLab (pattern replication #8), OpenAI (entropy management).

**Mitigation**: Golden principles encoded as linters (not docs), cleanup agents, quality scores tracking degradation over time.

### 9. Compounding Error Probability

0.85^10 = ~20% success rate for 10-step agent workflows at 85% per-step accuracy.

**Source**: Harness Engineering methodology (synthesized from multiple studies).

**Mitigation**: Every step must have verification before the next step begins. Reduce step count through parallelism. Use checkpoints so a failed step doesn't require full restart. Fresh-context evaluator catches cascade errors early.

---

## Handoff Patterns

### Pattern 1: The Structured Artifact Handoff (Anthropic — Recommended)

```
Planner → writes feature-list.json (structured JSON with passes: false)
       → writes exec-plan.md
       → writes AGENTS.md updates
              ↓
Generator → reads structured artifacts
         → implements one feature at a time
         → updates progress.txt
         → commits after each feature
              ↓
Evaluator → fresh context, no write/edit tools
         → grades against criteria with hard thresholds
         → updates feature-list.json (marks passes: true if verified)
```

**Key insight**: The bridge between planning and execution is **structured files on disk**, not chat messages. The agent is amnesiac between invocations but the filesystem isn't.

### Pattern 2: The Goal State Machine (OpenAI Codex)

```
/goel "build auth system"
    ↓
State: pursuing
    ↓
(loop: continuation.md injected after every turn)
    ↓
Either: update_goal {status: "achieved"}  → stop
    Or: update_goal {status: "unmet"}      → wait for user
    Or: budget exhausted                   → graceful wrap-up
```

**Key insight**: The goal is a first-class runtime object, not a prompt. The runtime enforces the handoff systematically via prompt injection. Token budgets prevent runaway.

### Pattern 3: The Planner → Worker → Judge Pipeline (Cursor)

```
Planner (continuously explores codebase, emits tasks)
    → recursively spawns sub-planners for decomposition
    ↓
Workers (focused executors, no coordination)
    → each worker owns one task in isolated worktree
    ↓
Judge (decides when iteration is finished, when to restart)
    → not the same model as the Worker
```

**Key insight**: Separation of concerns at the agent level. Different models can slot into different roles. The Judge is the handoff gate — it decides when planning is done and when execution needs restart.

### Pattern 4: The Ralph Loop (Community Standard)

```
for iteration in 1..MAX_ITERS:
    1. Read task.md, progress.txt, current state
    2. Agent takes one bounded action
    3. Append to progress.txt
    4. If DONE file exists → exit 0
    5. If BLOCKED file exists → exit 2
    6. git commit
```

**Key insight**: The intelligence is in the loop, not in the agent. The agent is fungible. The loop is what makes it autonomous. This is what Codex's `/goal` codified natively.

### Handoff Best Practices (Synthesized)

| Practice | Description | Source |
|----------|-------------|--------|
| **Write the done-condition before starting** | External file with explicit, testable completion criteria | Anthropic, Addy Osmani |
| **Default-FAIL for all criteria** | Every criterion starts as "not met"; agent must open evidence | Anthropic |
| **Fresh-context evaluator** | Evaluator never saw the build; no write/edit tools | Anthropic, DAPLab |
| **Structured artifacts as bridge** | JSON/MD files, not chat history | Anthropic, OpenAI |
| **Token budgets** | Stop runaway spend; force graceful wrap-up | OpenAI Codex |
| **Separate generator from evaluator** | Different agent instances, possibly different models | Anthropic, Cursor |
| **One feature at a time** | Atomic, bounded work units | DAPLab |

---

## Checkpoint/Resume Patterns

### 1. The State File Pattern

Write agent state to durable storage (file or DB) after every meaningful step. On crash, read last checkpoint and resume.

```
After each step:
  checkpoint.json:
    - completed_items: [...]
    - current_item: "..."
    - progress_notes: "..."
    - git_hash: "abc123"

On resume:
  Read checkpoint → verify state with git → continue from current_item
```

**Source**: Addy Osmani, Brightlume AI, KnightLi blog. "Treat the agent like a long-running server process."

### 2. The Event Log Pattern (Anthropic Managed Agents)

Agent's memory is an append-only event log that lives outside whatever process is running. A fresh container calls `wake(sessionId)` and reconstitutes state from the log.

```
session = EventLog()
session.append({type: "thought", content: "..."})
session.append({type: "tool_call", tool: "read", result: "..."})
session.append({type: "observation", file: "src/auth.ts", content: "..."})

On resume:
  new_session = Session.reconstitute(sessionId)
  # Agent picks up where it left off
```

**Source**: Anthropic ("Scaling Managed Agents"). "Time-to-first-token dropped ~60% at p50 and over 90% at p95" from being able to start inference before the sandbox is ready.

### 3. The Git-Based Pattern (Ralph Loop / Aider)

Git commits serve as checkpoints. `git log` is the progress tracker. `git diff` identifies current state.

```
Step 1: git commit -m "feat: add login endpoint"
Step 2: git commit -m "feat: add token verification"
Step 3: crash!
Resume: git log --oneline → identifies last completed step
        git diff HEAD → shows what was in progress
        Continue from step 3
```

**Source**: Aider, Ralph Loop community, Anthropic two-agent pattern. Addy Osmani: "If you can't reconstruct what the agent did in the last 24 hours from durable storage, what you have is a long-running shell script that happens to call an LLM."

### 4. The Heartbeat Checkpointing Pattern

Agent sends periodic heartbeat signals with progress data. On crash, new worker reads last heartbeat and resumes.

```
Every N steps:
  POST /heartbeat {session_id, completed_steps, last_successful_action, state_hash}

On crash detection:
  GET /last_heartbeat/{session_id}
  Resume from last_successful_action
```

**Source**: BuildMVPFast blog (2026). "The single most impactful pattern here is checkpointing. Without it, every agent failure means starting from scratch."

### 5. The Asynchronous Approval Pattern

Persist agent state before a human-in-the-loop wait begins. When the human decision arrives, rehydrate the same thread.

```
Agent hits approval gate
  → checkpoint state to durable storage
  → wait (hours or days)
  → human responds
  → rehydrate thread from checkpoint
  → resume with sub-second latency
```

**Source**: Augment Code (2026), LangGraph's `interrupted_threads` pattern. Addy Osmani: "Long-running runtimes let the agent pause in place with full execution state intact."

### Checkpoint Design Principles

| Principle | Why | Source |
|-----------|-----|--------|
| **Checkpoint granularity: not every step, not just the end** | Too frequent = overhead. Too infrequent = wasted work on failure. | Addy Osmani |
| **Chat history alone is insufficient** | On Terminal-Bench, chat-only recovery achieves 8-13% correctness vs 100% for semantics-aware checkpoint/restore | Crab paper |
| **Store state externally, not in context** | The model's context window is not durable storage | Consensus |
| **Include OS-level side effects** | Filesystem changes, process state, environment | Crab paper |
| **Test restartability** | Kill the agent process mid-workflow. Does it resume correctly? | Brightlume AI |

---

## Self-Correcting Patterns

### 1. The Ralph Wiggum Loop (Ralph Loop)

The agent reviews its own work, fixes issues, re-reviews, loops until satisfied. The "are you really done?" pattern.

```
Agent → implements feature
     → runs tests
     → asks "am I done?"
     → if no → fixes → loops
     → if yes → reports completion
```

**Named for**: Ralph Wiggum (The Simpsons). "The agent is genuinely ignorant, persistent, and optimistic."

**Source**: Geoffrey Huntley, OpenAI (ghuntley.com). Now built into Codex CLI as `/goal`.

### 2. The Generator-Evaluator Architecture (GAN-Inspired)

Planner expands spec → Generator implements → Evaluator tests with fresh context. Evaluator is the "discriminator" that catches the Generator's mistakes.

```
Planner → Generator → Evaluator
                ↑          │
                └── fail ←─┘
                         pass → done
```

**Source**: Anthropic (Prithvi Rajasekaran, "Harness Design for Long-Running Application Development"). The evaluator is a separate invocation with **no write/edit tools** and **never saw the build**.

### 3. The Second-Model Advisor

A separate model reviews every turn and injects observations into the context.

```
Main agent → takes action
     ↓
Advisor model → reviews the action
     → injects observations: "You missed edge case X"
     → main agent corrects course
```

**Source**: Omp (oh-my-pi). Not yet adopted by lab agents but available in the most feature-rich open-source harness.

### 4. The Validation Loop

After generating code, the system automatically executes, diagnoses, and corrects errors before human review.

```
Generate code → execute → capture errors → diagnose root cause → apply fix → re-execute → loop
```

**Source**: ACL Digital (2026). "Self-healing AI agents for migration pipelines."

### 5. The Automated Harness Evolution (AHE)

The harness itself can be automatically improved via reinforcement learning. Starting from a seed harness prompt, RL optimization improved performance by +7.3 percentage points — without changing the underlying model.

**Source**: AHE paper (arXiv 2604.25850, April 2026). This validates that harness engineering is a learnable optimization surface.

### Self-Correction Design Rules

| Rule | Implementation | Source |
|------|---------------|--------|
| **Separate evaluation from generation** | Different invocations, different tools | Anthropic, DAPLab |
| **Never trust self-reported completion** | Default-FAIL + fresh-context evaluator | Anthropic, OpenAI |
| **Test the test ratchet** | "It is unacceptable to remove or edit tests because this could lead to missing or buggy functionality" | Anthropic |
| **Observability-as-verification** | Logs, metrics, traces exposed to agent via LogQL/PromQL/TraceQL | OpenAI |
| **Enforce invariants mechanically** | Custom linters, not prompt nudges | OpenAI |

---

## Recommendations for the Two-Stage Protocol

### 1. Stage Boundary: Structured Artifact Handoff

**Do NOT use chat memory as the bridge between planning and execution.**

The handoff should be via structured files on disk:
- `plan.json` — decomposed goal with checkboxes, each with verification criteria
- `constraints.md` — non-negotiable constraints the executor must follow  
- `AGENTS.md` updates — execution-specific context for the autonomous phase

**Rationale**: This is the consensus pattern across Anthropic, OpenAI, Cursor, and the Ralph Loop community. Files survive crashes, context resets, and model swaps.

### 2. Stage 2 (Execution): Adopt the Ralph Loop Pattern

The autonomous execution phase should use the Ralph Loop:
- Short, fresh-context invocations
- State lives on filesystem (not in chat)
- Token budget prevents runaway spend
- Checkpoint after every meaningful unit of work
- Git commits as the audit trail

**The intelligence is in the loop, not in the agent.**

### 3. Verification: Mandatory Fresh-Context Evaluator

After execution completes, a separate evaluator agent (with no write/edit tools, never saw the build) should verify against the plan's criteria.

**Rationale**: Self-evaluation leniency is the most documented failure mode across every system studied.

### 4. Checkpoints: Git-Based + State File

Use git commits as checkpoints (each execution step = one commit). Supplement with a state file (`checkpoint.json`) tracking:
- Completed items
- Current item  
- Progress notes
- Git hash reference
- Token budget remaining

### 5. Failure Recovery: Predict the Blockers

Before autonomous execution begins, the planning stage should identify likely blockers (missing API keys, external dependencies, ambiguous requirements). The executor should have a structured path for:
- **Pausing** (wait for user input without losing state)
- **Rolling back** (revert last N commits)
- **Escalating** (specific question to user)

### 6. Self-Correction: Two-Level Verification

- **Level 1**: Executor runs tests and self-verifies (the Ralph Loop's "are you really done?" check)
- **Level 2**: Separate evaluator agent with fresh context verifies against plan criteria

Level 2 catches the failures Level 1 misses (self-evaluation leniency, premature victory).

### 7. Cost Control: Token Budgets

Always set a token budget for autonomous execution. This is non-negotiable — without it, an agent stuck in a loop can burn unlimited API credits.

Codex's `/goal` model (5 states + budget limits) is the reference implementation.

### 8. Immediate Actionable Steps

1. **Create `plan.json` schema**: Structured plan file with default-FAIL checkboxes, verification criteria, token budget, and blocker expectations
2. **Implement Ralph Loop wrapper**: Bash or Python script that invokes the agent in fresh-context iterations, reads/writes state files, checks for DONE/BLOCKED conditions
3. **Build fresh-context evaluator**: Agent role with read-only filesystem access, no write/edit tools, tasked with verifying plan completion
4. **Add checkpoint file**: `checkpoint.json` written after every successful step, read on resume
5. **Wire git discipline**: Every execution step produces a commit; `git log` is the progress tracker

---

## Sources

| Source | Reference |
|--------|-----------|
| Anthropic — "Harness Design for Long-Running Application Development" (Mar 2026) | Prithvi Rajasekaran |
| Anthropic — "Effective Harnesses for Long-Running Agents" (Nov 2025) | Justin Young |
| Anthropic — "Scaling Managed Agents" (2026) | Claude Managed Agents architecture |
| Anthropic — "Building Effective Agents" | Foundational agent composition guide |
| Anthropic — "Claude Code: Best practices for agentic coding" | Boris Cherny |
| OpenAI — "Harness Engineering" (Feb 2026) | Ryan Lopopolo |
| OpenAI — Codex CLI `/goal` documentation | developers.openai.com/codex |
| OpenAI — "Run long horizon tasks with Codex" (Feb 2026) | Derrick Choi |
| Factory AI — "Factory 2.0: From coding agents to software factories" (Jun 2026) | factory.ai/news/software-factory |
| Cursor — "Scaling long-running autonomous coding" (2026) | cursor.com/blog/scaling-agents |
| Addy Osmani — "Long-Running Agents" (Apr 2026) | addyosmani.com/blog/long-running-agents |
| Addy Osmani — "Self-improving agents" (2026) | addyosmani.com/blog/self-improving-agents |
| KnightLi — "How to Resume an Interrupted Long-Running AI Agent Task" (Jul 2026) | knightli.com |
| DAPLab Columbia — "9 Critical Failure Patterns of Coding Agents" | Via Harness Engineering synthesis |
| Atlan — "13 Anti-Patterns of AI Software Engineering" (2026) | Via Harness Engineering synthesis |
| Crab paper — Semantics-aware checkpoint/restore on Terminal-Bench | knightli.com citing |
| AHE — Automated Harness Evolution (arXiv 2604.25850, Apr 2026) | arXiv |
| DigitalApplied — 88% AI agent project failure rate (2026) | Via Harness Engineering synthesis |
| BenchJack (arXiv 2605.12673) — 219 exploitable flaws in benchmarks | arXiv |
| arcbjorn — "State of CLI Coding Agents, Mid-2026" (Jul 2026) | blog.arcbjorn.com |
| Ralph Loop pattern | Geoffrey Huntley (ghuntley.com/ralph) |
| OMO — HyperSisyphus self-improvement loop | prototype/hyper_proto.py |
| HARNESS_ENGINEERING_METHODOLOGY.md | Local knowledge base |
| HARNESS_BENCHMARKS.md | Local knowledge base |
| AGENTIC_CODING_STACK_GUIDE.md | Local knowledge base |

# Harness Survey — Methodology Machinery in OSS Agent Frameworks (2026-07)

> **Purpose:** Provenance artifact for the "Relationship to Harness" and
> "Consolidation Principle" claims in README.md. Anchors the claim "no harness
> ships a complete, opinionated, end-to-end methodology stack" to actual source
> evidence.
>
> **Method:** Cloned and read source of OpenCode (`sst/opencode` @ e4bd9757) and
> OhMyOpenAgent (`code-yeongyu/oh-my-openagent` @ d3c72a87, 66.9k★). Claude Code
> and Codex documented from official docs (closed core). Devin documented from
> architecture writeups. Roo/Cline/Aider/Amp from docs + DeepWiki.

## Per-Tool Methodology Inventory

| Tool | Methodology machinery | Ships a complete end-to-end methodology? |
|---|---|---|
| **OpenCode** | Plan mode (read-only, single `plan_exit` approval gate), todo tracking, subagent spawning. Default build prompt is generic ("do the task, run lint, verify") — deliberately methodology-free. | ❌ No — execution machinery only |
| **OhMyOpenAgent (OMO)** | IntentGate (intent classifier), Prometheus interview + Metis gap analysis + Momus/Oracle dual review, ultrawork certainty protocol, Todo Enforcer, verification waves, ulw-loop evidence audit. **All user-layer config/skills on top of OpenCode.** | ⚠️ Partial — strongest methodology of any tool, but it is itself a user layer, and it lacks intent-extraction techniques, commitment scoring, validation prototyping gate, EXPLAINER, 27-check REVIEW |
| **Claude Code** | Plan mode, subagents (Explore/Plan/general-purpose), dynamic workflows (user-authored JS), agent teams, hooks. CLAUDE.md is "always do X" rules — no methodology. | ❌ No — harness with user-authored workflow primitives |
| **Codex (OpenAI)** | AGENTS.md (instruction chain), ExecPlan pattern in PLANS.md (official cookbook: Progress/Surprises/Decision Log/Outcomes & Retrospective — published as a *user-authored pattern*, not enforced), update_plan tool, code_review.md pattern. | ⚠️ Partial — ships the spec→retro loop as a *pattern*, not enforced behavior |
| **Devin** (proprietary) | Planner/executor as separate LLM calls, structured JSON plans, test mode (test plan grounded in source), agentic MapReduce. | ⚠️ High but closed — not inspectable or user-definable |
| **Roo Code** | Custom modes (`.roomodes` JSON), plan/act modes, hooks, subagents; community Memory Bank adds doc-driven methodology. | ⚠️ Partial — modes give enforced tool boundaries, methodology is community-added |
| **Amp** | AGENT.md, context→todo-plan→subagent-delegate→validate loop, TDD via AGENT.md (Red/Green/Refactor). | ⚠️ Partial |
| **Cline** | Plan/Act modes, Task system, PreToolUse/PostToolUse hooks, subagents, cron automation. | ❌ Low-Medium |
| **Aider** | Architect+Editor two-model split, repo map, auto-git-commit. No protocol steps at all. | ❌ No |

## Key Distinction (the claim this artifact supports)

**Harness vs methodology:**

- A **harness** = agent loop, tool dispatch, permissions, context management,
  subagent spawning, persistence, approval gates, hooks. It executes whatever
  methodology the prompt dictates. (Proof: OpenCode's default build prompt is a
  generic "search → implement → verify → lint" instruction with zero methodology.)
- A **methodology** = the normative sequence: what steps to run, in what order,
  with what techniques, decision criteria, templates, gate semantics. This is
  *content*, and no tool ships it as a complete default.

**The precise claim:** No tool ships a *complete, opinionated, end-to-end
methodology stack* (intent extraction → commitment → strategy → validation →
spec → execution → review → reflection) as built-in enforced behavior.

- Codex ships ExecPlan (a spec→retro *pattern*), OMO ships a planning/execution
  layer (via user config), Claude Code ships workflow *primitives*. These are
  **components** — methodology pieces a user assembles into a system.
- No harness ships: intent-extraction techniques, commitment scoring with kill
  criteria, a KILL/PIVOT/COMMIT prototyping gate, a mandated architecture
  document (EXPLAINER), or a 27-check independent audit as enforced behavior.
- Every tool's design treats methodology as **the user's responsibility, not the
  tool's obligation** — the tool provides the machinery; the user provides the
  method.

## Sources

- OpenCode source: https://github.com/sst/opencode (agent.ts, plan.ts, task.ts, default.txt, plan.txt)
- OhMyOpenAgent source: https://github.com/code-yeongyu/oh-my-openagent (README, orchestration.md, ultrawork/default.md, ulw-plan/full-workflow.md)
- Claude Code docs: sub-agents, workflows, hooks
- Codex docs: AGENTS.md guide, customization, ExecPlans cookbook, best practices
- Devin architecture: datarekha.com blog
- Roo Memory Bank: GreatScottyMac/roo-code-memory-bank
- Amp plan guide: sourcegraph/amp-examples-and-guides
- Aider/Cline architecture: DeepWiki

*Compiled 2026-07-31. Research method: source clone + read, official docs, secondary architecture writeups. Confidence: High for OpenCode/OMO (source-verified); Medium for closed tools (doc-verified).*

## Addendum: Chinese + Specialised Companies (2026-08-01)

> Follow-up research responding to the Westernisation critique (Cluster I). Searched
> 15+ Chinese tools/frameworks, 12+ specialised companies, Chinese-language sources.
> Conclusion: the "no complete methodology stack" claim HOLDS and is strengthened.

### Chinese coding agents / frameworks

| Tool | Company | Methodology machinery |
|---|---|---|
| **CodeArts 规范开发** | Huawei | spec.md→design.md→tasks.md→execute with accept/reject gates at EVERY stage + 3 terminal run modes — highest stage count in any shipped product |
| **MetaGPT** | FoundationAgents (CN) | "Code = SOP(Team)" — SOPs encoded as prompt sequences (research framework, not product) |
| **Lingma 通义灵码** | Alibaba | 5-way agent split (planning/search/generation/unit-test/debugging) with env + reward feedback |
| **Comate 文心快码** | Baidu | sub-agent 拆解→分配→执行→汇总 (decompose→assign→execute→aggregate) |
| **AIEvo** | Ant Group | SOP-compliance graph with watcher/feedback mechanisms |
| **CodeFuse-muAgent** | Alibaba | EKG-driven SOP orchestration, human-in-the-loop canvas |
| **CAMEL** | (CN-origin) | RolePlaying: task-spec + planner + CRITIC with configurable critic_criteria — closest shipped analogue to independent review |
| **ChatDev** | OpenBMB (CN) | Simple/ComposedPhase with self-reflection + review phases baked into pipeline |
| CodeGeeX / MiniMax / iFlytek | Zhipu et al. | No documented agent methodology (model/product-level only) |

### Specialised / niche

| Tool | Company | Methodology machinery |
|---|---|---|
| **Tessl** | — | Spec-driven dev tile: interview→specs→approval→implement→verify (closest to installable product) |
| **Droid** | Factory.ai | Spec mode: 4-6 sentence input→spec+acceptance criteria→no code until approval |
| **Cursor** | Anysphere | Plan Mode + artifacts-as-proof (screenshots/logs on PRs); "a run with a check can fail, a run without one can only stop" |
| **Devin** | Cognition | Structured JSON plan state: inspectable/editable/crash-safe, per-step goals + success criteria |
| **Magic.dev** | Magic | Ships ZERO methodology — pure model layer ($320M Series C). Even here the community bolts methodology on |
| Replit / Bolt / Lovable / v0 / Pico | — | Intent extraction + plan, no gates/retrospectives (UI-generation out of scope) |

### The demand-side evidence (why this proves the gap)

Three independent practitioner-authored protocol systems exist because no product ships one:
1. **qiuranke99/v3-agent-standard** — 8-phase state machine (P0-P7), role separation, evidence-gated merge
2. **ryviuszero 复盘 post-mortem** (CN) — SPEC-first role-split, 复盘 (retrospective) as standard stage
3. **teratron/magic-spec** — Thought→Spec→Task→Run→Code, quarantine cascade, session hard-stops, SHA256 integrity, automated retrospectives

### Consolidation targets for the Development Protocol

- Huawei per-stage gates (accept/reject at every stage)
- MetaGPT SOP-as-code ("Code = SOP(Team)")
- CAMEL critic-with-criteria (configurable independent review)
- magic-spec quarantine cascade + session-isolation hard stops + automated retrospectives
- Devin structured plan state (inspectable/editable/crash-safe)
- Cursor artifacts-as-proof (reviewer validation without checkout)

### Revised bottom line

**No shipped tool — Chinese, Western, specialised, or general — covers the full methodology loop with enforced gates.** Huawei is the closest single product (4 gated stages). The strongest confirmation: three independent practitioners wrote full protocols by hand. The OMO/HyperSisyphus stack (intent→commitment→strategy→validation→review→reflection) remains differentiated, and now has consolidation targets from Chinese and specialised companies.

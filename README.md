# Development Protocol

> Stop your AI agent from coding the wrong thing. A step-by-step method that turns a vague idea into a scoped plan — what to build, whether to build it, and how — before any code gets written.

For builders and coding agents who run AI loops and want fewer reworks. Not for one-off prompts or code generation without review.

**In 15 seconds:** you ask `add dark mode`. The agent runs 4 checks: 1. understand what you actually want, 2. check whether it is worth building, 3. pick the best approach, 4. plan then execute. A full run with real decisions: [colour-blind trace](docs/traces/colour-blind-85-100.md) (mistakes kept in).

Quick links: [Try it](#try-it-in-5-steps) · [Lived traces](#it-works-two-lived-traces) · [Pipeline](#the-pipeline-at-a-glance) · [Principles](#core-principles)

## The 4 checks

1. **Understand** what you actually want, including what you have not said yet.
2. **Should-build**: commit to the problem, schedule it, or drop it — before researching alternatives.
3. **Which version**: same, scaled, adjacent, or more — picked after looking at the landscape.
4. **Plan and build**: prototype, lock the plan, execute, verify, learn.

A person approves the strategy once, then the agent builds with checks along the way. (The step codes — P1, P2a, P2b, P3, P4 — live in `docs/PHILOSOPHY.md`; you do not need them to start.)

## Try it in 5 steps

1. Brain dump into `INBOX.md`, then group similar thoughts and pick one to pursue.
2. Run `EXTRACTION.md` on it: state the real problem in one sentence, separate from your first solution idea.
3. Run `SERIOUSNESS.md`: score commitment honestly. Exit is COMMIT, SCHEDULE, or DROP. Only a COMMIT continues.
4. Run `DECOMPOSITION.md`, `AMBITION.md`, then `LANDSCAPE.md`. End with a written choice of which version to build, then get human approval in `STRATEGY.md`.
5. Prototype in `VALIDATION.md`, lock the plan in `SPECIFICATION.md`, build via `EXECUTOR.md`, then verify with `REVIEW.md` and `REFLECT.md`.

Each step file states its entry condition, so you can also run steps standalone without starting from the top.

Tip: start with `INBOX.md` even if you think you know the problem. The one-sentence extraction in step 2 often changes what you build in step 4.

## It works: two lived traces

- [colour-blind-85-100](docs/traces/colour-blind-85-100.md): a vague wish to make the chain watertight → picked a broader fix over a narrow patch → shipped the missing wiring plus a one-page illustration.
- [local-search-review](docs/traces/local-search-review.md): "search feels weak" → kept the same scope → shipped a full Tavily-compatible search surface with better ranking.

Both ran the same steps you will run, with commits and decisions linked. The record is the work — no reconstruction.

## The pipeline at a glance

WANT (what do you really want, including what you have not said yet) -> SHOULD-BUILD (commit or drop) -> WHICH-VERSION (same, scaled, adjacent, or more) -> BEST PLAN -> BUILD AND VERIFY.

The two gates in the middle are deliberate. The first asks if you should build at all. The second asks which version is worth building, after you have looked at the landscape. You do not research alternatives until you have committed to the problem.

The full step order is INBOX, EXTRACTION, SERIOUSNESS, FUNDAMENTALS, DECOMPOSITION, AMBITION, LANDSCAPE, STRATEGY, VALIDATION, SPECIFICATION, EXECUTOR, REVIEW, REFLECT.

![PoP Pipeline — P1 WANT → P2 SHOULD → P3 PLAN → P4 EXECUTE](docs/diagrams/pop-pipeline.svg)

_Interactive version: [pop-pipeline.html](docs/diagrams/pop-pipeline.html) — archify showcase, click nodes for detail._ The pipeline fits any project type, software notes live in the [Engineering Plugin](docs/engineering-plugin.md).

If the diagram feels detailed, follow the bold line above and open the interactive view only when you need a specific step.

## Contents by phase

- **Want (P1):** `INBOX.md`, `PRIORITIZE.md`, `EXTRACTION.md`: capture and clarify what you actually want.
- **Should-build (P2a):** `SERIOUSNESS.md`, `FUNDAMENTALS.md`: decide if it is worth building now, later, or not at all.
- **Which-version plus plan (P2b, P3):** `DECOMPOSITION.md`, `AMBITION.md`, `LANDSCAPE.md`, `STRATEGY.md`: choose which version to build and get it approved.
- **Execute (P4):** `VALIDATION.md`, `SPECIFICATION.md`, `EXECUTOR.md`, `REVIEW.md`, `REFLECT.md`: prototype, specify, build, review, and learn.
- **Rules and quality:** `RULES.md`, `STANDARDS.md`, `docs/QUALITY_BAR.md`, `docs/SKIP_CATALOG.md`, `docs/METHOD_LEDGER.md`: governance and checks.

For file purposes and step entry or exit criteria, see the linked step docs and `RULES.md`.

The step docs are the single source of truth for how to run a gate. This README groups them so you can find the right file for your current altitude.

## Composability

- **Module mode:** `EXTRACTION` then `SERIOUSNESS` alone evaluates an idea fast. `VALIDATION` through `EXECUTOR` builds without re-extracting. `LANDSCAPE` then `REVIEW` audits existing research.
- **External methods:** swap AMBITION for Shape Up pitching, run VALIDATION as a Design Sprint week or Lean Build-Measure-Learn loop, run EXECUTOR milestones as Scrum sprints.
- **Do not reorder:** `EXTRACTION` through `AMBITION` must run in order. Skipping one leaves the plan without a checked foundation.

Pick the slice that fits your job, run it, and log what you skipped with a catalog code so review can still check it.

## Core principles

1. **You approve strategy once.** The agent proposes a written plan, you accept or amend it, then the agent builds. No per-action approvals.
2. **Check consequential claims.** Any step can call for verification, graded by cost of being wrong and grounded in retrieved evidence rather than model memory.
3. **Keep it light.** Ceremony costs effort, so additions must earn their place. Default is ship at roughly 80 percent, except one-way doors.

Full set and rationale: [Standing Principles](docs/STANDING_PRINCIPLES.md). Research basis: [harness survey](docs/research/harness-survey-2026-07.md).

These three shape every other rule. If a proposed addition does not serve one of them, it does not ship.

## Flagship Adoption Probe

When a new flagship model arrives, run the protocol once on a live task without changing any method. Keep the method unless the run exposes a genuinely new failure class. Model gains so far land in execution, not in the want and should-build gates, so adopt the model and keep the gates.

## Appendix

Deeper material lives in `docs/`: protocol model, method ledger schema, skip catalog, quality bar, explainer, spec sync, standards, ADRs, and research notes including the harness survey. Review runs archive under `.omo/reviews/`.

- `docs/PROTOCOL_MODEL.md`: the state machine and valid transitions
- `docs/METHOD_LEDGER.md` and `docs/SKIP_CATALOG.md`: how completeness is tracked
- `docs/QUALITY_BAR.md`: the per-project quality contract
- `docs/EXPLAINER.md` and `docs/SPEC_SYNC.md`: closing the build-to-docs loop

## Contribute

Small, cited improvements beat big rewrites. Propose a change with its source and the effort it saves, and keep prose decision-changing or cut it. Open an issue or PR with the failing trace attached.

If you can show the change saves a future run 15 minutes or prevents a class of mistake, it belongs.

## License and links

License: see LICENSE in this repo. Links: [Issues](../../issues) · [Traces](docs/traces/) · [Diagrams](docs/diagrams/pop-pipeline.html) · Companion projects: [Standards](https://github.com/B67687/Standards) and Lessons.

## Origin

Built by running the protocol on itself (v3.0.0): the prep phase centered the prototyping gate as the key addition. July 2026.

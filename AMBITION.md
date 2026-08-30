# AMBITION.md — Intent Clarification Dialogue (Research-Interleaved)

> This is a **research-interleaved conversation protocol** between you and the AI.
> The AI does NOT just ask questions from its training knowledge. Between each round,
> it researches (web search, codebase exploration, tool analysis) based on what you
> just said, then uses fresh findings to ask a better question next round.
>
> The cycle: **You share → AI researches → AI asks → You answer → AI researches deeper → ... → convergence**

> The output is a falsifiable goal that both parties are confident enough to commit to.

> **This is not a linear pipeline. It is a tightening loop.** Each cycle reduces the
> remaining ambiguity by roughly half. Stop when the remaining unknowns no longer
> block architecture decisions.

## Dialogue Rounds

Each round follows: `AI asks → You answer → AI researches → AI asks next (research-informed) → ...`
The research step between rounds is MANDATORY. If the AI skips it, the next question is uninformed.

### Ground Rules

1. **Real research between rounds** — The AI must search the web, check existing code, or consult docs between each user answer. It must NOT rely only on training knowledge to steer the dialogue.
2. **Three-strikes convergence** — If round N's research adds nothing new (all findings duplicate round N-1), the AI flags convergence and moves to Lock.
3. **Explicit citation** — When AI researches and returns, it cites what it found: "I looked at X (source), it shows Y. This means our approach should Z."
4. **Permission to say 'I don't know'** — The AI can flag when research found nothing. This itself is a finding (proves the space is unexplored).

**Q:** "If this project were to take 1 week, 2 weeks, 6 weeks — which feels right?"
**A:** "I have like 1 month of free time, but preferably we can make this perfect in only 1 week. I will likely invest time in this forever."

**Locked:** 1 week to build. Continuous lifelong iteration expected.

### Round 1 — Open (Unfiltered)

**Prompt:** "What's the real itch here? What keeps bringing you back to this space?"

**Response:** The thing that isn't solved yet is something that can take me from intention to the final product. I realized I quite like excellent work. But sometimes I start projects then realize the project is way too hard and might take too long to become excellent (zentryide, UI/UX for opencode), or someone built it way better than me (OMO vs what agentic-workflows once was), or I do build it but I keep restructuring and breaking my previous work because my ambition raised during the project, or I keep seeing new ways to improve while on the way (Bus-Hop, Ithmb-Codec). The goalpost shifting is very tiring.

My thought: maybe the problem is because I simply didn't know the entire scope of my want and project well before starting. Most problems can be solved at the planning stage. There's so much usable experience from other projects and people and online resources that we CAN solve most problems with a project beforehand. Potential new problems found during the project would be way less.

I need a thing that takes me from intention to final product (end to end), and possibly helps me prototype fast to find all the problems that can only be found during making the project. This thing that can massively reduce my problems during the project, and help me use agents to quickly finish a project at a pinned ambition (usually quite big) — that's the thing that's missing.

### Round 2 — Interrogate (AI pushes back)

**Challenge 1 (Assumption):** "Can most problems really be solved at the planning stage in software? Schön argues the problem reveals itself through building."
**Response:** I believe there is still quality in this because for computer science specifically, there is so much experience to take from online for any kind of software-related project. Some problems will still need to be revealed during building — so the protocol needs a prototyping engine that reveals most problems using as little effort as possible.

**Challenge 2 (Alternative-perspective):** "Shape Up fixes time and flexes scope — what if less planning and more timeboxing is the answer?"
**Response:** The kind of projects I build are all highly ambitious. I built this for myself. I never really care about building projects halfway — though reflectively I realize they were just prototypes for me to understand what is actually useful. So again, the same prototyping point.

**Challenge 3 (Meta-cognitive):** "How much of this is about the tooling vs. the emotional pattern of creation?"
**Response:** It certainly is part of how we create, so I guess fast prototyping should be part of our protocol. It's possible after prototyping we decide not to do the serious project. Or after prototyping I become more clear what is worth building and change the type or topic.

**Challenge 4 (Understanding / Education check):** "Let me test my own explanation. I've been describing [concept]. In your own words, how would you explain it back to me?" This is the Teach-Back method — if the user can't restate the concept clearly, they don't understand it well enough to make informed decisions yet. If they struggle, the AI offers a clearer explanation and checks again.

**Response:** *[User attempts to restate the concept in their own words. If they can restate clearly, AI confirms and moves on. If they struggle, AI offers a clearer explanation and checks again: "Let me try explaining differently..." followed by "Now in your own words."]*
### Round 3 — Steer (AI reframes)
> **Bias check (load `BIAS_CATALOG.md`):** Which biases are most likely this round? Check §Anchoring (AI's proposal anchors) and §Planning Fallacy (budget estimates). Ask: "What was the first number mentioned? Would you change your answer if it were 50% higher/lower?"

**Insight from AI:** The pipeline should center on a **prototyping gate** as the critical decision mechanism — not just a step before the spec, but the central decision point.

```
INTENT → AMBITION → LANDSCAPE →
    ↓
PROTOTYPING GATE (critical decision)
    ├── "This is worth the full build" → SPECIFICATION → EXECUTE → POLISH
    └── "Not worth it / wrong direction" → Kill or Pivot → back to INTENT
```

Prototype cheaply, learn what only building can teach, then decide. This matches the real pattern: abandoned projects (agentic-workflows, zentryide) were prototypes that revealed the project wasn't worth the full build.

**User response:** Yes, this feels right. Prototyping should be part of our pipeline.

### Round 4 — Converge (build the ambition)

**Hypothesis:**

> I believe that a **document-driven protocol with a rapid prototyping gate as the central decision mechanism** will reduce goalpost fatigue — enabling me to ship ambitious projects within their original scope by revealing the critical unknowns before committing to the full build, and by providing a clear KILL/PIVOT/COMMIT decision point that saves wasted effort on projects that aren't worth the full build.

**Success Criteria:**

1. **Clarity speed:** I can take a vague intention and reach a clear KILL/COMMIT decision within 3 days of starting the protocol
2. **Build stability:** After a COMMIT decision, the build phase completes with documented shifts when discoveries arise - the spec adapts without derailing
3. **Retrospective validation:** Projects I KILL during prototyping are clearly "not worth it" in hindsight, not "I wish I'd pushed through"

**Constraints:**

- Any team or individual — solo dev + AI is the current use case, but the protocol is team-agnostic
- Document-driven — Markdown is the source of truth, CLI is optional acceleration
- Must handle any project type (library, app, research, port)
- Must produce shippable output, not just documentation
- The prototyping gate must be faster than doing the real project — if prototyping takes longer than shipping would have, the protocol failed

### Round 5 — Lock

**Read-back:**

> "We are building a **document-driven protocol with a rapid prototyping gate as the central decision mechanism** because we believe it will reduce goalpost fatigue — enabling shipping ambitious projects within their original scope by revealing critical unknowns before committing to the full build. We will know it worked when: (1) we can reach a KILL/COMMIT decision within 3 days, (2) builds that COMMIT ship without scope warps, and (3) killed projects are clearly 'not worth it' in hindsight. We are bounded by: any team size, document-driven, any project type, shippable output, and the prototyping gate must be faster than the full build."

**Quality bar (proposed):** [Profile A | Profile B] — one-paragraph risk rationale (cross-ref docs/QUALITY_BAR.md). Ratified at STRATEGY as part of scope.

**Confirmed:** YES — this is the goal.

## Pacing & Budget (folded from PACING.md)

Budget is allocated and locked here, alongside the scope. Track actuals after each phase in `.omo/pacing-track.md`.

**Default phase budgets (% of appetite):**

| Phase | % | Human/AI | Session budget |
|---|---|---|---|
| EXTRACTION | 5% | Human-heavy | 1 session, ≤90 min |
| FUNDAMENTALS (incl. MULTI) | 5% | Mixed | 1 session |
| DECOMPOSITION | 5% | Mixed | 1 session |
| AMBITION (incl. PACING) | 10% | Human-heavy | Max 3 rounds/session |
| LANDSCAPE | 10% | AI-heavy | 1-2 sessions |
| STRATEGY | 5% | Mixed (human ratifies) | 1 session |
| VALIDATION | 10% | AI-heavy | 1 spike, fresh context |
| SPECIFICATION | 10% | Mixed | 1-2 sessions |
| EXECUTOR (incl. FINISH) | 30% | AI-very-heavy | Per milestone; FINISH reserves 20-30% |
| REVIEW (incl. EXPLAINER + SPEC_SYNC) | 10% | AI-heavy (independent) | 1 fresh session |

**Adjustment rule:** if appetite < 40h, EXECUTOR gets 40% and prep phases compress proportionally; if appetite ≥ 6 weeks, prep phases expand to absorb the larger budget.

**Tracking:** after each phase, log budget/actual/delta in `.omo/pacing-track.md`; cumulative delta carries forward; a phase >50% over budget fires a **Pace Alert** → diagnose (unrealistic budget / scope creep / verification tax) → adjust (transfer budget from a later phase, reduce scope, or extend appetite with explicit override).

**Session boundaries:** per-phase max turns/reads, compact at limit (EXTRACTION 20/10, AMBITION 30/15, LANDSCAPE 50/40, VALIDATION 40/30, SPECIFICATION 40/30, EXECUTOR per-milestone, REVIEW 30/10). Context-decay signals — inconsistent naming, repeated questions, ignoring recent edits, loss of ask-before-destructive instinct — trigger proactive compaction, never wait for 95%.

**Session-isolation hard stops** (magic-spec pattern): EXTRACTION→FUNDAMENTALS, AMBITION→LANDSCAPE, LANDSCAPE→STRATEGY, EXECUTOR→REVIEW, REFLECT→next cycle — fresh session mandatory at each.

**Human energy rules:** max 3 AMBITION rounds per session; POLISH during peak hours; 30 min recovery between human-heavy phases; EXTRACTION separated from AMBITION by ≥1h.

**Macro pacing (>2 weeks):** Cycle (4-6 weeks, one milestone, shipped artifact) → Cool-down (1 week: bug fixes, deps, POLISH/REVIEW for prior cycle) → Betting (cycle start; items compete fresh; no auto-extension). Circuit breaker: late cycle → drop and re-shape unless all remaining work is downhill, scope is hammered, and human override is recorded.

**Effort estimation:** T-shirt S/M/L/XL (<1h / 1-4h / 4-8h / >8h) for early milestones; decomposition (sum sub-tasks) for locked specs; reference-class for repetitive work. Calibrate after 3 milestones by average error. Estimates are ranges, not promises.

**Cost tracking:** human time + AI compute (session, model, tokens) + felt effort (low/med/high) logged per phase; combined cost >150% of phase budget = Pace Alert. A durable mismatch between process effort and work value is a Pace Alert + REVIEW input — Effortlessness applies to the protocol's own execution.

**Probabilistic forecasting (future):** once 5+ projects of actual-vs-estimate data exist, replace deterministic budgets with Monte Carlo distributions.

**Example** (this protocol's own run): appetite 1 week (~20h); EXTRACTION 1h | FUNDAMENTALS 1h | DECOMPOSITION 1h | AMBITION 2h | LANDSCAPE 2h | STRATEGY 1h | VALIDATION 2h | SPECIFICATION 2h | EXECUTOR 6h | REVIEW 1h; buffer 2h (10%).

**Absolute time estimates (for a 40-hour appetite):**

| Phase | % Budget | Hours |
|-------|----------|-------|
| INBOX | 2% | 1h |
| EXTRACTION | 5% | 2h |
| SERIOUSNESS | 2% | 1h |
| FUNDAMENTALS | 5% | 2h |
| DECOMPOSITION | 5% | 2h |
| AMBITION | 10% | 4h |
| LANDSCAPE | 10% | 4h |
| STRATEGY | 5% | 2h |
| VALIDATION | 10% | 4h |
| SPECIFICATION | 10% | 4h |
| EXECUTOR | 30% | 12h |
| REVIEW | 5% | 2h |

**Overrun rule:** If a phase exceeds 1.5× its budget, STOP and reassess — re-estimate or re-classify the project.
Human confirms or adjusts; baseline recorded in `.omo/pacing-track.md`.
---

## Adaptive Depth (SERIOUSNESS-Gated)

> Same protocol, different depth — gated by SERIOUSNESS score.

| Mode | SERIOUSNESS Score | Phases Run | Time Budget |
|------|------------------|------------|-------------|
| **Light** | 35–50 (SCHEDULE-low) | INBOX → EXTRACTION → SERIOUSNESS → AMBITION → SPECIFICATION → EXECUTOR | <2 hours |
| **Standard** | 51–80 | All 12 phases | 2–40 hours |
| **Deep** | 81–120 | All 12 + EXTRACTION bouncing + LANDSCAPE deep research + STRATEGY premortem | 40+ hours |

- **Light skips:** DECOMPOSITION (problem is simple), LANDSCAPE (no research needed), STRATEGY (approach obvious), VALIDATION (spike overkill), REVIEW (self-review sufficient)
- **Light risks:** documented — "Skipping means: no bias check, no landscape mapping, no strategic ratification. Acceptable for low-stakes, reversible decisions."
- **Mode override:** Human can force Deep on any project. AI recommends; human decides.

---

## Exit Criteria

All checks pass:

- [x] A falsifiable hypothesis statement exists
- [x] Both parties agree on it
- [x] 3 measurable success criteria are defined
- [x] 5 constraints are defined
- [x] The AI challenged 3 assumptions (and they survived or were reframed)
- [x] The intent has been explicitly locked — no more changes after this document

**Schon's 5 frame criteria:**

1. **Specificity** — YES (AI can read this and know what to build)
2. **Falsifiability** — YES (we'll know if builds still drift after prototyping)
3. **Testability** — YES (3 success criteria are measurable)
4. **Baseline** — YES (better than current pattern of abandoned projects)
5. **Congruence** — YES (matches the actual felt need)

---

## XY Problem Check

The [XY Problem](https://xyproblem.info/) is when someone asks about Y (their attempted solution)
instead of X (the actual problem). A user who says "build a CLI that does X" might
actually need something X achieves in a simpler way.

**After the dialogue, check:** Did the user frame a solution (Y) as the problem (X)?
If so, unpack: "You asked for Y. Before we design Y — what problem does Y solve for you?"
This prevents building the wrong thing correctly.

## Meta: How This Dialogue Ran

**Duration:** ~30 minutes (within the 90-minute timebox)
**Rounds used:** 0, 1, 2, 3, 4, 5 (all 6 — circuit breaker not triggered)
**Failure modes detected:** None triggered
**Key insight from process:** The prototyping gate emerged through dialogue — neither party arrived with it. It was constructed through the back-and-forth. This validates the dialogue protocol itself.

---

## Origin

Generated by following the Development Protocol's AMBITION.md dialogue protocol on itself — a recursive self-test of the intent clarification process. July 2026.

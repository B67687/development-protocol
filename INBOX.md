# INBOX.md — Thought Capture & Triage (v4)

> **This is Step -1** of the Development Protocol. It runs before EXTRACTION, before any single-problem work.
>
> **Purpose:** A user often arrives with multiple unrelated ideas. This step captures ALL of them without forcing commitment, organizes into natural clusters, and selects which one to feed into EXTRACTION.
>
> **Without this step:** The loudest thought wins by default, or the protocol forces a single Y too early, and other ideas resurface as distractions mid-protocol.

## Protocol Suitability Check (30 sec)

Before running INBOX, check whether the protocol fits the work at hand:

| Question | If No, consider... |
| --- | --- |
| Is the problem worth the protocol's overhead (5+ hours)? | Skip protocol. Use a note + direct action. |
| Is your problem framing clear enough to start? | Think/write freely first. Come back when clearer. |
| Do you have the energy to follow a structured process? | Come back when you're fresh. A tired run wastes both time. |
| Is the AI assistant available for this session? | The protocol assumes AI availability. Results degraded without it. |
| Is this the right time to start a project? | Park in Someday/Maybe. The protocol creates commitment momentum. |

If all 5 pass: proceed with INBOX. If 2+ fail: skip the protocol and do whatever fits the gap.

#### Run-Shape Selector (~30 sec)

If the protocol fits, pick the run shape. Default when unsure: **Standard**.

| Mode | Use when | Run shape |
| --- | --- | --- |
| **Light** | Small, well-scoped, reversible work (single fix, small script) | Simplified pass: capture → extraction-lite → implement → verify. Skip heavyweight steps (FUNDAMENTALS/AMBITION/LANDSCAPE/STRATEGY) unless a gate flags risk. **Verification floor never skipped**: review + targeted tests stay. One-way doors (schema/API/data/security) → Standard regardless of size. Escalate/re-triage if the work grows past its bounds.
| **Standard** (default) | Everything else | The full pipeline as documented. |
| **Brownfield** | Work in an existing project not built with the protocol | **Bounded reanchor pass first**: AI reads the repo, proposes a compressed goals/philosophy/point statement, user corrects/ratifies it, THEN EXTRACTION proceeds with the reanchored framing. Timeboxed — ends in the ratified statement, never open-ended.

Run-Shape (front door) is orthogonal to DECOMPOSITION § Level of Care (per-leaf routing inside Standard runs). Log the chosen mode and any reanchor ratification to the method ledger.

**Calibration rule (pick by evidence, not optimism):** choose the run shape from the reference class — unknown count, one-way doors, blast radius, novelty, and how similar past work actually went (planning fallacy: inside-view estimates are systematically optimistic). When unsure: **Standard**. Mid-run, if the work crashes past its bounds (the "this isn't working" moment), RE-SELECT the shape upward (Light → Standard; Standard is the full pipeline — there is no higher shape) — a logged shape decision (per Invariant 9) that revokes the front-door skip for the remaining scope and re-runs the skipped heavyweight gates before forward progress (Invariant 6 ordering holds), not a manual restart and not an admission of failure.

### Brownfield direction assessment

The Brownfield row's reanchor pass is **evaluative, not just descriptive**: a codebase is authoritative about the past and feasibility, SILENT about value and future direction. Direction evidence lives in the intent layer (README, ADRs, changelog, issue clusters, roadmap, support history) — code is evidence about the vehicle, not the direction. Bounded step (≤1 session, ~45 min):

1. **Pre-committed kill criteria** — before gathering evidence, name what would make each verdict wrong (premortem over ALL FOUR verdicts: continue / turn-in-place / turn-via-migration / full-migrate). Those failure stories become the evidence checklist (Klein premortem — ~30% more correct risk identification).
2. **Intent-layer recovery (timeboxed, ~45 min)** — README, ADRs, changelog, issues, roadmap; ONE stakeholder conversation (the user, unless the user names others). Output: one-paragraph reconstructed original problem ("built to do X, for Y, assuming Z about the world") + its "still real if…" test (Chesterton's Fence, two-sided: recover the reasons, then TEST them against current reality — "is this still the world?").
3. **Two scored questions, scored independently** — (a) Is the problem still real? (value — from outside the repo); (b) Is the architecture the right vehicle? (feasibility-of-direction). Never let "ugly/hard to change" input into either score (Spolsky rewrite trap: Netscape's 3-year rewrite never shipped). Characterization tests are direction-BLIND by design — barred from direction evidence.
4. **Ratification — one interaction, one gate**: the user ratifies the reanchored statement AND the direction verdict + execution-mode together (Invariant 11 single-gate entry, ledger-logged). continue / turn-in-place: default-autonomous with an argued rationale in the ledger; turn-via-migration / full-migrate: expensive-to-reverse → auto-escalated user gate (Invariant 11 clause — EXECUTOR Migration Protocol's ratification note already does this). "Continue" is argued, never implicit.
5. **Verdict template** — output is the (direction, execution-mode) pair: continue / turn-in-place / turn-via-migration / full-migrate. Execution mode selection is a cheap architecture probe, not a second full assessment: Choke Point Test — viable iff ~90%+ of calls intercept at one layer (route by tenant/endpoint/flag, compare outputs); fail → "you don't have a strangler project yet, you have an architecture project". The Migration Protocol (EXECUTOR) is the execution arm.

The direction verdict is a value judgment = slow-velocity → log velocity classification per METHOD_LEDGER rule 9.

## When to Use This

Every time someone has multiple thoughts, not one clear Y:

- "I've been thinking about three things..." (classic)
- "So I've got this idea, and also this other thing..." (sprawl)
- "I have a lot of ideas, not sure where to start" (paralysis)
- Even a single Y: run a quick check — "Anything else on your mind?"

## Phase 1: Capture — Raw Dump (10 min)

No filtering, no judging. Externalize everything before any processing begins.

1. Set a 10-minute timer.
2. Ask: "What's on your mind? List everything — ideas, worries, curiosities."
3. Capture verbatim. One sentence per thought. Do not paraphrase.
4. Prompt for hidden thoughts: "Anything else? What have you been meaning to get to?"
5. When timer ends, read back the full list.

Output:

```markdown
## INBOX — RAW DUMP

1. Build a CLI flashcard tool for CS review
2. Redesign the landing page
3. Write a blog post about the extraction protocol
4. Fix the database migration bug in the API
5. Maybe start a newsletter about dev tools
6. Worried about team velocity this quarter
7. Thinking about learning Rust
8. Should I upgrade the CI pipeline?
```

Rules: No editing, no combining, no deleting during capture. Everything goes in, including stupid ideas.

## Phase 2: Organize — Natural Clustering (10 min)

Let themes emerge. Do NOT decide categories in advance.

1. Read each thought aloud. Ask: "Do these feel related?"
2. Group naturally — cluster thoughts that share a common theme.
3. Let single items form their own clusters. Small clusters are valid.
4. Label each cluster with a noun phrase, not a solution.

Output:

```markdown
## CLUSTERS

Cluster A: "CS Study Tools" (items 1, 5, 7)

- CLI flashcard tool for CS review
- Newsletter about dev tools
- Learning Rust

Cluster B: "Frontend Polish" (item 2)

- Redesign landing page

Cluster C: "API Reliability" (items 4, 6)

- Fix DB migration bug
- Worried about team velocity

Cluster D: "Infrastructure" (item 8)

- Upgrade CI pipeline

Cluster E: "Writing/Sharing" (item 3)

- Blog post about extraction protocol
```

## Phase 3: Triage — Cluster Selection (5 min)

Score each cluster on 3 quick questions. No deep analysis — gut check.

| Question                                        | Yes ✓ | Maybe ~ | No ✗ |
| ----------------------------------------------- | ----- | ------- | ---- |
| Energy: Do you feel motivation around this?     | ✓     | ~       | ✗    |
| Timing: Does this need attention soon?          | ✓     | ~       | ✗    |
| Tractability: Can you make progress in 5 hours? | ✓     | ~       | ✗    |

> **Alignment with PRIORITIZE:** these three triage dimensions map onto the four PRIORITIZE dimensions — Energy ↔ Want (motivation), Timing ↔ Matters (urgency/cost-of-delay), Tractability ↔ Work (5h progress) — with Know (approach confidence) assessed during PRIORITIZE proper. Triage picks candidates; PRIORITIZE scores them.

Sort clusters by (✓ count, then intuition).

```
Ranking:
1. Cluster C: "API Reliability" — ✓✓✓ (energy + urgent + doable)
2. Cluster A: "CS Study Tools" — ✓✓~ (energy + doable, not urgent)
3. Cluster D: "Infrastructure" — ✓~~ (urgent, low energy)
```

The #1 cluster enters EXTRACTION. Everything else goes to Someday/Maybe.

| Selected               | Parked                                   |
| ---------------------- | ---------------------------------------- |
| Cluster C → EXTRACTION | Clusters A, B, D, E → .omo/inbox/parked/ |

## Phase 4: Traffic Light Review (30 sec — optional)

| Signal                     | Action                                                           |
| -------------------------- | ---------------------------------------------------------------- |
| 3+ clusters tied for #1    | User may be avoiding a hard choice. Run "$100 bet" tiebreaker.   |
| All clusters score low     | User may need rest, not protocol. Take a break.                  |
| Same cluster won last week | User is stuck. Run EXTRACTION with "What's blocking you?" probe. |
| More than 6 clusters       | User is fragmented. Do raw dump only. Come back tomorrow.        |

## Phase 5: Raw-Thinking Pass (5-10 min — GENERATION before EXTRACTION)

> The Intuition-First principle made STRUCTURAL. Before EXTRACTION strips Y into X,
> run a pure generation pass on the selected cluster: no gates, no ceremony, no
> structure. Surface every intuition, perspective, connection, and alternative
> framing — including ones the user did NOT raise.

**The pass:**
1. **Generate freely** — divergent, uncensored, no self-criticism. List every
   perspective on the cluster, every adjacent domain it might touch, every hidden
   connection to other parked clusters, every possible framing of the problem.
2. **Timebox** (5-10 min). Raw thinking is a layer, not a mode of procrastination.
3. **Produce the thick frame** — a connection map (what relates to what, and the
   unstated assumptions), the alternative framings, and the candidate X's. This
   frame IS the input to EXTRACTION.

> **The guard (why this is not an excuse to skip the protocol):** raw GENERATES,
> protocol VERIFIES. Raw thinking widens the frame; EXTRACTION/AMBITION/STRATEGY
> then narrow it with evidence, gates, and contracts. A raw insight that skips
> EXTRACTION is an unverified guess. The gates stay — they now operate on a rich
> frame instead of a thin one-line statement.

**Why this exists:** research shows most failures are in problem FRAMING, not
solving (Thread 7 / Cluster AM). A thick frame caught at generation time prevents
EXTRACTION from over-narrowing onto the user's stated wording — the exact failure
mode "you don't consider enough perspectives." The frame's connection map is the
permanent form of the connection-mining pass that Cluster AM ran once.

After INBOX completes, update the checkpoint (`.omo/protocol-state.md`):

```
INBOX              [✓] COMPLETE
  [✓] Raw dump: 8 items captured
  [✓] Clusters: 5 identified
  [✓] Triage: Cluster C selected
  [✓] Raw-Thinking Pass: thick frame produced (connection map + alternative framings)
  [✓] Others parked at .omo/inbox/parked/

SOMEDAY/MAYBE      [ ] PENDING (next review: 2026-08-03)
  [ ] Cluster A: "CS Study Tools"
  [ ] Cluster B: "Frontend Polish"
  [ ] Cluster D: "Infrastructure"
  [ ] Cluster E: "Writing/Sharing"

EXTRACTION         [->] IN PROGRESS
  [✓] Language clarified
  [✓] Intent landscape explored
  [✓] 10 extraction techniques applied
  [→] X confirmed
```

## Strategic Alignment Check

Before proceeding to EXTRACTION, ask one question about the selected cluster:

> "Where does this project fit in my 1/3/10 year trajectory?"

This is not a commitment. It's a directional check — the answer may be "it
doesn't" (this is exploration), "it builds capability X" (this is skill development),
or "it directly delivers value toward goal Y" (this is execution).

The answer informs the rest of the protocol:

| Answer | Implication |
| --- | --- |
| **Directly delivers** toward long-term goal | Full protocol — this is a priority project |
| **Builds capability** I'll need later | PRODUCTION QUALITY requirements should match future need, not just current scope |
| **Exploration** — learning what I need | Use DISCOVER-FIRST route; lighter SPEC; accepting failure as outcome |
| **Unsure** — I don't know where this fits | This is a signal to spend more time on self-strategy before committing |

This step solves a gap identified by the Foundation Audit: the protocol
previously had no concept of multi-project trajectory or strategic patience.

## Integration

The pipeline becomes:

```
RAW INTENT → [INBOX] → [PRIORITIZE (optional)] → EXTRACTION → SERIOUSNESS → FUNDAMENTALS (incl. MULTI) → DECOMPOSITION → AMBITION (incl. PACING) → LANDSCAPE → STRATEGY → VALIDATION → SPECIFICATION → EXECUTOR (incl. POLISH) → REVIEW (incl. EXPLAINER + SPEC_SYNC) → REFLECT → ship
```

The `.omo/inbox/parked/` directory accumulates over sessions — enabling pattern detection of recurring concerns.

## Provenance

| Phase                | Source                                           | Evidence                                                  |
| -------------------- | ------------------------------------------------ | --------------------------------------------------------- |
| Capture (Phase 1)    | GTD (Allen 2001), Morning Pages (Cameron 1992)   | Zeigarnik effect — externalization reduces cognitive load |
| Rapid format         | Bullet Journal (Carroll 2018)                    | Short-form prevents flow interruption                     |
| Clustering (Phase 2) | Affinity Diagramming / KJ Method (Kawakita 1967) | Standard UX research, 30+ years                           |
| Triage (Phase 3)     | GTD Clarify + Mom Test                           | Lightweight SERIOUSNESS adaptation                        |
| Parking lot          | GTD Someday/Maybe, Pivotal Icebox                | Must have review cadence or trust erodes                  |

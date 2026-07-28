# INBOX.md — Thought Capture & Triage (v4)

> **This is Step -1** of the Development Protocol. It runs before EXTRACTION, before any single-problem work.
>
> **Purpose:** A user often arrives with multiple unrelated ideas. This step captures ALL of them without forcing commitment, organizes into natural clusters, and selects which one to feed into EXTRACTION.
>
> **Without this step:** The loudest thought wins by default, or the protocol forces a single Y too early, and other ideas resurface as distractions mid-protocol.

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

## Output Format

After INBOX completes, update the checkpoint:

```
INBOX              [✓] COMPLETE
  [✓] Raw dump: 8 items captured
  [✓] Clusters: 5 identified
  [✓] Triage: Cluster C selected
  [✓] Others parked at .omo/inbox/parked/

SOMEDAY/MAYBE      [ ] PENDING (next review: 2026-08-03)
  [ ] Cluster A: "CS Study Tools"
  [ ] Cluster B: "Frontend Polish"
  [ ] Cluster D: "Infrastructure"
  [ ] Cluster E: "Writing/Sharing"

EXTRACTION         [->] IN PROGRESS
  [✓] Language clarified
  [✓] Intent landscape explored
  [✓] 7 extraction techniques applied
  [→] X confirmed
```

## Integration

The pipeline becomes:

```
RAW INTENT → [INBOX] → EXTRACTION → SERIOUSNESS → FUNDAMENTALS → DECOMPOSITION → AMBITION → PACING → LANDSCAPE → VALIDATION → SPECIFICATION → EXECUTOR → POLISH → EXPLAINER → SPEC_SYNC → REVIEW → ship
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

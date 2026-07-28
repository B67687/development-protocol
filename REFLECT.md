# REFLECT.md — Protocol Retrospection (v4)

> This is a **mandatory gate** between REVIEW and ship. Every project must pass through REFLECT before shipping.
>
> **Purpose:** Reflect on how well the protocol itself performed during this project.
> Not a project retrospective (what did we build?) — a **protocol retrospective** (how well did the protocol guide us?).

## Method — 4 Questions, 20 Minutes

Answer each question in 1-3 sentences. Record in `.omo/reflect.md`.

### Q1: What did the protocol catch that you would have missed?

This is the protocol's value proposition. Concrete evidence that the gates worked.

> *Example: "EXTRACTION caught that I didn't actually want a flashcard CLI — I wanted knowledge retention. That saved building the wrong thing."*

### Q2: What did the protocol miss that it should have caught?

This is how the protocol improves. Every gap found here is a fix for the next version.

> *Example: "PACING said EXECUTOR gets 30% of appetite, but the AI spent 50% on debugging. The budget didn't account for verification tax."*

### Q3: What changed in the protocol during this project? (Learning Shifts)

List any learning shifts that occurred. These are the protocol's own evolution during the project.

> *Example: "Added SERIOUSNESS gate between EXTRACTION and FUNDAMENTALS. INBOX step created for multi-barrelled thoughts."*

### Q4: What should the protocol learn for next time?

This becomes a Lesson entry. Feed it into the Lessons repo.

> *Example: "Don't use sed for shell script edits. Always use the edit tool or write the whole file."*

## Output

After answering, update two places:

1. **`.omo/reflect.md`** — the reflection record for this project
2. **`~/.config/opencode/LESSONS.md`** — append any new lessons (if Q4 produced something new)

## Frequency

Run REFLECT after every project (every REVIEW pass). Do NOT skip even for small projects — small projects surface different gaps than large ones.

## Relationship to METHODOLOGY.md

- **METHODOLOGY.md** — post-project retrospective (what did we build? how was the team?)
- **REFLECT.md** — post-protocol retrospective (how well did the protocol guide us?)
- **Lessons** — cross-project knowledge (what should all future projects know?)

METHODOLOGY.md answers "how was the project?" REFLECT.md answers "how was the protocol?" Lessons answers "what should we carry forward?"

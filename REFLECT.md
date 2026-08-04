# REFLECT.md — Protocol Retrospection (v4)

> This is a **mandatory gate** between REVIEW and ship. Every project must pass through REFLECT before shipping.
>
> **Purpose:** Reflect on how well the protocol itself performed during this project.
> Not a project retrospective (what did we build?) — a **protocol retrospective** (how well did the protocol guide us?).

## Step 0: After-Action Review (AAR)

Before the 7 questions, run the AAR gate (STRATEGY.md, FM 7-0): the human explains expected vs actual outcome first, then the AI contributes. Compare the outcome to the strategy's prediction (Phase-2 falsification signals). This closes the outcome feedback loop — the AAR gate was defined but never wired into the pipeline.

## Method — 7 Questions, 35 Minutes

Answer each question in 1-3 sentences. Record in `.omo/reflect.md`.

### Q1: What did the protocol catch that you would have missed?

This is the protocol's value proposition. Concrete evidence that the gates worked.

> *Example: "EXTRACTION caught that I didn't actually want a flashcard CLI — I wanted knowledge retention. That saved building the wrong thing."*

### Q2: What did the protocol miss that it should have caught?

This is how the protocol improves. Every gap found here is a fix for the next version.

> *Example: "The AMBITION budget said EXECUTOR gets 30% of appetite, but the AI spent 50% on debugging. The budget didn't account for verification tax."*

### Q3: What changed in the protocol during this project? (Learning Shifts)

List any learning shifts that occurred. These are the protocol's own evolution during the project.

> *Example: "Added SERIOUSNESS gate between EXTRACTION and FUNDAMENTALS. INBOX step created for multi-barrelled thoughts."*

### Q4: What should the protocol learn for next time?

This becomes a Lesson entry. Feed it into the Lessons repo.

> *Example: "Don't use sed for shell script edits. Always use the edit tool or write the whole file."*

### Q5: What did you learn about yourself during this session?

This captures the human's learning — the Learning ZPD dimension. What thinking pattern did you notice in yourself? What assumption of yours got surfaced by the protocol?

> *Example: "I noticed I immediately jump to solutions when feeling anxious about scope. The protocol's No-Computer Check slowed me down to describe the problem first."*

### Q6: What thinking pattern did you notice that you'd like to watch for next time?

This builds self-awareness across sessions. Over multiple projects, patterns in your cognition emerge — allowing you to anticipate your own biases.

> *Example: "I tend to over-constrain the solution space when I don't fully understand the domain. Next time I'll flag 'I'm doing this because I'm uncertain' earlier."*

### Q7: Graduation Check — Has the protocol become easier?

This is the self-transcendence gate. If you answered Q1-Q6 smoothly and
quickly — if the protocol's patterns feel natural rather than forced — you may
be ready for the GRADUATE path.

Rate each:

- [ ] EXTRACTION techniques feel automatic (I naturally ask "why?" without prompting)
- [ ] I catch my own cognitive biases before the protocol does
- [ ] The checklist in EXECUTOR's FINISH gate matches what I'd check anyway
- [ ] I can predict what SERIOUSNESS will find before running it
- [ ] REFLECT's questions feel redundant (I already thought through them during execution)

**If 4-5 checked:** You've internalized the protocol. Next project: skip the step
you're strongest at (skip only with a catalog code — CAT-SKIP-01 applies; skipping without a code is a ledger red flag). The protocol becomes shorter as you grow.

**If 0-2 checked:** The protocol is still teaching you — that's normal. Exposure
builds pattern recognition over multiple projects.

This is Musashi's insight: the master transcends methodology. The protocol
should help you outgrow it, not trap you in it.

### Automated Retrospective Trigger (magic-spec pattern)

REFLECT is not only a manual step — it is ALSO triggered automatically at
defined checkpoints, so retrospectives never get skipped in the rush to ship:

| Trigger | Action |
| --- | --- |
| Project ships | Run full REFLECT (Q1-Q7) — mandatory |
| Milestone completes | Run condensed REFLECT (Q1, Q2, Q6) |
| 3+ consecutive failures in one component | Run REFLECT Q2 (what did the protocol miss?) + quarantine review |
| Method ledger shows noncompliance | Run REFLECT Q4 (what should the protocol learn?) + conformance fix |
| Protocol itself changes | Run REFLECT Q3 + Q4 (the change's own retrospective) |

> Source: magic-spec's automated retrospectives (Level 1 snapshot vs Level 2 full).
> Ensures learning is captured even when the human forgets to reflect. Confidence:
> Medium-High.

## Output

After answering, update two places:

1. **`.omo/reflect.md`** — the reflection record for this project
2. **`~/.config/opencode/LESSONS.md`** — append any new lessons (if Q4 produced something new)
3. **Artifact hygiene (retention):** prune the `.omo/` corpus per the use-based criterion — keep what's cited/live/resume-required, archive stale files to `.omo/archive/`, delete nothing irreversibly. Bounded; rides this step (no new pipeline stage).

## Frequency

Run REFLECT after every project (every REVIEW pass). Do NOT skip even for small projects — small projects surface different gaps than large ones.


---

## Input from REVIEW

Before answering REFLECT questions, read the latest REVIEW findings from `.omo/reviews/latest.md`.
The review may have found protocol-level gaps that inform Q2 (What did the protocol miss?).
Do not answer REFLECT in isolation - REVIEW findings are the primary input.


## SHIP Exit Checklist

Ship only when all three pass:
1. **State block written** — `.omo/protocol-state.md` records the completed pipeline
2. **Success criteria closed** — 2-item closure: projected outcome vs actual (the AAR comparison), and what the strategy predicted vs what happened
3. **Ledger clean** — `.omo/method-ledger.jsonl` passes ledger-check.py (0 invalid / 0 omitted / 0 rule9)

Also declare maintenance status (active / maintained / archived) in the state block.
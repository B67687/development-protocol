# Constitution

**Status:** Ratified. This is the philosophy and the goal everything below serves.

## The philosophy

**Fix in planning what can be fixed in planning. Prototyping is planning.**

A defect costs more the later it is caught. While the intent is still words it costs a sentence. After the first real build it costs a rebuild. After something depends on the artifact it costs trust. So this protocol spends its effort where the cost is lowest: before the first real build, on cheap artifacts — a probe, a spike, a paragraph, a written decision.

Two consequences carry the whole method:

- **You cannot specify what you have not seen.** People learn what they want by reacting to something concrete, so the loop has to run, on the cheapest thing that can carry the reaction. Prototypes, spikes and prose are planning artifacts, not building.
- **Cheap iterations before expensive commitments.** Every gate exists to make the next commitment smaller, or to make it clearly wrong before anything depends on it.

### What follows

1. **Decide before you build, and write the decision down.** An undocumented decision gets re-derived later by someone who did not make it.
2. **Raise the cheapest artifact that can falsify the claim.** A spike before a build. A sentence before a page.
3. **The human decides, the agent argues.** Testimony, not verdict. An agent that decides for the human has broken the method; a human who rubber-stamps has abandoned it. A decision the human names in writing and hands over is neither: delegation is explicit, scoped to what was named, logged, and revocable, and it never covers a one-way door.
   - *Scoped by the house standard (`docs/HOUSE_STANDARD.md`): the agent owns the house and objective layers (T0/T1) and declares them; applies an evidenced preference with the inference flagged (T2); asks on novel taste (T3), which is where the never-judge-taste rule bites.*
4. **A claim must survive a repeat.** If a number or a judgement flips when it is checked again, the work is not done.
5. **Name what you are deliberately not fixing.** The last stretch belongs to the human, so it is handed over in writing, with no structural defect hidden inside it.

### What it is not

Not waterfall: the loop runs many times, on cheap artifacts. Not ceremony: an artifact nobody reads costs without benefit, and skipping it with a logged reason is legitimate. Not everything up front: the aim is not a perfect plan but the cheapest artifact that can falsify the expensive decision.

### How to tell you are following it

- No build started before a ratified plan or spec.
- Every skipped gate logged with its reason.
- Every measured number reproducible, with its method stated.
- The residue handed to the human is small, named, and free of structural defects.

## The goal

> **Altitude spine (authoritative: `steps/QUICKSTART.md`; every file quotes this line verbatim).** P1 WANT (INBOX → EXTRACTION) → P2a SHOULD-BUILD-X? (SERIOUSNESS Bar 1: DROP/COMMIT) → P2b WHICH-X? (LANDSCAPE + STRATEGY + AMBITION Bar 2) → P3 BEST_PLAN (FUNDAMENTALS → DECOMPOSITION → VALIDATION) → P4 EXECUTE (SPECIFICATION → EXECUTOR → REVIEW → REFLECT → PRIORITIZE). Bar 1 must clear before Bar 2. This file defines the scoreboard; QUICKSTART defines the spine.

Every run of this protocol leaves three things better than it found them:

1. **The project** — scoped tighter, verified harder, shipped.
2. **The human** — more judgment, less abdication, carrying further next time.
3. **The method** — the protocol itself, revised by what this run taught.

The strategist-at-every-level is the engine. The three-output scoreboard is the point.

## Where each output lives

- Project: SPECIFICATION → EXECUTOR → VALIDATION → REVIEW.
- Human: the learning layer (FEATURES F-017) — tripwire, explain-back, standing decisions, per-cycle signal.
- Method: cross-cycle seeding (REFLECT Q9), kill-gate calibration (KILL_LOG retros), the 3-pass shipping rule (STANDING_PRINCIPLES).

## Self-evolution guard

The protocol may rewrite itself, but every self-run needs one external check — a stranger, a live run, or a metric. Without friction it self-congratulates.

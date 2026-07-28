# VALIDATION.md — The Prototyping Gate

> This is a **decision gate**, not a development phase. The goal is not to build something
> that ships — it's to learn enough to make a KILL/PIVOT/COMMIT decision with confidence.
>
> The output is a decision, not code. Treat prototypes as throwaway by default.
> If you wouldn't delete it after the decision, you're over-investing.

---

## When to Use This

You've completed AMBITION.md (Gate 1 — goal is clear) and LANDSCAPE.md (research done). You have unknowns that only building can reveal. This gate tests those unknowns cheaply before you commit to the full build.

**Prep-pipeline arrival:** You have a falsifiable hypothesis from AMBITION.md and identified unknowns from LANDSCAPE.md. Skip directly to SPIKE design.

**Fresh-start:** No prep done. Start with the loop below.

---

## The Prototype Mindset (Required)

Before you prototype, adopt this stance. It determines whether this gate works or wastes time:

1. **Disposability** — This prototype WILL be deleted. If you're designing it to last, you're over-investing.
2. **Reactions over feedback** — Don't ask "do you like it?" Watch what people DO. The Design Sprint principle: "Make it look real enough to get real reactions, nothing more."
3. **Goldilocks quality** — Not too polished (that signals it's done and discourages honest critique). Not too broken (nobody can interact with it). Just barely real enough.
4. **The one question** — Every prototype answers ONE question. If you have multiple questions, run multiple prototypes. A prototype that tries to answer everything answers nothing.
5. **Fail fast is success** — If the prototype disproves your assumption, that's a WIN. You learned before building.

---

## The Validation Loop

```
[START]
    │
    ├── FORMULATE HYPOTHESIS (15 min)
    │   └── "I believe [specific assumption] is true"
    │       → Must be falsifiable with evidence
    │
    ├── CHOOSE PROTOTYPE TYPE (5 min)
    │   └── What type of prototype tests this assumption fastest?
    │
    ├── SPIKE (timeboxed)
    │   └── Build the thinnest possible test
    │
    ├── TEST (1 hour)
    │   └── Run against real conditions
    │
    └── DECISION (15 min)
        ├── COMMIT → SPECIFICATION.md
        ├── PIVOT → Reformulate hypothesis or try different approach
        └── KILL → Document learnings, exit
```

---

## The Prototyping Phase

### Step 1: Formulate a Falsifiable Hypothesis

Required. Every spike starts with a falsifiable claim. Without this, you're tinkering, not prototyping.

> I believe that **[specific approach/intervention]** will **[specific outcome]** for **[specific context]** .

**Apply the pre-mortem before prototyping:** Imagine it's 3 months from now and your prototype was a complete failure. Write a 2-sentence story of WHY it failed. This surfaces hidden assumptions before you invest time.

**Check:** Can you define what evidence would DISPROVE your hypothesis? If not, reformulate.

### Step 2: Choose the Prototype Type (Decision Tree)

Not all prototypes are code. Choose the type that tests YOUR riskiest assumption fastest:

| Assumption type | Fastest prototype | Example |
|----------------|-------------------|---------|
| **Will users do X?** | Smoke test (landing page with "Buy" button, no product) | Dropbox explainer video before building |
| **Does the UX feel right?** | Paper prototype / wireframe | Sketch screens, test with user clicks |
| **Does the algorithm work?** | Coded functional prototype | Thinnest-possible implementation |
| **Does the backend scale?** | Wizard of Oz (human simulates AI) | Customer service "AI" that's really a person |
| **Is the concept valuable?** | Concierge MVP (do it manually for one user) | Zappos: buy shoes, ship manually |
| **Is the integration feasible?** | Breadboarding (Shape Up) | Wire the components, skip the UI |
| **Which approach is better?** | Parallel spikes (A/B test two approaches) | Compare two frameworks in 2 hours each |

**Rule:** Pick the prototype type that tests the RISKIEST assumption in the SHORTEST time. If you can test it with a Wizard of Oz in 2 hours instead of building an API in 2 days, do the Wizard.

### Step 3: SPIKE (Timeboxed)

**Timebox:** Fixed — 2-8 hours. If it takes longer, you're overbuilding.

**Rules:**
- ONE happy path only. No error handling. No edge cases. No tests.
- No polish. No documentation. No CI.
- Use the fastest tool for the job (Python > TypeScript > C# > Rust for spikes).
- If the spike takes more than 8 hours, stop and reassess. You're either overbuilding or testing the wrong assumption.

**Output:** Evidence for or against the hypothesis.

### Parallel Prototyping

When multiple approaches could work or the core assumption has several independent unknowns:

- Run 2-3 spikes simultaneously
- Each spike tests a different approach or assumption
- Compare results at the decision gate

**Entry-Point Routing:**
- **Fresh-start users:** Begin with IDEA GENERATION → FILTER → SPIKE
- **Prep-pipeline arrivals:** Skip to SPIKE with your pre-existing hypotheses

### Step 4: Test (1 Hour)

Run the spike against real conditions:
- Real data (sample dataset, real API)
- Real user (yourself, a friend, target audience)
- Real environment (actual hardware, actual constraints)

**For coded prototypes:** Does it compile/run? Does it produce correct output on sample data?
**For Wizard of Oz:** Did the user interaction reveal what you expected?
**For smoke tests:** Did people click/buy/sign up?
**For paper prototypes:** Did the user flow make sense to a real person?

### Step 5: Decision (15 Minutes)

The decision uses structured criteria, not gut feel.

---

## Kill Criteria Matrix (Weighted)

Each criterion has a weight. Core assumption validation is the MOST important — it gets 4x the weight of other criteria.

| Criterion | Weight | Pass (2) | Weak Pass (1) | Fail (0) | Score |
|-----------|--------|----------|---------------|----------|-------|
| **Core assumption validated** | **4x** | Spike proves the assumption | Partially works, needs more | Disproves the assumption | |
| **Feasible within constraints** | 2x | Buildable with resources | Buildable with compromises | Not buildable | |
| **Worth building** | 1x | Excited to build | Interested | Not interested | |
| **Learning value** | 1x | Valuable even if it fails | Learns something useful | Learns nothing | |

**Decision rules:**
- **COMMIT:** Core assumption Pass (score ≥ 8) AND total score ≥ 12
- **PIVOT:** Core assumption Weak Pass (score 4-6) — try different approach
- **KILL:** Core assumption Fail (score 0-2) OR total score < 12
- **MIXED:** The spike with the highest weighted score wins. Don't average.

The weighted scoring prevents the common failure mode of "everything looks OK except the one thing that matters."

---

## Learnings Capture

After each spike (regardless of outcome):

```
Spike: [name]
Hypothesis tested: [the falsifiable claim]
Outcome: [COMMIT / PIVOT / KILL]
What worked: [surprising successes]
What didn't: [surprising failures]
Evidence: [specific data that supported the decision]
Learnings to carry forward: [insights that inform SPEC.md]
Carry-forward rule: LEARNINGS ONLY, NOT CODE. The spike code is throwaway.
```

---

## Confirmation Bias Safeguards

Prototypes naturally confirm what you want to believe. These safeguards prevent that:

1. **Pre-mortem** — Before prototyping, write a 2-sentence story of why the prototype failed. Done.
2. **Devil's advocate** — Before the decision, argue FOR the alternative. If you got COMMIT, argue why it should be KILL. If you got KILL, argue why it should be COMMIT.
3. **The "so what?" test** — After the prototype, ask: "If this works, does it actually matter?" A working prototype of a useless thing is still useless.
4. **External observer** — If possible, have someone who wasn't involved evaluate the results.
5. **Default OFF** — Treat the decision as "not COMMIT" by default. The evidence must push it to COMMIT, not the other way around.

---

## Failure Modes of Prototypes

Recognize these patterns:

| Failure mode | Looks like | What's really happening |
|-------------|-----------|------------------------|
| **Overdelivery** | The prototype is production-quality, takes 2 weeks | You're not prototyping, you're building. Stop and restart with a thinner spike. |
| **Underdelivery** | The prototype is too broken to test anything | You're not testing the right assumption. Reformulate the hypothesis. |
| **False positive** | The prototype works in isolation but fails in reality | The spike was too narrow. Test against real conditions. |
| **False negative** | The prototype fails but the idea is sound | You tested the wrong assumption or the wrong approach. Reformulate. |
| **Confirmation spiral** | Each prototype "works" but the real project keeps getting harder | You're testing safe assumptions, not the risky ones. Reverse your priority order. |

---

## Tools for Fast Spikes

| Language | Framework | Bootstrap | Best for |
|----------|-----------|-----------|----------|
| Python | FastAPI | `pip install fastapi && uvicorn main:app` | APIs, data processing, ML |
| TypeScript | Next.js / Hono | `npx create-next-app` | Web apps, UIs |
| Python | Streamlit | `pip install streamlit && streamlit run app.py` | Data dashboards, demos |
| Shell | bash | Any Linux machine | System tools, file processing |
| Python | — | `python3 -c "..."` | The fastest possible test |
| Rust | clap | `cargo new && cargo run` | When you want to & it compiles fast |
| Balsamiq / Figma | Wireframe | Drag and drop | UI flow / paper prototype |
| Typeform / Google Forms | Smoke test | Form builder | Market validation |

**Default to Python for throwaway spikes.** For the real project, use whatever fits.

---

## Relationship to RULES.md

VALIDATION.md is the **central decision gate** in the PREP PHASE pipeline. It runs after AMBITION.md and LANDSCAPE.md, and before SPECIFICATION.md and EXECUTOR.md.

Once VALIDATION produces a COMMIT decision, the learnings (NOT code) flow into SPECIFICATION.md, which produces the locked execution specification. EXECUTOR.md then hands off to RULES.md for autonomous execution.

The spike prototype remains throwaway — DO NOT carry code into the real project. Only carry forward the learnings, assumptions, and validated decisions.

```
PREP PHASE:
AMBITION.md → LANDSCAPE.md → VALIDATION.md (GATE 2) → SPECIFICATION.md → EXECUTOR.md → RULES.md
```

---

## Meta: How to Prototype Effectively

1. **Design the experiment before the prototype** — What's the hypothesis? What evidence would disprove it? Only then build.
2. **The BEST prototype is the one someone else built** — before building, search for an existing tool/library that could serve as the prototype.
3. **Prototype the riskiest assumption FIRST** — people naturally prototype what they know how to build. Force yourself to prototype the thing most likely to fail.
4. **The 90% rule** — a prototype that's 90% done and disproves the hypothesis is perfect. A prototype that's 90% done and "almost works" is a trap — don't spend the extra 90% time to get it to 100%.
5. **Kill with pride** — a killed project is a success. You learned before committing weeks. Document why and move on.
6. **Two prototypes are often better than one** — parallel prototypes force comparison, which makes the decision more objective than evaluating a single prototype's absolute "good enough" feel.

---

## Origin

Rewritten July 2026 following a deep research pass on prototyping methodology. Incorporates findings from Design Sprint (Google Ventures), Lean Startup (Eric Ries), Shape Up (Basecamp), Wizard of Oz prototyping (HCI literature), decision frameworks (Cooper's Stage-Gate), and experimental design methodology.

# Research: Rapid Prototyping Methodology for Software Projects

> **Date**: 2026-07-11
> **Context**: Research to inform the rewrite of `VALIDATION.md` — the prototyping gate in the Development Protocol's PREP PHASE. The current VALIDATION.md introduces a SPIKE → TEST → DECISION loop with KILL/PIVOT/COMMIT outcomes. This research provides the methodological foundation for that loop.
> **Method**: Web search + existing knowledge base synthesis across Design Sprint (GV), Lean Startup (Ries), Shape Up (Singer/Basecamp), HCI prototyping research, and decision science.

---

## Summary

The Development Protocol's VALIDATION phase is a **rapid prototyping gate** — a timeboxed spike to test core assumptions before committing to a full build. This is the protocol's central innovation. The research confirms this is sound but reveals several gaps:

1. **The "prototype mindset"** — a specific psychological stance characterized by speed, disposability, and Goldilocks quality — is critical and currently unarticulated in VALIDATION.md.
2. **The MVP concept is widely misunderstood**: an MVP tests the *riskiest assumption*, not the smallest feature set. VALIDATION.md's "thinnest possible prototype" aligns with this but could be sharper.
3. **Shape Up's shaping/betting distinction** offers a useful pre-prototyping filter that VALIDATION.md doesn't currently have — the "appetite" conversation should happen *before* the spike.
4. **Multiple prototype types exist** (Wizard of Oz, concierge, smoke test, paper prototype, breadboard) and the right choice depends on what *kind* of assumption is being tested. VALIDATION.md currently only describes coded functional prototypes.
5. **Confirmation bias is the #1 threat to prototyping validity**. The current document has no safeguards against it.
6. **Decision frameworks** for KILL/PIVOT/COMMIT exist in the Stage-Gate literature (Cooper) and Lean Startup (Ries). The current Kill Criteria Matrix is a good start but needs refinement with falsifiable hypotheses.

---

## Sources Surveyed

| Source | Type | Key Contribution |
|--------|------|-----------------|
| Knapp et al., *Sprint* (2016) | Book | 5-day Design Sprint; prototype mindset; Goldilocks quality; façade prototyping |
| Ries, *The Lean Startup* (2011) | Book | Build-Measure-Learn; MVP as learning vehicle; value/growth hypotheses; innovation accounting |
| Singer, *Shape Up* (2019) | Book/Free | Appetite vs. estimation; breadboarding; circuit breaker; shaping as pre-prototyping |
| Ries, "The Prototype Mindset" (2016) | Article | Four principles of the prototype mindset (Sprint excerpt) |
| Blank, *The Four Steps to the Epiphany* (2013) | Book | Customer development; get out of the building; riskiest assumption first |
| Cooper, "Stage-Gate Systems" (1990-present) | Framework | Go/kill decision points; predefined criteria; portfolio management |
| Strategyzer, "Don't Build When You Build-Measure-Learn" (2016) | Article | Reverse the loop: Learn → Measure → Build; MVP taxonomy |
| Nielsen Norman Group, "Wizard of Oz Method in UX" (2024) | Article | WoZ methodology; closed/open/hybrid response modes; practical setup |
| Arkaro, "The Prototyping Trap" (2025) | Article | Three failure modes: overdelivery, underdelivery, inadequate risk ID |
| Stack Overflow / SE, "Throwaway vs. Evolutionary Prototyping" | Community | Decision criteria for prototype disposition |
| UXPin, "What is Throwaway Prototyping" (2026) | Article | When to throw away vs. evolve |

---

## 1. Prototyping Techniques by Project Type

Different project types demand different prototype forms. The right prototype tests the *specific assumption* the project hinges on — not a generic "does it work?".

### Prototype Type Reference Table

| Technique | Fidelity | Time | Best For | Assumption Tested | Example |
|-----------|----------|------|----------|-------------------|---------|
| **Paper prototype** | Low | Hours | UI flow, navigation | "Users can find feature X" | Sketch screens, test with users |
| **Wireframe / clickable mockup** | Medium | 1-2 days | App/website UX | "This layout makes sense" | Figma prototype with hotspots |
| **Breadboarding** (Shape Up) | Low-Medium | Hours | Feature design | "The right components connect correctly" | Box-and-arrow UI sketch (no styling) |
| **Fat-marker sketch** (Shape Up) | Low | Minutes | High-level solution shape | "The scope fits the appetite" | Thick-pen drawing on paper |
| **Coded functional prototype** | High | 2-8 hours | Technical feasibility | "This core algorithm/API works" | Python script, single endpoint |
| **Wizard of Oz** | Medium-High | 1-3 days | AI/backend behavior, automation | "Users will trust/use this automated system" | Human simulates AI responses |
| **Concierge MVP** | High (service) | Days-weeks | Value proposition | "Customers will pay for this solution" | Founder personally delivers service |
| **Smoke test / landing page** | Low | Hours | Demand validation | "People want this enough to sign up" | Pre-launch page with CTA |
| **Single-feature prototype** | Medium-High | 1-3 days | Core mechanic | "The key feature provides value" | Build one feature, nothing else |
| **Data sheet / brochure** | Low | Hours | Positioning / messaging | "This value proposition resonates" | One-pager shown to prospects |
| **Storyboard** | Low | 1-2 hours | Problem-solution fit | "Users recognize this pain/solution pattern" | Comic strip of user journey |
| **Video prototype** | Medium | 1-2 days | Complex/product demos | "Seeing this in action creates desire" | Mockup video of product use |
| **Learning prototype** (Strategyzer) | Varied | Hours-days | Core business hypothesis | "The key unknown is resolvable" | Quick functional test of riskiest assumption |

### Decision Tree: Which Prototype Type?

```
What is the riskiest assumption?

├── TECHNICAL: "Can we build it?"
│   └── → Coded functional prototype, breadboarding
│
├── UX: "Will users understand it?"
│   └── → Paper prototype, wireframe, clickable mockup
│
├── VALUE: "Do people want this?"
│   └── → Smoke test, concierge MVP, landing page
│
├── AUTOMATION: "Will users accept AI/automation?"
│   └── → Wizard of Oz prototype
│
├── SCOPE: "Does the solution fit the time budget?"
│   └── → Fat-marker sketch, breadboarding (Shape Up)
│
└── MESSAGING: "Does the value proposition resonate?"
    └── → Data sheet, brochure, storyboard
```

### When to Use Each by Project Type

**Libraries / frameworks / tools:**
- Coded functional prototype (always the primary choice)
- Breadboarding for API design
- Smoke test (landing page / GitHub stars) for demand validation

**Web/mobile apps:**
- Wireframe/clickable mockup for UX validation
- Coded prototype for technical feasibility
- Concierge MVP for value proposition (manually serve users)
- Wizard of Oz for AI/ML features

**Data / ML / AI products:**
- Wizard of Oz (simulate the AI output before building the model)
- Jupyter notebook as prototype
- Learning prototype with a simple heuristic instead of the full model

**Research / exploration projects:**
- Coded functional prototype (most common)
- Storyboard or video for communicating concept
- Paper prototype for very early ideation

**Hardware / physical products:**
- Product box mockup
- Video prototype
- Wizard of Oz (simulate automation)

---

## 2. Design Sprint Prototyping (Google Ventures)

### Source
Jake Knapp, John Zeratsky, Braden Kowitz. *Sprint: How to Solve Big Problems and Test New Ideas in Just Five Days* (2016). Based on 150+ sprints at Google Ventures.

### The 5-Day Structure

| Day | Focus | Key Activity |
|-----|-------|-------------|
| Monday | Map | Unpack the problem, choose a target |
| Tuesday | Sketch | Individual solution sketching (competing ideas) |
| Wednesday | Decide | Critique solutions, storyboard the winner |
| **Thursday** | **Prototype** | **Build a realistic façade in one day** |
| Friday | Test | 5 user interviews with the prototype |

### The Prototype Mindset (Thursday)

The Design Sprint's Thursday is the critical day for prototyping methodology. Knapp identifies four principles:

**1. You Can Prototype Anything**
Optimism and conviction matter. If you believe there's *some way* to prototype and test, you will find it. This is a deliberate psychological stance.

**2. Prototypes Are Disposable**
Never prototype anything you aren't willing to throw away. The longer you work on a prototype, the more attached you become. After one day, you're receptive to feedback. After three months, you're committed. **Emotional attachment is the enemy of objective decision-making.**

**3. Build Just Enough to Learn, But Not More**
The prototype is meant to answer questions. Keep it focused. You don't need a fully functional product — you need a real-looking façade to which customers can react.

**4. The Prototype Must Appear Real**
To get trustworthy results, you cannot ask customers to use their imaginations. Show something flimsy (a paper prototype, simplified wireframe) and the illusion breaks. Once the illusion breaks, customers switch into "feedback mode" — they try to be helpful and think up suggestions. **Reactions are solid gold; feedback is worth pennies on the dollar.**

### Goldilocks Quality
As Daniel Burka (GV design partner) says: "If the quality is too low, people won't believe the prototype is a real product. If the quality is too high, you'll be working all night and you won't finish." The ideal is **Goldilocks quality** — just real enough to evoke genuine reactions, just fast enough to finish in one day.

### The Façade Metaphor
A Design Sprint prototype is like an Old West movie set — a saloon façade with nothing behind it. You don't need plumbing, wiring, or structural engineering. You need a surface that looks real for the few minutes the user will interact with it. This insight allows teams to reach "90% real" on day one rather than spending 90 days to get from 0% to 90%.

### Key Materials and Techniques
- **Keynote/PowerPoint** for realistic digital prototypes (surprisingly effective for app demos)
- **Figma/Sketch** for interactive mockups
- **InVision/ProtoPie** for click-through prototypes
- **Paper and markers** for early-stage sketching
- **Existing products as templates** (borrow UI patterns from real apps)
- **Asset collectors** — designate someone to find real photos, icons, content rather than building from scratch

### Application to VALIDATION.md
The Design Sprint's Day 4 is essentially a timeboxed prototype sprint. Key takeaways:
- The **prototype mindset** (disposability, Goldilocks quality, reaction-over-feedback) should preface the SPIKE phase
- The **façade approach** — building only the surface the test needs to see — is the correct stance for a spike
- The **one-day constraint** validates the 2-8 hour timebox in the current protocol
- The **4:1 screen/interaction ratio** for prototype efficiency (not every screen needs to work; only the ones in the test flow)

---

## 3. Lean Startup MVP (Eric Ries)

### Source
Eric Ries, *The Lean Startup* (2011). Also: Steve Blank, *The Four Steps to the Epiphany* (2013); Strategyzer, "Don't Build When You Build-Measure-Learn" (2016).

### The MVP Is a Learning Vehicle, Not a Product

The single most important insight from Lean Startup for VALIDATION.md:

> "It is not necessarily the smallest product imaginable... it is simply the fastest way to get through the Build-Measure-Learn feedback loop with the minimum amount of effort."
> — Eric Ries, *The Lean Startup*

An MVP is **not** a stripped-down version of the final product. It is an **experiment designed to produce learning**. The goal is not to ship; the goal is to validate or invalidate a hypothesis.

### The Build-Measure-Learn Loop

```
[IDEAS] → [BUILD] → [PRODUCT] → [MEASURE] → [DATA] → [LEARN] → [IDEAS] → ...
```

**Critical insight from Strategyzer:** Plan tests in **reverse order**:
1. What do I need to **LEARN**?
2. What **MEASUREMENT** will produce that learning?
3. What **MVP** should I BUILD to get that measurement?

Most teams start at BUILD and build too much. Starting at LEARN forces you to design the prototype around the *specific question*, not the *feature set*.

### Riskiest Assumption Test

Ries and Blank emphasize: **test the riskiest assumption first**, not the easiest one. The riskiest assumption is the one whose falsification would most damage the business case.

**Technique: Assumption Ranking**
1. List all assumptions the project depends on
2. For each: "If this assumption is false, does the project still make sense?"
3. The assumption whose falsification most damages the project = the riskiest
4. Test that one first

**Two fundamental hypotheses** (Ries):
- **Value hypothesis**: Does the product deliver value to customers once they're using it?
- **Growth hypothesis**: How will new customers discover the product?

For a software prototype, the value hypothesis is almost always the riskiest.

### Innovation Accounting

Ries proposes replacing vanity metrics with **innovation accounting**:
1. Establish the baseline (current state with current product/behavior)
2. Tune the engine (experiments to move toward the ideal)
3. Pivot or persevere (decision after the experiment)

### Application to VALIDATION.md

- The SPIKE phase should start with a **falsifiable hypothesis statement** (not a feature list)
- Pre-spike work should identify the **riskiest assumption** using the ranking technique
- The TEST phase should measure **one clear metric** tied to the hypothesis
- The DECISION phase should reference **innovation accounting** — did the metric move meaningfully?
- **MVP breadth vs. depth tradeoff**: A single-feature prototype (vertical slice) beats a shallow multi-feature prototype every time

---

## 4. Shape Up (Basecamp / Ryan Singer)

### Source
Ryan Singer, *Shape Up: Stop Running in Circles and Ship Work that Matters* (2019). Available free at basecamp.com/shapeup.

### Shaping vs. Prototyping

Shape Up draws a crucial distinction that VALIDATION.md currently lacks:

| Activity | Output | When | Who |
|----------|--------|------|-----|
| **Shaping** | A **pitch** (problem + appetite + solution sketch + rabbit holes) | Before betting | Senior team |
| **Prototyping** | A **working feature** | During the cycle | Team |
| **Building** | The **shipped product** | End of cycle | Team |

**Shaping is NOT prototyping.** Shaping produces a rough, solved, bounded description of what to build — at the level of breadboards and fat-marker sketches. It happens *before* the team starts building. It identifies rabbit holes and eliminates them before they reach the team.

### Key Shape Up Concepts for Prototyping

**Appetite (not estimation):**
Instead of asking "how long will this take?" ask "how much time is this worth?" The appetite is a **creative constraint**, not a guess. "Small batch" = 1-2 weeks. "Big batch" = 6 weeks.

**Fixed time, variable scope:**
The timebox is fixed. If the solution doesn't fit, **hammer the scope** — don't extend the time. This directly supports the SPIKE timebox in VALIDATION.md.

**Circuit breaker:**
If work doesn't ship within the appetite, by default it's killed. No extensions. This prevents throwing good time after bad. The decision is automatic — you must actively decide to extend, not actively decide to cancel.

**Breadboarding:**
A UI design technique that shows *components and connections* without visual design. Named after electronics breadboards. It's the right level of abstraction for prototyping a solution: concrete enough to know what to build, abstract enough that there's room for the team to make design decisions.

**Fat-marker sketches:**
Draw interfaces with a thick marker so you *cannot* include too much detail. This enforces the right level of abstraction during shaping.

**Scopes and the hill chart:**
Work is tracked on a hill — going up means figuring out unknowns, coming down means execution. A prototype moves a scope "uphill" by resolving unknowns.

**The pitch document** (5 ingredients):
1. Problem — what's the raw idea?
2. Appetite — how much time is this worth?
3. Solution — breadboard/fat-marker sketch
4. Rabbit holes — identified risks
5. No-gos — explicitly excluded

### Application to VALIDATION.md

- Add a **pre-spike shaping step** where the appetite is set and the solution is sketched at breadboard level
- The **circuit breaker** concept should apply to the SPIKE timebox: if it runs over, default is KILL
- **Scope hammering** during the spike: when you hit complexity, cut scope, not quality
- The SPIKE output should include a **rabbit holes identified** section
- Add **breadboarding** as a recommended technique for the shaping step before the coded spike

---

## 5. Wizard of Oz Prototyping

### Source
Nielsen Norman Group, "The Wizard of Oz Method in UX" (2024); Strategyzer MVP Taxonomy; Jeff Kelley (1983, coined the term).

### Definition

The **Wizard of Oz (WoZ)** method is a moderated research method where a user interacts with an interface that appears to be autonomous but is fully or partially controlled by a human ("the wizard"). The user believes the system is real, but a human is pulling the levers behind the curtain.

### When to Use

WoZ is appropriate when:
- The technology doesn't exist yet (AI, ML, complex algorithms)
- The technology is too expensive to build for testing
- You need to test user response to automation before building it
- The interface involves real-time data retrieval, personalization, or natural language

### The Zappos Canonical Example

Before building warehouses, inventory systems, or automation, Zappos founder Nick Swinmurn tested the value proposition by:
1. Setting up a simple website with shoe photos
2. When orders came in, going to a local shoe store, buying the shoes, and mailing them
3. The customer had no idea — the system appeared fully automated

This validated that people would buy shoes online before investing in infrastructure.

### Response Modes

| Mode | Description | Best For |
|------|-------------|----------|
| **Closed** | Wizard chooses from preset responses | Simple, predictable interactions |
| **Open** | Wizard creates responses on the fly | Complex flows, conversational UIs |
| **Hybrid** | Preset list + ability to create new | Most flexible; recommended default |

### Setup Process (NN Group)

1. **Create the prototype** — Figma, coded, or proxy tool
2. **Decide response modes** — closed/open/hybrid
3. **Create a study protocol** — roles, questions, fallback responses
4. **Choose and prepare the wizard** — must know the product concept and response system
5. **Pilot the study** — ensure the wizard can keep up

### Key Distinction: WoZ vs. Concierge MVP

- **WoZ**: User does NOT know a human is involved. Tests perceived automation and trust.
- **Concierge MVP**: User KNOWS a human is involved. Tests value proposition directly.

Both deliver value manually without building technology. The difference is **user awareness** — and this determines what assumption is being tested.

### Application to VALIDATION.md

- WoZ is critical for AI/ML projects — test whether users trust the AI *before* building it
- Add WoZ as a recognized prototype type in the SPIKE phase
- Document the 5-step setup process for teams running WoZ spikes
- The **fallback response** ("Loading...", "Under construction") is essential for handling unexpected user actions
- The **wizard should not be the interviewer** — emotional investment in the facade biases the test

---

## 6. Experimental Design for Spikes

### The Problem: Confirmation Bias

Confirmation bias is the single greatest threat to prototyping validity. People naturally build prototypes that confirm what they already believe. This is well-documented in cognitive science and software engineering research.

Common confirmation bias patterns in prototyping:
- Testing only the happy path
- Choosing easy-to-verify assumptions over risky ones
- Interpreting ambiguous results as "validated"
- Building the prototype in a way that eliminates failure modes
- Dismissing negative results as "real-world would be different"

### Falsifiable Hypotheses

The single most effective technique against confirmation bias: **write the hypothesis in falsifiable form BEFORE the spike**.

**Bad:** "I think users will like this feature."
**Good:** "Users will complete the core workflow in under 30 seconds on their first attempt without instruction."
**Falsifiable:** If users average >30 seconds, the hypothesis is disproven.

The test of a good hypothesis: **Would you accept the result if it's negative?** If the answer is no, the hypothesis is not falsifiable.

### Pre-Mortem Technique (Klein, 1989)

Before the spike, write a **pre-mortem**: "Assume it's 6 months from now and this project failed completely. Write the history of how it failed."

This primes the team to look for failure modes rather than confirmation. It's one of the most effective debiasing techniques in the decision science literature.

### Hypothesis Template for Spikes

```
Core assumption: [The thing that must be true for the project to be viable]
Falsifiable claim: [Specific, measurable outcome that would disprove it]
Minimum success criterion: [The threshold that indicates the assumption held]
Test design: [What the spike will actually do, and how you'll measure]
Riskiest alternative: [What would it mean if the hypothesis is wrong?]
```

### Avoiding the "Happy Path Trap"

VALIDATION.md currently says "Works on ONE happy path only." This is correct for speed. But the TEST phase must include at least one **negative condition** — what happens when the happy path fails? Even a manual check ("what if the API returns 500?") reveals whether the core assumption is truly robust.

### The Scientific Method for Spikes

1. **Observation** — "I think projects fail because teams don't validate assumptions"
2. **Hypothesis** — "A structured prototyping gate with KILL/PIVOT/COMMIT will reduce failed projects by X%"
3. **Prediction** — "Teams using the gate will kill bad ideas sooner and commit to good ideas more confidently"
4. **Experiment** — "Build the VALIDATION phase and run it on 3 projects"
5. **Analysis** — "Did it work? Were decisions clearer? Faster? More accurate?"

### Application to VALIDATION.md

- The SPIKE phase must start with a **falsifiable hypothesis** in the template format
- Add a **pre-mortem step** before the spike to surface hidden assumptions
- The TEST phase should include at least one **failure case** even for a happy-path prototype
- Add an explicit **debiasing step**: "What would disprove this? Are we willing to accept that?"
- The hypothesis must include a **minimum success criterion** — not just "works"

---

## 7. Failure Modes of Prototypes

### Source
Arkaro, "The Prototyping Trap: How Testing Failures Destroy Product Launches" (2025); Addy Osmani, "The Value of a Prototype Is in the Insight It Imparts" (2023).

### Three Critical Failure Modes

**1. Technical Overdelivery (Type I Error)**
The prototype works but solves the wrong problem.
- *Juicero*: $400 Wi-Fi juicer whose juice packets could be squeezed by hand
- *Google Glass*: Impressive engineering that users didn't want to wear in public
- *Segway*: Breakthrough gyroscopic tech that solved a non-existent transportation problem
- **Why it happens**: Teams fall in love with technical solutions and skip customer value validation
- **How to detect it**: The prototype demonstrates impressive capabilities, but you can't articulate *why a customer would care*

**2. Technical Underdelivery (Type II Error)**
The prototype fails but the real project would work.
- *Theranos Edison device*: Promised capabilities that the technology fundamentally couldn't deliver
- *Monsanto Roundup Ready*: Resistance predicted to take decades emerged in 5 years
- *New Coke*: 200,000 taste tests validated the flavor but missed brand attachment
- **Why it happens**: The prototype tests the wrong thing or uses flawed methodology
- **How to detect it**: The failure mode is unrelated to the core value proposition (e.g., UI bugs when testing a core algorithm)

**3. Inadequate Risk Identification (Type III Error)**
The testing didn't cover critical scenarios.
- *StarLink corn*: Contamination of 50% of US corn supply because buffer strips were insufficient
- *John Deere brake linkage*: Worked in standard testing, failed across diverse field conditions
- *New Coke*: Tested taste but not brand attachment
- **Why it happens**: Testing focuses on upside potential while neglecting downside scenarios
- **How to detect it**: The "what could go wrong" list is shorter than the "what we're testing" list

### The Prototype Paradox

Addy Osmani: "The value of a prototype is in the insight it imparts, not the code." A prototype that fails technically but teaches something valuable is more successful than a prototype that works technically but teaches nothing.

### When Prototypes Lie: Recognition Heuristics

| Symptom | Likely Failure Mode | What to Do |
|---------|---------------------|------------|
| "It works but I don't know why anyone would use it" | Technical overdelivery | Test value proposition with non-technical users |
| "The prototype is beautiful but crashes constantly" | Technical underdelivery | Fix the core before polishing the surface |
| "Users love it in the demo but it flops in production" | Inadequate risk identification | Add environmental stress to testing |
| "The team is excited but can't articulate the assumption being tested" | Confirmation bias | Rewind to hypothesis formation |
| "We keep adding features to the prototype" | Scope creep / emotional attachment | Enforce the circuit breaker |

### Application to VALIDATION.md

- Add a **failure mode checklist** to the SPIKE phase — explicitly name which failure mode you're at risk of
- **Technical overdelivery** is the most dangerous for the Development Protocol's audience (solo devs/teams excited about cool tech)
- The DECISION phase should include a **failure mode analysis**: "Did our prototype suffer from overdelivery, underdelivery, or inadequate risk ID?"
- The "prototype works but real project doesn't" scenario is usually **overdelivery** — the prototype demonstrated feasibility of a non-critical feature

---

## 8. Throwaway vs. Evolutionary Prototyping

### Sources
Stack Overflow (community discussion); UXPin, "What is Throwaway Prototyping" (2026); Simplicable comparison.

### The Canonical Distinction

| Dimension | Throwaway | Evolutionary |
|-----------|-----------|--------------|
| **Purpose** | Answer specific questions | Iterate toward final product |
| **Code quality** | Deliberately low (speed) | Production-quality from start |
| **Lifespan** | Hours to days | Weeks to months |
| **Technology choice** | Fastest language (Python > TS > Rust) | Best language for the domain |
| **Error handling** | None | Production-grade |
| **Tests** | None | Full test suite |
| **Documentation** | None | Complete |
| **Decision criteria** | Specific assumption validated | User feedback incorporated |

### Decision Criteria

**Build a throwaway prototype when:**
- You have fundamental unknowns about feasibility or value
- Speed of learning is the priority
- The technology choice for the prototype differs from the target production stack
- You need to compare multiple approaches quickly
- The prototype answers a binary question (works/doesn't work)
- The core insight is the *knowledge*, not the *code*

**Build an evolutionary prototype when:**
- The requirements are relatively well understood
- The prototype is also a viable path to production
- User feedback is the primary unknown (not technical feasibility)
- The production stack is known and matches the prototype
- Rewriting from scratch would cost more than the accumulated technical debt

### The Risk: Throwaway Becoming Production

The most dangerous anti-pattern: a throwaway prototype that "works" gets pushed into production because it saves time. This consistently leads to:

- Unmaintainable code (no tests, no error handling)
- Security vulnerabilities (no security considerations in the prototype)
- Performance issues (prototypes optimize for speed of building, not speed of running)
- Technical debt that costs more to fix than the rewrite would have cost

**Rule**: If you're considering keeping the prototype, the first step is NOT "clean it up" — it's "rewrite the core, keep the learnings." VALIDATION.md already has this right: "Carry-forward rule: LEARNINGS ONLY, NOT CODE."

### Application to VALIDATION.md

- VALIDATION.md correctly defaults to throwaway. This is the right choice.
- Add an explicit **evolutionary prototyping escape hatch** for cases where:
  1. The technology stack is identical for prototype and production
  2. The prototype is the first vertical slice of production
  3. The team explicitly agrees to the carry-forward cost (time to refactor, add tests, handle errors)
- The default should remain throwaway. Evolutionary requires **active opt-in**.

---

## 9. How Much Prototype Is Enough?

### The Thin Enough Test

The Design Sprint answers this with "Goldilocks quality" and "one day." Shape Up answers with the appetite. Lean Startup answers with "the fastest way through the Build-Measure-Learn loop."

Synthesized: **A prototype is "enough" when you have sufficient evidence to make a decision and insufficient attachment to change your mind.**

### Objective Stopping Criteria

**Technical prototypes:** "Enough" is the point where you can answer the core feasibility question. If you can answer it with 10 lines of Python, that's enough. If you need a 3-service architecture to test latency, build that.

**Value prototypes:** "Enough" is the point where you've observed user behavior (not opinions) that either validates or invalidates the value hypothesis. One user session that shows clear confusion is enough. Five sessions that all show the same pattern is enough.

**UX prototypes:** "Enough" is the point where users can complete the task unassisted. If the prototype needs the designer to narrate, it's not real enough.

### The Diminishing Returns Curve

Knapp's insight: the curve from 0% to 90% real is fast (one day). The curve from 90% to 100% real is slow (months). **The prototype should stop at 90%.**

| % Real | Time to Reach | Learning Value | Risk |
|--------|---------------|----------------|------|
| 0-50% | Hours | Low (too abstract) | False negatives |
| 50-90% | 1 day | High (real enough to react) | Sweet spot |
| 90-99% | Weeks | Low (diminishing returns) | Emotional attachment |
| 99-100% | Months | Negligible | Sunk cost |

### How to Know You've Overbuilt

- You're adding edge cases
- You're writing tests
- You're setting up CI/CD
- You're optimizing performance
- You're refactoring for maintainability
- You've caught yourself saying "while we're at it..."
- You can't bring yourself to delete the prototype

If *any* of these apply, you've overbuilt. Stop and assess.

### Application to VALIDATION.md

- Add the **90% rule**: stop at 90% real. The last 10% adds cost but little learning.
- Add the **overbuilding checklist** as a self-diagnostic during the spike
- The SPIKE timebox (2-8 hours) should be enforced as a **hard constraint**, not a suggestion
- Add a **minimum viable prototype (MVP) criteria**: "Can I complete ONE full user journey from start to finish?"

---

## 10. Decision Frameworks (KILL/PIVOT/COMMIT)

### Sources
Cooper, "Stage-Gate Systems" (1990-present); Ries, *The Lean Startup*; LeanPivot.ai, "The Go/No-Go Decision Framework" (2026); Incertive, "Go/No-Go Decision Framework for Business Leaders" (2026).

### Existing Framework in VALIDATION.md

The current Kill Criteria Matrix (Pass / Weak Pass / Fail) is a good start:

| Criterion | Pass | Weak Pass | Fail |
|-----------|------|-----------|------|
| Core assumption validated | Spike proves core works | Partially works, needs more | Disproves assumption |
| Feasibility | Buildable with resources | Buildable with compromises | Not buildable |
| Interest | Excited to build | Interested | Not interested |
| Learning value | Valuable even if fails | Learns something useful | Learns nothing |

### Where It Needs Strengthening

1. **Falsifiability**: The criteria need binary, pre-defined thresholds. "Partially works" is dangerously ambiguous.
2. **Weighting**: Not all criteria are equal. Core assumption validation should outweigh interest.
3. **Emotional bias**: "Excited to build" / "Not interested" is highly susceptible to post-hoc rationalization.
4. **Missing criterion**: **Strategic fit** — does this project serve the larger goal?

### Cooper's Stage-Gate Model

Robert Cooper's Stage-Gate model provides the canonical go/kill framework:

**Gate Criteria Types:**
- **Must-meet** (knockout questions): If any fails, KILL immediately
- **Should-meet** (weighted scoring): Used to compare across projects

**For VALIDATION.md, the equivalent is:**

**Must-meet (any fail = KILL):**
1. Core assumption is validated (pre-defined threshold)
2. Technically feasible within appetite
3. Problem is worth solving

**Should-meet (weighted composite):**
4. Team interest (weight: 20%)
5. Strategic fit (weight: 30%)
6. Learning value (weight: 20%)
7. Ambition match (weight: 30%)

### The Lean Startup Decision Framework

Ries offers three Pivot types that map to VALIDATION.md's PIVOT:

| Pivot Type | What Changes | Example |
|------------|-------------|---------|
| Zoom-in | One feature becomes the whole product | Chat feature → Chat app |
| Zoom-out | The product becomes a feature of a larger solution | Standalone app → plugin |
| Customer segment | Same solution, different users | Consumer tool → enterprise tool |
| Platform | Product → platform or vice versa | SaaS → API |
| Value capture | Monetization model changes | Subscription → freemium |
| Engine of growth | Growth model changes | Viral → paid |
| Technology | Same solution, different tech | Web app → mobile app |

### The Pre-Commitment Technique

To make the decision objective rather than emotional, **pre-commit to the decision criteria before the spike results are known**. This is the most effective debiasing technique in decision science.

**How it works:**
1. Before the spike: Write down "If [result X], then I will [decision Y]"
2. Publish the commitment (to a colleague, a document, a README)
3. When results come in, the decision is already made

**Example:**
> "If the prototype processes a 1GB file in under 5 seconds, I COMMIT. If it fails to process at all, I KILL. If it processes but takes >30 seconds, I PIVOT to a batched approach."

### The Decision Matrix (Enhanced)

| Criterion | Weight | Pass (3) | Weak Pass (1) | Fail (0) | Score |
|-----------|--------|----------|---------------|----------|-------|
| Core assumption validated | 40% | 3 | 1 | 0 | — |
| Technical feasibility | 20% | 3 | 1 | 0 | — |
| Strategic fit | 20% | 3 | 1 | 0 | — |
| Interest / team energy | 10% | 3 | 1 | 0 | — |
| Learning value | 10% | 3 | 1 | 0 | — |

**Decision rules:**
- **Weighted score ≥ 2.5** → COMMIT (with weighted confidence)
- **Weighted score 1.5–2.4** → PIVOT (examine which criteria dragged the score down)
- **Weighted score < 1.5** → KILL

Any single **Must-meet** criterion at Fail → KILL regardless of weighted score.

### The Reversible/Irreversible Test

Jeff Bezos's decision framework: Is this decision reversible?

- **Type 1 (irreversible)**: High consequence, hard to undo → Slow, deliberate, more evidence
- **Type 2 (reversible)**: Easy to undo → Fast, lightweight, default to yes

**For VALIDATION.md:**
- COMMIT to prototyping is often Type 2 (reversible — you can stop prototyping anytime)
- COMMIT to building is Type 1 (irreversible — you're committing weeks of work)
- The prototyping gate's purpose is to convert an irreversible decision (build or not) into a series of reversible decisions (spike → assess → decide)

### Application to VALIDATION.md

The current Kill Criteria Matrix should be **upgraded**:
1. Add **weights** to criteria
2. Separate **must-meet** from **should-meet** criteria
3. Add **pre-commitment** as a required step before the spike
4. Define **PIVOT types** (zoom-in, zoom-out, customer segment, tech change) so the pivot isn't a vague "try something else"
5. Add the **weighted scoring decision matrix** with explicit thresholds for KILL/PIVOT/COMMIT
6. Add the **reversibility check**: "Is the real build decision reversible? If so, consider skipping the full VALIDATION gate for this specific choice."

---

## 11. Recommendations for VALIDATION.md

Based on the research above, here are specific, actionable recommendations for rewriting VALIDATION.md:

### 1. Add the Prototype Mindset as a Pre-SPIKE Section
Before the SPIKE phase, add a **Prototype Mindset** section with the four principles from the Design Sprint:
- You can prototype anything
- Prototypes are disposable
- Build just enough to learn
- The prototype must appear real
- Goldilocks quality (not too high, not too low)

### 2. Add a Pre-Spike Shaping Step (from Shape Up)
Before the SPIKE phase, add a **SHAPE** step:
- Set the appetite (how much time is this worth?) — currently implicit as 2-8 hours, should be explicit
- Identify rabbit holes before the spike
- Write a one-paragraph pitch (problem + solution + risks)
- Use breadboarding for the pre-spike sketch

### 3. Add a Falsifiable Hypothesis Statement to SPIKE
The SPIKE phase must require a falsifiable hypothesis in this format:
```
Core assumption: [What must be true]
Falsifiable claim: [Specific, measurable outcome]
Minimum success criterion: [Threshold]
```
Without this, the spike is just tinkering.

### 4. Add Prototype Types (not just coded)
Expand the SPIKE phase to recognize different prototype types:
- Coded functional prototype (current default)
- Wizard of Oz (for AI/ML/automation assumptions)
- Smoke test / landing page (for demand assumptions)
- Breadboard / fat-marker sketch (for scope/solution assumptions)

Add a **decision tree** for choosing the right prototype type based on the riskiest assumption.

### 5. Add a Pre-Mortem Step
Before the spike, add a 5-minute **pre-mortem**:
> "Assume it's 6 months from now and this project failed. Write a one-line obituary explaining why."

This surfaces hidden assumptions and primes for failure detection.

### 6. Add Pre-Commitment to the DECISION Phase
Before the spike results are known, require the answer to:
> "If the prototype produces result X, what will I decide?"

Write it down. This prevents post-hoc rationalization.

### 7. Upgrade the Kill Criteria Matrix
Replace the current 4-criteria Pass/Weak Pass/Fail table with:
- **Must-meet** criteria (any fail = KILL): Core assumption, feasibility
- **Should-meet** criteria (weighted scoring): Interest, strategic fit, learning value
- Explicit **weighted thresholds**: ≥2.5 = COMMIT, 1.5-2.4 = PIVOT, <1.5 = KILL

### 8. Add PIVOT Types
Define specific PIVOT types so "pivot" isn't vague:
- Zoom-in (feature becomes product)
- Zoom-out (product becomes feature)
- Customer segment (same solution, different users)
- Technology change (same product, different stack)
- Scope reduction (same product, narrower V1)

### 9. Add the Overbuilding Diagnostic
During the SPIKE, if any of these apply, you've overbuilt and should stop:
- Writing tests
- Setting up CI/CD
- Adding error handling
- Optimizing performance
- Refactoring for maintainability
- Saying "while we're at it..."

### 10. Add the Throwaway vs. Evolutionary Escape Hatch
The default is throwaway. Add an explicit opt-in for evolutionary:
- Criteria must be met (same stack, team agrees, rewrites cost more)
- If choosing evolutionary, the first step is rewriting the core with tests

### 11. Add the Diminishing Returns Curve
Visualize the 90% rule: stop at 90% real. The last 10% adds cost but no learning.

### 12. Add Confirmation Bias Safeguards
- The SPIKE must include at least one **failure test** (not just the happy path)
- The hypothesis must be **falsifiable** (would you accept a negative result?)
- Add a **devil's advocate persona**: "If I wanted to prove this wrong, how would I design the spike?"
- The interviewer (if testing with users) should NOT be the prototype builder

---

## References

1. Knapp, J., Zeratsky, J., & Kowitz, B. (2016). *Sprint: How to Solve Big Problems and Test New Ideas in Just Five Days*. Simon & Schuster. https://www.thesprintbook.com/
2. Ries, E. (2011). *The Lean Startup*. Crown Business. https://theleanstartup.com/principles
3. Singer, R. (2019). *Shape Up: Stop Running in Circles and Ship Work that Matters*. Basecamp. https://basecamp.com/shapeup
4. Ries, E. (2016, March 8). "The Prototype Mindset." *Galleys* (Medium). https://medium.com/galleys/the-prototype-mindset-396c979a356f
5. Blank, S. (2013). *The Four Steps to the Epiphany* (2nd ed.). K&S Ranch.
6. Cooper, R.G. (1990). "Stage-Gate Systems: A New Tool for Managing New Products." *Business Horizons*, 33(3), 44-54.
7. Garner, B. (2016, May 6). "Don't Build When You Build-Measure-Learn." *Strategyzer*. https://www.strategyzer.com/library/dont-build-when-you-build-measure-learn
8. Paul, S. & Rosala, M. (2024, April 19). "The Wizard of Oz Method in UX." *Nielsen Norman Group*. https://www.nngroup.com/articles/wizard-of-oz/
9. Blackwell, M. (2025, July 4). "The Prototyping Trap: How Testing Failures Destroy Product Launches." *Arkaro*. https://arkaro.com/prototyping-failure-testing-product-launches/
10. Osmani, A. (2023, August 3). "The Value of a Prototype Is in the Insight It Imparts." https://addyosmani.com/blog/prototypes/
11. LeanPivot.ai. (2026, January 15). "Chapter 8: The Go/No-Go Decision Framework." https://leanpivot.ai/playbook-03-feasibility/go-no-go-decision/
12. Incertive. (2026, May 16). "Go/No-Go Decision Framework for Business Leaders." https://incertive.com/blog/go-no-go-decision-framework
13. "Evolutionary vs Throwaway Prototyping." *Stack Overflow*. https://stackoverflow.com/questions/1077317/evolutionary-vs-throwaway-prototyping
14. UXPin. (2026, May 9). "What is Throwaway Prototyping?" https://www.uxpin.com/studio/blog/throwaway-prototyping/
15. Kromatic. (2023, September 12). "Wizard of Oz Prototyping vs. Concierge Test: The Key Difference." https://kromatic.com/blog/concierge-vs-wizard-of-oz-test/

# Research: Goal Clarification in Software Development

**Date**: 2026-07-11
**Purpose**: Inform the rewrite of `AMBITION.md` from a static form-filling exercise into a back-and-forth dialogue protocol between user and AI.
**Context**: The Development Protocol's PREP PHASE (AMBITION → LANDSCAPE → VALIDATION → SPECIFICATION → EXECUTOR). AMBITION.md currently has 4 form fields (The Itch, The Hypothesis, Success Criteria, Constraints) plus a Meta section with clarification techniques. The user reported it should be a dialogue, not a form.

---

## Summary

Goal clarification is not a linear form-filling exercise. It is a **reflective conversation** — between the designer and the situation (Schön), between stakeholders and the problem space (Design Sprint Monday), between appetite and solution (Shape Up), and between user and AI (Socratic prompting). The common thread across all methodologies: **clarity emerges through iterative exchange, not monologue.**

The existing AMBITION.md treats clarification as 4 independent form fields to fill in sequence. The research shows this is wrong in two ways: (1) the sections interact — changing your hypothesis changes what success looks like — and (2) the real work happens in the **backtalk** between articulations, where each attempt reveals what was missing. A dialogue protocol must replace the form.

---

## Methodology Sources

### 1. Shape Up (Basecamp/37signals) — Appetite, Boundaries, Circuit Breaker

- **Source**: Ryan Singer, *Shape Up: Stop Running in Circles and Ship Work that Matters* (Basecamp, 2019). See [Set Boundaries](https://basecamp.com/shapeup/1.2-chapter-03) and [Decide When to Stop](https://basecamp.com/shapeup/3.5-chapter-14).
- **Key insight**: Instead of asking "how long will this take?" (estimation), Shape Up asks "how much time is this worth?" (appetite). This flips the entire framing: **fixed time, variable scope**. The appetite is a creative constraint, not a guess.
- **Key techniques**:
  - **Appetite**: "Small Batch" (1-2 weeks) or "Big Batch" (6 weeks). The conversation starts with appetite, not features.
  - **Circuit breaker**: If work doesn't ship within the appetite, the project is killed by default. No extensions.
  - **Scope hammering**: Continuously reducing scope to fit the timebox. "There's always more work than time. Shipping on time means shipping something imperfect."
  - **Narrowing the problem**: Instead of "What could we build?" ask "What's really going wrong?" The calendar case study: a customer wanted "a calendar" — after digging, they needed "see free spaces" (a dot grid, not a calendar).
  - **Grab-bag detection**: "Redesign the Files section" is not a project. "Sharing multiple files takes too many steps" is a starting point.
  - **Baseline comparison**: "Is it better than what they have now?" not "Is it perfect?"
- **Application to AMBITION.md**: The appetite concept should be part of the dialogue — not a field to fill in, but a constraint to negotiate. The circuit breaker provides a stopping rule for the clarification phase itself: if after N rounds the ambition isn't clearer, kill it (or set a narrower appetite). The "narrow down the problem" technique is a dialogue pattern: the AI, playing shaper, asks "What specifically feels wrong?" not "What do you want to build?"

### 2. Design Sprint (Google Ventures) — Monday Problem Definition

- **Source**: Jake Knapp, *Sprint: How to Solve Big Problems and Test New Ideas in Just Five Days* (2016). See [GV Design Sprint](https://www.gv.com/sprint/).
- **Key insight**: Monday (Day 1) is entirely about problem definition before any solutioning. Structured as: **Start at the End** (long-term goal) → **Make a Map** (of the challenge) → **Ask the Experts** (capture what's known) → **Pick a Target** (ambitious but manageable piece).
- **Key techniques**:
  - **Start at the End**: "Where do we want to be in 6 months? 1 year? 5 years?" This forces long-term thinking first.
  - **The Map**: Visualize the challenge as a flow diagram. Who is involved? What are the steps? Where's the bottleneck?
  - **Lightning talks**: Subject matter experts share 15-minute overviews to surface hidden knowledge.
  - **Target selection**: Given the map, pick the single most important place to focus. "You can't solve everything in a week."
- **Application to AMBITION.md**: The "Start at the End" technique is directly applicable — the dialogue should start with "What does success look like?" not "What's the itch?" The Map technique helps the user visualize the problem space. The Target selection provides an exit criterion: have we picked a focused enough target to proceed?

### 3. Socratic Method / Requirements Elicitation — Question-Driven Clarification

- **Source**: Kent McDonald, "Socratic Questioning — A Powerful Requirements Elicitation Tool" (Mastering Business Analysis, 2019). See also: [Socratic Questioning in Software Engineering](https://avishekbhattacharya.wordpress.com/2024/05/02/socratic-method-in-software-engineering-a-philosophical-approach-to-communication/).
- **Key insight**: The six categories of Socratic questions map directly to goal clarification:
  1. **Clarification**: "What do you mean by X?" "Can you give an example?"
  2. **Probing assumptions**: "What are you assuming here?" "Is that always true?"
  3. **Probing reasons/evidence**: "What evidence supports this?" "Why do you think that?"
  4. **Questioning viewpoints**: "What would someone who disagrees say?" "What's the opposite view?"
  5. **Probing implications**: "What would be the consequence of X?" "What happens if you're wrong?"
  6. **Questions about the question**: "Why is this question important?" "Is this the right question to ask?"
- **Key technique — Context-free questions** (from software engineering requirements elicitation):
  - "Who is the user?"
  - "What problem does this solve?"
  - "What is the output?"
  - "What are the success criteria?"
  - "What constraints exist?"
  These are called "context-free" because they don't bias toward any particular solution.
- **Application to AMBITION.md**: The dialogue between user and AI should follow a Socratic structure. The AI's role is to ask — not to answer. Each question category drives a different kind of clarification. The six categories form a natural conversation arc: start with clarification, surface assumptions, test reasoning, explore alternatives, examine implications, then meta-reflect.

### 4. Schön's Reflective Practitioner — Backtalk, Problem Setting, Framing

- **Source**: Donald A. Schön, *The Reflective Practitioner: How Professionals Think in Action* (1983). See also: [Stanford interview with Schön](https://hci.stanford.edu/publications/bds/9-schon.html), [Dan Saffer's notes](https://odannyboy.medium.com/notes-on-donald-sh%C3%B6ns-the-reflective-practitioner-e67f753879d8).
- **Key insight**: "Problems do not present themselves to practitioners as givens. They must be constructed from the materials of problematic situations." This is the core problem with AMBITION.md's form-filling approach — it assumes the "itch" is pre-formed and just needs articulation.
- **Key concepts**:
  - **Problem setting** vs. problem solving: "Naming the things to which we will attend and framing the context in which we will attend to them." This is a **conversation**, not a monologue.
  - **Backtalk**: The situation "talks back" through unintended effects. Each attempt to articulate the ambition reveals something unexpected. The dialogue is between the user and their own articulations, mediated by the AI.
  - **Reflection-in-action**: Thinking about what you're doing while doing it. The user doesn't know what they want until they try to say it and hear what comes out.
  - **Criteria for a good frame** (from Schön via Saffer):
    1. Can I solve the problem I have set?
    2. Do I like what I get when I solve this problem?
    3. Have I made the situation coherent?
    4. Have I made it congruent with my fundamental values and theories?
    5. Have I kept inquiry moving?
  - **Frame clashes**: Different stakeholders (or even the same person at different times) may frame the same ambition differently. The dialogue must surface these frames rather than suppress them.
  - **The gyroscope**: An expert practitioner has "their own gyroscope" — an internal sense of when something is good enough. The dialogue should help the user develop this gyroscope for their own ambitions.
- **Application to AMBITION.md**: This is the theoretical foundation for the dialogue approach. The AI's job is to provoke backtalk — to ask questions that create productive surprises in the user's thinking. Schön's five criteria for a good frame become the **stopping criteria** for the dialogue: can you solve the problem you set? Do you like what you get? Is it coherent? Is it congruent? Is inquiry still moving?

### 5. Design Thinking — Problem Framing and "How Might We"

- **Source**: [Problem Framing in Design Thinking: Best Practices](https://voltagecontrol.com/articles/problem-framing-in-design-thinking-best-practices/), [BYU Design Review on Framing](https://www.designreview.byu.edu/collections/design-thinking-part-4-framing-and-reframing-design-problems).
- **Key insight**: "How Might We" (HMW) questions reframe problems to open up solution space. The transition from "We need to build X" to "How might we help Y achieve Z?" is the crucial reframing move.
- **Key techniques**:
  - **HMW generation**: Generate 10+ HMW questions for any problem. This prevents premature fixation on one frame.
  - **Problem statement formula**: "[User] needs [need] because [insight]" — the user-centered framing tool from the Define phase.
  - **5 Whys**: Root cause probing. Each "why" peels back a layer of abstraction.
  - **Frame reflection**: "What if the problem is not [current frame] but [alternative frame]?" — active frame-shifting.
- **Application to AMBITION.md**: The HMW technique is a specific dialogue move the AI can use when the user is stuck. The 5 Whys is a natural back-and-forth pattern. Problem statement formula provides a structured output for the dialogue.

### 6. AI-Specific Goal Clarification — Socratic Prompting, Flipped Interaction, Iterative Refinement

- **Source**: "The Socratic Prompt: How to Make a Language Model Stop Guessing and Start Thinking" (Towards AI, 2026), [Socratic Prompting - Praxis Library](https://www.praxislibrary.com/learn/socratic-prompting.html), [Prompt Chaining Guide](https://www.promptingguide.ai/techniques/prompt_chaining), [SocraticScope framework](https://westpoint.io/insights/youve-heard-about-socratic-prompting).
- **Key insight**: The flipped interaction pattern — where the AI **asks you questions instead of answering them** — is the most effective pattern for goal clarification.
- **Key techniques**:
  - **Socratic Prompting**: Instruct the AI to "ask clarifying questions before answering; challenge the framing; surface hidden assumptions; propose alternative hypotheses; test for internal consistency; demand definitions."
  - **Flipped Interaction**: The AI interviews the user. "You are a Socratic analyst. Your first job is to remove ambiguity, not to answer."
  - **Iterative Refinement Chain**: Generate → Critique → Revise → Finalize. Each round narrows the focus.
  - **The "Interrogation Before Execution" pattern**: The AI refuses to proceed until it has gathered enough clarifying information. The prompt says "pause and ask" — force the model to ask questions about your goal before it attempts to solve it.
  - **SocraticScope** (Westpoint, 2026): "Structured dialogue to interrogate the problem space before implementation. The goal is to produce a clearer specification that names edge cases, boundaries, expected failures, and decisions." This is the closest existing framework to what AMBITION.md should become.
  - **Refinement Loop**: A 2-prompt cycle. Prompt A generates. Prompt B critiques. Prompt A revives based on critique. Repeat until quality threshold met.
- **Application to AMBITION.md**: This is the direct implementation model. The AI persona should be "Socratic analyst / coach" whose job is to ask, not answer. The flipped interaction pattern is the dialogue structure. The iterative refinement chain is the process structure.

### 7. Falsifiability and Stopping Criteria — When Is a Goal "Clear Enough?"

- **Source**: Popper's falsifiability (applied to software ambitions), Schön's frame criteria, Shape Up's circuit breaker, Simon's satisficing (Herbert Simon, *Administrative Behavior*, 1947).
- **Key insight**: A goal is clear enough when you can **test whether you've achieved it**. This is falsifiability applied to ambition: "I believe [intervention] will [outcome]" is falsifiable if you can define what evidence would disprove it.
- **Key criteria for "clear enough"** :
  - **Specificity**: Can an AI read this and know what to build? (This is the LANDSCAPE.md handoff test.)
  - **Falsifiability**: "What would prove this hypothesis wrong?"
  - **Testability**: "How would you know if you succeeded?"
  - **Satisficing**: "Good enough" is the new optimum. The opposite of a bad decision isn't no decision — it's a good-enough decision made on time.
  - **Shape Up's baseline test**: "Is this better than what they have now?"
  - **Schön's frame congruence test**: "Have I made it congruent with my fundamental values and theories?"
- **Application to AMBITION.md**: The dialogue needs an explicit exit criterion. When the user can answer "yes" to Schön's 5 questions AND the ambition is specific enough for LANDSCAPE.md research, the conversation is done.

### 8. Failure Modes of Goal Clarification

- **Source**: Karl Wiegers, "10 Requirements Traps to Avoid" (2000), [Analysis Paralysis anti-patterns](https://www.minware.com/guide/anti-patterns/analysis-paralysis), requirements elicitation literature.
- **Key failure modes**:
  1. **Analysis paralysis**: Endless refinement, never "good enough" to proceed. Mitigation: timebox each dialogue round (Shape Up's appetite applies to the clarification itself).
  2. **Premature commitment**: Settling on a frame too early (Schön's "falling in love with an initial design idea"). Mitigation: generate alternative framings (HMW, opposite framing).
  3. **Scope ambiguity**: The ambition is so broad it could mean anything (Shape Up's "grab-bag" problem — "Redesign X 2.0"). Mitigation: narrow to a specific baseline scenario.
  4. **Premature solutioning**: Jumping to "how to build" before "what to build." Mitigation: the dialogue protocol must explicitly forbid solution talk until the frame is stable.
  5. **Frame clash**: The user has conflicting frames and doesn't know it. Mitigation: surface contradictions through Socratic probing.
  6. **The "pet solution" trap**: User arrives with a solution disguised as a problem ("We need a calendar"). Mitigation: Shape Up's technique — ask *when* the problem occurs, not *what* to build.
  7. **False consensus**: User thinks they're clear but the ambition is still ambiguous. Mitigation: the "externalize" test — explain to someone outside the domain.
- **Application to AMBITION.md**: The protocol must recognize and guard against each failure mode. Include circuit-breaker text: "If after [N] rounds the ambition isn't clearer, here's what to do."

---

## Key Patterns

1. **The Flipped Interaction Pattern**: The AI leads by asking, not answering. This is the single most important pattern. The AI persona is "coach/shaper/Socratic analyst" whose first job is to remove ambiguity.

2. **The Backtalk Cycle**: Articulate → Surprise → Reframe. Each attempt to articulate the ambition produces unexpected reactions in the user. The AI facilitates this by reflecting back what the user said and noting contradictions.

3. **The Appetite Negotiation**: Before solutioning, establish how much time/effort this ambition is worth. This constrains and clarifies. The AMBITION dialogue should start with appetite, not description.

4. **The Frame Generation Pattern**: Generate multiple frames for the ambition before committing to one. "What if the problem is X vs. What if the problem is Y?" Schön's frame analysis.

5. **The Baseline Scenario Extraction**: Instead of "What do you want?" ask "What's the specific situation that feels wrong?" (Shape Up's calendar case study technique).

6. **The Stopping Criteria Check**: After each dialogue round, check against clear-enough criteria (specificity for LANDSCAPE.md, falsifiability, congruence, coherence).

7. **The Circuit Breaker**: If the ambition can't be clarified within N rounds (or M minutes), it's killed or deferred. This prevents analysis paralysis.

---

## What Works

| Technique | Why It Works | Source |
|---|---|---|
| **Socratic questioning before answering** | Forces user to articulate hidden assumptions | Socrates, Socratic Prompting |
| **Fixed time, variable scope** | Appetite as creative constraint forces trade-offs | Shape Up |
| **Start at the end (long-term goal)** | Gives direction to all subsequent clarification | Design Sprint Monday |
| **Baseline extraction ("When did you want this?")** | Grounds abstract ambition in concrete scenario | Shape Up calendar case study |
| **Generate 10+ HMW questions** | Prevents premature fixation on one frame | Design Thinking |
| **Falsifiability test** | Makes the ambition testable | Popper (applied) |
| **Externalize test** | Explain to outsider; if they understand, it's clear | AMBITION.md Meta section |
| **Write the opposite** | Inversion reveals what you actually want | AMBITION.md Meta section |
| **Schön's 5 criteria for a good frame** | Comprehensive stopping check | The Reflective Practitioner |
| **Schön's backtalk concept** | Legitimizes surprise as productive, not a failure | The Reflective Practitioner |

---

## What Doesn't Work

| Anti-pattern | Why It Fails | Source |
|---|---|---|
| **Form-filling (linear fields)** | Assumes the itch is pre-formed; ignores backtalk | Current AMBITION.md |
| **Direct question → answer** | User gives first answer that comes to mind, not the one they mean | Socratic method |
| **"What do you want to build?"** | Invites premature solutioning | Shape Up |
| **Unlimited refinement** | Analysis paralysis | Wiegers, Shape Up circuit breaker |
| **Mixing clarification and solutioning** | Each contaminates the other | Design Thinking |
| **"Just be more specific"** | Without a technique, the user can't comply | — |
| **One-shot clarification** | The first articulation is never the real one | Schön |
| **Group consensus without frame surfacing** | False consensus, hidden frame clashes | Schön |

---

## Recommendations for AMBITION.md

### Structural Transformation

Replace the 4-section form with a **Dialogue Protocol** structured as phases, not fields:

**Phase 0: AI Persona Activation**
The AI announces its role: "I'm going to act as a Socratic shaper. My job is to ask questions, not give answers. I will challenge your assumptions and push you to clarify. Ready?"

**Phase 1: Baseline Extraction (Shape Up technique)**
AI asks about a **specific situation**, not a general desire.
- "When did you feel this itch? What was happening?"
- "Who was there? What were you trying to do?"
- "What did you try that didn't work?"
This replaces "The Itch" field.

**Phase 2: Appetite Negotiation**
Before going deeper, establish the appetite.
- "How much time is this worth? Days? Weeks?"
- "Is this a quick experiment or a major investment?"
- "What's the minimum you'd be satisfied with?"
This is new — not in current AMBITION.md.

**Phase 3: Frame Generation (Socratic probing)**
AI generates multiple framings of the problem, asking the user to select.
- "What if the problem is [Frame A] vs. [Frame B]?"
- "What are you assuming is true?"
- "What would disprove your hypothesis?"
This replaces "The Hypothesis" field.

**Phase 4: Success Criteria (Start at the End)**
- "Where do you want to be in 6 months?"
- "How will you know if this worked?"
- "What does 'good enough' look like?"
This replaces "Success Criteria" field — but as dialogue output, not input.

**Phase 5: Constraint Surfacing (Socratic probing)**
AI identifies hidden constraints by asking about them.
- "What else is competing for your time?"
- "What are you NOT willing to sacrifice?"
- "What skills/resources do you have vs. need?"
This replaces "Constraints" field.

**Phase 6: Synthesis and Exit Check**
AI synthesizes the dialogue into an ambition statement, then checks:
- "Is this specific enough to research? (LANDSCAPE.md test)"
- "Is this falsifiable?"
- Schön's 5 criteria
- "Shall we proceed to LANDSCAPE.md?"

### Specific Dialogue Moves to Include

1. **The Socratic Probe**: AI asks 3+ clarifying questions before allowing the user to settle on any statement.
2. **The Reframing Move**: When user is stuck, AI offers an alternative frame: "What if we think of this as [X] instead of [Y]?"
3. **The Contradiction Surface**: AI identifies and names contradictions: "You said [A] earlier, but now you're saying [B]. Can you reconcile these?"
4. **The Baseline Scenario**: AI asks for a concrete story, not an abstract goal. "Tell me about the last time this was a problem."
5. **The Opposite Test**: "Describe the worst possible version of this project."
6. **The Circuit Breaker**: "We've been at this for [time limit]. We have [Frame A] and [Frame B]. Let's pick one or defer."
7. **The Handoff Statement**: A formatted output that LANDSCAPE.md can directly consume.

### AI Persona Specification

The AI in AMBITION.md should be instructed to act as:

> **Role**: Socratic Shaper / Ambition Coach
> **Core instruction**: Your first job is to remove ambiguity, not to answer. Before offering any synthesis, ask clarifying questions. Challenge the user's framing. Surface hidden assumptions. Propose alternative frames. Test for internal consistency. Demand concrete examples over abstractions.
> **Constraints**:
> - Do NOT propose solutions. Your job is to clarify the problem, not solve it.
> - Keep each dialogue round focused on ONE of the 5 dimensions (Baseline, Appetite, Frame, Success, Constraints).
> - After each user response, reflect back what you heard before asking the next question.
> - When you detect ambiguity, use Socratic probes, not direct instructions.
> - Stop when the ambition passes the 5-criteria test, not before.

### Exit Criteria (When to Stop)

The conversation is complete when:

1. **Specificity**: "Can LANDSCAPE.md research this ambition without further clarification?"
2. **Falsifiability**: "What evidence would disprove this hypothesis?" (User can answer.)
3. **Coherence**: Schön's Q1-Q4 all pass.
4. **Appetite set**: Time/scope constraint is explicit.
5. **Baseline known**: "What situation is this replacing?" is identified.
6. **Minimum delight**: "If we ship only this, is it better than what they have now?"

### Migration Path

1. Replace the 4 table sections with 6 dialogue phases (above).
2. Move current Meta section techniques into specific dialogue moves.
3. Add AI persona instruction block at the top.
4. Add circuit breaker section (what to do if clarification stalls).
5. Remove static form-filling language ("Output: 2-3 sentences").
6. Keep the handoff statement format (output feeds LANDSCAPE.md).

---

## Sources Cited

1. Singer, R. *Shape Up: Stop Running in Circles and Ship Work that Matters*. Basecamp, 2019. https://basecamp.com/shapeup
2. Knapp, J. *Sprint: How to Solve Big Problems and Test New Ideas in Just Five Days*. Simon & Schuster, 2016. https://www.gv.com/sprint/
3. Schön, D. *The Reflective Practitioner: How Professionals Think in Action*. Basic Books, 1983. https://hci.stanford.edu/publications/bds/9-schon.html
4. Schön, D. *Educating the Reflective Practitioner*. Jossey-Bass, 1988.
5. McDonald, K. "Socratic Questioning — A Powerful Requirements Elicitation Tool." Mastering Business Analysis, 2019.
6. Bhattacharya, A. "Socratic Method in Software Engineering." 2024. https://avishekbhattacharya.wordpress.com/2024/05/02/socratic-method-in-software-engineering/
7. Saffer, D. "Notes on Donald Schön's The Reflective Practitioner." 2023. https://odannyboy.medium.com/notes-on-donald-sh%C3%B6ns-the-reflective-practitioner-e67f753879d8
8. Voltage Control. "Problem Framing in Design Thinking: Best Practices." 2026. https://voltagecontrol.com/articles/problem-framing-in-design-thinking-best-practices/
9. Towards AI. "The Socratic Prompt: How to Make a Language Model Stop Guessing and Start Thinking." 2026. https://towardsai.com/p/machine-learning/the-socratic-prompt-how-to-make-a-language-model-stop-guessing-and-start-thinking
10. Praxis Library. "Socratic Prompting." https://www.praxislibrary.com/learn/socratic-prompting.html
11. Prompting Guide. "Prompt Chaining." https://www.promptingguide.ai/techniques/prompt_chaining
12. Westpoint. "SocraticScope: A Socratic Development Framework." 2026. https://westpoint.io/insights/youve-heard-about-socratic-prompting
13. Wiegers, K. "10 Requirements Traps to Avoid." 2000.
14. minware. "Analysis Paralysis Anti-Patterns." https://www.minware.com/guide/anti-patterns/analysis-paralysis
15. Popper, K. *The Logic of Scientific Discovery*. 1934 (falsifiability).
16. Simon, H. *Administrative Behavior*. 1947 (satisficing).

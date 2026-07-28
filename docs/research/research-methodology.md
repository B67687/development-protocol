# Research: How to Do Effective Research for Software Projects

> **Purpose:** This document is the deep-methodology companion to LANDSCAPE.md. Where LANDSCAPE.md gives the protocol (WHAT to do and WHEN), this document gives the skill (HOW to do it well). It synthesizes techniques from academic systematic review methods, information science, competitive analysis, evidence-based medicine, and software engineering research.
>
> **Use this when:** You're about to start a LANDSCAPE phase and want to level up your research skill before diving in. Or after a shallow research pass to diagnose why it felt thin.
>
> **Relationship to LANDSCAPE.md:** Read this once to build research competence. Then execute LANDSCAPE.md as your checklist. Return here when you hit a specific problem (stuck, biased, unsure about a source, can't tell signal from noise).

---

## Summary

Research for software projects is a craft with known techniques, known failure modes, and known stopping criteria. The 80/20 of effective research reduces to:

1. **Ask better questions** before searching — framing determines everything
2. **Search systematically** — use citation chaining and multiple search strategies, not just Google
3. **Evaluate sources ruthlessly** — not all evidence is equal; calibrate your confidence
4. **Watch for bias** — you will find what you look for; design against this
5. **Know when to stop** — saturation is real; more research after saturation is theater

Each section below deepens one of these skills.

---

## Sources Surveyed

This document was built from the following foundations:

| Source | Key Insight Used |
|--------|-----------------|
| PRISMA 2020 Statement (Page et al., BMJ 2021) | Systematic review as a four-phase flow: Identification → Screening → Eligibility → Inclusion. The PRISMA flow diagram is a model for research transparency. |
| Kitchenham, "Guidelines for performing SLRs in Software Engineering" (2007) | Adapted systematic review methodology for software engineering; introduced tertiary studies and structured protocol-first research. |
| Wohlin, "Guidelines for Snowballing in SL Studies" (EASE 2014) | Backward and forward citation chaining as a primary search strategy — often more effective than database search in fast-moving fields. |
| Kuhlthau, "Information Search Process" (1991, 30th anniversary ed. 2025) | Six-stage ISP model: Initiation → Selection → Exploration → Formulation → Collection → Presentation. The key insight: confusion and doubt PEAK in early exploration — this is normal, not a signal to stop. |
| Glaser & Strauss, "The Discovery of Grounded Theory" (1967) | Theoretical saturation — the criterion for stopping data collection. Adapted for project research as the "no new sources" test. |
| CRAAP Test (Blakeslee, Meriam Library 2004) | Five-axis source evaluation: Currency, Relevance, Authority, Accuracy, Purpose. Simple enough for daily use. |
| Evidence Pyramid (Sackett et al., Oxford CEBM) | Hierarchy of evidence: systematic reviews > RCTs > cohort studies > case-control > case series > expert opinion. Adapted for software below. |
| Porter, "Five Forces" (HBR 1979) | Industry analysis framework applicable to OSS ecosystem mapping: rivalry, new entrants, substitutes, supplier power, buyer power. |
| Kahneman, "Adversarial Collaboration" (EDGE 2022) | Getting researchers with opposing views to jointly design a study — the strongest antidote to confirmation bias. |
| Center for Open Science (COS), Registered Reports | Pre-registering research questions and analysis plans before seeing results; prevents p-hacking and post-hoc storytelling. |
| Dunning & Kruger (1999), "Unskilled and Unaware of It" | The metacognitive bias where low competence correlates with high confidence. As a research tool: recognize when you're in the "peak of Mount Stupid." |

---

## The Research Process (Systematic Model)

Academic systematic review methodology provides the most rigorous model for structured research. While a full systematic review is too heavy for most software project needs, its structure can be **lightweighted** into a practical process.

### The Four-Phase Flow (adapted from PRISMA)

```
┌─────────────────────────────────────────────────────────┐
│ PHASE 1: IDENTIFICATION                                 │
│ Search databases, snowball from seed sources, ask       │
│ experts. Produce a pool of candidate sources.           │
├─────────────────────────────────────────────────────────┤
│ PHASE 2: SCREENING                                      │
│ Apply inclusion/exclusion criteria. Remove duplicates.  │
│ Filter by title/abstract. Keep only relevant sources.   │
├─────────────────────────────────────────────────────────┤
│ PHASE 3: ELIGIBILITY                                    │
│ Full-text review. Extract structured evidence.          │
│ Tag by type (direct/adjacent/contextual) and quality.   │
├─────────────────────────────────────────────────────────┤
│ PHASE 4: INCLUSION                                      │
│ Final set of sources that inform synthesis.             │
│ Map what you found to your research questions.          │
└─────────────────────────────────────────────────────────┘
```

### Lightweight Adaptation for Software Projects

**For a typical LANDSCAPE pass (4-8 hours):**

1. **Protocol-first (15 min):** Write your research questions before searching. This is NOT optional — it's the single highest-leverage step. A written protocol prevents drifting into "interesting but irrelevant" rabbit holes.

2. **Seed searching (30 min):** Find 3-5 highly relevant starting sources. Use GitHub, package registries, technical blogs, academic databases (IEEE Xplore, ACM DL, arXiv), and conference proceedings (ICSE, ESEC/FSE).

3. **Snowballing (1-2 hours):** From your seeds, do two passes:
   - **Backward snowballing:** Check each source's references. What do THEY cite?
   - **Forward snowballing:** Use Google Scholar / Semantic Scholar "cited by" to find who cited your seeds.
   - Stop at 2-3 levels deep unless a vein is very rich.

4. **Structured extraction (1 hour):** For each kept source, capture: finding, source, relevance, key insight, implication, confidence. Batch this — don't interrupt your reading flow.

5. **Synthesis (30 min):** Map findings to questions. What's confirmed? What's contradicted? What remains unknown?

### Systematic Mapping Studies vs. Systematic Reviews

A distinction from software engineering research:

| Type | Question | Output | When to Use |
|------|----------|--------|-------------|
| **Systematic Map** | "What exists in this area?" | Categorization, gaps, clusters | Early landscape: you don't know the space |
| **Systematic Review** | "What does the evidence say about X?" | Aggregated findings, meta-analysis | Focused question: you know what you're looking for |

**Rule of thumb:** If your research question starts with "What exists?" — do a mapping study. If it starts with "Does X work?" — do a review.

### Citation Chaining Depth

Academic research recommends chaining through references 2-3 levels deep. Why?

- **Level 1:** Directly relevant sources (your seed)
- **Level 2:** Foundational work that seeds build on (often the real origin of key ideas)
- **Level 3:** The intellectual context — how ideas diverged or converged

Beyond level 3, you're usually in diminishing returns. Exception: if every level-2 source independently cites the same level-3 paper, that paper is probably foundational.

---

## Source Quality Evaluation

Not all evidence is created equal. The ability to rapidly evaluate source quality is the most transferable research skill.

### The CRAAP Test

Developed by librarians at CSU Chico (2004). It's the industry standard for source evaluation:

| Criterion | Questions to Ask | Red Flags |
|-----------|-----------------|-----------|
| **C**urrency | When was this published? Has it been updated? Is it still relevant? | No date. Outdated in a fast-moving field. "Last updated" 3+ years ago for software topics. |
| **R**elevance | Does this directly address your question? Who is the intended audience? | Tangential. Too basic/advanced. Marketing masquerading as information. |
| **A**uthority | Who wrote this? What are their credentials? Are they recognized in the field? | No author named. Anonymous blog. Author has no other publications in the area. |
| **A**ccuracy | Is the information supported by evidence? Can you verify it elsewhere? | No citations. Factual errors. Claims without data. Vague assertions. |
| **P**urpose | Why does this exist? To inform? To sell? To persuade? | Clear bias. Overt promotional content. Emotional language. Cherry-picked data. |

**Scoring:** Rate each criterion 0-3 (0= fails, 3= excellent). Total ≥ 11 = high confidence, 7-10 = moderate, ≤ 6 = treat with skepticism.

### Evidence Hierarchy for Software Projects

Adapted from evidence-based medicine (Sackett et al.) and evidence-based software engineering (Kitchenham, Dybå):

```
HIGHEST ─────────────────────────────────────────────────────────┐
                                                                  │
  Tier 1: Systematic reviews / Meta-analyses                      │
  → Multiple studies synthesized with explicit methodology        │
  → Example: "A systematic review of microservices migration"     │
                                                                  │
  Tier 2: Controlled experiments / Quasi-experiments              │
  → A/B tests, controlled user studies, randomized trials         │
  → Example: "Measuring productivity with vs. without type hints" │
                                                                  │
  Tier 3: Industrial case studies / Field studies                 │
  → Real projects, real context, but no control group             │
  → Example: "How Netflix migrated to GraphQL"                    │
                                                                  │
  Tier 4: Expert opinion / Well-reasoned analysis                 │
  → Deep experience, but personal and possibly idiosyncratic      │
  → Example: "Why we chose Rust for our CLI tool" (blog post)     │
                                                                  │
  Tier 5: Anecdote / "I tried it once" / Hype content             │
  → No systematic method, one data point, survivorship bias       │
  → Example: "X framework is dead" (hot take on Hacker News)      │
                                                                  │
LOWEST ───────────────────────────────────────────────────────────┘
```

**Key insight for software research:** Unlike medicine, software engineering has very few Tier 1 or Tier 2 studies. Most "evidence" in software is Tier 3-5. This doesn't mean it's worthless — it means you must:
1. **Triangulate:** Multiple Tier 3-4 sources agreeing is stronger than one Tier 2 source alone
2. **Be explicit about confidence:** Label your findings by evidence tier
3. **Identify which claims NEED higher-tier evidence** — these become prototype targets in VALIDATION.md

### Calibrating Confidence by Source Type

| Source Type | Typical Tier | Appropriate Confidence | Appropriate Use |
|-------------|-------------|----------------------|-----------------|
| Peer-reviewed paper | 1-3 | High (for that finding) | Foundations, established knowledge |
| Technical blog (engineering team) | 3-4 | Medium-high | Practical patterns, real trade-offs |
| Personal blog / Medium post | 4-5 | Low-medium | Ideas to explore, not to build on |
| GitHub repo (popular, maintained) | 3-4 | Medium | "This approach exists and works" |
| GitHub repo (unmaintained, few stars) | 5 | Low | "Someone tried this once" |
| Hacker News / Reddit discussion | 5 | Very low | Signals of controversy, not truth |
| Conference talk video | 3-4 | Medium | Trends, vision, industry direction |
| Documentation / official guides | 3 | Medium-high | How it works, not how well it works |
| Expert interview (you conducted) | 3-4 | Medium | Context-specific, may be opinion |
| Vendor / marketing content | 5 | Very low | What they want you to believe |

---

## Avoiding Bias in Research

The most dangerous bias in software project research is **confirmation bias**: you find what you're looking for, and miss what contradicts your hypothesis.

### Why This Matters in Software

When you have a hypothesis (e.g., "we should use event sourcing"), you will naturally:
- Search for "event sourcing benefits" (not "event sourcing drawbacks")
- Click on results that confirm your belief
- Interpret ambiguous evidence as supporting your view
- Dismiss counter-evidence as exceptional or outdated

This is not a moral failing — it's how human cognition works. The fix is **systematic procedure**, not willpower.

### Technique 1: Pre-Register Your Research Questions

Before you start searching, write down:
1. Your hypothesis (from AMBITION.md)
2. The specific questions you're investigating
3. **What evidence would DISCONFIRM your hypothesis?** (This is the crucial step)

**Example:**
```
Hypothesis: Event sourcing will simplify our audit logging
Questions:
  - What are the operational costs of event sourcing? (looking for BOTH pros and cons)
  - What are known failure modes? (actively searching for negatives)
  - In what contexts has event sourcing been ABANDONED? (searching for disconfirmation)
  
Evidence that would DISCONFIRM: 
  - Studies showing event sourcing increases complexity without audit benefit
  - Projects that migrated AWAY from event sourcing
  - Performance benchmarks showing unacceptable overhead
```

If you can't articulate what would disprove your hypothesis, you don't have a testable hypothesis.

### Technique 2: Actively Search for Negative Results

The academic "publication bias" problem — positive results get published, null results get filed — has a software analog. For every benefit claim, deliberately search for:

- "X considered harmful" / "X is dead" / "why we moved away from X"
- "{technology} drawbacks / limitations / pitfalls"
- "{technology} vs {alternative}" (balanced comparisons, not hit-pieces)
- "When NOT to use {technology}"
- "{pattern} failure modes / anti-patterns"

**Research shows** that searching "X vs Y" (comparative) rather than "X benefits" (promotional) yields significantly more balanced results.

### Technique 3: The "Devil's Advocate" Pass

After you've gathered supporting evidence, do a dedicated pass where you:
1. Pretend you believe the OPPOSITE of your hypothesis
2. Search for evidence supporting that opposite view
3. Write a summary of the strongest counter-arguments

If your counter-argument summary is weak, you haven't researched thoroughly enough.

### Technique 4: Adversarial Collaboration (Heavyweight)

Inspired by Daniel Kahneman — researchers with opposing views jointly design a study. For software projects:

- Find someone who disagrees with your approach
- Jointly define what evidence would be convincing
- Run the research together
- Agree beforehand on how you'll interpret results

This is heavy for routine use but invaluable for high-stakes decisions (architecture choices, framework selections, build-vs-buy).

### Technique 5: Blind Analysis

Where possible, have someone ELSE evaluate your evidence without knowing your hypothesis. Or at minimum, have someone review your synthesis before they know your recommendation.

---

## Researching Unknown Domains

The hardest research problem: you don't know enough to know what you don't know. The techniques below are specifically for starting from zero.

### Kuhlthau's Information Search Process (ISP)

Carol Kuhlthau's research (Rutgers, 1985-2025) maps the emotional journey of research:

```
Stage              Feeling              What's Happening
──────────────────────────────────────────────────────────────────
1. Initiation      Uncertainty,         You recognize you need
                   apprehension         information. Vague sense
                                        of what's needed.
                                        
2. Selection       Optimism,            You pick a general area.
                   excitement           "I'll research microservices."
                                        
3. Exploration     Confusion,           You start reading and 
                   doubt, frustration   realize it's MORE complex
                   ← PEAK NEGATIVE      than you thought. This is
                                        where people quit.
                                        
4. Formulation     Clarity,             Patterns emerge. You can
                   confidence           now articulate what you're
                   returning            actually looking for.
                                        
5. Collection      Focus,               Targeted searching. You
                   efficiency           know what to look for.
                                        
6. Presentation    Satisfaction,        You can explain what you
                   relief (or           learned. Synthesis happens.
                   disappointment)
```

**Critical insight:** The confusion and doubt of Stage 3 (Exploration) are NOT a sign you're doing it wrong. They're the sign you're doing it right — you're discovering the depth of your ignorance. The danger is quitting here.

**Practical application for LANDSCAPE.md:**
- Track which stage you're in
- If you're in Stage 3 (confused), keep going — you haven't failed, you're just not done yet
- If you're NOT confused at some point, you may not be digging deep enough

### The Dunning-Kruger Curve as a Research Tool

The Dunning-Kruger effect describes how confidence and competence relate:

```
Confidence
    ↑
    │   ┌─── Peak of "Mount Stupid" ←─ You know JUST ENOUGH
    │   │   (overconfidence)           to be dangerous. You've seen
    │   │                              one example and think you
    │   │                              understand.
    │   │
    │   │         ┌─── Slope of Enlightenment
    │   │         │   (growing awareness of depth)
    │   │         │
    │   │         │         ┌─── Plateau of Sustainable Confidence
    │   │         │         │   (actual competence)
    │   │         │         │
    │   └─────────┴─────────┴─────→ Competence
        Valley of Despair
        (realizing how much you don't know)
```

**How to use this in research:**

1. **If you feel very confident after 30 minutes of searching:** You're probably on "Mount Stupid." Keep digging — you likely haven't found the complexity yet.
2. **If you feel confused and overwhelmed:** You've entered the "Valley of Despair." This means you've moved past superficial understanding. Keep going — formulation is next.
3. **If you can articulate both the strengths AND weaknesses of options:** You may be approaching genuine competence.

**Practical rule:** If your research confidence graph is a straight line up (getting more confident with every source), you're probably not challenging your beliefs enough.

### The Question Bank Technique

When you know nothing about a domain, you can't formulate good research questions. The question bank technique gets you unstuck:

1. **Start with meta-questions (5 min):**
   - "What are the key terms and concepts in this area?"
   - "Who are the main communities and thought leaders?"
   - "What are the major debates or controversies?"
   - "What are the canonical examples or case studies?"

2. **Find one overview source (30 min):**
   - A Wikipedia article, a survey paper, a "state of the art" blog post
   - Don't judge quality yet — you just want orientation

3. **Extract questions from that source (15 min):**
   - Every section heading becomes a question
   - Every referenced paper or project becomes a question
   - Every "this is an open problem" becomes a question
   - Every comparison becomes a question

4. **Categorize and prioritize your questions:**
   - **Must understand** (core concepts, blocking questions)
   - **Should understand** (context, trade-offs)
   - **Nice to understand** (detail, edge cases)

5. **Answer the must-understand questions first.** Your next research pass will be targeted, not aimless.

This converts "I don't know what I don't know" into a structured exploration.

### The "Three Lenses" Technique for Unfamiliar Domains

Approach any new domain through three lenses:

1. **The Conceptual Lens:** What are the core ideas? (Read Wikipedia, survey papers, tutorials)
2. **The Concrete Lens:** What are the real implementations? (Explore GitHub, read code, try tools)
3. **The Critical Lens:** What are the debates and controversies? (Read comparison posts, "why we left" posts, HN discussions, academic critique)

Moving between these lenses prevents the "all theory, no practice" or "all practice, no principles" traps.

---

## Signal vs. Noise

Research produces infinite leads. The art is distinguishing what matters from what's merely interesting.

### The 80/20 Rule for Research

The Pareto principle applied to research:

- **20% of your search effort will yield 80% of your useful sources** — the question is which 20%
- **20% of your sources will contain 80% of the insight** — learn to identify these rapidly
- **20% of your findings will drive 80% of your decisions** — everything else is context

**Practical tactics:**

1. **Pre-filter before deep reading:** For any source, spend 2 minutes evaluating: read the title, abstract/summary, headings, conclusion. If none of these contain a valuable insight in 2 min, the full text won't either.

2. **Identify the generative sources:** Some sources will be cited by everyone (foundational papers, canonical blog posts, widely-used tools). Find these first — they're force multipliers.

3. **Look for the "tells" of high-signal sources:**
   - They cite counter-arguments, not just supporting evidence
   - They articulate limitations clearly
   - They distinguish between what they know and what they believe
   - They provide specific, falsifiable claims (not "X is better" but "X reduced latency by 40% in our case")

4. **Use the "so what?" test:** After reading any source, ask "So what?" If you can't articulate how this affects your decision, it's noise (for now).

### Relevance Trees

A relevance tree maps your research terrain hierarchically:

```
Research Question: "Should we use WebSockets for real-time updates?"
│
├── Technology Maturity
│   ├── Browser support for WebSockets
│   ├── Fallback options (SSE, polling, long-polling)
│   └── Library ecosystem quality
│
├── Performance Characteristics
│   ├── Latency compared to alternatives
│   ├── Throughput/scaling ceiling
│   └── Resource consumption (memory, connections)
│
├── Operational Complexity
│   ├── Server infrastructure requirements
│   ├── Connection handling at scale
│   └── Monitoring / debugging tooling
│
└── Failure Modes
    ├── Known issues with proxies/load balancers
    ├── Reconnection handling patterns
    └── Graceful degradation strategies
```

**How to use:**
1. Create the tree before searching — it's your research map
2. Sub-questions are branches — search each branch systematically
3. When you find a surprising connection, add it as a branch
4. When a branch has 3+ confirming sources and no open questions, prune it

### The "Interesting But Irrelevant" Trap

This is the most common research failure mode. You find something genuinely fascinating — a clever technique, a related domain, a cool tool — that doesn't actually answer your research questions.

**Discipline: The 2-Question Filter**

Before following a tangential lead, ask:
1. "Will this affect my decision?" If no → skip (capture as a note for later)
2. "Is this a dependency?" If no → skip

**The "Parking Lot" technique:** Keep a separate list of "interesting but off-topic" finds. They're not lost — they're deferred. This honors the curiosity while protecting the research focus.

### The Reverse Search

When you're drowning in results, reverse your approach:
- Instead of "find everything about X," search for "X case study failure" or "X comparison"
- The broad search gives you volume; the specific search gives you signal
- Use the specific results to validate whether the broad set is worth exploring

---

## Competitive Analysis for Software

Software project research often requires understanding the competitive or ecosystem landscape. Business analysis frameworks adapt well here.

### Porter's Five Forces (Adapted for OSS/Software)

Michael Porter's framework analyzes industry attractiveness through five forces. Adapted for open-source and commercial software:

| Force | Software Translation | Questions to Ask |
|-------|---------------------|------------------|
| **Rivalry among existing competitors** | How many projects solve the same problem? | Are there 5+ actively maintained alternatives? Is there a clear market leader? |
| **Threat of new entrants** | How easy is it to start a competing project? | Low barrier? High switching costs? Network effects? |
| **Bargaining power of buyers** | How much leverage do users have? | Can users easily switch? Are they locked in? Many options available? |
| **Bargaining power of suppliers** | How dependent are you on upstream dependencies? | Critical dependencies? Only one provider? Vendor lock-in risk? |
| **Threat of substitutes** | Do adjacent solutions solve the same problem differently? | Could users solve this with a completely different approach? |

**How to apply:**
- Score each force as Low / Medium / High threat
- Forces with High threat represent risk to your project's viability
- Forces with Low threat represent moats or advantages

### SWOT for Project Positioning

| | Helpful | Harmful |
|---|---|---|
| **Internal** | **Strengths** — What advantages does your approach have? (novelty, performance, simplicity) | **Weaknesses** — Where are you vulnerable? (missing features, smaller community, younger) |
| **External** | **Opportunities** — What gaps exist in the landscape? (unmet needs, emerging trends) | **Threats** — What external forces could kill your project? (big player entering, shifting standards, dependency risk) |

**Limitation:** SWOT is static and subjective. Use it as a starting framework, not an ending one. Pair with the evidence you've gathered.

### Feature Comparison Matrices

The workhorse of software landscape research. A simple table:

| Feature/Capability | Project A | Project B | Project C | Your Idea |
|--------------------|-----------|-----------|-----------|-----------|
| Core feature X | ✅ | ✅ | ❌ | ✅ |
| Feature Y | ✅ Limited | ✅ Full | ✅ Full | ⏳ Planned |
| Performance (metric) | 100ms | 250ms | 50ms | Target: 80ms |
| License | MIT | Apache 2.0 | AGPL | MIT |
| Community size | 15K stars | 40K stars | 2K stars | New |
| Language | Rust | Go | Python | Rust |

**How to build:**
1. List 5-10 projects in the space (direct and adjacent)
2. Identify 5-8 comparison dimensions (features, non-functional, ecosystem)
3. Fill from publicly available data (docs, benchmarks, reviews)
4. Mark unknowns — these are research gaps

**Output:** Either your idea fills a genuine gap (green field) or competes in a crowded space (red field). Both are valid — but you need to know which you're in.

### The "Good-Better-Best" Framework

For positioning your project relative to existing options:

- **Good:** Does the job. Minimal. Low learning curve. (MVP)
- **Better:** Solves the job well. Smart defaults. Good DX. (Competitive)
- **Best:** Delightful. Opinionated. Powerful. Handles edge cases. (Differentiator)

**Map each existing project** to Good/Better/Best on the dimensions that matter for your users. Your project needs at least one "Best" dimension to justify its existence.

---

## Research Saturation — When to Stop

The hardest research skill: knowing when you've done enough.

### Theoretical Saturation (Glaser & Strauss, 1967)

From grounded theory methodology — the point at which new data no longer produces new insights:

> "Saturation means that no additional data are being found whereby the researcher can develop properties of the category." — Glaser & Strauss

**Practical tests for saturation:**

1. **The "No New Sources" Test:** You've been searching for 30 minutes and every new source says the same thing as the last 3. The arguments, findings, and trade-offs are repeating.

2. **The "I Can Predict What the Next Source Will Say" Test:** Before opening a new source, try to predict its main points. If you're consistently right, you've absorbed the space.

3. **The "Diminishing Returns" Curve:** Track insights per hour:
   - Hour 1: 10 new insights (rapid learning)
   - Hour 2: 5 new insights (deeper)
   - Hour 3: 2 new insights (niche)
   - Hour 4+: 0-1 insights (saturation)
   
   When you hit 0-1 insights per hour consistently, stop.

4. **The "Confidence Threshold" Test:** Pre-define what confidence level you need. "I need to be 80% sure before prototyping." When your evidence supports that confidence, stop researching and start prototyping.

### When More Research Stops Reducing Risk

Research reduces two types of risk:
- **Knowledge risk:** "We don't know what exists"
- **Decision risk:** "We might make the wrong choice"

**For knowledge risk:** Saturation happens when you can describe the landscape from memory — the major projects, their approaches, their trade-offs.

**For decision risk:** Saturation happens when:
- You've found evidence supporting AND contradicting your hypothesis
- You understand the main trade-offs
- The remaining unknowns are UNKNOWABLE without building something (you've hit the "build to learn" boundary)

### The Build-to-Learn Boundary

This is the most important stopping criterion for software project research. Some things CANNOT be learned by reading — they must be built:

- How a library feels in practice (not just its API docs)
- Whether performance claims hold in YOUR context
- Whether an architecture pattern works with YOUR data model
- Whether a community is responsive to YOUR questions

**Rule:** If a question can only be answered by building, and you've exhausted what reading can teach you, STOP RESEARCHING and start prototyping. This is exactly what the transition from LANDSCAPE → VALIDATION is for.

### Why People Over-Research

Common drivers of research-phase bloat:
- **Fear of missing something important** (the "what if" spiral)
- **Analysis paralysis disguised as diligence** (research as procrastination)
- **Not having clear stopping criteria** (no exit condition)
- **Confusing activity with progress** (feeling productive while searching)

**The cure:** Set a timebox BEFORE you start. LANDSCAPE.md specifies: 30 min initial scan, 2 hours deep dive per question. Stop when the timebox expires or when you reach saturation, whichever comes first.

---

## Evidence-Based Software Engineering

Evidence-Based Software Engineering (EBSE) was introduced at ICSE 2004 by Kitchenham, Dybå, and Jørgensen. It adapts evidence-based medicine principles to software engineering.

### The Core Insight

Software engineering decisions are often made based on:
- Personal experience ("this worked for me")
- Vendor claims ("our tool improves productivity by 5x")
- Popularity ("everyone on Hacker News is using this")
- Tradition ("we've always done it this way")

EBSE argues these are weak evidence. Stronger evidence comes from systematic gathering and evaluation.

### The Evidence Hierarchy for Software

Adapted from the Oxford CEBM levels:

| Level | Evidence Type | Example in Software | Confidence |
|-------|--------------|-------------------|------------|
| 1a | Systematic review of multiple studies | SLR on microservices migration challenges | Highest |
| 1b | Meta-analysis | Aggregate effect of TDD on defect rates | |
| 2a | Randomized controlled trial | Controlled experiment: typed vs. untyped code | |
| 2b | Quasi-experiment | Pre/post adoption study at one company | |
| 3a | Cohort study | Following projects over time | |
| 3b | Case-control study | Comparing successful vs. failed projects | |
| 4 | Industrial case study | "How Netflix does X" (single context) | |
| 5 | Expert opinion | Blog post, technical talk, book | |
| 6 | Anecdote / "I heard" | Hacker News comment, water cooler | Lowest |

### What This Means for LANDSCAPE Research

**You can't always get high-tier evidence.** Most software project research will be Level 4-5. That's okay — but you must:

1. **Acknowledge the limitation:** "This is one company's experience" (not generalizable truth)
2. **Triangulate across sources:** 3 different companies' case studies > 1 widely-cited paper
3. **Tag findings by evidence level:** This is why LANDSCAPE.md's evidence capture includes "Confidence"
4. **Identify claims that NEED higher evidence:** If a decision hinges on a claim backed only by Level 5 evidence, that claim needs prototyping

### How to Apply Evidence Levels in Practice

When evaluating any claim (e.g., "TypeScript reduces bugs"):

```
Claim: Using TypeScript reduces production bugs by 40%.

Evidence found: 
  - Blog post from Company A (Level 4) — found 40% reduction after migration
  - Blog post from Company B (Level 4) — found 15% reduction
  - Academic study, 20 projects (Level 2a) — found 0-10% reduction, not statistically significant
  - Hacker News anecdote (Level 6) — "TypeScript didn't help us at all"

Synthesis: 
  - The strongest evidence (Level 2a) shows minimal impact
  - Case studies show mixed results (40% and 15%)
  - The claim "reduces bugs by 40%" is overconfident
  - More accurate: "TypeScript may help prevent certain class of bugs, 
    but magnitude varies significantly by context"
  
Next step: Prototype to test in YOUR context, don't take the claim at face value.
```

---

## Research Output Formats

The output of the LANDSCAPE phase must be USEFUL — it feeds directly into VALIDATION.md. Different formats serve different purposes.

### 1. Landscape Map (Strategic Overview)

A 2-3 paragraph summary answering: "What does this space look like?"

**Format:**
```
Landscape: [One-sentence summary of the space]

Major projects: [List 3-7 with 1-line description each]
  - Project A: What they do, key approach, limitations
  - Project B: What they do, key approach, limitations
  - ...

Key dynamics: [What's changing, who's winning, trends]
  - Trend 1
  - Trend 2

Gaps identified: [What's not being addressed]
  - Gap 1 (→ prototype target)
  - Gap 2 (→ prototype target)
```

**Best for:** High-level orientation, executive summary, alignment with stakeholders.

### 2. Evidence Table (Structured Findings)

A table of all captured evidence. This is the raw material.

| Finding | Source | Type | Evidence Level | Confidence | Implication |
|---------|--------|------|---------------|------------|-------------|
| Event sourcing simplifies audit | Martin Fowler blog | Direct | 4 | Medium | Supports approach |
| Event sourcing increases storage costs 10x | Netflix case study | Direct | 3 | High | Must plan for this |
| Event sourcing hard to change schema | Multiple sources | Direct | 3 | High | Risk — needs prototype |

**Best for:** The researcher's working document. Feeds synthesis.

### 3. Decision Matrix (Choices with Evidence)

Evaluating options against criteria with weighted scoring:

| Criterion | Weight | Option A | Option B | Option C |
|-----------|--------|----------|----------|----------|
| Performance | 3 | 5 (15) | 3 (9) | 4 (12) |
| Learning curve | 2 | 4 (8) | 5 (10) | 2 (4) |
| Ecosystem maturity | 3 | 5 (15) | 2 (6) | 3 (9) |
| **Total** | | **38** | **25** | **25** |

**Important:** Scores are subjective. The value is making trade-offs EXPLICIT, not the numeric score itself. The discussion of why Option A scores 5 on performance is more valuable than the 38.

**Best for:** Comparison-heavy decisions (framework selection, build-vs-buy, architecture choice).

### 4. Unknowns Map (Prototype Inputs)

The most critical output for the protocol — what still needs to be learned:

```
Unknowns to Validate in Prototyping:
  1. [Unknown] with highest risk/uncertainty
     Why it matters: 
     What we need to learn:
     How we'll test it (prototype idea):
     
  2. [Unknown] — moderate risk
     ...
```

**Best for:** The bridge to VALIDATION.md. This is the formal handoff.

### 5. Reference List (Curated Sources)

Not a bibliography dump. Curated with annotations:

```
Key Sources:
  1. [Title](URL) — Essential. Explains the core concept. (3 pages)
  2. [Title](URL) — Important counterpoint. Challenges the dominant narrative. (10 min read)
  3. [Title](URL) — Deep dive on implementation. Read after deciding to proceed. (1 hour)
```

**Best for:** Future reference, building team shared context, onboarding.

### Choosing Your Output Format

| If your research question is... | Output format |
|-------------------------------|---------------|
| "What exists in this space?" | Landscape Map |
| "What does the evidence say?" | Evidence Table |
| "Which option should we choose?" | Decision Matrix |
| "What don't we know?" | Unknowns Map |
| "What should we build?" | All four (prioritize Unknowns Map) |

---

## Recommendations for LANDSCAPE.md

Based on the above, here are specific recommendations to deepen LANDSCAPE.md's "Meta: How to Research" section:

### 1. Replace the 5-bullet technique list with structured subsections

Current: 5 bullets in 10 lines. Proposed structure:
- **Framing research** (pre-registration, question bank, falsifiability test)
- **Search skills** (snowballing depth, parallel search, the three lenses)
- **Source evaluation** (CRAAP test adapted for software, evidence hierarchy)
- **Bias countermeasures** (negative results search, devil's advocate, pre-mortem)
- **Saturation detection** (the four tests above)

### 2. Add the emotional roadmap

Most researchers hit the "Valley of Despair" (Kuhlthau Stage 3) and interpret it as failure. LANDSCAPE.md should normalize this: "If you feel confused after 1 hour of research, you're on track. This is the exploration-formulation transition."

### 3. Add the "build-to-learn" boundary

The most common failure is over-researching. Add explicit guidance: "If a question can only be answered by building, and you've exhausted what reading can teach, stop researching and start prototyping. LANDSCAPE feeds VALIDATION — it doesn't replace it."

### 4. Add evidence-level tagging

The current "Confidence: high/medium/low" is good but vague. Replace with a tiered system tied to evidence types, so researchers calibrate consistently.

### 5. Add the unknowns map as a required output

The bridge to VALIDATION.md is currently implicit. Make it explicit: "The primary output of LANDSCAPE is not what you confirmed — it's what you still don't know. Your Unknowns Map is the input to VALIDATION.md's prototype selection."

### 6. Add saturation criteria

Add explicit saturation tests to the "Stop when" guidance: "Stop when either: (a) your timebox expires, (b) you've passed the 'no new sources' test for 30 minutes, or (c) you can articulate the main trade-offs from memory."

### Summary of Recommended Changes to LANDSCAPE.md's Meta Section

| Current | Proposed |
|---------|----------|
| 5 techniques in 10 lines | Structured subsections with sub-techniques |
| No emotional roadmap | Kuhlthau's stages, Dunning-Kruger awareness |
| No build-to-learn boundary | Explicit "stop and prototype" guidance |
| "Confidence: high/medium/low" | Evidence-tier-based confidence calibration |
| No explicit bridge to VALIDATION | Unknowns Map as required output |
| "stop when timebox expires OR answered" | Add saturation criteria alongside timebox |

---

## References and Further Reading

- Page MJ, et al. "The PRISMA 2020 statement: an updated guideline for reporting systematic reviews." BMJ 2021;372:n71.
- Kitchenham B. "Guidelines for performing Systematic Literature Reviews in Software Engineering." EBSE Technical Report, Keele University, 2007.
- Wohlin C. "Guidelines for snowballing in systematic literature studies and a replication in software engineering." EASE 2014.
- Kuhlthau CC. "Seeking Meaning: A Process Approach to Library and Information Services." 30th Anniversary Edition, Bloomsbury Libraries Unlimited, 2025.
- Glaser BG & Strauss AL. "The Discovery of Grounded Theory: Strategies for Qualitative Research." Aldine, 1967.
- Blakeslee S. "The CRAAP Test." LOEX Quarterly, 2004.
- Porter ME. "How Competitive Forces Shape Strategy." Harvard Business Review, March 1979.
- Kruger J & Dunning D. "Unskilled and unaware of it." Journal of Personality and Social Psychology, 77(6), 1999.
- Sackett DL, et al. "Evidence-Based Medicine: How to Practice and Teach EBM." Churchill Livingstone, 2000.
- Kahneman D. "Adversarial Collaboration." EDGE Lecture, 2022.
- Center for Open Science. "Registered Reports." cos.io/rr/
- Petticrew M & Roberts H. "Systematic Reviews in the Social Sciences." Blackwell, 2006.
- Dybå T, Kitchenham B, Jørgensen M. "Evidence-Based Software Engineering for Practitioners." IEEE Software, 22(1), 2005.
- Booth A, Sutton A, Papaioannou D. "Systematic Approaches to a Successful Literature Review." 3rd ed., Sage, 2022.

---

*This document is part of the Development Protocol. Read alongside LANDSCAPE.md for the full research workflow.*

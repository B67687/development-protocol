# LANDSCAPE.md — Research Protocol

> Use this after AMBITION.md has produced a falsifiable hypothesis. Turns "I don't
> know what exists" into a structured landscape map. Timebox the whole phase —
> unbounded research is research theater.
>
> **Pacing note:** Budget for this phase is set in AMBITION (PACING folded), which runs before LANDSCAPE.
> The research scope determined here directly informs the phase budget, not the other way around.

---

## When to Use This

AMBITION.md produced a hypothesis that needs testing against reality. Before validating with prototypes, establish:

- What already exists?
- What's adjacent and worth learning from?
- What approaches have been tried and failed?
- Where are the real unknowns vs. perceived unknowns?

## Skip Conditions

> **Skip when:** you've built this exact thing before AND the domain hasn't changed AND no competitor has moved since your last build.
> **Risk of skipping:** competitor solved it better, or a known failure mode repeats; you ship what already exists.
> **Light mode:** skipped — acceptable only when you're the domain expert on this exact shape.

> **P2b entry gate — Bar 1 must have cleared:** Do not enter LANDSCAPE unless `SERIOUSNESS` = COMMIT (P2a). LANDSCAPE must emit a `WHICH-X?` decision — same / scaled / adjacent / more-than-X — logged before STRATEGY. If SKIP applies, log which variant was implicitly chosen and why P2b was not needed. If `docs/appendix/p2b-mapping-appendix.md` applies (capability has tables), STRATEGY must cite WHICH-X trace id; G7 keep/drop is logged there.

## Governing Modes — Intuition-First Route

> **A protocol-wide standing mode, applied here first.** The AI answers from latent
> pattern-sensing first, always — no CoT, no tools, no scaffolding. This is the
> base layer of every interaction across ALL protocol steps. Structure is the
> opt-in overlay, not the assumption. (Full protocol-wide principle in README.md;
> this section is the operational application within research.)

### When to answer directly (default)

Answer directly from pattern-sensing when the question is about:

| Domain                   | Examples                                                                                    |
| ------------------------ | ------------------------------------------------------------------------------------------- |
| **Pattern recognition**  | "Does this architecture remind you of something?" "What's the shape of this problem?"       |
| **Synthesis / judgment** | "What should I prioritize?" "What's the simplest path?" "Is this a good direction?"         |
| **Comparison**           | "Which approach is more robust?" "What's the tradeoff here?"                                |
| **Meta-decisions**       | "Which problem matters most?" "Should I trust this plan?" "Is this worth doing?"            |
| **Familiar ground**      | Domains where the agent's prior in-domain claims have survived verification (method ledger) |

### When to overlay structure (opt-in, per case)

Add chain-of-thought, tools, or agentic machinery only when:

1. **Exactness demanded** — math, facts, dates, versions, APIs, compliance
2. **Stakes are high** — one-way doors, large bets, irreversible decisions
3. **The answer fails a cheap sanity probe** — it contradicts known constraints, or the calibration signal says "plausible shape, not recognized pattern"

### The calibration signal (the honest report)

Every direct answer should carry an honest calibration tag:

| Tag                                       | Meaning                                             | Default treatment                                                                |
| ----------------------------------------- | --------------------------------------------------- | -------------------------------------------------------------------------------- |
| **"I recognize this pattern"**            | Strong latent match — high-confidence intuition     | Accept and act; if the claim is consequential, still apply RoD tier (L1 minimum) |
| **"This shape is plausible"**             | Weak match — pattern feels right but isn't anchored | Sanity-probe; if consequential, apply RoD tier (L1 minimum)                      |
| **"I'm uncertain / this is speculative"** | No reliable match                                   | Research or decompose before acting (RoD tier per consequence)                   |

The calibration tag is a REPORT, not a permission gate. The user decides what to do with the answer — trust it, probe it, or check it (via Research on Demand tiers).

### Why this is correct (evidence)

- LLMs demonstrably compute answers in latent space before verbalizing (Implicit Reasoning Survey 2509.02350; "Let's Think Dot by Dot" 2404.15758; Coconut 2412.06769)
- Direct answers equal or beat explicit CoT outside math/symbolic domains (Sprague, "To CoT or not to CoT" 2409.12183)
- Overthinking is a documented failure: o1 burns ~1953% more tokens on "2+3=?"; first answer correct 92%+ of the time (2412.21187); removing reasoning improved agent performance ~30% with 43% less compute (2502.08235)
- Human experts operate intuition-first by default; analysis is the exception (Dreyfus five-stage model; ADP 6-0: "intuitive decision making is faster than analytic"), and expert gut feeling carries real signal (BMJ 2012, LR 25.5)
- Product precedent: OpenAI `reasoning_effort: none` — direct generation without reasoning is a first-class API mode

> **The honest caveat (Polanyi's Revenge):** intuition is unverbalized expertise — legitimate when it carries honest calibration and is fed by an outcome loop. Log intuition-driven decisions and their outcomes to build the feedback loop that makes intuition trainable over time (Kahneman & Klein: valid intuition requires high-validity environment + opportunity to learn).

_LANDSCAPE.md is a meta-protocol for doing research — HOW to research, how to evaluate sources, how to avoid bias, and when to stop._

---

## The Research Process (Systematic, Not Random)

4-phase cycle adapted from systematic review methodology: 1. FRAME → 2. SEARCH → 3. EVALUATE → 4. SYNTHESIZE (return to FRAME if remaining unknowns change framing).

### Step 1: Frame

Before searching, define what you're looking for and what you're NOT looking for.

**Framing questions (from the hypothesis):**

- **Direct:** What existing solutions solve this problem? What's their approach? What are their limitations?
- **Adjacent:** What neighboring fields have solved similar problems? What can we borrow?
- **Technology:** What tools, frameworks, or platforms are relevant? Mature vs. experimental?
- **Failure:** What attempts in this space have failed? Why? (Negative results are gold.)
- **Unknowns:** What don't you know that you don't know? (Question bank: generate 10 naive questions about this domain.)

**Scope boundary:** Define what you will NOT research:

- Out of scope topics (prevents rabbit holes)
- Geographic/community coverage: DECLARE the ecosystems, languages and communities you will
  research (not just exclude) — default to sampling non-Western (esp. Chinese) and specialised
  sources in their own languages (e.g. Baidu/Zhihu/CNKI/Qiita, not English-with-region-appended);
  justify exclusions, never leave them implicit
- Time horizon (only current, or include historical?)

**Output:** 3-5 research questions + scope boundaries.

**Read Everything First — Meta-Principle:** Before forming any judgment about a source, read the ENTIRE source. Do not skim the first page and extrapolate. Do not search for confirmatory quotes before reading the full document. The most common research failure is forming a conclusion on partial information and then filtering everything else to confirm it. If you haven't read the full source, your "assessment" is a guess. If you have 50+ sources, batch-read: read all of them before writing the synthesis.

### Step 1a: Pattern & Reference Survey

Surface what you already know before searching for novelty: 64-81% of expert design thinking is pattern recognition, not analytical invention (Kannengiesser & Gero 2019); experts recognize the situation as a known type and the response follows (Klein 1985; Manteuffel 2018).

**The Default Project question:** "What project, architecture, or problem does this remind me of?" — surface that match explicitly, even as guesswork; experts begin with a "default template" and refine by finding what's _different_ (Dorst & Cross 2001).

**Reference types (in priority order):**

| Priority | Type                       | What to look for                                                                     | How it works                                                                                           |
| -------- | -------------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------ |
| **1st**  | **Pattern / architecture** | What structural shape does this problem have? What similar architecture have I seen? | Tacit pattern matching — the expert "sees" the situation as a type (Klein: 80-90% of expert decisions) |
| **2nd**  | **Direct precedent**       | Existing solutions to the exact problem                                              | Explicit adaptation — open-source projects, commercial products, published architectures               |
| **3rd**  | **Negative precedent**     | Attempts that failed and why                                                         | Postmortems, discontinued projects, migration case studies                                             |
| **4th**  | **Component precedent**    | Reusable building blocks                                                             | Libraries, frameworks, APIs, SDKs, platforms                                                           |

**Survey method:**

1. **Default Project** — Ask "What does this remind me of?" and write the answer. Surfaces the tacit pattern match before it fades.
2. **Pattern check** — What 2-3 architectural patterns does the situation suggest? Most real systems use 1-3 patterns (Harrison & Avgeriou 2008); more than 3 = overcomplicating.
3. **Search for each reference type** — 5-10 minutes per type, timeboxed. Pattern recognition first, solution-finding second.
4. **Document** each reference: name, approach, key insight, what it tells you about what's _different_ about your situation.
5. **Before concluding:** ask "What if there's already an architecture that handles 80% of this?" (fights NIH bias).

**Output:** A pattern map — the 1-3 architectural patterns/default templates framing this problem, plus existing solutions/components to leverage. Key insight: what's _different_ from the template, not what to copy.

### Step 2: Search

**Search strategies (in priority order):**

1. **Direct search** — GitHub, language registries (crates.io / PyPI / npm), academic databases, technical blogs
2. **Citation chaining** — When you find a good source, check what IT cites (backward), then who cites IT (forward, via Google Scholar). Two to three levels deep is usually enough.
3. **Adjacent search** — "What tools do [adjacent field] use?" or "How does [similar problem] work in [different domain]?"
4. **Expert search** — Who's writing or talking about this? (blog posts, talks, interviews)
5. **Negative results search** — Actively search for "why X failed" or "X doesn't work" — not just "X best practices."

**Search backend (agnostic — no hard dependency):** any web search satisfies this protocol: public SearXNG, Tavily/Exa/Serper, even raw web + fetch. Requirements are the 5 strategies above + per-claim URLs + Verification Tiers below. Nothing in this file requires a specific engine.

**Enhanced path (optional, progressive enhancement):** if the operator has a local stack (e.g. self-hosted SearXNG + extraction sidecar + `authoritative-search` / `meta-research` skills with connectors, trust scoring, effort modes), use it — it upgrades breadth (14 connectors, 4 langs), depth (full-page extract + archive fallback), and rigor (consensus → CONFIRMED/LIKELY/CONFLICTED/UNVERIFIED maps to L1/L2 below). Absence degrades gracefully to default search; never block on it, never hardcode its URLs.

**Routing (intention clarity decides how-exhaustive):**

| Situation                                              | Route                                                                                                                      | Why                                                          |
| ------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| No real intention / unknown domain (valley-of-despair) | Exhaustive fan-out (`meta-research` if available, else 3–5 manual sub-Qs: mechanism/landscape/implications)                | User can't frame Qs yet — cover what they didn't know to ask |
| Framed Qs from Step 1 (AMBITION gave hypothesis)       | One targeted search per Q (`authoritative-search` if available, else any search + fetch full page for load-bearing claims) | Fan-out per Q is 5× waste once framed                        |
| Single consequential claim                             | Verification Tier L1–L3 below (1 call → 2+ sources → deep + refutation)                                                    | Cheapest correct rigor                                       |

**Depth guidance:** Initial scan 30 min/question (surface answer); deep dive 2 hours/question (if surface reveals important unknowns); stop when questions answered OR timebox expires OR sources converge (3+ independent sources agreeing). Enhanced-stack mapping: quick scan ≈ `effort=little` (3), default ≈ `medium` (5), wide sweep ≈ `full` (parallel fan-out).

### Step 3: Evaluate Sources

Not all sources are equal. Use the **CRAAP test** to calibrate confidence:

| Criterion     | What to check                           | High confidence             | Low confidence            |
| ------------- | --------------------------------------- | --------------------------- | ------------------------- |
| **Currency**  | When was it published?                  | < 2 years old for tech      | > 5 years old             |
| **Relevance** | Does it directly address your question? | Direct match                | Tangential, feel adjacent |
| **Authority** | Who wrote it? What's their expertise?   | Domain expert, practitioner | Unknown author, anonymous |
| **Accuracy**  | Is the information verifiable?          | Cites sources, reproducible | Anecdotal, no evidence    |
| **Purpose**   | Why does this exist?                    | Inform, educate             | Sell, promote, evangelize |

**Evidence hierarchy (highest to lowest):** 1. Replicated controlled experiments → 2. Single controlled experiments → 3. Systematic case studies → 4. Expert opinion with documented experience → 5. Anecdotes ("this worked for me" — useful for hypotheses) → 6. Marketing/vendor content (directional at best).

**Calibration rule:** A blog post from a domain expert (tier 4) can beat a 2018 academic paper about tools that no longer exist. Hierarchy = guidance, not dogma.

### Parallelize research lanes (when to fan out — applies to Steps 2–4)

LANDSCAPE is the protocol's most parallel phase: research lanes are independent by construction — outputs merge without coordination at Step 5's evidence-table synthesis. Fan out to a swarm (team-mode / parallel subagents) when: sub-tasks partition cleanly, each lane lands in the shared evidence table, and lanes never need to talk mid-task. **The partition test:** if members can't be separated so they never communicate mid-task, you have a meeting, not a swarm. Never parallelize coordinated implementation — EXECUTOR is linear, one verified transition at a time. Swarms are safe here because the evidence table records every lane's output, the method ledger records each lane's invocation, the feature inventory gives parallel lanes one shared contract, and Step 5's evidence-table synthesis re-syncs the single thread of truth between parallel bursts.

### Verification Tiers (Research on Demand)

Every claim that could affect planning or decisions receives a verification tier.
The tier is determined by the CONSEQUENCE of being wrong, never by the AI's
confidence — research shows verbalized confidence is a poor correctness predictor
(calibration gaps up to 45 points; Xiong et al. 2024). Web search access reduces
hallucination 73-86% (FActScore), so grounding consequential claims in retrieved
evidence is the single highest-impact intervention.

| Tier                                                                  | Trigger                                                                                               | Action                                                                                                                                                                                                                  | Cost                  |
| --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- |
| **L0 — No check**                                                     | Trivial claim, no decision impact; output of executed code/tool (self-verifying)                      | State it; no tool calls                                                                                                                                                                                                 | ~0                    |
| **L1 — Lightweight authoritative check** (DEFAULT for factual claims) | Any claim that could affect planning/decisions; any number, date, name, version, API, or citation     | One targeted search against an authoritative source (official docs, spec, standards). CRAAP-screen for authority + currency.                                                                                            | 1 call                |
| **L2 — Cross-check**                                                  | Decision-relevant, surprising, contested, or newly-claimed; would trigger rework if wrong             | 2+ independent sources; corroborate; attach per-claim citations; VERIFY each citation resolves and supports the claim (CJR study: AI search engines mis-cite >60% of the time)                                          | 2-4 calls             |
| **L3 — Deep verification**                                            | One-way-door / high-stakes decision (kill/pivot/commit; spec commitments; legal/regulatory/financial) | Multi-source research with adversarial refutation pass; independent corroboration; external oracle where one exists (run the test/compile, don't ask the model); human review gate; explicit abstention if unverifiable | Mini-research project |

**Operating rules:**

1. **Decisiveness gate**: before checking, ask "would the decision change if this claim were false?" If no → L0. If yes → L1 minimum.
2. **Disagreement escalates, confidence never de-escalates**: two sources disagreeing escalates a tier. AI certainty never lowers one.
3. **Ground, don't recall**: consequential claims must come from retrieved evidence, not model memory. Closed-book claims are L2 minimum.
4. **Separate writer from checker**: research output passes through a citation-verification pass by a different role/agent (journalism pattern).
5. **Hazard-scaled effort with carve-out**: the whole deliverable doesn't need L3 — only the decision-critical aspects (NASA/FDA pattern).
6. **Corroborate or abstain**: a decision-critical claim with a single source is marked provisional. If it can't be corroborated and is decisive, say so explicitly rather than guess.
7. **Verification budget**: escalate only while expected-loss reduction exceeds check cost (value of information theory).

**This principle is STANDING** — it applies to every protocol step, not just LANDSCAPE. Any step may dispatch verification when a consequential claim is made. Backend-agnostic: L1 needs 1 authoritative source (official docs/spec/standard), L2 needs 2+ independent sources + resolved citations, L3 adds adversarial refutation + human gate. Enhanced-stack label mapping (optional): L2-corroborated ≈ CONFIRMED, single-authoritative ≈ LIKELY, disagreed ≈ escalate, community-only ≈ UNVERIFIED/provisional.

### Step 4: Capture Evidence

For each finding:

```
Finding:
Source: [URL, paper, conversation]
Relevance: [direct / adjacent / contextual]
Key insight: [1-2 sentences]
Implication: [how this affects your hypothesis]
Confidence: [high / medium / low — based on CRAAP + evidence tier]
Type: [green/supports hypothesis / red/contradicts / gray/unknown]
```

**Batch capture rhythm:** Research 1 hour, then capture 15 minutes. Do NOT capture as you go — it breaks flow.

### Step 5: Synthesize

1. **Landscape map** — 2-3 sentence summary of the competitive/technical landscape
2. **Key findings** — 3-5 most important insights (what surprised you, what was confirmed)
3. **Evidence table** — Structured comparison of options with confidence ratings
4. **Unknowns map** — organized by: **Known unknowns** (questions you have but haven't answered), **Assumptions** (taken as true with no evidence), **Unknown unknowns** (surfaced during synthesis)
5. **Reference list** — The most important sources for future reference

---

## Competitive Analysis (For Product/Project Research)

**Feature comparison matrix:**

| Feature        | Your concept | Competitor A | Competitor B |
| -------------- | ------------ | ------------ | ------------ |
| Core feature 1 | ✅ Planned   | ✅ Existing  | ❌           |
| Core feature 2 | ❌           | ✅           | ✅           |
| Unique feature | ✅           | ❌           | ❌           |

**Positioning analysis:**

- Where does your project fit on: simple ↔ powerful, opinionated ↔ flexible, library ↔ platform?
- What do competitors NOT do well? That's your opportunity.
- What do competitors do well that you should learn from? That's your reference.

---

## Confirmation Bias Safeguards

Research naturally confirms what you want to believe. These are required:

1. **Pre-register your questions** — Before you start, write down what you expect to find. Compare with actual findings.
2. **Active negative results search** — Search for "why [approach] failed" and "limitations of [tool]" — not just success stories. Required, not optional.
3. **Devil's advocate pass** — After a finding, actively argue the opposite: "What if this source is wrong?" "What if I'm misinterpreting this?"
4. **Source diversity** — All confirmatory evidence from one source type (all blogs, all vendor docs) is not confirmation. Need ≥2 source types.
5. **The "so what?" test** — After each finding: "Does this actually change what I would build?" If no, it's interesting but not actionable. Move on.

---

## Researching Unknown Domains (When You Know Nothing)

1. **Normalize the discomfort** — The "Valley of Despair" early on is expected. You're supposed to feel lost.
2. **Build a question bank** — Generate 10 naive questions about the domain. Even bad questions structure the search.
3. **The three-lens method** — research each question through: **Conceptual** (What is this? How does it work?), **Concrete** (What does it look like in practice?), **Critical** (What's wrong with it?).
4. **Start broad, then narrow** — "what is [domain]" → "how does [domain] handle [problem]" → "why does [approach] fail in [context]".
5. **Use the Dunning-Kruger curve** — High confidence (Mount Stupid) → low confidence (Valley of Despair) → rising confidence. Track where you are to calibrate whether to keep researching or move to prototyping.

---

## Signal vs. Noise: When to Stop Searching

Research has diminishing returns. Most value comes from the first 2 hours:

| After   | % of value found | Should you continue?                 |
| ------- | ---------------- | ------------------------------------ |
| 30 min  | ~40%             | Yes — surface scan                   |
| 2 hours | ~70%             | Maybe — if important unknowns remain |
| 4 hours | ~85%             | Probably not — diminishing returns   |
| 8 hours | ~95%             | Stop — you're over-researching       |

**Saturation signals (any ONE means stop):**

1. **No new sources** — The last hour of searching returned nothing novel. Same findings repeated.
2. **Convergence** — 3+ independent sources in different categories agree on the same finding.
3. **Diminishing relevance** — New findings are increasingly tangential to your hypothesis.
4. **The build-to-learn boundary** — You're reading about something a 2-hour prototype would tell you faster. Cross this boundary → move to STRATEGY.md.

**The "interesting but irrelevant" trap:** Fascinating but non-affecting findings go in a parking lot and STOP. Fascination is not a reason to continue.

---

## Evidence-Based Decision Making

**Worked example — "Should we use Rust or Go?"**

| Claim                                    | Evidence                               | Tier   | Confidence       |
| ---------------------------------------- | -------------------------------------- | ------ | ---------------- |
| "Rust is memory-safe by default"         | Academic papers, 10+ years of evidence | Tier 1 | High             |
| "Go is faster to write"                  | Surveys, practitioner reports          | Tier 4 | Medium           |
| "Our specific use case needs >10M req/s" | Benchmark your prototype, not research | —      | Prototype needed |

The output of LANDSCAPE.md is not "which is better" — it's "what's the evidence landscape, and where should we prototype to resolve the remaining uncertainty?"

---

## Meta: How to Research Effectively

1. **Ask better questions, not more searches** — Research quality is determined by question quality. Spend 15% of research time on framing.
2. **Follow the citation trail** — One good paper/blog post → its references → who cites it. Two to three levels deep maps the field.
3. **Look for negative results** — "What didn't work" is often more valuable than "what worked." Actively search for failures.
4. **Timebox aggressively** — Research has infinite depth. Set a timer per question. When it rings, synthesize what you have.
5. **Research with a hypothesis** — "I believe X is true — let me find evidence that would disprove it."
6. **Know when to stop and build** — When a 2-hour prototype tells you more than 2 more hours of reading, stop. The build-to-learn boundary.
7. **Document the unknowns, not just the findings** — What you don't know feeds VALIDATION.md.

### 8. Cache research results — don't repeat yourself

Cache results in structured markdown docs so future research waves build on past work, not repeat it.

**Cache rules:**

- **Document every significant research wave** in a versioned markdown file (e.g., `research/SUPER_ANALYSIS.md`) with a `Date:` header
- **Check cache before researching** — if a cached doc addresses the question, verify its date and only refresh if stale
- **Staleness heuristic:** fast-changing topics (model benchmarks) stale after 2 weeks; stable topics (architectural patterns) valid for months
- **Diff-only refresh** — when refreshing, only re-search topics that changed since the cache date. Skip confirmed-stable sections.
- **Maintain a source index** — keep a `research/` directory with raw data (community configs, benchmark tables) separate from analysis (synthesis, gap analysis). Raw data is reusable across syntheses.

**Example from practice:** The Agent-Stack repo maintains 3 research files: `SUPER_ANALYSIS.md` (508 lines, 6 community comparisons), `COMMUNITY_CONFIGS.md` (407 lines raw data), and `GAP_RESEARCH.md` (178 lines gap analysis). After 12 days, only OMO/OpenCode release notes and model price changes needed refreshing — the architectural comparisons were still valid.

The output of this document feeds into STRATEGY.md, which turns the research map into a ratified strategic intent — then informs VALIDATION.md, which prototypes the most important unknowns.

---

## Origin

Rewritten July 2026 following a deep research pass on research methodology. Incorporates: PRISMA systematic review methodology, Kitchenham's evidence-based guidelines, Kuhlthau's Information Search Process model, Glaser & Strauss's theoretical saturation, Wohlin's snowballing technique, CRAAP test for source evaluation, and Porter's Five Forces adapted for technical projects.

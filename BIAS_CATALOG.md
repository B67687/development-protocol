# BIAS_CATALOG — Cognitive Bias Catalog for Planning Decisions

Load this catalog at INBOX Phase 1 and at any phase transition where a decision is made. This is a detection-only catalog — treatment protocols live in the phase docs themselves.

---

## 1. Anchoring

**The first number or idea dominates all subsequent judgments.**

**Most dangerous in:** PRIORITIZE (scoring order), AMBITION (budget/timebox anchoring), SERIOUSNESS (first-dimension score sets the frame).

**Current defense:** PRIORITIZE.md § Scoring Rules (L38): score in random order — shuffle before starting.

**Gap:** No defense in AMBITION dialogue rounds — the first timebox or scope number the AI proposes anchors the entire negotiation. No shuffle or counter-anchor mechanism.

**Detection prompt:**
> "What was the first number mentioned? Would my answer change if it were 50% higher or lower?"

---

## 2. Sunk Cost

**Continuing because of past investment, not future value.**

**Most dangerous in:** SERIOUSNESS (idea evaluation), EXECUTOR (spec is wrong but expensive), LANDSCAPE (research hours sunk).

**Current defense:** SERIOUSNESS.md § Anti-Sunk-Cost Mechanisms (L181–L189): 7 mechanisms — pre-commit scoring, default-to-DROP, probes go down never up, sunk-cost caps D4 at 10, gate is cheap, decision journal, action framing.

**Gap:** No defense in EXECUTOR when the spec is wrong but implementation is partially done. Mid-execution pivots lack a structured off-ramp.

**Detection prompt:**
> "If I were starting fresh today with no history, would I choose this same path?"

---

## 3. Confirmation

**Seeking evidence that supports what you already believe.**

**Most dangerous in:** LANDSCAPE (research framing), EXTRACTION (problem definition), STRATEGY (ratification).

**Current defense:** LANDSCAPE.md § Confirmation Bias Safeguards (L225–L233): 5 required safeguards — pre-register questions, active negative-results search, devil's advocate pass, source diversity (≥2 types), "so what?" test.

**Gap:** No defense in EXTRACTION. The phase relies on the user's initial framing. EXTRACTION techniques can refine a confirmed hypothesis without ever challenging the core assumption.

**Detection prompt:**
> "What evidence would prove this wrong? Have I searched for it specifically?"

---

## 4. Availability

**Recent or vivid events are overweighted; base rates ignored.**

**Most dangerous in:** INBOX (recency bias in thought capture), EXTRACTION (recent problems feel most urgent), AMBITION (scope anchored to latest project's pain).

**Current defense:** None.

**Gap:** Full. No protocol step counterbalances recency or vividness. INBOX clusters by topic, not temporal bias. No base-rate check exists.

**Detection prompt:**
> "Am I prioritizing this because it's recent/vivid, or because the base rate says it matters most?"

---

## 5. Planning Fallacy

**Underestimating time, costs, and complexity of future actions.**

**Most dangerous in:** AMBITION (timebox and scope setting), EXECUTOR (milestone estimates), PACING (budget allocation).

**Current defense:** AMBITION.md § Pacing & Budget (L103–L142): phase budgets, actuals tracking in `.omo/pacing-track.md`, Pace Alert on >50% over, reference-class estimation, calibration after 3 milestones.

**Gap:** No calibration against base rates from prior projects at budget-setting time. Probabilistic forecasting is deferred until 5+ projects exist.

**Detection prompt:**
> "Based on similar past projects, how accurate were estimates at this stage? What's the reference-class base rate?"

---

## 6. IKEA Effect

**Overvaluing things you created yourself.**

**Most dangerous in:** PRIORITIZE (idea selection), SPECIFICATION (falling in love with the spec), STRATEGY (defending own proposals).

**Current defense:** PRIORITIZE.md § The Debiasing Question (L40–L46): "If someone else proposed this idea instead of me, would I still want to do it?" — applied to top 2 candidates.

**Gap:** No defense in SPECIFICATION. Once a spec is written, there is no debiasing question that asks "would you ship this if someone else wrote it?" The spec writer's attachment is unchallenged.

**Detection prompt:**
> "If someone else wrote this spec/plan, would I find it as compelling — or critique it more harshly?"

---

## 7. Survivorship Bias

**Studying successes and ignoring failures.**

**Most dangerous in:** LANDSCAPE (research framing), EXTRACTION (problem definition), STRATEGY (option generation).

**Current defense:** LANDSCAPE.md § Pattern & Reference Survey (L100–L122): "Negative precedent" is 3rd priority type. Step 2 Search includes "Negative results search" as strategy #5.

**Gap:** No explicit failure-case research requirement. Negative precedent is listed but not mandatory — a researcher could skip it if time-pressured.

**Detection prompt:**
> "What projects in this space failed? What killed them? Have I looked at failures as carefully as successes?"

---

## 8. Status Quo Bias

**Preferring the current state over change, even when change is beneficial.**

**Most dangerous in:** SERIOUSNESS (defaulting to SCHEDULE instead of DROP), EXTRACTION (accepting the stated problem without questioning it), STRATEGY (ratifying the familiar over the novel).

**Current defense:** None.

**Gap:** Full. SCHEDULE preserves status quo by defaulting to "keep thinking about it." EXTRACTION accepts the user's framing without a "what if we did nothing?" check. No step argues for change over inertia.

**Detection prompt:**
> "If I were already doing this new thing, would I switch back to the current state? If not, why am I hesitating?"

---

## Integration

Load at **INBOX Phase 1** (priming awareness before any decision) and at **any phase transition** where a decision is made. Surface the relevant bias and detection prompt at each decision point — standing check, not optional ceremony.

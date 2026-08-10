# EXTRACTION.md — Problem Discovery Protocol (v3)

> This is **Step 0** of the Development Protocol. It runs before DECOMPOSITION, before AMBITION, before any solution work.

> Before any technique, you must clarify the LANGUAGE. Language is fickle.
> Assume you do NOT know what the user means. Every sentence has 5 possible meanings.
> 
> **Dual ZPD Principle:** This step operates in two zones simultaneously — extracting the real problem (task zone) and raising the user's cognitive level (learning zone). Each technique also serves as a teaching instrument. The user should leave EXTRACTION not just with a clear X, but with a clearer mind.
> 
> **Method Manifest (invoke ALL or skip-with-catalog-code, per METHOD_LEDGER.md):**
> 1. Goal Climb · 2. No-Computer Check · 3. Why-Tree · 4. Contextual Probe ·
> 5. Mom Question · 6. Problem Statement Wall · 7. Job/Pain/Gain Map ·
> 8. Laddering · 9. Socratic Probe · 10. Cognitive Interview
> Log each in `.omo/method-ledger.jsonl` (applied/skipped-CAT-code/omitted-flag).

---

## Step 1: Clarify the Language

1. **Restate** what you heard in different words. Ask the user to correct you.
2. **Probe multiple dimensions** of the same statement: functional (what), social (perception), emotional (feeling)
3. **Probe multiple scales** of the same statement: micro (this task), meso (this week), macro (6 months)
4. **Probe multiple perspectives** of the same statement: yours, a peers, an outsiders

**The rule:** Do not proceed to any extraction technique until you can articulate
the users intent back to them AND they confirm yes that is right. Language is not transparent.

---

## When to Use This

Every time someone says "I want Y" or "We need Y":

- Y can be a feature ("a dashboard with real-time charts")
- Y can be an app ("a flashcard CLI for spaced repetition")
- Y can be a framework ("a protocol that takes intent to product")
- Y can even be a problem-statement-that-smells-like-a-solution ("we need better monitoring")

**If Y contains a solution word** (app, tool, system, dashboard, CLI, framework, API, platform) — you need EXTRACTION.

## Step 2: The Protocol — 4 Steps

### Step 2.1: Capture Y (the stated request)

> **Input:** INBOX's Raw-Thinking Pass produces the thick frame — the connection map,
> alternative framings, and candidate X's. Use it as the working context for this step.
> If no frame exists (cluster entered directly), spend 2-3 min generating one first —
> never extract from a thin one-line statement alone. Raw generates, EXTRACTION verifies.

Write down exactly what the person said, verbatim. Do NOT paraphrase.

> "I want a flashcard CLI with spaced repetition."

### Step 2.2: Apply the Applicable Techniques

The techniques below each extract X from a different angle; apply the ones applicable to this run (a smaller set is legitimate only with a catalog-code rationale or size-routing — per METHOD_LEDGER.md):

| #   | Technique                  | How to apply                                                                                                                                                       | Provenance                                                           |
| --- | -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------- |
| 1   | **Goal Climb**             | Ask "What goal does Y serve?" repeatedly until you hit a strategic mission goal. Write the chain.                                                                  | KAOS — van Lamsweerde, used in safety-critical systems for 30+ years |
| 2   | **No-Computer Check**      | "If we had no technology at all, how would this problem manifest?" If you can't describe it without solution words, you haven't extracted.                         | Problem Frames — Jackson, standard SE method                         |
| 3   | **Why-Tree**               | Ask "Why?" at each level, but FORCE branching: "Is there another reason? What if the answer is different?" Build a tree, not a chain.                              | Five Whys — Ohno/Toyota, adopted industry-wide                       |
| 4   | **Contextual Probe**       | "Show me the last time this was a problem. Walk me through exactly what you did." No summaries. No hypotheticals. Specific incidents.                              | Contextual Inquiry — Beyer & Holtzblatt, used by Microsoft/IBM/SAP   |
| 5   | **Mom Question**           | "What's the hardest part about [situation]?" / "What have you already tried?" / "What are you spending on this right now?" Past behavior only.                     | Mom Test — Fitzpatrick, used in startups globally                    |
| 6   | **Problem Statement Wall** | Write one paragraph describing the problem. NO solution words allowed. If "app," "tool," "system," "dashboard," "CLI," "API," "framework" appears — rewrite.       | Goal-Directed Design — Cooper, standard UX practice                  |
| 7   | **Job/Pain/Gain Map**      | Map Y to a Customer Profile (Value Proposition Canvas). Which functional/social/emotional job does Y serve? Which pain does it relieve? Which gain does it create? | Osterwalder, used by 20,000+ companies                               |
| 8   | **Laddering**               | Climb from Attributes → Consequences → Values: "Specifically what?" → "Why is that good/bad?" → "Why is that important to you?" Surface-level answers stall at Attribute. Struggle at Value level is diagnostic — it signals the boundary of articulated self-awareness. | Means-End Chain — Gutman (1982), 40+ years in consumer psychology    |
| 9   | **Socratic Probe Sequence** | Route through 6 question types systematically: Clarify → Challenge assumptions → Test evidence → Explore alternatives → Trace implications → Reframe the question. Move to next type only when current type is exhausted. | Paul & Elder Socratic Taxonomy (2007), used in critical thinking education worldwide |
| 10  | **Cognitive Interview**      | "Walk me back to the last time this was a problem. Report everything, even if trivial. Now tell it in reverse order. Now describe it from [competitor/expert/newcomer]'s perspective." Disrupts the user's narrative schema, surfacing details lost in abstract retelling. | Cognitive Interview — Geiselman & Fisher (1985), 41% more correct details per meta-analysis |

**The default is to invoke the applicable methods;** skip any method only with a catalog-code rationale (per METHOD_LEDGER.md) or size-routing (per INBOX.md Run-Shape Selector). For users who seem overconfident or give surface-level answers, prioritize Laddering, Socratic Probe, and Cognitive Interview — they're designed for the non-self-aware user.

### Step 2.3: Test X (the extracted problem)

After applying the applicable techniques (per INBOX.md Run-Shape Selector, Light mode may justify a smaller set), you should have a candidate X. Test it:

- [ ] X can be written in one sentence with **no solution words**
- [ ] **3+ independent perspectives** agree on X (if not, there are multiple X's)
- [ ] At least **3 alternative solution paths** exist (Y is just one option)
- [ ] X has a **measurable magnitude** ("customers lose 3 hours/week" or "47% of support tickets")
- [ ] **No one can ask "why?"** to X without getting a circular answer

**Convergence rule:** If 2+ techniques converged on the same X, stop. You found it.
If all 10 techniques gave different answers, you haven't extracted enough — the person doesn't know what they actually need yet.

### Incomplete Extraction Detection

These signals indicate the extraction is incomplete despite technique convergence — the user may be performing self-awareness rather than achieving it:

| Signal | What it looks like | Response |
| --- | --- | --- |
| **Performative convergence** | User agrees with X quickly, but the techniques didn't surface any struggle or discovery | Apply Laddering (Technique 8) — push to Values level. If still smooth, restart with Cognitive Interview. |
| **Overconfident agreement** | User says "yes that's exactly it" with high confidence, but can't explain the problem to another person | Teach-Back: "Describe this problem to me in your own words as if I'm a complete newcomer." If vague, incomplete. |
| **Abstract X** | X has no concrete incident supporting it — described abstractly | Apply Mom Question (Technique 5) or Contextual Probe (Technique 4). No incident = X is hypothesized, not extracted. |
| **Circular X** | When asked "why does X matter?", the answer loops back to X itself | Apply Goal Climb (Technique 1) again, pushing one more level up. |
| **Emotional avoidance** | User deflects emotional probes (changes subject, becomes abstract) | Don't force. Switch to No-Computer Check (Technique 2) to reframe technically. The avoidance itself is data. |

### Multiple Working Hypotheses (Set-Based Mode)

When the domain is high-uncertainty, carry 2-3 candidate X's through
FUNDAMENTALS before converging. This applies Set-Based Concurrent
Engineering to problem extraction — delay convergence to gather information.

**How it works:**

1. After applying techniques, identify 2-3 genuinely different candidate X's.
2. Progress them all through FUNDAMENTALS — document one-way doors and
   LLM bias risks for EACH candidate.
3. Downselect at the end of FUNDAMENTALS. The candidate with the fewest
   one-way doors and most falsifiability wins.
4. The downselected X enters AMBITION normally.

**When to use:** High-uncertainty domains. **When not:** Low-uncertainty.
Set-based candidates proceed through SERIOUSNESS/DECOMPOSITION normally; downselect happens after FUNDAMENTALS.


### Step 2.4: Decide

| If X is...                                         | Then...                                                                                            |
| -------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| **Validated** (passes all tests above)             | Proceed to SERIOUSNESS.md. Run the Seriousness Gate to evaluate whether X is worth pursuing.       |
| **Unclear** (partially extracted, some tests pass) | Do one more round with different techniques. Decompose only the clear parts.                       |
| **Unknown** (no convergence after 10 techniques)    | Prototype the most likely X. Run VALIDATION before decomposition (prototype IS the extraction; return to DECOMPOSITION with the validated X). |


---

## Protocol Output Format

Every AI response during protocol execution follows this structure:

### Layer 1: Overview (Sub-checkmarks in active phase)

Shows granular progress inside the current phase. Phases with sub-checkmarks
get a blank line separator. Phases without are packed together.

| ```
| EXTRACTION        [✓] COMPLETE
| FUNDAMENTALS      [✓] COMPLETE
| AMBITION          [✓] COMPLETE
| LANDSCAPE         [✓] COMPLETE
| VALIDATION        [✓] COMPLETE
| SPECIFICATION     [✓] COMPLETE
| EXECUTOR          [✓] COMPLETE
| REVIEW            [→] IN PROGRESS
| SHIPPED           [ ] PENDING
```
### Layer 2: Verbose (Raw conversation)

The full content of the current step. Questions, answers, reasoning.
This IS the conversation as it happens.

```

Step 1: Clarify the Language
Restate: [what I heard in different words]
Probe dimensions: [functional, social, emotional]
...
Awaiting confirmation...

```

### Layer 3: Round Checkmarks (What was completed this round)

A log of what was accomplished since the LAST message. Each message
appends what got done. This is the running satisfaction tally.

```

Round 1: Clarified language, confirmed intent
Round 2: Applied 3 techniques, found X
Round 3: One-way doors validated, appetite set
-> Now: prototype checkpoint format

```

The three layers update as phases progress. New sessions catch up by reading this file (checkpoint: `.omo/protocol-state.md`).
---

## Step 3: Intent Landscape (Explore Alternative Framings)

After extracting X, do NOT assume X is the only valid framing. Explore alternatives
at different abstraction levels, system boundaries, perspectives, and constraint
configurations. This takes ~15 minutes and produces 3-7 alternative X's.

1. **LADDER** — Move X up (Why? 3x) and down (How? 3x). Place each on a ladder.
   Sweet spot: abstract enough for optionality, concrete enough to act on.
2. **BOUNDARY** — Reframe the system using CATWOE (Customers, Actors, Transformation,
   Worldview, Owners, Environment). Different boundary = different valid X.
3. **HORIZONTAL** — At each level of the causal chain, ask "What ELSE could cause this?"
   Find sibling framings sharing the same root.
4. **LENSES** — View X through 6 value frames: Efficiency, Equity, Reliability,
   Experience, Cost, Safety. Each produces a different X.
5. **RELAX** — List constraints implicit in X. For each assumed constraint, ask:
   "Without this, what would X look like?"
6. **CONSOLIDATE** — Merge duplicates, discard non-viable, keep 3-7 candidates.
7. **CHOOSE** — Original X survives the landscape? Strengthen (you know WHY it wins).
   Alternative better? Pivot.

**Decision rule:** If no alternative is clearly better, keep X but document what you
considered. X is now stronger because you know it's not a local optimum.

---

## Example

**Y stated:** "I want a flashcard CLI with spaced repetition."

**Step 2.2 — Apply the applicable techniques**

Technique 1 — Goal Climb:

- "What goal does the CLI serve?" → "To review CS concepts efficiently"
- "Why does that matter?" → "To retain knowledge long-term"
- "Why does that matter?" → "To perform better in university exams"
- "And why does THAT matter?" → "To graduate with honors and get a good job"
- X: "Retain technical knowledge long-term for academic/career success."

Technique 4 — Contextual Probe:

- "Show me the last time you tried to review CS concepts. Walk me through it."
- User describes: opens notes folder, scrolls through PDFs, realizes they don't remember last week's topic, feels anxious, closes the folder.
- X consistent with above: retention is the problem, not the CLI.

Technique 6 — Problem Statement Wall:

- Attempt: "I need a system to retain CS knowledge over semesters without forgetting it between study sessions."
- Check: "system" is a solution word. Rewrite.
- Final: "CS knowledge learned in one semester is forgotten before the next exam, requiring full re-learning."
- No solution words. PASS.

**X:** CS knowledge learned in one semester is forgotten before the next exam.

**Step 2.3 — Test:**

- ✅ One sentence, no solution words
- ✅ Goal Climb + Contextual Probe + Problem Statement Wall all converged
- ✅ 3+ alternative solution paths (CLI tool, Anki deck, weekly study group, tutor)
- ✅ Measurable: hours spent re-learning vs. refreshing
- ✅ Terminal — cannot break down further

**Decision:** COMMIT — proceed to SERIOUSNESS.

## Provenness Requirement (v3 Rule)

Every technique in this protocol has empirical evidence:

1. **Goal Climb** — KAOS used in 50+ safety-critical projects (van Lamsweerde 2009)
2. **No-Computer Check** — Problem Frames taught in SE curricula globally (Jackson 2001)
3. **Why-Tree** — Toyota Production System, adopted globally (Ohno 1988)
4. **Contextual Probe** — 20+ years at Microsoft/IBM/SAP (Beyer & Holtzblatt 1998)
5. **Mom Question** — Validated in startup practice (Fitzpatrick 2013, 10,000+ copies)
6. **Problem Statement Wall** — Standard UX practice (Cooper 2007)
7. **Job/Pain/Gain Map** — 20,000+ companies use Value Proposition Canvas (Osterwalder 2014)

**New techniques added to the protocol MUST include a citation or a validation test.**

## Origin

Created July 2026 after research identified that the protocol was missing a step to extract the real problem (X) before decomposing the stated solution (Y). Synthesized from 8 research traditions across requirements engineering, product design, and systems thinking.

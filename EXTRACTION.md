# EXTRACTION.md — Problem Discovery Protocol (v3)

> This is **Step 0** of the Development Protocol. It runs before DECOMPOSITION, before AMBITION, before any solution work.

> Before any technique, you must clarify the LANGUAGE. Language is fickle.
> Assume you do NOT know what the user means. Every sentence has 5 possible meanings.

---

## Step 0: Clarify the Language

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

## The Protocol — 4 Steps

---

## Protocol Output Format

Every AI response during protocol execution follows this structure:

### Layer 1: Overview (Sub-checkmarks in active phase)

Shows granular progress inside the current phase. Phases with sub-checkmarks
get a blank line separator. Phases without are packed together.

```
EXTRACTION        [✓] COMPLETE
  [✓] Language clarified
  [✓] Intent landscape explored
  [✓] 7 extraction techniques applied
  [✓] X confirmed

FUNDAMENTALS      [✓] COMPLETE
  [✓] One-way doors identified
  [✓] Risk assessment done

AMBITION          [✓] COMPLETE
LANDSCAPE         [✓] COMPLETE
VALIDATION        [✓] COMPLETE
SPECIFICATION     [✓] COMPLETE
EXECUTOR          [✓] COMPLETE

REVIEW            [->] IN PROGRESS
  [✓] Output format finalized
  [->] Awaiting user confirmation

SHIPPED           [ ] PENDING
```

REVIEW [ ] PENDING
SHIPPED [ ]

```

### Layer 2: Verbose (Raw conversation)

The full content of the current step. Questions, answers, reasoning.
This IS the conversation as it happens.

```

Step 0: Clarify the Language
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

### Layer 3: Checkmarks

```

[✓] Language clarified
[✓] Intent landscape completed
[✓] 7 extraction techniques applied
[✓] X confirmed
[->] FUNDAMENTALS — one-way door analysis pending
[ ] AMBITION
[ ] DECOMPOSITION
[ ] EXECUTOR
[ ] REVIEW
[ ] SHIPPED

```

The three layers update as phases progress. New sessions catch up by
reading this file.

AMBITION [ ] PENDING
FUNDAMENTALS [ ] PENDING
EXECUTOR [ ] PENDING
REVIEW [ ] PENDING
SHIPPED [ ]

```

The checkpoint log IS the project memory. Every conclusion, every alternative
considered. New sessions catch up by reading this file.
---

## Step 2: Intent Landscape (Explore Alternative Framings)

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

### Step 1: Capture Y (the stated request)

Write down exactly what the person said, verbatim. Do NOT paraphrase.

> "I want a flashcard CLI with spaced repetition."

### Step 2: Apply 3+ Techniques (choose from 7)

Choose any 3 of these 7 techniques. Each extracts X from a different angle:

| #   | Technique                  | How to apply                                                                                                                                                       | Provenance                                                           |
| --- | -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------- |
| 1   | **Goal Climb**             | Ask "What goal does Y serve?" repeatedly until you hit a strategic mission goal. Write the chain.                                                                  | KAOS — van Lamsweerde, used in safety-critical systems for 30+ years |
| 2   | **No-Computer Check**      | "If we had no technology at all, how would this problem manifest?" If you can't describe it without solution words, you haven't extracted.                         | Problem Frames — Jackson, standard SE method                         |
| 3   | **Why-Tree**               | Ask "Why?" at each level, but FORCE branching: "Is there another reason? What if the answer is different?" Build a tree, not a chain.                              | Five Whys — Ohno/Toyota, adopted industry-wide                       |
| 4   | **Contextual Probe**       | "Show me the last time this was a problem. Walk me through exactly what you did." No summaries. No hypotheticals. Specific incidents.                              | Contextual Inquiry — Beyer & Holtzblatt, used by Microsoft/IBM/SAP   |
| 5   | **Mom Question**           | "What's the hardest part about [situation]?" / "What have you already tried?" / "What are you spending on this right now?" Past behavior only.                     | Mom Test — Fitzpatrick, used in startups globally                    |
| 6   | **Problem Statement Wall** | Write one paragraph describing the problem. NO solution words allowed. If "app," "tool," "system," "dashboard," "CLI," "API," "framework" appears — rewrite.       | Goal-Directed Design — Cooper, standard UX practice                  |
| 7   | **Job/Pain/Gain Map**      | Map Y to a Customer Profile (Value Proposition Canvas). Which functional/social/emotional job does Y serve? Which pain does it relieve? Which gain does it create? | Osterwalder, used by 20,000+ companies                               |

### Step 3: Test X (the extracted problem)

After applying 3+ techniques, you should have a candidate X. Test it:

- [ ] X can be written in one sentence with **no solution words**
- [ ] **3+ independent perspectives** agree on X (if not, there are multiple X's)
- [ ] At least **3 alternative solution paths** exist (Y is just one option)
- [ ] X has a **measurable magnitude** ("customers lose 3 hours/week" or "47% of support tickets")
- [ ] **No one can ask "why?"** to X without getting a circular answer

**Convergence rule:** If 2+ techniques converged on the same X, stop. You found it.
If all 7 techniques gave different answers, you haven't extracted enough — the person doesn't know what they actually need yet.

### Step 4: Decide

| If X is...                                         | Then...                                                                                            |
| -------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| **Validated** (passes all tests above)             | Proceed to SERIOUSNESS.md. Run the Seriousness Gate to evaluate whether X is worth pursuing.       |
| **Unclear** (partially extracted, some tests pass) | Do one more round with different techniques. Decompose only the clear parts.                       |
| **Unknown** (no convergence after 7 techniques)    | Prototype the most likely X. Run VALIDATION before decomposition. The prototype IS the extraction. |

## Example

**Y stated:** "I want a flashcard CLI with spaced repetition."

**Step 2 — Apply techniques**

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

**Step 3 — Test:**

- ✅ One sentence, no solution words
- ✅ Goal Climb + Contextual Probe + Problem Statement Wall all converged
- ✅ 3+ alternative solution paths (CLI tool, Anki deck, weekly study group, tutor)
- ✅ Measurable: hours spent re-learning vs. refreshing
- ✅ Terminal — cannot break down further

**Decision:** COMMIT — proceed to DECOMPOSITION.

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

```

```

```

```

```

```

# STRATEGY.md — Strategic Ratification Gate (v1)

> **This is a pipeline step** between LANDSCAPE and VALIDATION. It runs after research
> (LANDSCAPE) has mapped the landscape and before prototyping
> (VALIDATION) commit resources.
>
> **Purpose:** Produce a ratified strategic intent — the "how we win" — before any
> budget is allocated or prototype built. LANDSCAPE answers "what exists?"; STRATEGY
> answers "given what exists, how do we win?" and locks the approach with the human's
> explicit sign-off.

> **THE single ratification gate (Invariant 11):** AMBITION's locked scope + the strategic kernel + the phase budget are ratified here together, in one interaction. No other step asks for user ratification by default.
>
> **The posture (protocol-wide, see README):** the AI leads strategy as a world-class
> strategist. It proposes; the human ratifies; the human owns execution. This is the
> commander's intent model (ADP 6-0) — the AI sets the strategic direction, the human
> is the accountable commander. Applies to ANY work, regardless of user expertise.

## When to Use This

Run STRATEGY when these are all true:

- [ ] LANDSCAPE research is complete (the landscape map exists)
- [ ] AMBITION is locked (the ambition is concrete, falsifiable)
- [ ] The approach is NOT yet committed (no budget allocated, no prototype built)

If the approach is already clear and low-risk, STRATEGY may be abbreviated to the
proposal + ratification (Phase 2 + 3 only). The premortem gate is mandatory for any
one-way-door or high-stakes decision.

## Skip Conditions

> **Skip when:** SERIOUSNESS score <50 AND project appetite <1 week AND no one-way doors.
> **Risk of skipping:** no strategic ratification — approach may be wrong and you commit without sign-off.
> **Light mode:** skipped — strategy IS the ratification gate (Invariant 11), so skipping means you're in Light mode by definition.

## The Core Loop (4 Phases)

### Phase 1 — Pre-Commitment (human first)

The human states the strategic problem and their **own tentative intent** in their
own words, BEFORE the AI proposes.

- Why: prevents anchoring on the AI's frame and forces genuine thinking. Evidence:
  delegation without structured engagement degrades skill ~17% (Shen & Tamkin 2026,
  RCT); pre-commitment is the anti-deskilling keystone.
- Format: the human writes 1-3 sentences: _What is the challenge? What outcome
  matters? What would I do if I had to decide now?_

### Phase 2 — Proposal (AI, kernel format)

The AI drafts the strategy as a Rumelt kernel with intent and falsification blocks:

| Block                     | Required content                                                                          |
| ------------------------- | ----------------------------------------------------------------------------------------- |
| **Diagnosis**             | What is happening, and _why does it persist_? (dual diagnosis: surface + structural)      |
| **Guiding policy**        | The chosen approach — including an explicit **NOT-list** (what we will not do)            |
| **Coherent actions**      | 3-7 mutually consistent actions derived from the policy                                   |
| **Intent block**          | Purpose, key tasks, desired end state, limits (ADP 6-0 style, 3-5 sentences)              |
| **Falsification signals** | Observable events that would invalidate the strategy, plus a review trigger               |
| **Confidence statement**  | Calibrated confidence per decision domain + the strongest counterargument (contra-ruling) |

The AI leads here — it decides the strategy from the LANDSCAPE map and its
cross-domain knowledge, not from the user's prior. The user is not expected to
know the domain.

### Phase 3 — Ratification (human, bounded editing)

The human **edits the proposal** rather than accepting/rejecting wholesale.

- Why: bounded editing reduces algorithm aversion AND improves outcomes (Dietvorst
  et al. 2018: modifiable algorithms are trusted more and perform better).
- Mechanism: amendments are logged and visible. The human must write a ratification
  reason in their own words (no rubber-stamping).
- The amended proposal is final only after the human's explicit sign-off.

> **Bias check (load `BIAS_CATALOG.md` §IKEA Effect):** Before ratifying, ask — "Would I still choose this if someone else proposed it?" If the answer hesitates, the attachment is to authorship, not strategy.

### Phase 4 — Execution & Review Gates

Two gates bookend execution:

- **Premortem gate (before commitment):** assume the strategy failed 18 months out;
  generate top 3 causes; patch the plan. Klein (2007): premortems increase
  failure-cause identification ~30% and reduce overconfidence.
- **AAR gate (after execution):** After-Action Review — self-discovery first (the
  human explains expected vs actual, then the AI contributes). Compare outcome to
  the strategy's prediction (the falsification signals from Phase 2).

## Authority-Boundary Rules

| #   | Rule                                                                                                                           | Rationale / Evidence                       |
| --- | ------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------ |
| 1   | **No unratified action.** The AI executes nothing before human sign-off.                                                       | Core posture; EU AI Act Art. 14 HITL       |
| 2   | **Intent is 3-5 sentences.** Purpose, key tasks, end state, limits.                                                            | ADP 6-0: longer intent is not internalized |
| 3   | **Deviations are reported, not hidden.** Report deviation as soon as possible when intent cannot be followed.                  | ADP 6-0 ¶18                                |
| 4   | **Underwrite mistakes of initiative, not omission.** Honest errors from initiative are accepted; not reporting is not excused. | ADP 6-0 ¶26                                |
| 5   | **Disciplined initiative only.** Acting outside intent is the failure mode.                                                    | Sickles/Meade at Gettysburg                |
| 6   | **No scope creep.** New domain/novel situation → new cycle, not auto-extension of prior mandate.                               | Prevents drift beyond ratified intent      |
| 7   | **The AI cannot overrule the human.** Final say = human accept / amend / disregard-and-decide.                                 | Meaningful human oversight                 |

## Keeping the AI Honest

- **Mandatory calibrated-confidence statements.** LLMs show ~9% average
  overconfidence with a hard-easy effect — most confident exactly when most likely
  wrong. The AI states confidence per decision domain and flags low-confidence areas.
- **Contra-ruling.** Every proposal includes the strongest counterargument and the
  conditions under which its recommendation would flip. Counters sycophancy
  (Anthropic 2023; Cheng et al. Science 2026: sycophantic AI raises conviction but
  lowers responsibility).
- **Truthfulness over agreement.** The AI prefers being wrong-and-corrected over
  agreeable-and-wrong.
- **Prefer boring-but-testable.** Elegant-sounding plans are a risk signal (Blotto
  tournaments: LLM strategies were "more stereotyped"; humans outperformed).
  Require falsifiability over flourish.

## Preventing Deskilling (the human must not atrophy)

The largest empirical risk is not AI overreach — it is the human atrophying.

- The **Augmentation Trap** (Caosun & Aral 2026): novices who lean on AI
  augmentation can deskill to zero — worse than never having the tool.
- **Cognitive surrender** (Shaw & Nave 2026): delegation correlates with +25pp
  adoption but −15pp independent reasoning.
- **Delegation degrades skill** (Shen & Tamkin 2026): −17% from plain delegation.

Countermeasures (mandatory):

1. **Pre-commitment** (Phase 1) — the human forms intent BEFORE seeing the AI proposal.
2. **Ratification reason in own words** — forced articulation is deliberate practice.
3. **Scaffolding ladder with fading** — start with modeling + coaching; fade AI
   support as the human demonstrates competence; end with the human drafting first
   and the AI critiquing. Fading is conditional on demonstrated competence, not time.
4. **Cold-start no-AI drills** — for a new domain, the human produces a complete
   unaided strategy before the first AI proposal, then compares.
5. **Full "AI decides and acts" (Sheridan-Verplank level 8+) is contraindicated**
   for novices. Acceptable only for an expert human ratifying under time pressure —
   and even then the AAR gate applies.

## Output Format

```
STRATEGY           [✓] COMPLETE
  [✓] Phase 1: Human pre-commitment captured
  [✓] Phase 2: AI kernel proposal (diagnosis, policy, actions, intent, falsification, confidence)
  [✓] Phase 3: Human ratification — amendments logged, reason in own words
  [✓] Phase 4: Premortem gate passed (top 3 failure causes patched)
  [→] Ratified strategic intent → VALIDATION (prototype)
```

## Integration

```
INBOX → PRIORITIZE → EXTRACTION → SERIOUSNESS → FUNDAMENTALS (incl. MULTI) → DECOMPOSITION → AMBITION (incl. PACING) → LANDSCAPE → [STRATEGY] → VALIDATION → SPECIFICATION → EXECUTOR → REVIEW (incl. EXPLAINER + SPEC_SYNC) → REFLECT → ship
```

STRATEGY consumes: AMBITION's locked ambition + LANDSCAPE's research map + P2b WHICH-X? decision (same / scaled / adjacent / more-than-X) — the chosen variant's scope is the strategy's subject. Without a logged WHICH-X (or explicit P2b skip rationale), STRATEGY is blocked. Ready means the capability has a signal type per `docs/appendix/p2b-mapping-appendix.md` §3 → `docs/appendix/p4-late-appendix.md` stencil; if Light, log why late P4 skipped.
STRATEGY produces: the ratified strategic intent that VALIDATION tests.
STRATEGY produces: the ratified strategic intent that VALIDATION tests.

## Provenness

| Element                                | Source                                         | Evidence                                      |
| -------------------------------------- | ---------------------------------------------- | --------------------------------------------- |
| **Kernel-format proposal**             | Rumelt, _Good Strategy/Bad Strategy_           | Widely validated in practice; high confidence |
| **Commander's intent model**           | ADP 6-0 Mission Command                        | U.S. Army doctrine; high                      |
| **Bounded-editing ratification**       | Dietvorst et al. (2018), modifiable algorithms | Experiments; high                             |
| **Pre-commitment anti-deskilling**     | Shen & Tamkin (2026), RCT                      | Randomized trial; high                        |
| **Premortem gate**                     | Klein (2007), HBR                              | Field + experimental; high                    |
| **AAR gate**                           | FM 7-0 Appendix K                              | U.S. Army doctrine; high                      |
| **Confidence statements**              | LLM overconfidence research (arXiv 2605.23909) | Benchmarks; high                              |
| **Contra-ruling / sycophancy defense** | Anthropic (2023); Cheng et al. Science (2026)  | Red-team + experiments; high                  |
| **Augmentation Trap**                  | Caosun & Aral (2026)                           | Longitudinal analysis; medium-high            |

_Full reference: agentic-workflows/AI_STRATEGY_PROTOCOL.md (standalone reference doc)._

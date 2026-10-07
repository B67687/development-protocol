# G9: Question Fatigue / Communication Burden / Gate Ordering — Must Evidence Precede AMBITION?

- **Date:** 2026-10-04
- **Status:** Research (pre-decision — no reorder executed)
- **Scope:** Development Protocol gate ordering question: should an EVIDENCE gate precede AMBITION, and should AMBITION move earlier? Rule under test: research before reorder, never same commit.
- **Sources:** SearXNG web search + extractor reads (QuestionPro, SurveySparrow 2026 benchmarks, Culture Amp). No INDEX/git operations per task constraint.

## 1. Research Question and Scope

The G9 question as posed: does the protocol ask too much, too early — and is the gate order itself part of the burden? Specifically: must evidence precede AMBITION, and would moving AMBITION earlier reduce question fatigue — or merely relocate it?

Scope boundaries for this document:

- **In scope:** external evidence on question fatigue, communication/interruption burden, and what that evidence implies for gate ordering; the procedural rule (research before reorder; never bundle research and reorder in one commit).
- **Out of scope:** executing any reorder. This document is the research artifact that must exist *before* a reorder proposal is written, let alone committed.
- **Non-goal:** settling the full protocol phase design. G9 answers one ordering question, not the whole gate architecture.

The falsification criterion for the AMBITION-earlier hypothesis: if evidence shows early ambition-setting *increases* downstream rework questions (because ambition stated without evidence gets revisited), then moving AMBITION earlier fails its own fatigue-reduction goal.

## 2. Question Fatigue: What External Evidence Says

Survey-methodology literature gives the strongest quantitative base for "asking costs answer quality":

- **Response collapse over time.** OpinionLab via Forbes (cited in QuestionPro's survey-fatigue review): survey response rates fell from ~20% twenty years ago to ~2% today; 80% of customers have abandoned a survey halfway through, and 52% say they will not spend more than 3 minutes on a feedback form. Source: https://www.questionpro.com/blog/survey-fatigue/
- **Volume surge.** SurveySparrow's 2026 benchmarks: survey request volume up 71% since 2020; many organizations saw response rates drop from ~30% to ~18% within six months during 2025–2026; ~70% of drop-offs attributed to exhaustion. Source: https://surveysparrow.com/blog/survey-fatigue-benchmarks-2026/
- **Mechanism — satisficing, not just refusal.** The consistent finding across the literature (Krosnick-style satisficing; Tourangeau cognitive-response model: comprehension → retrieval → judgment → response): as burden rises, respondents do not merely quit — they answer *worse*. Documented markers: straight-lining (same-scale repetition), 75 seconds per question on single-question surveys collapsing to ~19 seconds per question in 26–30-question surveys, open-ended response shrinkage to single words, and +10–64% item-skip probability as survey time extends by an hour (SurveySparrow 2026 synthesis).
- **Pre/mid/post structure.** Fatigue operates in three phases: pre-survey (ignore the request — the 12–15-minute-estimate deletion reflex), mid-survey (rush/quit), post-survey (learned non-participation — future requests poisoned by past burden). Any gate reorder must be evaluated against all three, not just the in-the-moment question count.

Protocol translation: each protocol question is a survey item; each gate is a survey page. The evidence predicts that front-loading questions (AMBITION earlier, i.e., more up-front interrogation) risks *pre-survey fatigue* (user disengages before work starts) even if it reduces total item count. Fewer total questions ≠ less fatigue if they arrive before trust is established.

## 3. Communication Burden and Interruption Cost

Two adjacent literatures constrain the "just ask" instinct:

- **Interruption science (Mark et al.).** The widely replicated attention-residue results: average ~47 seconds per screen before attention shifts; ~25 minutes to fully re-orient after an interruption (Gloria Mark, UC Irvine — cf. Cambridge ThinkLab talk). Every clarifying question is an interruption of the user's own task thread, with a resumption lag the protocol never budgets.
- **Agent clarifying-question behavior.** Current AI-agent product discourse converges on: status checks, policy questions, and repeated clarifying back-and-forth dominate support/queue load; the fix direction is *participant listening + asking only when critical details are missing* (e.g., agent joins as participant, types clarifying questions only on genuinely blocking ambiguity), not maximal up-front elicitation.
- **Implication for gate order.** Interruption cost is *timing-sensitive*: a question asked after partial evidence exists can be answered by pointing ("see the retrieved X"); the same question asked cold requires recall and composition. Evidence-first ordering converts some would-be questions into confirmations, which are cheaper on both sides. This is the core pro-evidence-first mechanism (developed in §5).

Caveat: SearXNG returns on "gate ordering / evidence before ambition" as protocol-internal terms were noise (logistics/gate-product results) — correctly so, since the ordering question is internal to this protocol. Sections 5–6 therefore reason by mechanism transfer, not by direct citation, and are marked as such.

## 4. Lack-of-Action Fatigue: The Stronger Explainer

Culture Amp CEO Didier Elzinga's thesis — "people don't get survey fatigue, they get lack-of-action fatigue" — reframes the problem: the dominant driver of non-participation is not frequency or length but the broken feedback loop (listen → act → communicate). Two to three surveys-without-visible-action suffice to poison future participation. Source: https://www.cultureamp.com/blog/survey-fatigue-lack-of-action-fatigue

Applied to the protocol:

- A gate sequence that asks (EXTRACTION/AMBITION) and then visibly *uses* the answers (plan cites them, execution traces to them) sustains willingness across gates. A sequence that asks the same thing twice — or asks then ignores — triggers lack-of-action fatigue regardless of order.
- This cuts both ways for the reorder debate. Moving AMBITION earlier only helps if downstream gates demonstrably consume the ambition statement. If AMBITION-earlier is followed by evidence gates that re-ask what ambition already settled, the reorder *manufactures* lack-of-action fatigue.
- Diagnostic: declining answer quality at later gates is ambiguous between "too many questions" (fatigue) and "my earlier answers weren't used" (action gap). Before reordering, instrument which one is occurring — e.g., check whether later-gate answers repeat earlier-gate content (action-gap signal) versus degrade into satisficing (fatigue signal).

## 5. Must Evidence Precede AMBITION? Mechanism Analysis

The question decomposes into two distinct claims that must not be conflated:

**Claim A (epistemic): ambition stated without evidence is low-quality ambition.** Stating what to build before retrieving what exists invites assumption hardening (FP-022): early guesses lock in without verification, and the protocol then spends later gates defending rather than stress-testing. Evidence-first (retrieval of codebase, standards, prior art) grounds ambition in what is actually there — the Understand-First gate logic at protocol scale.

**Claim B (economic): evidence-first reduces total question burden.** Per §3, evidence converts open questions into confirmations. "We found X — proceed on that basis?" is cheaper than "What is X?" asked cold. This holds *if* retrieval is cheap relative to interrogation, which for codebase/standards context it normally is (indexed, local, non-interruptive), versus user attention, which is the scarcest resource in the loop.

**Counter-mechanism (the genuine risk of evidence-first):** unbounded evidence gathering without an ambition bound becomes its own fatigue — the agent burns context and time retrieving things the ambition statement would have scoped out. Evidence needs a *provisional* ambition (a working want-statement) to bound retrieval; otherwise research sprawls. Note this argues for a *lightweight provisional want* before evidence, not for moving the full AMBITION gate earlier — a distinction §6 makes structural.

**Provisional verdict (research-stage, not decision):** the mechanisms favor evidence-before-AMBITION *provided* retrieval is bounded by a provisional want. The evidence does not support unconditional evidence-first (research without any want-statement), nor does it support ambition-first (full commitment before grounding).

## 6. AMBITION-Earlier Reorder: Arguments For and Against

**For moving AMBITION earlier:**

1. *Scoping economy* — an early ambition bound limits evidence retrieval (§5 counter-mechanism), preventing research sprawl.
2. *User-model alignment* — users often arrive with a want; making them wait through evidence gates before stating it can feel like pre-survey fatigue (§2): the protocol appears to ignore what they already know.
3. *Action-loop visibility* — stating ambition early and then visibly grounding it in evidence demonstrates the listen→act loop (§4), potentially sustaining engagement.

**Against moving AMBITION earlier:**

1. *Pre-survey fatigue transfer* — front-loading the highest-cognitive-load gate (full ambition articulation) before trust/grounding risks the deletion reflex: user disengages at the hardest question, asked at the coldest moment.
2. *Assumption hardening* — a fully stated ambition pre-evidence becomes a commitment device; later evidence gets filtered through confirmation rather than revision (FP-022, FP-024 confident wrongness).
3. *Rework questions* — ambition stated without evidence gets revisited once evidence arrives ("given X, is the ambition still Y?"), so the reorder may *increase* total questions — failing its own goal by its own metric (§1 falsification criterion).
4. *Satisficing at the worst gate* — the hardest questions asked earliest, when respondent effort budget is full but context is thinnest, produce confident-but-ungrounded answers: maximum effort, minimum information.

**Synthesis direction (not yet a decision):** the for/against structure suggests the resolution is not a binary swap but a *split*: a lightweight provisional want early (bounds research, honors the user's arrival model) with the full AMBITION commitment gate remaining post-evidence. Whether that split is adopted is a decision for a separate proposal document — this research only establishes that both pure orderings have evidence-backed failure modes.

## 7. Research-Before-Reorder, Never-Same-Commit: The Procedural Rule

Regardless of which ordering wins, the task statement imposes a procedural constraint, and this section records its rationale so the rule survives contact with urgency:

1. **Research before reorder** — gate order is load-bearing protocol architecture: it changes what every downstream gate assumes. Reordering on intuition repeats the exact failure the protocol exists to prevent (acting before grounding). This document is the required prior artifact; a reorder proposal must cite it or explain which of its findings it disputes.
2. **Never the same commit** — research and reorder must not share a commit for three reasons: (a) *reviewability* — a commit mixing evidence-gathering with structural change cannot be reviewed for either purpose; atomic-commit discipline (one logical change per commit) forbids it; (b) *reversibility* — if the reorder proves wrong, revert must restore exactly the prior order without discarding research; a mixed commit makes surgical revert impossible; (c) *falsifiability* — the research commit fixes the evidentiary baseline against which the reorder's effects (question counts, rework rates, answer quality) are later measured. Moving the baseline in the same commit as the change destroys the comparison.
3. **Sequence mandated:** (i) this research doc commits alone; (ii) a separate reorder *proposal* (citing this doc, with falsification criteria from §1) is reviewed; (iii) only then a reorder commit, alone, with before/after gate map.

No reorder is executed, proposed, or staged by this document. Compliance statement: no INDEX or git operations were performed in producing it.

## 8. Recommendation and Decision Log

**Recommendation (research-stage):** do not reorder on this document alone. The next step is a bounded reorder proposal that:

- Adopts or rejects the §6 split-gate direction (provisional want early, AMBITION commitment post-evidence), with explicit falsification criteria (rework-question count, later-gate answer quality, evidence-citation rate in ambition statements).
- Instruments the §4 diagnostic first (action-gap vs. fatigue signals in current gate transcripts) so the proposal targets the actual burden source.
- Ships as its own commit, after this research commit, never with it.

**Decision log:**

| Date | Event | Status |
|---|---|---|
| 2026-10-04 | G9 research executed (SearXNG + extractor; no INDEX/git) | Done — this document |
| 2026-10-04 | Gate reorder (AMBITION-earlier or split-gate) | Explicitly NOT done — awaits proposal citing this research |
| TBD | Instrument action-gap vs. fatigue diagnostic on gate transcripts | Open |
| TBD | Reorder proposal with falsification criteria | Open, blocked on diagnostic |

**Sources consulted:** QuestionPro survey-fatigue review (OpinionLab/Forbes figures); SurveySparrow 2026 fatigue benchmarks (71%-volume, 30%→18%, satisficing markers); Culture Amp lack-of-action-fatigue thesis; Gloria Mark attention-residue literature (via secondary); agent clarifying-question product discourse. Gate-ordering terms returned no applicable external literature (protocol-internal vocabulary) — §§5–6 are mechanism transfer, marked accordingly.

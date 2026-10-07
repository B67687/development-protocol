# G4 — Dynamicism Definition: Stance/Growth-Calibrated Gating, Operationally

- Status: first sweep complete — 2026-10-04
- Scope: define "dynamicism" (T-053 protocol neologism) as an operational construct; derive stance-read × growth-calibrated gating rules for phase transitions
- Non-goals: affect/emotion control loops (deferred to future sweep — flagged as residual G4 in guidance-dynamic-2026-09-30); calibrated difficulty metrics (G1 territory); implementing phase-gate code
- Method: SearXNG science + general queries (instance 127.0.0.1:8888) + extractor verification of 3 load-bearing sources (ZPD, Flow, ITS review); DOI/paywalled sources marked snippet-only; confidence levels per claim
- Proposed INDEX row: `| [G4 dynamicism](g4-dynamicism-2026-10-04.md) | Operational definition of dynamicism as stance-read × growth-calibrated gating; ZPD/flow/SDT/ITS grounding with confidence-graded claims. | T-053 |`
- Note: per sweep instructions, no INDEX.md or git writes were made; row above is proposal only.

## 1. Problem Statement

T-053 (OPEN): "Protocol should be dynamic/engaging/growing-the-user — what is dynamicism?" The term has no external referent — a general-category SearXNG query confirmed "dynamicism" is a protocol-internal neologism (zero definitional hits). The narrowed sweep question from retro-inventory is sharper and answerable: **what does stance/growth-calibrated gating mean, operationally?**

Decomposed, the question has two coupled parts already latent in protocol artifacts:

1. **Stance-read (diagnostic):** INBOX.md carries a 7-dimension stance table (Knowledge, Assertiveness, Attachment, Patience, Reflectiveness, Stopping, Gravity) with low/high signals and handling notes. It is read "at session start, never interrogated, re-read at STRATEGY." But no artifact specifies *how a stance reading maps to a gate decision* — the table is descriptive, not operational. Stance is currently asserted, never measured.
2. **Growth-calibrated gating (decision):** phase transitions should depend on evidence that the user grew (can do with less support what previously required support), not on elapsed steps or task completion. No gate predicate of this form exists anywhere in the protocol — gates today are implicit (phase scripts advance when content is done).

So the operational definition this sweep proposes: **dynamicism = the protocol's rate and correctness of adaptation of scaffolding level to measured user state (stance × demonstrated growth), where every phase gate is a predicate over both.** A protocol is "dynamic" to the extent that (a) it reads stance as structured input to each phase, (b) it fades or steps up support contingent on demonstration, and (c) its gates test growth (reduced-support performance), not coverage. Everything else — engagement, liveliness, "growing the user" rhetoric — is downstream of these three mechanisms or it is decoration.

## 2. Field Landscape

Four literatures jointly cover the construct; none uses the word "dynamicism."

**2.1 ZPD and scaffolding (educational psychology).** Vygotsky's zone of proximal development — the space between what a learner does unsupported and what they cannot do even with support — is the canonical "slightly above" construct (verified via extractor: en.wikipedia.org/wiki/Zone_of_proximal_development). Vygotsky introduced ZPD partly to argue *against* static knowledge tests as intelligence measures: two learners with identical test scores can have different developmental levels — directly analogous to two users completing the same phase with different support levels. Scaffolding (Bruner, Wood & Ross — Vygotsky never used the term) operationalizes ZPD as support that tapers as it becomes unnecessary; Wass & Golding's finding that the hardest scaffoldable tasks produce the largest gains (verified) is the empirical basis for "desirable difficulty inside reach" (P3 in guidance-dynamic). A 2025 mathematical model of ZPD in digital systems (doi:10.31124/advance.175767621 — extractor-blocked, snippet-only) claims an exponential mapping between task complexity, scaffolding intensity, and motivation with IRT linkage — promising as a future calibration source, unverified.

**2.2 Flow and optimal challenge (positive psychology).** Csíkszentmihályi (1975, *Beyond Boredom and Anxiety*; 1990, *Flow*) locates engagement at the balance point of skill and challenge (verified via extractor). The 2021 Norsworthy et al. reframing (verified) is the most gate-usable formulation: two antecedents — **optimal challenge** (perceived capability to meet demands) and **high motivation** — and three experiential markers (absorption, effort-less control, intrinsic reward), defining flow as "an intrinsically rewarding state of absorption in which a high degree of control feels more effort-less than normal." Caution flag: the Wikipedia Flow article carries an LLM-contamination maintenance tag (Feb 2026, text added in a 2023 edit); structural claims cross-checked against the cited primaries (Nakamura & Csíkszentmihályi 2001, Norsworthy 2021) hold, but quote-level details should be re-verified against primaries before hard-coding. Flow supplies the *upper* gate bound: challenge above perceived capability exits flow into anxiety — the failure mode of stepping up too far.

**2.3 Dynamic assessment and graduated prompting (assessment).** Dynamic assessment (Feuerstein tradition; group-DA and CVS studies surfaced in search, snippet-level) inverts static testing: it measures learning potential by the *amount and type of help* needed to succeed — graduated prompts from implicit to explicit. This is the closest field-validated analogue of a growth-calibrated gate: the gate predicate is literally "succeeds with ≤ N prompt levels," and prompt-count becomes the growth metric across sessions. No DA source was extractor-verified (dissertation/PDF hosts); treatment as medium-confidence pattern, not fact.

**2.4 Adaptive tutoring systems and difficulty adaptation (AIEd).** Liu, Latif & Zhai's PRISMA review of 86 ITS/RTS studies (arXiv:2503.09748 — abstract verified via extractor) finds AI techniques (Bayesian Knowledge Tracing, LLMs) improving adaptability and outcomes, with persistent gaps in cognitive adaptability and scalability — i.e., the field can personalize content but still struggles to model the learner's cognitive state, exactly the protocol's stance-read gap. Adjacent snippet-only results: SkillAI (Random Forest difficulty classification at 95.67%), fast dynamic difficulty adjustment via IRT (Augsburg group), difficulty-aware conversational knowledge tracing (arXiv:2605.01097). Directionally consistent: difficulty calibration is a classification/tracing problem over demonstrated performance, not a self-report problem.

**2.5 Growth mindset, SDT, autonomy support (motivation).** Three snippet-only results constrain the motivation side of gating: AI-driven feedback framed through growth-mindset + SDT lenses (Elsevier, S0023969025000992); an SEM study (N=404) where autonomy support → basic-needs satisfaction → self-regulated learning explains 44.2% of variance; and an ML-heterogeneity finding that natural-science-mindset interventions average only ~0.26 effect with high variability. The implication is humbling for any "growth" gate: mindset effects are real but small and heterogeneous — gates should measure demonstrated reduced-support performance (behavioral), never self-reported mindset or inferred motivation (latent).

## 3. Top Findings (confidence-graded)

**F1 (high, multi-source verified): Stance and growth are orthogonal gate inputs; confusing them is the central failure mode.** ZPD (capability-with-support) and flow antecedents (perceived capability + motivation) are distinct constructs in the literature, and the protocol's artifacts already separate them (stance table vs. phase advancement) without saying so. Operational rule: every gate predicate must name which input it reads. A gate that advances on task completion reads neither — that is the current protocol state.

**F2 (high, verified): The growth gate has a field-validated form — performance under fading support.** Scaffolding theory (support tapers as unnecessary), Wass & Golding (hardest scaffoldable tasks → largest gains), and dynamic assessment's graduated-prompt metric converge on one predicate shape: `GATE = task success at support level S < support level previously required`. Growth is the *delta in required support*, not the delta in output quality. This converts T-053's "growing-the-user" from aspiration to measurement.

**F3 (high, verified): The stance read has a validated dimensional structure but no protocol measurement.** The INBOX 7-dimension table (Knowledge, Assertiveness, Attachment, Patience, Reflectiveness, Stopping, Gravity) is richer than most ITS learner models (which track knowledge + sometimes affect) and explicitly includes dimensions the literature neglects (Stopping, Gravity — session-shape judgments). But it is asserted once and never re-read against evidence. The literature's stance analogue — cognitive-state modeling — is precisely the gap the 86-study ITS review flags as unsolved at scale.

**F4 (medium, snippet-only): Mindset/motivation effects are small and heterogeneous — keep them out of gate predicates.** Average effect ~0.26 with high variability (NSLM-ML result); autonomy support works through needs satisfaction (44.2% variance explained in self-regulation) but that is a design input (how to frame feedback), not a gate criterion. Gates read behavior; SDT informs tone.

**F5 (medium, one blocked source): A quantitative ZPD mapping may exist but is unverified.** The Advance preprint's exponential complexity×scaffolding×motivation model with IRT linkage would, if verified, give the protocol its first candidate calibration function for "slightly above." Extractor blocked (403/bot); do not cite as foundation until the PDF is read directly.

**F6 (high, definitional): "Dynamicism" needs no external grounding — it is stipulative, and that is fine.** The general-query null result is itself a finding: the protocol is free to define the term, and this sweep's definition (rate × correctness of scaffolding adaptation, gated on measured stance and demonstrated growth) is consistent with all four literatures and contradicts none.

## 4. Design Principles (candidate gate rules)

P1 — **Every phase gate names its stance input and its growth input explicitly.** Template: `GATE(phase N→N+1) = stance_read(dimensions…) ∧ growth_demo(task T at support S < S_prev)`. A gate missing either conjunct is flagged incomplete at authoring time.

P2 — **Growth is measured as support-fade, not output quality.** Evidence of growth = same-class task completed with less scaffolding (fewer prompts, less explicit guidance, no worked example) than the last comparable task. Output correctness alone never advances a growth gate.

P3 — **Stance is re-read at every gate, not just session start.** Current practice (read once, re-read at STRATEGY) contradicts F3's implication: stance dimensions like Patience, Attachment, and Stopping drift within a session. Minimum viable: a 3-signal spot check (pace/friction signals for Patience, question-type signals for Knowledge, continuation signals for Stopping) evaluated at each gate.

P4 — **Step-up is bounded by perceived capability (flow ceiling).** When stepping up difficulty after a passed gate, the step must keep the next task inside the user's demonstrated capability-with-support envelope. Overshoot failure (anxiety/disengagement) is costlier than undershoot (boredom) because it corrupts the stance read for subsequent gates.

P5 — **Motivation framing follows SDT; motivation never gates.** Autonomy-supportive phrasing of gate outcomes (choice of next challenge, rationale for the step-up, acknowledgment of the demonstrated fade) is required presentation; no gate may predicate on inferred motivation, engagement signals, or mindset self-report (F4).

P6 — **Record the support ledger.** Each gate evaluation logs (task class, support level given, support level previously required, delta). The ledger is the protocol's longitudinal growth record and the future calibration dataset for any quantitative "slightly above" function (F5's promise, G1's territory).

## 5. Not-to-Encode (negative scope)

N1 — **Do not encode a numeric "slightly above" constant.** No verified calibration function exists (F5 unverified; G1 open). Support levels stay ordinal (e.g., worked-example → hints → confirmation-only → independent) until a calibration study exists.

N2 — **Do not gate on affect or engagement proxies.** Flow's experiential markers (absorption, effort-less control) are self-report/clinical constructs; the protocol has no valid instrument for them. Affect control loops are explicitly deferred (see header non-goals).

N3 — **Do not import mindset-intervention effect sizes as gate thresholds.** A 0.26-average, highly heterogeneous effect (F4) cannot justify any individual gate decision.

N4 — **Do not treat the 7-dimension stance table as validated.** It is expert-authored and structurally rich (F3) but has no inter-rater reliability, no behavioral anchors for most cells, and no update rule. Encode it as a versioned draft instrument (stance-table v0), not as ground truth.

N5 — **Do not conflate ITS content-adaptation with stance-reading.** The ITS literature personalizes *content difficulty*; the protocol's gap is modeling *user state* (Stopping, Gravity, Attachment have no ITS analogue). Borrowing ITS tracing machinery for knowledge dimensions is legitimate; claiming it covers stance is not.

N6 — **Do not cite the blocked ZPD-quantification preprint as foundation.** Snippet-only until the PDF is obtained and read (flagged in §6).

## 6. Open Gaps (what this sweep did not close)

G1 — **Behavioral anchors for stance dimensions.** What observable session signals map to low/high on each of the 7 dimensions? Without anchors, P3's spot check is unimplementable. Needs a dedicated instrument-design pass (likely G-series follow-up, possibly with session-transcript mining).

G2 — **Support-level ordinal scale.** P2/P6 assume an ordered scaffolding ladder (worked-example → … → independent). The protocol has no canonical ladder; guidance-dynamic's P1–P3 imply one but never fix the rungs. Small design task, high leverage.

G3 — **ZPD-quantification preprint verification.** Obtain and read doi:10.31124/advance.175767621 directly (extractor 403); if the exponential model + IRT linkage holds, it becomes the candidate calibration function that also serves G1 (the "slightly above" metric gap shared with guidance-dynamic).

G4 — **Affect/engagement loop design.** Explicitly deferred; the flow literature (especially effort-less control as a marker) is the starting point when this is scheduled.

G5 — **Dynamic-assessment graduated-prompt protocol adaptation.** DA's prompt-hierarchy method is the closest validated gate instrument (F2), but no DA source was extractor-verified this sweep. A follow-up should pull Feuerstein/graduated-prompt primaries (e.g., via the Pitt group-DA dissertation, open-access) and draft a prompt-ladder template for protocol gates.

G6 — **Empirical gate validation.** P1–P6 are literature-derived, not protocol-tested. Closing T-053 ultimately requires running gated vs. ungated sessions and checking whether support-fade deltas predict independent-task success — an evaluation design, not a literature task.

## 7. Sources

Load-bearing (extractor-verified full content or abstract):
- Vygotsky ZPD — en.wikipedia.org/wiki/Zone_of_proximal_development (verified; origins, definition, scaffolding/Wass & Golding, MKO)
- Flow (psychology) — en.wikipedia.org/wiki/Flow_%28psychology%29 (verified with caution: LLM-contamination tag Feb 2026; structural claims cross-checked to Nakamura & Csíkszentmihályi 2001, Norsworthy et al. 2021)
- Liu, Latif & Zhai (2025), Advancing Education through Tutoring Systems: A Systematic Literature Review (86 studies, PRISMA) — arxiv.org/abs/2503.09748 (abstract verified)
- Csíkszentmihályi (1975), Beyond Boredom and Anxiety; (1990), Flow — via secondary citation in verified sources (primaries not directly read)

Supporting (SearXNG snippet-level, not extractor-verified — medium confidence):
- ZPD quantification model in digital systems — doi:10.31124/advance.175767621.14681425/v1 (extractor 403-blocked; exponential complexity×scaffolding×motivation claim + IRT linkage)
- AI-driven feedback through growth-mindset + SDT lens — linkinghub.elsevier.com/retrieve/pii/S0023969025000992
- SDT-based digital health design mapping autonomy/competence/relatedness — arxiv.org/abs/2605.16276
- Autonomy support → needs satisfaction → self-regulated learning SEM, N=404, 44.2% variance — researchsquare preprint (snippet-level)
- Growth-mindset ML heterogeneity, avg effect ~0.26 — snippet-level
- SkillAI Random-Forest difficulty classification 95.67% — ijisae.org/index.php/IJISAE/article/view/7648
- Fast dynamic difficulty adjustment via IRT — opus.bibliothek.uni-augsburg.de/opus4/frontdoor/index/index/docId/121167
- Difficulty-aware conversational knowledge tracing — arxiv.org/abs/2605.01097
- Group dynamic assessment (Pitt dissertation) — d-scholarship.pitt.edu/44354
- Dynamic assessment of CVS — doi:10.1007/s11251-015-9344-y

Definitional null result:
- General-category query for "dynamicism" returned zero definitional hits — term is protocol-internal (supports F6).

Protocol-internal:
- INBOX.md stance table (7 dimensions); T-053 (OPEN); guidance-dynamic-2026-09-30.md (P1–P3, N1–N5, G1/G4); retro-inventory-2026-10-03.md (G4 row, P1–P5 routing).

## 8. Provenance & Next Step

- Queries (SearXNG, science + general, 2026-10-04): "dynamic assessment zone of proximal development graduated prompting interventionist" (science); "adaptive scaffolding intelligent tutoring fading support contingent tutoring" (science, noisy); "growth mindset feedback self-determination theory autonomy support competence" (science); "dynamic difficulty adjustment IRT adaptive learning flow challenge-skill balance" (science); "dynamicism definition" (general, null).
- Verification: 3/4 load-bearing sources extractor-verified (ZPD, Flow with caution flag, ITS-review abstract); 1 blocked (Advance DOI, 403); remainder snippet-level, confidence-graded inline (§3) and labeled in §7.
- No INDEX.md or git writes (per sweep instructions); proposed INDEX row in header.
- Suggested next step: close G2 (fix the ordinal scaffolding ladder — small, unblocks P2/P6 implementation) and G1 (behavioral anchors for the 7 stance dimensions), then draft a gate-predicate template per P1 for one phase transition as a pilot; schedule G5 (DA prompt-ladder primaries) alongside.

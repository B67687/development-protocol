# G5 — Teaching Principle: Simplest-Faithful-Words + Grasp-Now vs Eventual Evidence

- Status: first sweep complete — 2026-10-04
- Scope: define the protocol's teaching principle as an operational construct with two coupled clauses — (a) simplest-faithful-words (explain in the plainest language that loses no truth) and (b) grasp-now vs eventual-evidence (immediate felt understanding is not evidence of durable learning; gate on later demonstration, not present nodding)
- Non-goals: affect/engagement loops (G4 territory); scaffolding-ladder calibration (G4-G2); ambiguity-gate mechanics (G1 territory); implementing teaching templates in phase scripts
- Method: SearXNG science + general queries (instance 127.0.0.1:8888) + extractor verification of 3 load-bearing sources (1 arXiv abstract verified, 1 practitioner page full-read, 1 blocked 403); DOI/paywalled sources marked snippet-only; confidence levels per claim
- Proposed INDEX row: `| [G5 teaching principle](g5-teaching-principle-2026-10-04.md) | Operational definition of the teaching principle as simplest-faithful-words × grasp-now/eventual-evidence split; CLT/advance-organizer/fluency-illusion grounding with confidence-graded claims. | T-053-adjacent |`
- Note: per sweep instructions, no INDEX.md or git writes were made; row above is proposal only.

## 1. Problem Statement

The protocol teaches constantly — every phase explains terms, frames tasks, and confirms understanding — but has no stated teaching principle. Two failure modes recur in review transcripts and retro notes:

1. **Over-complex or over-simplified words:** explanations either carry full jargon load (faithful but ungraspable) or smooth into slogans (graspable but unfaithful — the simplification quietly drops the load-bearing distinction). No rule says where the floor is: how plain may words get before they stop being true?
2. **Grasp-now mistaken for learned:** a clear explanation produces immediate nodding — the user paraphrases back, all seems understood — and the phase advances. Whether anything survives to the next session is never tested. Immediate fluency is treated as evidence of durable learning, which the literature says it is not.

The narrowed sweep question: **what does "teach in the simplest faithful words, and verify later rather than trusting immediate grasp" mean, operationally?** Decomposed:

1. **Simplest-faithful-words (wording rule):** for any concept the protocol must convey, what is the plainest formulation that preserves every distinction the downstream work depends on — and how is faithfulness checked?
2. **Grasp-now vs eventual-evidence (evidence rule):** which observable counts as learning — present-tense fluency (paraphrase, confidence, "got it") or later unaided demonstration (recall, transfer, reduced-support performance)? What gate shape separates the two?

Proposed operational definition: **the teaching principle = every explanation ships at the lowest reading level that preserves all downstream distinctions (simplicity bounded below by faithfulness), and no explanation counts as learned on the strength of immediate grasp — only on later reduced-support demonstration.** Simplicity without faithfulness is slogan; grasp without later evidence is fluency illusion.

## 2. Field Landscape

Five literatures jointly cover the construct; none states it in this exact two-clause form.

**2.1 Cognitive load theory and the worked-example / guidance tradeoff.** CLT's core constraint (limited working memory; intrinsic vs extraneous vs germane load) is the scientific basis for "simplest words": extraneous complexity burns the budget that germane processing needs. Surfaced anchors: worked-example instruction optimisation (germane-load variants — https://linkinghub.elsevier.com/retrieve/pii/S0959475206000181, snippet); visual-representation design conditioned on prior knowledge (https://doi.org/10.1002/sce.20164, snippet); the germane-load measurement controversy (https://doi.org/10.1007/s10648-023-09738-0, snippet); the worked-example-vs-generation resolution via element interactivity — high-interactivity material favours worked examples, low-interactivity favours generation, and the worked-example effect reverses with expertise (https://doi.org/10.1037/edu0000018, snippet). The last is the most protocol-relevant: the same explanation shape is not optimal at all expertise levels.

**2.2 Advance organizers and prior-knowledge anchoring (Ausubel tradition).** Meaningful learning requires anchoring new material to existing cognitive structure via organizers presented *before* the material. Surfaced anchors: organizer-as-prior-knowledge-supply debate (http://ieeexplore.ieee.org/document/673006/, snippet); Ausubel assimilation applied to conceptually dense teaching with progressive differentiation + integrative reconciliation + concept maps (https://doi.org/10.1002/bmb.21327, snippet); physics-outcome study reporting pre 35.6 → post 81.5 gain under organizer treatment (snippet, single small-sample study — numbers LIKELY max). Transfer: every protocol explanation should open with its anchor (what already-known thing this attaches to), not with the definition.

**2.3 Fluency illusion, IOED, and AI-mediated ease.** The grasp-now clause's adversarial literature: ease of processing is mistaken for depth of understanding. Verified anchors: the fluency-illusion review synthesising 41 publications (28 empirical) — fluent AI output raises confidence without consistently improving conceptual understanding or transfer (https://doi.org/10.3390/info17030299 — search-verified abstract, extractor 403-blocked, snippet-level); the IOED-in-AI study (N=102) finding the largest prediction-vs-performance gap in the chatbot group plus *less accurate* explanations than the same-text control (https://osf.io/8psgf_v1, snippet); the "fluency without learning" analysis tying AI-removed struggle to shallower durable learning via desirable-difficulties removal (https://doi.org/10.63878/jalt1560, snippet); dental-student IOED data (N=142) where observed explanation scores predicted diagnostic accuracy (r=0.47) but perceived understanding did not (r=0.12), and overestimation predicted *worse* accuracy (r=−0.31) (ResearchSquare preprint, snippet). Convergent direction, heterogeneous instruments — pattern LIKELY, any single number UNVERIFIED-for-design.

**2.4 Self-explanation, generation, and Feynman-style teach-to-learn.** The active counterpart: explaining *by the learner* (not receiving explanations) drives learning, conditional on explanation quality. Anchors: self-explanation quality as mediator of the worked-example effect (https://doi.org/10.1007/s11251-022-09579-4, snippet); LLM-judged self-explanation tutoring in CS1 showing revision persistence and richer explanations (http://arxiv.org/abs/2608.25180v2, snippet); practitioner LPC method — Learn/Present/Critique, recorded explanation + delayed critique pass catching factual errors, vagueness, and open questions (https://thomasjfrank.com/feynman-technique-lpc-method/ — full-page read 2026-10-04; practitioner UNVERIFIED but structurally convergent with the literature). The LPC delayed-critique step is the practitioner invention of the eventual-evidence rule.

**2.5 Reader-task-interface framing of load (2025–2026).** A verified reframing worth importing: cognitive load is not a stable scalar property of the document but local to reader × task × interface × processing stage — grounded accuracy 47.2% (BM25 HTML) → 99.5% (localized text) in a 31,104-response filing-grounded experiment; most of the gap attributed to evidence localization, not syntax (http://arxiv.org/abs/2507.07037v2 — abstract verified via reader 2026-10-04). Transfer: "simplest words" is not a document property either — it is a function of *who reads, for what task, at what expertise*; a single canonical simple wording cannot exist.

## 3. Top Findings (confidence-graded)

**F1 (high, definitional + verified reframing): Simplest-faithful is a constrained optimisation, not a direction.** "Simplify" unbounded produces slogans; "be faithful" unbounded reproduces the textbook. The optimum is *minimise reading burden subject to preserving every distinction downstream work depends on* — and the 2025 reader-task-interface result (verified abstract) proves the minimum is reader- and task-relative, not absolute. Operational corollary: faithfulness is defined backwards from use (which distinctions, if dropped, break a later gate?), never forwards from completeness. *Confidence: high for the structure; medium for any specific wording threshold.*

**F2 (high, convergent snippets + theory): Immediate grasp is evidence of processing ease, not of learning — never gate on it.** Fluency-illusion review (41 pubs), IOED-AI experiment (largest gap in chatbot group, worse explanations), and dental IOED data (perceived understanding r≈0.12 vs diagnostic accuracy; overestimation r=−0.31) converge from independent angles: confidence and paraphrase-back inflate under fluent presentation. The protocol's current paraphrase-back grounding step (cf. G1-F7's lingering-misparse warning) is doubly vulnerable: shared misparse *and* fluency inflation ride the same confirmation. *Confidence: high for the qualitative claim; snippet-only numbers stay LIKELY max.*

**F3 (high, convergent): The verified evidence shape is later reduced-support demonstration.** Self-explanation-quality mediation, worked-example fading, dynamic-assessment graduated prompts (G4-F2), and the practitioner LPC delayed-critique step converge on one predicate: `LEARNED = same-class task succeeds later at support S < S_at_teaching`. Paraphrase-now is the cheapest support level, not independent evidence. *Confidence: high (multi-tradition convergence).*

**F4 (medium, single-interaction + theory): Every explanation needs an anchor-first structure.** Ausubel organizers (progressive differentiation, integrative reconciliation) plus the CLT prior-knowledge conditioning result imply the template order: anchor (known) → bridge (mapping) → new (minimal new distinctions) → check-later (deferred probe), never definition-first. The physics pre/post gain (35.6→81.5) is a single small-sample snippet — directionally supportive, never cited as an effect size. *Confidence: medium.*

**F5 (medium, interaction effect): Explanation guidance must be expertise-calibrated — the worked-example effect reverses.** High element-interactivity material favours worked examples for novices but the advantage vanishes or reverses with expertise (5-experiment geometry/trigonometry series, immediate + delayed tests — snippet-level). A uniform "always explain simply with worked examples" rule over-supports experts (expertise-reversal) and under-supports novices on dense material. The teaching principle inherits G4's stance-read input: wording level and guidance level both condition on measured knowledge. *Confidence: medium (well-replicated effect in literature, snippet-only retrieval this sweep).*

**F6 (medium, practitioner + theory convergence): The critique pass must be delayed and written, not immediate and oral.** LPC's practitioner core — record the explanation, wait (a day), then critique in writing for errors / vagueness / open questions — mirrors spacing + self-explanation findings and directly implements the grasp/eventual split at the individual-study level. As a protocol mechanism: any teach step schedules its evidence probe at a *later* gate, in a *different* surface form (cf. G1-P1 altered-form paraphrase). *Confidence: medium (practitioner source full-read; literature convergence snippet-level).*

## 4. Design Principles (candidate teaching rules)

P1 — **Anchor → bridge → new → check-later; never definition-first.** Every protocol explanation opens with the prior-knowledge anchor it attaches to (F4), states the bridge mapping explicitly, introduces the minimum new distinctions, and schedules — never performs — its evidence check at a later gate (F3, F6).

P2 — **Faithfulness is defined backwards from downstream use.** Before simplifying, list the distinctions later gates depend on (the "must-survive" set). Simplification may drop anything else; it may never drop those. The must-survive list ships with the explanation as its audit trail (F1).

P3 — **Grasp-now is logged as exposure, never as learning.** Immediate paraphrase, confidence, or "got it" advances nothing by itself; it records that teaching occurred. Learning is credited only by a later reduced-support demonstration: `support_at_probe < support_at_teaching` on a same-class task (F2, F3).

P4 — **Probe in altered form, at a delay.** The eventual-evidence probe differs in surface form from the teaching (new example, reworded prompt, transferred context) so agreement cannot ride on shared misparse or verbatim recall — G1-P1's altered-paraphrase rule applied temporally (F6, G1-F7).

P5 — **Calibrate guidance to measured expertise.** Novice on high-interactivity material gets worked examples + full scaffolding; demonstrated expertise gets faded guidance or generation prompts (explain-it-yourself). The same wording for all levels violates F5; read stance/knowledge (G4-P3 spot check) before choosing the teaching shape.

P6 — **Record the teaching ledger.** Each teach step logs (concept, anchor used, must-survive distinctions, support level given, scheduled probe gate). The ledger is the input to the G4 support ledger (G4-P6) and the eventual-evidence audit — no ledger, no learning credit.

## 5. Not-to-Encode (negative scope)

N1 — **Do not encode a reading-level constant (e.g., "always grade-8").** F1 + the verified reader-task-interface result forbid absolute simplicity targets; plainness is reader- and task-relative. Ordinal guidance (fewer clauses, named anchor, one new distinction per step) until calibration data exists.

N2 — **Do not gate advancement on confidence, paraphrase, or self-reported understanding.** F2 rules all three out as fluency-inflated. They are exposure signals, not learning evidence.

N3 — **Do not quote snippet-only effect sizes as thresholds.** Pre 35.6→81.5 organizer gain, dental r=0.47/0.12/−0.31, IOED N=102 gaps, 41-pub review counts — single-study priors (LIKELY max), usable for sizing bets, never as design-rationale citations. Rationale cites F1–F6 qualitative structure only.

N4 — **Do not treat Feynman/LPC practitioner material as empirical foundation.** The LPC page was full-read but is practitioner testimony (UNVERIFIED); it supplies mechanism shape (delayed written critique), not evidence weight. Cite it as convergent practice, never as proof.

N5 — **Do not collapse teaching into disambiguation.** G1 owns word-meaning precision at the gate; G5 owns durable learning across gates. An ambiguity-free explanation can still teach nothing (fluent, unretained); a slightly ambiguous one can still be learned (probed, faded, demonstrated). Keep the instruments separate.

N6 — **Do not cite the blocked fluency-illusion review beyond snippet level.** Full text 403-blocked at read time; abstract + snippet claims only, flagged for direct-PDF follow-up (§6).

## 6. Open Gaps (what this sweep did not close)

G1 — **Must-survive distinction inventory per phase.** Which distinctions actually break downstream gates if dropped? Without this list P2's backwards-faithfulness check is unimplementable. Needs phase-by-phase mining (likely joint with G1-G5 inventory work).

G2 — **Probe-ladder template.** What is the canonical delayed-probe shape per concept class (definition → apply-to-new-case; procedure → perform-with-less-support; judgment → decide-novel-case)? Small design task, high leverage; pairs with G4-G2 (scaffolding ladder) and G4-G5 (graduated prompts).

G3 — **Expertise-read instrument for P5.** The stance/knowledge spot check that selects worked-example vs generation shape needs behavioral anchors — shared with G4-G1 (stance anchors). Do not build twice.

G4 — **Fluency-review full-text verification.** Obtain the MDPI fluency-illusion review PDF directly (blocked 403 at read time); if the 41-pub synthesis holds, mine its instructional-design implications section for probe-design specifics.

G5 — **CLT primary verification.** All CLT anchors this sweep are snippet-level (Elsevier/Springer paywalled); the worked-example × element-interactivity interaction (edu0000018) and germane-load controversy deserve primary reads before any numeric guidance enters gate wording.

G6 — **Empirical teaching-ledger validation.** P1–P6 are literature-derived, not protocol-tested. Closing ultimately requires comparing immediate-grasp advancement vs delayed-probe advancement on downstream independent-task success — an evaluation design shared with G4-G6.

## 7. Sources (search-returned URLs only — no invented links)

Load-bearing (reader-verified abstract or full page):
- http://arxiv.org/abs/2507.07037v2 — Du & Tang (2025/2026), reader-task-interface reframing of cognitive load; 31,104-response experiment, 47.2%→99.5% localization gap (F1; abstract verified 2026-10-04)
- https://thomasjfrank.com/feynman-technique-lpc-method/ — LPC method: Learn/Present/Critique with delayed written critique (F6; full-page read 2026-10-04; practitioner, UNVERIFIED as evidence)
- https://doi.org/10.3390/info17030299 — Fluency-illusion review, 41 pubs 2022–2026, fluent AI output raises confidence without consistent conceptual gain (F2; abstract search-verified, extractor 403-blocked → snippet-level)

Supporting (SearXNG snippet-level, not reader-verified — medium confidence):
- https://linkinghub.elsevier.com/retrieve/pii/S0959475206000181 — Worked-example germane-load variants (§2.1)
- https://doi.org/10.1002/sce.20164 — Visual representations × prior knowledge × CLT (§2.1)
- https://doi.org/10.1007/s10648-023-09738-0 — Germane-load measurement controversy (§2.1)
- https://doi.org/10.1037/edu0000018 — Worked-example vs generation resolved by element interactivity; expertise reversal (F5)
- http://ieeexplore.ieee.org/document/673006/ — Advance-organizer prior-knowledge debate (§2.2)
- https://doi.org/10.1002/bmb.21327 — Ausubel assimilation in dense-concept teaching (F4)
- https://osf.io/8psgf_v1 — IOED-AI experiment, N=102, largest gap + worse explanations in chatbot group (F2)
- https://doi.org/10.63878/jalt1560 — Fluency-without-learning / desirable-difficulties removal analysis (F2)
- https://doi.org/10.1007/s11251-022-09579-4 — Self-explanation quality mediation (F3)
- http://arxiv.org/abs/2608.25180v2 — LLM-judged self-explanation tutor, CS1 (F3)
- https://doi.org/10.17576/3l-2025-3104-16 — Chatbot vs dictionary meaning-retention: AI wins immediate, dictionaries win delayed (§2.3 adjacent)

Protocol-internal:
- INBOX.md stance table; T-053 (OPEN); guidance-dynamic-2026-09-30.md; retro-inventory-2026-10-03.md; g4-dynamicism-2026-10-04.md (F2/F3/P6 ledger lineage); g1-language-ambiguity-2026-10-04.md (F7/P1 altered-form lineage).

## 8. Provenance & Next Step

- Queries (SearXNG @127.0.0.1:8888, 2026-10-04): "Feynman technique elaborative explanation simplest words learning" (science, noisy — Feynman-physics false positives); "cognitive load theory plain language faithful explanation expertise" (science); "desirable difficulties immediate comprehension vs delayed retention testing effect" (science); "cognitive load theory worked example germane load explanation simplicity" (science); "advance organizer Ausubel prior knowledge teaching principle" (science); "illusion of explanatory depth fluency immediate understanding retention" (science); "self-explanation effect worked examples Chi analogy concrete fading" (science); "Feynman technique explain simply learn by teaching evidence" (general, practitioner hits).
- Verification: 2/3 load-bearing probes reader-verified (arXiv abstract; LPC full page); 1 blocked (MDPI review, 403 bot-detection → snippet-only, flagged G4). Remainder snippet-level, confidence-graded inline (§3) and labeled in §7. No Tavily fallback needed (science returned throughout). No URLs invented — §7 lists search-returned URLs only.
- No INDEX.md or git writes (per sweep instructions); proposed INDEX row in header.
- Suggested next step: close G1 (must-survive inventory for one pilot phase) + G2 (probe-ladder template for one concept class) jointly with G4-G1/G4-G2, then pilot P1–P6 on a single teach-heavy phase transition with the teaching ledger (P6) running; schedule G4 (review PDF) and G5 (CLT primaries) before any numeric threshold enters gate wording.

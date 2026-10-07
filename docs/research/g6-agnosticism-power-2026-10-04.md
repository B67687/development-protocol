# G6 — Agnosticism-Power Tension: Is Most-Agnostic Most-Powerful, and What Breaks? (2026-10-04)

- Status: first sweep complete — 2026-10-04
- Scope: T-057 remainder. `power-verification-2026-09-30.md` §3/§6 bounded T-057 on both axes (agnostic core + calibration layer; power as model-relative delta) but left the *tension* unmeasured: the exact sense in which more agnosticism buys or costs power, and the enumerated breakage modes when agnosticism is pushed to its limit. This sweep closes that remainder with a narrowed question and fresh retrieval.
- Non-goals: verification placement (T-056 territory); master-student dynamics (T-055 territory); re-arguing POSIX/IFEval ground (cited as prior art, not re-fetched); implementing the calibration layer in gate files.
- Method: SearXNG science + general queries (instance 127.0.0.1:8888) + extractor verification of 4 load-bearing sources (127.0.0.1:8081 POST /extract — all 4 full-verified); DOI/paywalled sources marked snippet-only; confidence levels per claim.
- Proposed INDEX row: `| [G6 agnosticism-power tension](g6-agnosticism-power-2026-10-04.md) | T-057 remainder: most-agnostic is not most-powerful (bounded/inverted-U); four extractor-verified anchors plus portability/sensitivity deltas; agnostic-core-plus-calibration-layer with five breakage modes. | T-057 |`
- Note: per sweep instructions, no INDEX.md or git writes were made; row above is proposal only.

## 1. Problem Statement

The protocol aspires to be model-agnostic — LANDSCAPE declares its search backend agnostic by standing principle; power-verification §3 prescribes minimal rules + one exemplar per gate with per-model calibration quarantined outside the normative core. The unexamined assumption underneath: that pushing agnosticism further keeps (or increases) power — that the most portable text is also the most capable text.

Two failure directions make this assumption suspect:

1. **Least-common-denominator (LCD) tax:** a text that runs everywhere can only invoke the intersection of all models' capabilities. Every frontier-only affordance (long-horizon reasoning, tool-use discipline, fine-grained instruction adherence) left uninvoked is power surrendered for reach. At the limit, the most-agnostic protocol is the one that asks least of its executor.
2. **Silent-breakage portability:** a text that *reads* identically everywhere does not *behave* identically anywhere. Prompt-sensitivity, alignment-post-training differences, and surface-form brittleness mean the same document is effectively N different protocols on N models — agnosticism of text masking variance of behavior.

The narrowed sweep question: **over what range does agnosticism increase power (reach × compliance), where does the curve bend, and what are the enumerated breakage modes past the bend?** Decomposed:

1. **Curve shape:** is more-agnostic monotonically more-powerful, monotonically less, or peaked (inverted-U)? What mechanism sets the peak?
2. **Breakage inventory:** when agnosticism is over-pushed, what exactly fails — capability underuse, behavioral variance, eval illusion, calibration drift, or power misattribution?
3. **Architectural resolution:** what layering lets the protocol keep agnostic reach without paying the full LCD tax?

Proposed answer up front: **the curve is peaked, not monotonic — most-agnostic is not most-powerful. Maximum effective power sits at agnostic core + calibrated periphery; pure-agnostic pays the LCD tax, pure-specific pays portability death. Five breakage modes (§3 F4–F6, §4 P5) mark the bend.**

## 2. Field Landscape

Six literatures jointly cover the construct; none states the agnosticism-power curve in protocol terms.

**2.1 Task-agnostic scaffolding that wins (the agnostic-can-win anchor).** Meta-prompting (http://arxiv.org/abs/2401.12954v1 — extractor full-verified 2026-10-04): a single LM as conductor + expert instances under high-level instructions, zero-shot and task-agnostic, beats standard prompting by 17.1%, expert-dynamic prompting by 17.3%, and multipersona prompting by 15.2% averaged across Game of 24, Checkmate-in-One, and Python Puzzles (GPT-4, +Python interpreter). The agnostic entry wins *within one strong model* — reach across tasks without per-task tuning. Boundary condition matters: single-model result; cross-model travel untested.

**2.2 Portability-loss quantification (the what-breaks anchor).** The Grand Illusion (http://arxiv.org/abs/2309.07181v1 — extractor full-verified 2026-10-04): mainstream ML frameworks lose *more than 40% of key functions* when ported across hardware types, and portable survivors suffer extreme slowdowns rendering performance untenable; hardware-software co-evolution then restricts exploratory research to the mainstream stack. Transfer: portability is not free and not binary — function loss *plus* performance loss, with a second-order lock-in cost (diversity of approaches collapses onto whatever the portable subset supports).

**2.3 No-Free-Lunch formalism (the theory ceiling).** Wolpert's NFL synthesis (http://arxiv.org/abs/2007.10928v1 — extractor full-verified 2026-10-04): under a uniform distribution over induction problems all algorithms perform equally; anti-cross-validation (pick the *worst* out-of-sample performer) matches cross-validation absent a distributional assumption never formalized. Transfer: no universally-optimal protocol text exists without a commitment to a problem/model distribution. Agnosticism *is* the refusal to commit to a distribution — NFL prices that refusal exactly: without distributional assumptions, no agnostic choice beats any other. Every power claim smuggles a distribution (a model class, a task mix); honest agnosticism names it.

**2.4 Prompt/instruction sensitivity (the variance anchor).** Convergent snippet + one verified study: single-prompt evaluation misrepresents score distributions and leaderboard rank is not robust — under adversarial prompt selection *any* model can be promoted to first place (6 embedding models × 11 datasets; http://arxiv.org/abs/2605.22544v2 — extractor full-verified 2026-10-04). Supporting snippets: Webson et al. NAACL 2022 — 30+ NLI templates, irrelevant/misleading prompts learn as fast as "good" ones even at 175B and under instruction tuning (https://doi.org/10.18653/v1/2022.naacl-main.167); underspecification study — underspecified prompts show higher variance and lower relevant-token logits, effect emerging in final layers (http://arxiv.org/abs/2602.04297); jailbreak-variability concept paper — same prompt refused/partially-complied/fully-complied across Llama-2-Chat, Vicuna, Mistral-Instruct, attributed to alignment + refusal-routing rather than architecture alone (https://www.preprints.org/manuscript/202604.1776/v1). Prior art (cited, not re-fetched): POSIX one-exemplar effect, IFEval GPT-4 76.9% vs PaLM 2 S 43.1%, Sclar spurious-surface shifts.

**2.5 Model-agnostic defenses and cross-model attack portability (both directions).** SRD semantic-rollback defense, explicitly model-agnostic (no internal-state access), reports 98.27% average mitigation across open- and closed-source LLMs (https://www.ssrn.com/abstract=6276764, snippet); DAN-jailbreak review notes *cross-model portability of jailbreaks* as a standing risk and prescribes defense-in-depth (https://doi.org/10.20944/preprints202509.0081.v1, snippet). Both directions travel: defenses and attacks are each portable to a degree — agnosticism is not a property of good design only.

**2.6 Abstraction theory + capability envelopes (the mechanism language).** Two-types-of-abstraction analysis: abstractness interacts differently with specificity vs complexity depending on abstraction type (http://arxiv.org/abs/1709.01304v1, snippet) — the vocabulary for saying *which* agnosticism costs what. Capability-ceiling anchor: LongCoT (2,500 long-horizon reasoning problems) — best models <10% (GPT 5.2 9.8%, Gemini 3 Pro 6.1%; https://arxiv.org/abs/2604.14140, snippet). Agnostic text cannot elicit what the executor cannot do; the envelope binds before the wording does. Existence proof the other way: ULD single-hyperparameter agent matches/specialized-beats across 80 environments (https://arxiv.org/abs/2602.12643, snippet) — agnostic configuration *can* win when the representation (value-aligned latents), not the surface text, carries the adaptation.

## 3. Top Findings (confidence-graded)

**F1 (high, verified + theory): The curve is peaked, not monotonic — most-agnostic is not most-powerful.** Meta-prompting proves agnostic scaffolding can beat tuned alternatives (+17.1%, verified abstract) *within* a strong model; NFL proves no choice wins *across* all distributions without distributional commitment (verified abstract); Grand Illusion prices cross-substrate portability at >40% function loss (verified abstract). Joint reading: agnosticism gains reach and loses elicitation; the product peaks in the interior. Pure-agnostic (intersection-only invocations) surrenders frontier power; pure-specific (one model's affordances hardcoded) dies on transfer. *Confidence: high for the qualitative shape; the peak location is protocol- and model-mix-relative, never a constant.*

**F2 (high, verified + prior art): The agnostic core must be exemplar-shaped, not rule-shaped.** POSIX (prior art, power-verification §3: one exemplar beats ten refined rules; scale/instruction-tuning do not reliably reduce sensitivity) plus Webson's verified-snippet corollary (models learn as fast from misleading as from good prompts — wording optimizes weakly, examples anchor strongly) jointly imply: rules are the fragile part of agnostic text, exemplars the robust part. The protocol's §3 prescription (minimal rules + one worked exemplar per gate) is the correct agnostic shape, now with a second independent leg under it. *Confidence: high (convergent verified + peer-reviewed prior).*

**F3 (high, verified): Single-point evaluation cannot see the curve — any power claim without a sensitivity spread is mismeasurement.** The instruction-sensitivity result (verified abstract: default prompt systematically over- or under-states performance; rank not robust; any-model-can-be-first under adversarial selection) forbids bare power claims at the root. It independently re-derives power-verification's rule 8 (report power as model-relative deltas) from the eval side rather than the mechanism side. Operational corollary: every protocol-power number ships with (models × prompt variants) spread or it ships as anecdote. *Confidence: high for the requirement; medium for any specific spread protocol (unbuilt — G2).*

**F4 (medium-high, verified + convergent snippets): Breakage mode 1–2 — envelope overhang and behavioral variance.** (1) *Envelope overhang:* LongCoT ceilings (<10% best-model) bind before wording does; agnostic text pitched above the weakest executor's envelope fails open (IFEval 76.9%→43.1% prior art). The LCD tax is exactly this: intersection-capability invocation. (2) *Behavioral variance:* same text, N behaviors — jailbreak-variability (refuse/partial/comply across three aligned models, snippet), MedGemma sign-flips (CoT −5.7%, few-shot −11.9%, snippet/UNVERIFIED), underspecification variance localized to final layers (snippet). Agnostic text does not produce agnostic behavior; it produces a behavior distribution. *Confidence: medium-high (verified anchors + convergent snippets; MedGemma numbers UNVERIFIED).*

**F5 (medium, snippet + theory): Breakage mode 3–4 — eval illusion and calibration drift.** (3) *Eval illusion:* Webson (misleading prompts ≈ good prompts) + sensitivity-rank fragility jointly mean a protocol can *appear* powerful (judged compliance 0.815 vs rule-checked 0.574, power-verification §2 companion) while eliciting nothing — agnostic wording that flatters the judge rather than constraining the work. (4) *Calibration drift:* post-training (alignment, refusal routing) moves behavior under a fixed text — the jailbreak-variability thesis attributes most cross-model difference to post-training, not architecture. A frozen agnostic core silently de-calibrates as vendors ship new post-training; no text change, behavior change. *Confidence: medium (theory-strong, snippet-level retrieval this sweep).*

**F6 (medium, snippet + verified portability): Breakage mode 5 — lock-in by portable subset (the second-order cost).** Grand Illusion's deepest transfer is not the 40% number but the dynamic: when only the portable subset is usable, research converges onto whatever that subset supports, and non-mainstream approaches become inexpressible. For the protocol: an agnostic-only core that cannot *name* frontier affordances (verification hooks, tool discipline, long-horizon scaffolds) slowly redefines "good process" as "what the weakest supported model can follow." Power-verification's quarantine answer (calibration appendix outside the normative core) is necessary but insufficient — the appendix must be allowed to *lead* (pilot affordances before they are portable), or the core becomes a ceiling. *Confidence: medium (verified source, transferred dynamic).*

**F7 (medium, convergent: ULD + SRD snippets + LANDSCAPE standing rule): The resolution is layered commitment, not a point on the curve.** ULD (one config, 80 envs, matches specialists — representation carries adaptation), SRD (model-agnostic intent layer, 98.27% — operate at the intent level, not the surface level), and LANDSCAPE's standing backend-agnosticism (requirements fixed, engine free) converge on one architecture: fix *invariants* (gates, evidence predicates, verification tiers) agnostically; free *realizations* (wording, scaffolding depth, tool patterns) per model. Agnosticism belongs at the predicate level ("what counts as demonstrated"), never at the utterance level ("exact words that demonstrate it"). *Confidence: medium (convergent snippets + standing protocol rule; no protocol-level A/B exists — power-verification gap G4).*

## 4. Design Principles (candidate agnosticism rules)

P1 — **Fix predicates, free utterances.** The normative core states evidence predicates (what counts as disambiguated, demonstrated, verified) in model-agnostic language; no gate mandates exact surface wording as load-bearing. One worked exemplar per gate shows *a* realization; the predicate, not the exemplar, is the requirement (F2, F7).

P2 — **Report power as named-model deltas with spreads.** No bare power claim enters any document. Format: `protocol + model X gains Y [min–max over prompt variants] on task Z vs baseline B`. Single-point numbers are anecdotes (F3).

P3 — **Envelope-check before elicitation-check.** Every gate that demands a capability (multi-step reasoning, tool use, fine instruction adherence) names the minimum executor envelope it assumes. Below-envelope executors get the degraded-but-honest path (tripwire, not full gate), never a silent pass (F4-mode-1).

P4 — **Re-calibrate on post-training, not on schedule.** The calibration appendix carries a per-model behavior fingerprint (adherence probes); any vendor post-training release re-runs the fingerprint before the protocol claims continued power on that model. Frozen core + drifting post-training = silent de-calibration (F5-mode-4).

P5 — **Let the appendix lead.** Frontier affordances pilot in the per-model calibration layer *before* they are portable; promotion to the agnostic core requires N≥2-model demonstration. An agnostic-only core that cannot name new power becomes a ceiling, then a lock-in (F6).

P6 — **Judge the work, never the words.** Gate checks execute (tests, diffs, artifact predicates) or are rule-checked; model self-attestation and LLM-judged compliance are exposure signals, not evidence — they are the eval-illusion surface (F5-mode-3; power-verification rule 2, re-affirmed).

## 5. Not-to-Encode (negative scope)

N1 — **Do not encode "maximally agnostic" as a virtue.** F1 refutes monotonicity; any gate justified by "runs everywhere" without an elicitation argument is LCD-tax spending without a budget.

N2 — **Do not mandate a single canonical wording as load-bearing.** POSIX + Webson (F2) forbid it: wording optimizes weakly, exemplars anchor. Canonical surface (delimiters/casing per gate, per power-verification rule 3) is hygiene, not a power mechanism — do not cite it as one.

N3 — **Do not quote snippet-only numbers as findings.** SRD 98.27%, LongCoT 9.8%/6.1%, MedGemma −5.7%/−11.9%, ULD 80-env parity — single-study priors (LIKELY max), usable for sizing bets, never as design-rationale citations. Rationale cites F1–F7 qualitative structure only.

N4 — **Do not re-encode power-verification §3–§4 ground.** POSIX/IFEval/Sclar numbers, the three candidate principles, and rules 1–8 stand; this sweep adds the curve shape (F1), the five breakage modes (F4–F6), and P1–P6 layering — deltas only.

N5 — **Do not treat attack/defense portability as symmetric with protocol portability.** Jailbreaks and SRD-style defenses travel at the intent level (F §2.5); protocol gates travel at the compliance level, which is strictly more brittle (sensitivity rank-fragility, F3). Portable attacks do not imply portable process.

N6 — **Do not promote appendix pilots to core on single-model evidence.** P5's N≥2 bar is load-bearing; single-frontier demonstrations (however striking) stay quarantined until a second model confirms the affordance is real and not post-training accident.

## 6. Open Gaps (what this sweep did not close)

G1 — **Peak-location measurement.** The inverted-U is qualitative; where the protocol's current text sits on it is unmeasured. Needs the power-verification gap-G4 experiment: same document × N models × adherence delta, plus prompt-variant spreads (F3 instrument).

G2 — **Sensitivity-spread reporting template.** What is the canonical (models × variants) spread block attached to a power claim? Small design task, high leverage; pairs with G1 harness and power-verification rule 8.

G3 — **Per-model behavior fingerprint.** P4's re-calibration probe set is unbuilt: which adherence probes, how many, what drift threshold triggers re-calibration? Shared with any future calibration-layer work.

G4 — **Envelope taxonomy for gates.** Which gates assume which executor capabilities (P3)? Without the gate-by-gate envelope map, the degraded-but-honest path cannot trigger. Joint with G5-must-survive inventory lineage (must-survive distinctions ≅ must-have capabilities).

G5 — **Appendix-to-core promotion record.** P5 needs a promotion log (affordance, pilot model, confirming model, promotion date) or pilots accumulate without ever landing. Process gap, not research gap.

G6 — **Second-order lock-in monitoring.** F6's dynamic (portable subset redefining good process) needs an observable: e.g., periodic audit of which core gates no frontier model actually needs vs which frontier affordances the core cannot name. Unscoped; flagged, not designed.

## 7. Sources (search-returned URLs only — no invented links)

Load-bearing (extractor-verified abstracts, 2026-10-04):

- http://arxiv.org/abs/2401.12954v1 — Meta-Prompting: task-agnostic scaffolding, +17.1%/+17.3%/+15.2% over standard/expert-dynamic/multipersona (GPT-4; F1, §2.1)
- http://arxiv.org/abs/2309.07181v1 — Grand Illusion: >40% key-function loss cross-hardware + extreme portable slowdowns + stack lock-in (F1, F6, §2.2)
- http://arxiv.org/abs/2007.10928v1 — Wolpert NFL synthesis: uniform-distribution equality; anti-cross-validation equivalence absent distributional commitment (F1, §2.3)
- http://arxiv.org/abs/2605.22544v2 — Instruction Sensitivity: single-prompt misrepresentation; rank not robust; any-model-first under adversarial selection; 6 models × 11 datasets (F3, §2.4)

Supporting (SearXNG snippet-level, not extractor-verified — medium confidence):

- https://www.ssrn.com/abstract=6276764 — SRD model-agnostic semantic-rollback defense, 98.27% avg mitigation (F7, §2.5)
- https://doi.org/10.20944/preprints202509.0081.v1 — DAN-jailbreak review: cross-model portability + defense-in-depth prescription (§2.5)
- https://www.preprints.org/manuscript/202604.1776/v1 — Same-prompt cross-model jailbreak variability; post-training/refusal-routing thesis (F4–F5)
- https://doi.org/10.18653/v1/2022.naacl-main.167 — Webson et al.: irrelevant/misleading prompts ≈ good prompts, 30+ templates to 175B (F2)
- http://arxiv.org/abs/2602.04297 — Prompt-underspecification variance, final-layer emergence (F4)
- http://arxiv.org/abs/1709.01304v1 — Two abstraction types × specificity/complexity (F7 vocabulary, §2.6)
- https://arxiv.org/abs/2604.14140 — LongCoT: <10% best-model long-horizon accuracy, GPT 5.2 9.8% / Gemini 3 Pro 6.1% (F4 envelope)
- http://arxiv.org/abs/2602.12643 — ULD: single config across 80 envs matching specialists (F7)
- https://www.researchsquare.com/article/rs-8759042/v1 — MedGemma prompt-sensitivity: CoT −5.7%, few-shot −11.9% (F4; UNVERIFIED preprint, already so-flagged in power-verification)

Protocol-internal (cited, not re-argued):

- `power-verification-2026-09-30.md` §3/§6 (T-057 BOUNDED verdict; POSIX/IFEval/Sclar ground), §2 (judge inflation 0.815 vs 0.574), rules 1–8
- `retro-inventory-2026-10-03.md` T-057 row + G6 assignment (this sweep's charter)
- `LANDSCAPE.md` search-backend agnostic standing rule; `prompt-engineering-science.md` (instruction-following companion); `harness-survey-2026-07.md` (model × scaffold decomposition)

## 8. Provenance & Next Step

- Queries (SearXNG @127.0.0.1:8888, 2026-10-04): "model-agnostic versus model-specific prompting performance tradeoff large language models" (science); "cross-model prompt transfer portability brittleness jailbreak prompt generalization" (science); "no free lunch generalization specialization tradeoff abstraction generality cost" (science); "AI agent harness universal interface versus model-specific optimization capability elicitation" (general, weak recall — harness-marketing noise, marked thin accordingly); "prompt sensitivity same prompt different models performance variance instruction following" (science); "least common denominator standardization power expressiveness portability tradeoff software abstraction" (science); "reasoning models versus non-reasoning models scaffolding chain-of-thought effectiveness varies capability" (science).
- Verification: 4/4 load-bearing probes extractor full-verified (meta-prompting +17.1% trio; Grand Illusion >40%; NFL anti-cross-validation equivalence; instruction-sensitivity rank fragility). Remainder snippet-level, confidence-graded inline (§3) and labeled in §7. No Tavily fallback needed (science returned throughout). No URLs invented — §7 lists search-returned URLs only.
- No INDEX.md or git writes (per sweep instructions); proposed INDEX row in header.
- Suggested next step: close G1 (same-document × N-model adherence-delta harness with F3 spread template, G2) on one pilot gate, standing up the P4 fingerprint and P5 promotion log as the harness's first two artifacts; schedule G3/G4 (fingerprint probe set + gate envelope map) before any core-text change claims power gains.

# Improving Human Judgement via Protocol — Calibration, Practice & Rate of Gain (2026-10-03)

> How fast does judgement improve with protocol use, and which mechanisms carry the effect: brief debiasing training with feedback, calibration loops, deliberate practice of judgement, decision competence, and checklist/forcing-function scaffolds. Last updated: 2026-10-03. Retro research for THOUGHT_LOG T-068; downstream fold target is the REFLECT learning layer (calibration).

---

## 1. Executive Summary

**Key finding:** judgement improves *fast* under structured training-plus-feedback and *slowly or not at all* under sheer volume. A sub-one-hour debiasing module (CHAMPS KNOW) improved forecasting Brier scores 6–11% over control, consistently across four tournament years **[CONFIRMED]**; a single interactive debiasing session cut six biases by >30% immediately and >20% three months later **[LIKELY]**; meanwhile deliberate-practice hours explain only ~12–26% of skill variance across domains **[LIKELY]**, hours to chess-master status span a 22× range (728–16,120) **[LIKELY]**, and mere prior-tournament experience without structured feedback did not significantly improve accuracy **[CONFIRMED as measured]**. The moderator is feedback quality, not repetition count: genuine intuitive expertise requires frequent, rapid, high-quality feedback about previous judgements **[LIKELY]**. Protocol implication: a REFLECT layer that logs hours or accumulates entries without scored feedback is volume theatre; one that scores predictions, forces comparison classes, and checks calibration will move judgement on a weeks timescale.

| # | Finding | Confidence | Effect size |
|---|---------|-----------|-------------|
| 1 | Brief (<1 hr) debiasing training improves probabilistic judgement; effect replicates across 4 years | CONFIRMED | Brier +6–11% vs control; yr-1 probability-training +10%, scenario-training +11% |
| 2 | Single interactive debiasing session generalises across biases and persists for months | LIKELY | −30% bias immediately, −20% at 3 months (6 biases) |
| 3 | Practice volume explains a minority of skill variance; time-to-mastery varies 22× between individuals | LIKELY | R² ≈ 26% games, 21% music, 18% sports; 728–16,120 hrs to chess master |
| 4 | More information raises confidence without raising accuracy absent feedback (Oskamp); expertise needs rapid high-quality outcome feedback | LIKELY | Confidence 33→53%, accuracy flat <30% (Oskamp); qualitative conditions (Kahneman & Klein) |
| 5 | Procedural scaffolds (checklists, forcing functions, reference classes) cut error rates in the field | LIKELY | Complications 11.0→7.0%, deaths 1.5→0.8%; pooled checklist mortality OR 0.60 |

Method: SearXNG (`categories=science` first-pass, `general` second), Trafilatura sidecar (`127.0.0.1:8081/extract`) full-text for load-bearing claims, direct DOI extraction where available. Confidence per the house scheme: ≥2 independent peer-reviewed sources = CONFIRMED; 1 peer-reviewed + 1 community = LIKELY; preprint/snippet-only = LIKELY max; single/snippet-only = UNVERIFIED. One load-bearing primary (CHAMPS KNOW, JDM open-access) was fully extracted; Sage/Elsevier primaries were snippet- or secondary-only (see §8).

---

## 2. How Fast Judgement Improves: The Numbers

**Rule the evidence supports: expect protocol-driven judgement gains on a weeks-to-months timescale from structured training, not a years timescale from accumulated exposure — and measure Brier/calibration deltas, not hours logged.**

- **Sub-hour training, single-digit-percent Brier gains, four years running.** The CHAMPS KNOW cognitive-debiasing module (comparison classes, averaging, base rates, explicit uncertainty) lasted less than one hour yet improved mean standardized Brier scores 6–11% over control in each of four IARPA-tournament years; year-1 probability-training gained 10%, scenario-training 11% **[CONFIRMED]** (https://doi.org/10.1017/S1930297500004599 — full text extracted). The authors note these are likely *lower-bound* estimates given tutorial brevity and problem heterogeneity.
- **Single session, six biases, months of persistence.** Morewedge et al. (2015) interactive games + instructional video with personalized feedback and practice reduced anchoring, bias blind spot, confirmation bias, fundamental attribution error, projection bias and representativeness by >30% immediately and >20% at three months **[LIKELY — secondary (https://en.wikipedia.org/wiki/Debiasing citing primary https://doi.org/10.1177/2372732215600886); Sage primary unextracted, see §8]**.
- **Elite selection compounds training.** In the same tournament programme, tracking the top 2% into elite teams, plus teaming and training together, produced winners 35–72% more accurate than any rival team, with top forecasters reportedly 30% better than intelligence officers holding classified access **[LIKELY — secondary]** (https://en.wikipedia.org/wiki/Superforecasting citing Mellers et al. 2014, https://doi.org/10.1177/0956797614524255).
- **Mere exposure does not move the needle.** Within the CHAMPS KNOW programme, merely having prior tournament experience did *not* significantly improve Brier scores; what predicted accuracy was active practice volume (forecasts per question) and self-reported use of comparison classes **[CONFIRMED as measured]** (same full-text source). Design implication: REFLECT must distinguish *scored repetitions* from *elapsed participation* — T-068's "improves with use" holds only for use-with-feedback.
- **Caveat (contested).** A 2024 IRT re-analysis argues the Mellers training/teaming effects shrink, vanish or reverse once uncontrolled method variance and strategic responding are modelled **[UNVERIFIED — record/snippet-level only]** (https://doi.org/10.1177/09567976241266481). Do not encode training-effect point estimates as guarantees; encode the *direction with selection-plus-feedback* as the robust claim.

---

## 3. Calibration: The Core Skill and Its Conditions

**Rule the evidence supports: train calibration explicitly (confidence ↔ accuracy mapping), because confidence inflates with information while accuracy does not, and only scored, rapid outcome feedback corrects the mapping.**

- **Three faces, three failure modes.** Overconfidence decomposes into overestimation (of own performance), overplacement (vs others) and overprecision (unwarranted certainty) **[LIKELY]** (https://en.wikipedia.org/wiki/Overconfidence_effect citing Moore & Healy and follow-ups). Protocol calibration should score all three: absolute accuracy, relative rank claims, and interval-hit rates (90% intervals should contain truth ~90% of the time).
- **Oskamp's warning for protocol design.** Clinical psychologists given progressively more case information raised confidence 33%→53% while accuracy stayed flat under 30% **[LIKELY — secondary citing Oskamp 1965]** (same page). Any REFLECT layer that rewards *richer rationales* without scoring *outcomes* will manufacture Oskamp gradients: more confident, no more accurate.
- **The feedback conditions (Kahneman & Klein).** Genuine expert intuition — the kind a protocol wants to build — is acquired only with frequent, rapid, high-quality feedback about the quality of previous judgements; masters of unvalidated knowledge without such feedback are "respect experts," routinely beaten by simple algorithms **[LIKELY — secondary]** (https://en.wikipedia.org/wiki/Overconfidence_effect citing Kahneman & Klein 2009 and Kahneman, Sibony & Sunstein). Design implication: REFLECT needs a scored-prediction ledger (forecast → resolve → Brier/calibration update), not a lessons list.
- **Calibration is trainable, with feedback as the active ingredient.** The calibration literature's state-of-the-art review (Lichtenstein & Fischhoff 1982) and probability-assessor training reports (Alpert & Raiffa 1982) establish that personalized feedback on direction and degree of miscalibration improves calibration **[LIKELY — cited-review-level]** (refs via Overconfidence page). Weather forecasters — the canonical well-calibrated profession — are calibrated precisely because they forecast daily under rapid outcome feedback **[LIKELY — converging field consensus]**.
- **What decay to expect.** Calibration gains without continued scored practice decay; the Morewedge 30%→20% three-month fade is the best quantified decay curve available **[LIKELY]** (§2). REFLECT should therefore be a *standing loop* (score every judgement), not a *course* (train once).

---

## 4. Deliberate Practice of Judgement: What Transfers

**Rule the evidence supports: practice improves judgement only when it is deliberate — designed tasks at the edge of ability with immediate feedback and revision — and even then domain volume explains a minority of variance; what transfers is a small set of statistical habits.**

- **Deliberate ≠ generic practice.** Ericsson's criteria: task designed for improvement, at challenging level, with immediate actionable feedback and repeated revision; mentor feedback on specific weaknesses; mental representations built incrementally. Without feedback and concentration, hours do not compound **[LIKELY]** (https://en.wikipedia.org/wiki/Practice_(learning_method) citing Ericsson, Krampe & Tesch-Römer 1993, https://doi.org/10.1037/0033-295x.100.3.363).
- **The variance caps (Macnamara, Hambrick & Oswald 2014).** Deliberate-practice hours predict ~26% of skill variance in games, ~21% in music, ~18% in sports, ~4% in education and <1% in professions **[LIKELY — extracted community secondary (https://www.6seconds.org/2022/06/20/10000-hour-rule/) converging with primary https://doi.org/10.1177/0956797614531013, unextracted — see §8]**. Encode the *cap*, not just the mean: protocol repetition alone cannot be the improvement engine for professional judgement (the <1% professions figure is the directly relevant one).
- **The 22× range.** Hours to chess-master status span 728–16,120 across individuals (Gobet & Campitelli), and early starters reach higher adult skill at equal practice hours **[LIKELY — same extracted secondary]**. Design implication: never set hours-based mastery gates; set performance-based gates (calibration thresholds, Brier cutoffs).
- **What actually transfers: statistical habits.** Graduate training in statistics-heavy sciences improves domain-general reasoning (Nisbett et al. 1987, Science) **[LIKELY — secondary]** (https://en.wikipedia.org/wiki/Debiasing citing https://doi.org/10.1126/science.3672116); within CHAMPS KNOW only three self-reported principles predicted accuracy — comparison classes (C), quantitative modelling/averaging (M), and effort allocation (S) **[CONFIRMED as measured]** (full-text §2 source). The REFLECT curriculum is therefore short: reference classes, base rates, averaging independent estimates, explicit probabilities, post-mortems.
- **Purposeful vs deliberate (terminology for the protocol).** Solo protocol use without a coach is "purposeful practice" in Ericsson's taxonomy (all criteria except expert guidance). The protocol itself must play coach: worked examples, immediate scoring, targeted drills on the weakest calibration face (§3).

---

## 5. Decision Competence and Its Correlates

**Rule the evidence supports: individual decision competence is real, measurable, and predicts life outcomes — but its slow-moving correlates (age, cognitive ability, starting age, genetics) are humility priors, not protocol levers; the levers are the trainable habits in §2–§4.**

- **A-DMC predicts real outcomes.** Lower decision bias (Adult Decision-Making Competence battery: resistance to framing, calibration, applying decision rules, consistency in risk perception) is associated with more intact social environments, less substance use, lower delinquency, and superior planning/problem-solving **[LIKELY — secondary]** (https://en.wikipedia.org/wiki/Debiasing citing Parker & Fischhoff 2005, https://doi.org/10.1002/bdm.481).
- **Slow correlates bound the rate claim.** Cognitive ability contributed to forecasting accuracy independently of training and practice in the tournament data **[CONFIRMED]** (CHAMPS KNOW full text); early domain entry raises adult ceilings at equal practice **[LIKELY]** (§4); twin studies attribute large shares of basic ability variance to genetics (e.g. ~38% of measured musical abilities, Mosing; >50% of reading-skill variance, Plomin cohort) **[LIKELY — extracted secondary]** (6seconds page). None of these are actionable inside a protocol, but together they forbid promising uniform gains: encode *distribution shift*, never *mastery for all*.
- **The humility prior for T-068.** If practice volume explains <1% of professional-skill variance and ability/entry/genetics explain large shares, then "judgement improves with use" must be scoped: *calibrated, feedback-rich use improves the trainable margin (roughly single-digit Brier percent per training cycle), on top of a stable individual baseline*. That scoping is itself a calibration exercise — the protocol should state its own expected effect size (§8 implementation notes).

---

## 6. Protocol Mechanisms That Carry the Effect

**Rule the evidence supports: the portable, encodable mechanisms are comparison classes, explicit quantification, averaging, consider-the-opposite, checklists/forcing functions, and scored post-mortems — each with independent support; verbose reflection without these is unproven.**

- **Reference-class forecasting (outside view).** Systematically debiases estimates by replacing the inside view with distributional base rates; mandated for large UK/Danish infrastructure projects precisely to kill optimism bias **[LIKELY — secondary + practitioner mandate]** (https://en.wikipedia.org/wiki/Debiasing citing Kahneman, Thinking, Fast and Slow). Protocol encoding: every estimate ships with a comparison class or it is not an estimate.
- **Explicit quantification.** Coarsened/verbal uncertainty expressions sacrifice predictive accuracy at scale (n = 888,328 forecasts); supplementing natural language with numeric probabilities is empirically justified even for geopolitics **[LIKELY — abstract-level]** (https://doi.org/10.1093/isq/sqx078). Protocol encoding: confidence labels (CONFIRMED/LIKELY/UNVERIFIED) plus numbers where resolvable.
- **Consider-the-alternative.** "Consider the opposite" prompts measurably mitigate anchoring (survey experiment, N = 1,221 public managers) **[LIKELY — abstract-level]** (https://doi.org/10.1111/puar.13211, shared with thought-log sweep); teaching consider-an-alternative strategies is a listed effective training approach (Hirt & Markman 1995) **[LIKELY — cited-level]**. Protocol encoding: every synthesis carries one strongest-case-against (already proposed for thought-log SYNTHESIS; reuse the template).
- **Checklists and forcing functions.** WHO Surgical Safety Checklist adoption: complications 11.0%→7.0% (p<0.001), deaths 1.5%→0.8% (p=0.003) across 3,733/3,955 patients in eight hospitals; pooled emergency-laparotomy mortality OR 0.60 with checklist **[LIKELY — extracted secondary (https://en.wikipedia.org/wiki/WHO_Surgical_Safety_Checklist) citing Haynes et al., NEJM]**; Croskerry's debiasing reviews add the mechanism: decouple Type-1 intuition into Type-2 verification, preferably via forcing functions that cannot be skipped **[LIKELY — abstract-level]** (https://doi.org/10.1136/bmjqs-2012-001712, https://doi.org/10.1136/bmjqs-2012-001713). Protocol encoding: machine-checked gates (lint-style), never advisory checklists — consistent with the experimenter-led > document-only moderator (g 0.465 vs 0.277) from the thought-log sweep.
- **Post-mortems, scored.** CHAMPS KNOW's P principle (post-mortem conduct) correlated with *worse* concurrent Brier — because post-mortems follow failures — yet the programme's scored-prediction ledger was the substrate everything else learned from **[CONFIRMED as measured]**. Protocol encoding: post-mortems update base rates and checklists; they are scored by whether the *next* same-class judgement improves, not by whether the write-up is eloquent.
- **Reused prior (do not re-encode):** if-then implementation intentions raise goal attainment g ≈ 0.31–0.34 **[CONFIRMED]** (thought-log sweep §3). The REFLECT layer should use if-then form for its own habits ("if estimate made, then comparison class recorded").

---

## 7. Anti-Patterns

| # | Anti-pattern | What it looks like | Why the evidence forbids it |
|---|--------------|-------------------|----------------------------|
| 1 | **Hours-as-growth** | Mastery gates in hours; "10,000-hour" rhetoric; logging time-on-task as improvement | Practice volume explains 4–26% of variance (<1% in professions); 22× individual range **[LIKELY]** (§4) |
| 2 | **Rationale-rich, score-poor reflection** | Long lessons-learned with no resolved predictions, no Brier/calibration update | Oskamp gradient: confidence 33→53%, accuracy flat <30% **[LIKELY]** (§3) |
| 3 | **One-shot training** | Single debiasing workshop, no standing loop | 30%→20% three-month fade; calibration needs continuous scored practice **[LIKELY]** (§2–§3) |
| 4 | **Exposure-as-practice** | "You've done N sessions, so you're improving"; seniority-as-calibration | Mere prior-tournament experience did not improve Brier **[CONFIRMED]** (§2) |
| 5 | **Advisory checklists** | Guidance text nobody enforces; document-only disposition | Experimenter-led (0.465) > document-only (0.277); forcing functions beat advice **[CONFIRMED/LIKELY]** (§6) |
| 6 | **Uniform-gain promises** | Same expected improvement for every user | Ability/entry/genetics bound individual ceilings; encode distribution shift **[LIKELY]** (§5) |
| 7 | **Eloquent post-mortems** | Judging reflection quality by write-up depth instead of next-judgement accuracy | P-principle paradox: post-mortems cluster on failures; only scored follow-through counts **[CONFIRMED]** (§6) |

---

## 8. Confidence Ledger, Gaps & Implementation Notes

**Confidence ledger (every load-bearing claim, one row each):**

| Claim | Label | Basis |
|-------|-------|-------|
| CHAMPS KNOW <1 hr → Brier +6–11% vs control, 4 yrs; yr-1 +10%/+11% | CONFIRMED | Fully extracted peer-reviewed full text (https://doi.org/10.1017/S1930297500004599) |
| Mere prior experience n.s.; forecasts-per-question + comparison-class use predict accuracy | CONFIRMED | Same full-text source, measured |
| C/M/S principles predict accuracy; P/O associate with worse concurrent Brier | CONFIRMED | Same full-text source, measured |
| Morewedge single session −30% now, −20% at 3 mo (6 biases) | LIKELY | Extracted community secondary (Wikipedia Debiasing) citing paywalled primary (https://doi.org/10.1177/2372732215600886) |
| GJP 35–72% over rivals; top forecasters 30% over intel officers | LIKELY | Extracted community secondary citing Mellers et al. 2014 (https://doi.org/10.1177/0956797614524255) |
| Practice variance caps 26/21/18%; chess-master range 728–16,120 hrs | LIKELY | Extracted community secondary (6seconds) converging with unextracted primary (https://doi.org/10.1177/0956797614531013) |
| Oskamp 33→53% confidence, flat <30% accuracy | LIKELY | Extracted community secondary citing Oskamp 1965 |
| Kahneman & Klein rapid-high-quality-feedback conditions; "respect experts" | LIKELY | Extracted community secondary citing 2009 review |
| Lichtenstein & Fischhoff 1982 review; Alpert & Raiffa assessor training | LIKELY | Cited-review-level via Overconfidence page refs |
| Nisbett et al. 1987 stats-training transfer | LIKELY | Cited-level via Debiasing page |
| A-DMC → life outcomes (Parker & Fischhoff 2005) | LIKELY | Cited-level via Debiasing page ref [3] |
| Checklist 11.0→7.0% / 1.5→0.8%; pooled OR 0.60 | LIKELY | Extracted community secondary (WHO checklist wiki) citing Haynes et al. NEJM |
| Croskerry Type-1→Type-2 decoupling; forcing functions | LIKELY | 2 peer-reviewed abstracts (SearXNG science) + citation inside CHAMPS KNOW full text |
| Numeric > verbal probability expressions (n = 888,328) | LIKELY | Abstract-level (https://doi.org/10.1093/isq/sqx078) |
| Consider-the-opposite debiases anchoring (N = 1,221) | LIKELY | Abstract-level (https://doi.org/10.1111/puar.13211) |
| IRT critique shrinking Mellers training/teaming effects | UNVERIFIED | Record/snippet-level only (https://doi.org/10.1177/09567976241266481) |
| Genetics/entry-age shares of ability variance | LIKELY | Extracted community secondary; direction-convergent with primary literature |
| Local T-068 / REFLECT-layer observations | LOCAL-CONFIRMED | Retro brief assertion, not literature |

**Open gaps (do not encode beyond these):**

1. **Morewedge 2015 primary** — Sage paywall/502 class; 30%/20% figures rest on Wikipedia's reporting. Re-verify via institutional access before hardening any REFLECT persistence claim.
2. **Macnamara 2014 primary** — Sage paywall; 26/21/18% rest on 6seconds explainer converging with the paper's widely cited figures. The <1%-professions figure especially needs primary verification before citing against professional-judgement training.
3. **Haynes/NEJM primary** — 11.0→7.0 / 1.5→0.8 rest on WHO-checklist Wikipedia; full-text verification (case-mix, Hawthorne contribution) needed before using as a quantified protocol target.
4. **No direct literature on protocol-mediated judgement improvement rate** — CHAMPS KNOW is the closest analogue (tutorial + tournament platform) but is not a standing development protocol; the weeks-timescale claim for REFLECT is *derived design*, not science.
5. **Mamede structured-reflection RCTs** — not retrieved (name-collision search failure); Croskerry reviews cover the mechanism at abstract level only. Re-search via PubMed/Medline before encoding clinical-reflection specifics.
6. **IRT critique unresolved** — if the 2024 re-analysis holds, training/teaming point estimates overstate; the REFLECT design should prefer within-subject scored pre/post (which the ledger provides) over between-condition claims.
7. **Calibration-decay curve** — only the Morewedge 30→20 fade is quantified; no retrieved curve for Brier/calibration decay specifically. REFLECT cadence (how often to re-score) is therefore a guess with a reason, not a parameter from literature.

**Implementation notes (for the REFLECT gateporter — derived, not encoded here):** build a scored-prediction ledger (estimate → comparison class → numeric probability → resolution → Brier/calibration delta) as the REFLECT substrate; gate advancement on performance (calibration thresholds), never hours or session counts; encode the five mechanisms in §6 as machine-checked gates (park-without-trigger and fold-without-backlink analogues: estimate-without-comparison-class, judgement-without-numeric-probability); state the protocol's own expected effect size beside every gain claim (single-digit Brier percent per training cycle, fading without practice); reuse the thought-log SYNTHESIS template's strongest-case-against slot for consider-the-opposite.

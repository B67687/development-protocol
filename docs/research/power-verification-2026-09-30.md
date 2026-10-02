# Power, Verification, and Agnosticism — Evidence Base for T-055–T-057

**Date:** 2026-09-30
**Why this file exists:** T-055 asks whether the protocol is too powerful to hand out raw (baby holds gun) and whether it should behave as a dynamic master-student; T-056 asks where verification belongs (every round vs specs/everywhere); T-057 asks how model-agnostic and how powerful the protocol can be. This is the durable reference so the sweep is never redone. Companion to `prompt-engineering-science.md` (instruction-following) and `harness-survey-2026-07.md` (scaffold-vs-model attribution).
**Method:** authoritative-search discipline — SearXNG (`127.0.0.1:8888`, `categories=science`) for academic recall, ACL Anthology page fetch for the load-bearing POSIX claim, snippet-level evidence otherwise. Confidence labels are per claim (CONFIRMED / LIKELY / UNVERIFIED / CONFLICTED / IN-HOUSE ONLY). Nothing below restates in-house trajectory results as external evidence.

## 1. Tool vs agent framing; scaffolding safety (T-055, baby-with-gun)

**Guidance helps novices, harms experts — CONFIRMED by meta-analysis.** 2025 meta-analysis, Learning and Instruction (https://doi.org/10.1016/j.learninstruc.2025.102142), PRISMA, 60 experimental studies, 5,924 participants, metafor with dependency handling: low-prior-knowledge learners gain from high assistance (**d = 0.505**); high-prior-knowledge learners do better with low assistance (**d = −0.428**). Effect is asymmetric — helping novices matters more than unburdening experts — and is moderated by prior-knowledge measure, educational status, and domain.
→ A single fixed strictness cannot serve both ends. This is the external corroboration our in-house expertise-reversal claim previously lacked.

**Minimal guidance fails novices specifically — CONFIRMED (foundational).** Kirschner, Sweller & Clark, "Why Minimal Guidance During Instruction Does Not Work," Educational Psychologist 2006 (https://doi.org/10.1207/s15326985ep4102_1): half-century of evidence that unguided discovery loses to guided instruction; the guidance advantage recedes only once learners hold enough prior knowledge to supply "internal" guidance.
→ The baby-with-gun worry is empirically backwards in one respect: withholding the protocol from novices is the harmful direction. The danger is not power itself but *unfaded* power.

**Computer scaffolding works at small-to-moderate size — LIKELY (snippet-level, peer-reviewed).** Bayesian meta-analysis of computer-based scaffolding in STEM PBL (https://doi.org/10.1007/s10648-017-9419-1): significant effect on cognitive outcomes, **g = 0.385**, varying small-to-moderate across six moderators.
→ Expect scaffolding gains, not miracles; budget protocol strictness the way the meta-analytic mean suggests — real but bounded.

**Scaffolding is defined by contingency, fading, and transfer — LIKELY (canonical review).** Van de Pol, Volman & Beishuizen, "Scaffolding in Teacher–Student Interaction: A Decade of Research," Ed Psych Review 2010 (https://doi.org/10.1007/s10648-010-9127-6): contingency (calibrate to the learner), fading (withdraw as competence rises), transfer of responsibility. Effectiveness studies available were few but positive; measurement is the field's main challenge.
→ "Master-student" is not a metaphor to decorate — it is a three-part mechanism. A protocol that never fades is by definition not scaffolding; it is a crutch.
**Automation bias and complacency bound the upside of powerful defaults — LIKELY (canonical, unfetched).** Parasuraman & Manzey, 'Complacency and Bias in Human Use of Automation,' Human Factors 2010: operators over-rely on authoritative automation, missing its failures precisely when it is most trusted. Lee & See, 'Trust in Automation,' Human Factors 2004: appropriate reliance requires calibrated — not maximal — trust.
→ A 'powerful' protocol that always sounds certain breeds the failure mode it was built to prevent. Design for calibrated trust: show workings, invite override, log dissent. Power without contestability is the gun without a safety.


**Cognitive apprenticeship gives the phase order — LIKELY (canonical framework).** Collins, Brown & Newman: modelling → coaching → scaffolding → fading, with articulation and reflection throughout (https://www.taylorfrancis.com/books/9781135434984/chapters/10.4324/9781315044408-14; DTIC tech report https://apps.dtic.mil/sti/citations/tr/ADA203609).
→ Map protocol strictness onto this sequence: demonstrate early (worked examples), coach mid-chain (gates with feedback), fade late (light tripwires, self-check). "Grow with ability" (T-048/049) is this sequence under another name.

## 2. Verification economics: where checks pay (T-056, every round vs specs)

**Earlier-caught defects cost less; exact multipliers are contested — direction CONFIRMED, numbers CONFLICTED.** The Boehm cost-of-change curve and IBM Systems Sciences Institute ratios (often quoted 1:10:100 across requirements→test→field) are textbook consensus on direction; primary-source multipliers vary by study and era, and modern iterative data weakens the steepest claims. No primary source was fetched in this sweep.
→ Encode the direction (verify upstream), never a specific multiplier. Anyone citing "100×" as fact is laundering a slogan.

**Formal upstream reviews remove more defects per hour than testing downstream — LIKELY (textbook consensus, unfetched primary).** Jones's defect-removal-efficiency figures (formal requirements/design inspections ≈ 85%+ DRE; testing phases individually lower) and Fagan-inspection literature agree on ranking if not on exact points.
→ Verification belongs where the defect is born: at the spec and gate artifacts, not sprayed uniformly across rounds.

**Modern review catches mostly maintainability, not function — LIKELY (snippet-level, peer-reviewed).** Beller et al., MSR 2014 (https://doi.org/10.1145/2597073.2597082): ~1,400 reviewed changes across OSS projects; the change mix mirrors industry/academic inspection literature at a **75:25 maintainability-to-functional ratio**; reviewer identity had no impact on change count.
→ Per-round review effort buys polish, not correctness. Spend correctness budget on gate checks with teeth (compilation, tests, spec-conformance), not on re-reading every diff.

**Spec-language defects propagate into downstream artifacts — LIKELY (snippet-level, controlled experiment).** Femmer et al., Empirical Software Engineering 2024 (https://doi.org/10.1007/s10664-024-10582-1): 25 industry/university participants building domain models from requirements; Bayesian + frequentist analysis; ambiguous pronouns show a strong effect (incorrect associations), passive voice only minor — though both are equally condemned by style guides.
→ Verify the spec artifact differentially: ambiguity checks earn their keep, style checks do not. This discriminates "verify specs" from "verify everything."

**Model judges inflate compliance — CONFIRMED (companion).** `prompt-engineering-science.md` §1: LLM judges overestimate instruction compliance (5 instructions: 0.815 judged vs 0.574 rule-checked).
→ Verification done by the same model that did the work is theatre. Gate checks must be rule-checked or independently executed (tests run, artifacts diffed), never self-graded.
**Bugs found late cost the industry tens of billions; early detection is the addressable share — LIKELY (canonical report, unfetched).** NIST 'The Economic Impacts of Inadequate Infrastructure for Software Testing' (2002): inadequate testing costs the US ~$59.5B/year, with roughly a third addressable by earlier detection. Widely cited; methodology debated, direction undisputed.
→ The economic case for upstream verification is macro-scale, but the claim to encode is the share-shift (move detection earlier), not the dollar figure.

**Review-everywhere has a measured price: delay and confusion — LIKELY (snippet-level, peer-reviewed).** Ebert et al., 'An exploratory study on confusion in code reviews,' Empirical Software Engineering 2020 (https://doi.org/10.1007/s10664-020-09909-5): framework of 30 confusion reasons / 14 impacts / 13 coping strategies; reviews delay merges, and confusion amplifies the cost.
→ Uniform per-round verification taxes every step with latency and attention. Price verification like any intervention: concentrate it where §2 says defects are born.


## 3. Protocol agnosticism limits (T-057, how agnostic, how powerful)

**Bigger or instruction-tuned ≠ less prompt-sensitive; one exemplar fixes more than scale — CONFIRMED (fetched, peer-reviewed).** Chatterjee et al., POSIX, Findings of EMNLP 2024 (https://aclanthology.org/2024.findings-emnlp.852/, pp. 14550–14565, abstract verified by fetch): increasing parameter count or instruction tuning does **not** necessarily reduce prompt sensitivity, while adding even one few-shot exemplar almost always significantly decreases it; template alterations hurt most on MCQ-style tasks, paraphrasing on open-ended generation.
→ No protocol text is model-invariant. Agnosticism is bounded: one worked exemplar per gate travels across models better than ten refined rules.

**Instruction-following varies widely across models — CONFIRMED (companion).** `prompt-engineering-science.md` §5: IFEval baselines (Zhou et al., arXiv:2311.07911) — GPT-4 strict prompt-level 76.9% vs PaLM 2 S 43.1% on identical instructions.
→ "Powerful" is model-relative. A gate that holds on a frontier model fails open on a weak one; calibrate gate strength to the executing model, not to the document.

**Prompt techniques validated on one model class need not transfer — UNVERIFIED (single preprint, domain-specific).** MedGemma prompt-sensitivity preprint (ResearchSquare rs-8759042): CoT −5.7%, few-shot −11.9% on medical QA — same direction as our companion's CoT-harm finding but a different setting and unreviewed.
→ Treat every cross-model claim as guilty until A/B-tested on the target model. Agnostic core, model-specific calibration layer.
**Spurious prompt features move outputs without changing intent — UNVERIFIED (known paper, not fetched this sweep).** Sclar et al., 'Quantifying Language Models' Sensitivity to Spurious Features in Prompt Design,' arXiv:2309.04243: formatting, casing, and separator choices shift accuracy several points with zero intent change.
→ Agnosticism must cover surface form, not just wording. Pin one canonical surface (delimiters, casing, template) per gate exemplar; do not let every phase invent its own.

**Scaffold effects are model-relative, not absolute — LIKELY (in-repo companion).** `harness-survey-2026-07.md`: measured agent performance decomposes into model × scaffold, and which harness mechanism carries the effect varies by model.
→ 'How powerful can the protocol be' has no single answer: power = protocol × model × task. Report protocol gains as deltas against a named model baseline, never as absolute capability.


## 4. Three candidate principles (and what not to encode)

1. **Fade with demonstrated competence.** Contingent strictness per phase (calibrate to the user/model at hand), explicit fading schedule (strict → coached → tripwire), transfer test before withdrawal. Traced to §1 (d = 0.505 / −0.428 asymmetry, contingency-fading-transfer, apprenticeship sequence).
2. **Verify at birth, spot-check elsewhere.** Heavy rule-checked gates on spec and phase-transition artifacts (ambiguity, conformance, executable checks); lightweight sampling within rounds; never model-self-graded. Traced to §2 (75:25 review mix, spec-defect propagation, judge inflation).
3. **Exemplars over rules for cross-model travel.** Every gate carries one worked exemplar; rule text stays minimal and model-agnostic; per-model calibration lives in a separate layer, not in the core document. Traced to §3 (POSIX one-exemplar effect, IFEval variance).

**Not to encode:**
- fixed multipliers ('100× cheaper upstream') — direction is consensus, numbers are folklore;
- uniform every-round verification checklists — buys polish at latency price (75:25);
- bare-negation prohibitions as load-bearing gates (companion §1);
- formatting mandates justified by adherence claims (companion §3: GAP);
- any strictness level presented as fitting all users and models (refuted by §1 asymmetry);
- model-self-grading as a gate (judge inflation 0.815 vs 0.574).

**Operationalizing the three (what changes in the repo):**
- (a) a capability signal per user/model (prior-artifact quality, override rate) gating strict vs coached vs tripwire variants of each phase;
- (b) gate files that execute checks (lint, tests, artifact diffs, ambiguity spot-checks) instead of asking the model to self-attest;
- (c) one worked exemplar per gate file, with model-specific notes quarantined in an appendix layer outside the normative core.
- (d) a dissent log: every override of a powerful default is recorded, feeding the fading signal in (a).

## 5. Open gaps

1. No fetched primary for Boehm/Jones multipliers or inspection-vs-testing DRE ranking — direction is consensus, numbers are folklore until sourced.
2. No study of verification *placement* inside LLM-agent trajectories (gate-only vs every-step checking) — §2 is transferred from human software engineering, not measured on agents.
3. Fading schedules for agent protocols (when exactly to withdraw strictness) have no empirical schedule — education gives the mechanism, not the timetable.
4. Cross-model protocol portability (same document, N models, adherence delta) is unmeasured — POSIX measures prompt variants within models, not protocol documents across them.
5. Tool-vs-agent framing effects (does calling the protocol a "tool" vs a "master" change compliance?) — no study found; likely unmeasurable as posed.
6. Snippet-level claims (§1 scaffolding g, §2 review mix, spec-defect experiment, expertise-reversal abstract) await full-text verification; SearXNG science recall was strong on abstracts, weak on accessible full text.
7. POSIX and Sclar measure prompt-level sensitivity, not multi-file protocol documents — the transfer to our artifact class is reasoned, not measured.
8. Automation-bias and trust-calibration claims (§1) are transferred from human-automation interaction; no agent-protocol study confirms operators over-rely on protocol text specifically.
9. NIST/Jones/Boehm figures predate LLM-assisted development; whether AI-written code shifts the cost curve (cheaper late fixes? more spec defects?) is unstudied in the fetched set.

## 6. What this licenses for T-055–T-057

**T-055 (baby with gun → master-student): LICENSED with conditions.** Withholding power from novices is the empirically harmful direction (Kirschner 2006); uniform power is the harmful direction for experts (d = −0.428). The resolution is not less protocol but *contingent* protocol: strict by default for the unproven, fading on demonstrated competence, contestable at every level (automation-bias guard). Static strictness is a crutch; static laxity is abandonment; only fading counts as scaffolding.

**T-056 (where verification belongs): LICENSED as gate-heavy, round-light.** Verify where defects are born (specs, phase transitions) with rule-checked executable gates; sample within rounds; never self-grade. Uniform every-round checking buys 75%-maintainability polish at full latency price. One exception: after any compaction or context handoff, re-verify the active constraints (companion: assume fidelity is lost).

**T-057 (how agnostic, how powerful): BOUNDED on both axes.** Agnostic in core text (minimal rules + one exemplar per gate travel best), calibrated per model outside the core (IFEval spreads and POSIX sensitivity forbid single-text-fits-all). Powerful only as a delta on a named model baseline — report 'protocol + model X gains Y on task Z,' never bare power claims.

## 7. Actionable rules

1. **Default strict, fade on signal.** New user/model starts at full strictness; graduate phases to coached then tripwire on artifact quality and override rate. (§1: d = 0.505 / −0.428.)
2. **Gate checks must execute, not attest.** Compilation, tests, artifact diffs, ambiguity spot-checks — never 'confirm you followed the rules.' (§2: judge inflation, 75:25 mix.)
3. **One exemplar per gate, canonical surface.** Worked example beats ten refined rules across models; pin delimiters/casing per gate. (§3: POSIX.)
4. **Check ambiguity, skip style.** Spec verification discriminates: ambiguous references and untestable criteria are gates; voice, formatting, and polish are not. (§2: Femmer differential effect.)
5. **Show workings, invite override.** Every powerful default ships with its rationale visible and dissent logged — calibrated trust over maximal trust. (§1: automation bias.)
6. **Re-verify after compaction/handoff.** Active constraints are re-injected and re-checked at every context discontinuity. (Companion rot/drift.)
7. **A/B every strictness change against no-template.** No imposed template enters the core without beating its absence on the target model. (Companion §2 + §1 asymmetry.)
8. **Report power as model-relative deltas.** No claim of protocol power without the named model and baseline beside it. (§3: model-relativity.)

## 8. Search log (so this is never redone blind)

- SearXNG `categories=science`, 6 queries: scaffolding/fading meta-analysis; defect-cost escalation (weak recall); expertise reversal (hit: 2025 meta-analysis); code-review effectiveness (hit: Beller 75:25; Ebert confusion); Fagan/DRE/Jones (weak recall, textbook consensus used); prompt-sensitivity LLMs (hit: POSIX).
- Fetched full page: ACL Anthology POSIX entry (abstract verified, Findings EMNLP 2024, pp. 14550–14565).
- Snippet-level (abstract/secondary) evidence: Belland scaffolding g = 0.385; expertise-reversal d = 0.505 / −0.428; Beller 75:25; Ebert confusion framework; Femmer requirements-defect experiment.
- Canonical-unfetched (labelled LIKELY, not CONFIRMED): Kirschner minimal-guidance; van de Pol fading triad; Collins apprenticeship; Parasuraman automation bias; Lee & See trust; NIST testing economics; Jones DRE; Sclar spurious features.
- Excluded: crowdsourcing-waterfall case study (irrelevant to verification placement); PCB/pavement defect-detection papers (wrong 'defect' sense); trauma-bleeding guideline (search noise); MedGemma preprint (admitted as UNVERIFIED only).
- Tavily was not used (no quota pressure this sweep); extractor sidecar was bypassed in favour of direct ACL fetch, which succeeded.

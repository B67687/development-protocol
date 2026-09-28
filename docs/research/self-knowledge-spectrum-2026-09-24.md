# The Self-Knowledge Spectrum — Evidence Base for the Protocol's User Model

**Date:** 2026-09-24
**Why this file exists:** the protocol already measures what a user brings to a run (stance read, expertise calibration, stated/revealed/tacit preference layers, bouncing protocol, assumption ledger). All of those measure the *content* of the user's head, not how well the user knows their own head — and the method assumes the user can (a) put the want into words and (b) recognise a true statement when they hear it. This sweep gathers the evidence on that assumption and on what reliably extracts a usable signal from someone with low self-knowledge. Companion to `human-thinking-preferences-2026-09-24.md`.
**Method:** three parallel subagent sweeps via SearXNG (`127.0.0.1:8888`, `categories=science`) plus Trafilatura extractor sidecar (`127.0.0.1:8081/extract`). SearXNG rate-limited late in two sweeps (all engines suspended on exact-title queries); affected items are marked UNVERIFIED/unreachable, not asserted. Confidence labels are per claim: CONFIRMED (2+ independent authoritative sources agree), LIKELY (1 authoritative), UNVERIFIED (otherwise). No URL is invented; every source below was live-retrieved.

## 1. The limits of introspection and self-report

**No direct access to mental processes; reports are confabulated from causal theories — CONFIRMED.** Nisbett & Wilson (1977, Psych Rev 84:231–259, https://doi.apa.org/doi/10.1037/0033-295X.84.3.231): little or no introspective access to higher-order cognitive processes; verbal reports on why/how a choice was made rest on a priori implicit causal theories (plausibility, salience), not observed processes. Corroborated by Berger et al. (2016) revisit (https://doi.org/10.1521/soco.2016.34.3.167 — introspections about processing speed are limited and distorted by dynamics) and the extracted overview (https://en.wikipedia.org/wiki/Introspection_illusion — adaptive-unconscious update, Wilson 2002; unaware of being unaware).
→ Treat every "why I want this" as a theory about the want, not an observation of it. Weight the *choice* over the *story*.

**Choice blindness: people justify choices they never made — LIKELY for the phenomenon, UNVERIFIED for exact rates.** Johansson, Hall, Sikström & Olsson (2005, Science 310:116–119, via https://philpapers.org/rec/MOOHSC): in manipulated face/taste/moral-attitude trials the large majority fail to detect the swap and offer introspectively derived reasons for the non-chosen option; manipulated vs non-manipulated reports are nearly indistinguishable by word-frequency/LSA. Robustness argued in Bortolotti & Sullivan-Bissett (2019, https://doi.org/10.1007/s11229-019-02414-3) and Ganapini (2019, https://doi.org/10.1007/s11245-018-09629-y); consumer generalisation in Cheung et al. (2015, https://doi.org/10.1016/j.appet.2015.09.022). Exact detection-failure percentages were NOT retrieved (SearXNG rate-limited on exact-title queries) — do not cite a number.
→ A user nodding along to a restated want is weak evidence. The bouncing protocol's value is in the *friction*, not the assent.

**Better-than-average: most rate themselves above the median — CONFIRMED for phenomenon and moderators, UNVERIFIED for meta-analytic size.** Extracted overview (https://en.wikipedia.org/wiki/Illusory_superiority): egocentrism, focalism, selective recruitment; Kruger (1999) — above-median on easy tasks, below-median on difficult tasks regardless of ability; weaker/absent in East Asian samples. Zell et al. (2020) comprehensive meta-analysis query returned no usable record (unreachable) — no d cited.
→ Self-estimates of capability and of "how clear I am about what I want" skew high on easy-feeling tasks. Calibrate against behaviour, not self-rating.

**Dunning–Kruger pattern plus the statistical critique — CONFIRMED on both sides.** Empirical pattern (https://en.wikipedia.org/wiki/Dunning%E2%80%93Kruger_effect): bottom-quartile performers place themselves in the top two quartiles in relative terms; high performers underestimate via false consensus; stronger for relative than absolute judgments (original Kruger & Dunning 1999, doi:10.1037/0022-3514.77.6.1121). Artifact side: Magnus & Peresetsky (2022, https://doi.org/10.3389/fpsyg.2022.840180 — statistical model fits almost perfectly), Mazor & Fleming (2021, https://doi.org/10.1038/s41562-021-01101-z), Hofer et al. (2022, https://doi.org/10.3390/jintelligence10010010 — standard analyses show DK, improved methods only for verbal intelligence). Precise point estimates (e.g. bottom-quartile mean percentile) NOT verified — UNVERIFIED for numbers.
→ Low-expertise users are the least able to tell you their expertise is low. The stance read must be behavioural, never self-scored.

**Reason-analysis harms decisions that resist verbalisation — CONFIRMED for memory, LIKELY for decision quality.** Verbal overshadowing overview (https://en.wikipedia.org/wiki/Verbal_overshadowing): Schooler & Engstler-Schooler (1990) face recognition, Meissner & Brigham (2001) meta-analysis small-but-reliable negative effect; generalisation to wine/insight/decision quality (Wilson & Schooler 1991 jams/posters: reason-analysis lowers expert agreement and later satisfaction). Reviews: Chin & Schooler (2008, https://doi.org/10.1080/09541440701728623), Melcher & Schooler (2004, https://doi.org/10.3758/bf03195853). Wilson & Schooler (1991) original full text UNREACHABLE (rate-limited) — decision numbers UNVERIFIED.
→ Asking "why do you want that?" can *degrade* the want it tries to surface. Prefer showing over asking why.

## 2. Self-other agreement and predicting one's own behaviour

**SOKA asymmetry: self wins inside, others win outside — LIKELY.** Vazire's Self-Other Knowledge Asymmetry model: self better for low-observability/internal traits (feelings, motives), others better for high-observability/evaluative traits (intellect, likability); explicit feedback from close others beats introspection for blind spots. Sources: Bollich, Johannet & Vazire (2011, https://doi.org/10.3389/fpsyg.2011.00312), Neubauer & Hofer chapter (https://doi.org/10.1017/9781108770422.050). Vazire (2010) original UNREACHABLE via this sweep — asymmetry correlations UNVERIFIED.
→ For "what do I want" (internal, low-observability) the self has the edge *in principle*; for "how will this land / will I persist" (evaluative, behavioural) weight outside views and base rates.

**Connelly & Ones (2010) self-other agreement meta-analysis — UNVERIFIED (no usable record retrieved).** SearXNG returned only unrelated hits; the known record was not verified by the required method, so no r values are cited here. Needs a single-query follow-up after cooldown.
→ Do not cite agreement coefficients as fact until re-verified.

**Affective forecasting: accurate on valence, biased on intensity/duration — CONFIRMED, with a live artifact debate.** Classic: impact bias via focalism + immune neglect (fail to anticipate the psychological immune system); worst for negative events. Artifact moderator: people predict feelings *about* the event but report feelings *in general*; clarifying the question sharply reduces bias; 75–81% misinterpret general questions. Sources: extracted https://en.wikipedia.org/wiki/Affective_forecasting; Levine, Lench, Kaplan & Safer (2012, https://doi.org/10.1037/a0029544); Levine et al. (2013, https://doi.org/10.1037/a0034340); applied review Halpern & Arnold (2008, https://doi.org/10.1007/s11606-008-0719-5).
→ "How will you feel about this outcome?" is answerable; "how will you feel in general?" is systematically overstated. Ask the about-question, never the in-general question.

**Planning fallacy: own time/cost estimates are optimistic even against known history — CONFIRMED with numbers.** Buehler et al. (1994, via extracted https://en.wikipedia.org/wiki/Planning_fallacy): theses mean estimate 33.9 days (best 27.4, worst 48.6), actual 55.5 days; only ~30% finished on time; personal projects 13% done by 50% of allotted time, 19% by 75%, 45% by 99%; Canadian taxpayers ~1 week late despite knowing their own record. Mechanisms: best-case simulation, base-rate neglect, self-serving attribution, memory bias for past durations, temporal framing. Origin Kahneman & Tversky (1979); expansion Lovallo & Kahneman (2003).
→ Never accept a user's effort/urgency estimate at face value; multiply by the observed overrun ratio and show base rates.

**Metacognitive calibration in general — LIKELY, no calibration meta-analysis retrieved.** Convergent reviews (Fleming et al., https://doi.org/10.1146/annurev-psych-022423-032425; Fleming, Dolan & Frith, https://doi.org/10.1098/rstb.2012.0021; extracted https://en.wikipedia.org/wiki/Self-knowledge_(psychology)): confidence is inferential and dissociable from performance; accuracy varies by domain. No general self-estimate-vs-performance r retrieved — UNVERIFIED for a single number.
→ Confidence statements ("I'm sure I want X") carry almost no calibration information. Treat as tone, not data.

## 3. Stated versus revealed preference and the say-do gap

**Revealed preference exists because stated preference could not be trusted — CONFIRMED.** Samuelson (1938) founded revealed preference: infer preferences from purchases, not self-report (https://en.wikipedia.org/wiki/Revealed_preference, extracted; lineage https://link.springer.com/10.1007/978-94-009-7377-0_3). Contingent valuation (CV) is the canonical stated-preference method (Ciriacy-Wantrup 1947 theory, Davis 1963 first application; https://en.wikipedia.org/wiki/Contingent_valuation, extracted); the NOAA Arrow–Solow (1993) panel imposed in-person referendum format, detailed information, income constraints, and conservative WTP-not-WTA rules — a design burden that testifies to the distrust.
→ The protocol's revealed layer (what the user does, funds, revisits) outranks the stated layer by nearly a century of precedent.

**Hypothetical bias: stated WTP overstates real WTP by 2–3× — CONFIRMED.** Loomis (2011, https://doi.org/10.1111/j.1467-6419.2010.00675.x, 447 citations): "hypothetical WTP typically exceeds the actual value by a factor of two to three." Corroborated by Murphy & Stevens (2004, https://doi.org/10.1017/s1068280500005761) and an extreme infant-health case (~7×: $14/$26/$66 hypothetical vs $2/$3.70/$9.43 cash, https://doi.org/10.1017/s1074070800003163). Marketing update, Schmidt & Bijmolt (2019, https://doi.org/10.1007/s11747-019-00666-6 — 77 studies, 115 effect sizes): mean bias +21%, and indirect (conjoint-like) methods overestimate *more* than direct questioning; larger for high-value/specialty goods and within-subject designs — LIKELY (single meta, needs replication).
→ Discount any stated intensity ("I really need X", "I'd pay anything") by 1.3–3×; conjoint does NOT fix the bias, it can worsen it.

**The intention–behaviour gap is quantified — CONFIRMED (best numbers for the user model).** Conner & Norman (2022, https://doi.org/10.3389/fpsyg.2022.923464, 501 citations, full-text extracted): intentions explain 18–23% of variance in behaviour (correlational r+ 0.40–0.48); experimentally induced medium-to-large intention changes produce only small-to-medium behaviour changes (experimental r+ 0.08–0.18); self-report inflates — intention+PBC explain 26% of self-reported but 12% of objective physical activity (31% vs 20% across behaviours). Corroborated by Ajzen (2011, https://doi.org/10.1080/08870446.2011.613995, 4,686 citations, citing McEachan et al. 2011). Sheeran (2002) / Webb & Sheeran (2006) canonical numbers NOT directly retrieved — UNVERIFIED here, do not cite. TPB structure itself CONFIRMED (https://en.wikipedia.org/wiki/Theory_of_planned_behavior; Francis et al. manual https://openaccess.city.ac.uk/id/eprint/1735/1/TPB%20Manual%20FINAL%20May2004.pdf).
→ Model P(behaviour | stated intent) ≈ 0.2 correlational, ≈ 0.01–0.03 causal; halve again when the only measure is self-report. Commitment devices, not stronger statements, close the gap.

**Preferences are constructed by the elicitation, not retrieved — CONFIRMED.** Tversky & Thaler (1990, https://doi.org/10.1257/jep.4.2.201, 434 citations): pricing-vs-choice preference reversals violate fixed-preference assumptions; elicitation method changes attribute weights. Galizzi & Navarro-Martinez (2019, https://doi.org/10.1287/mnsc.2017.2908, 273 citations): lab social-preference games poorly explain field behaviour. Lichtenstein & Slovic (2006) *The Construction of Preference* returned no direct SearXNG hit — book UNVERIFIED here, resolve via catalogue. UX-specific say-do literature: no usable academic source retrieved — UNVERIFIED gap, fall back on the WTP and intention numbers above.
→ There is no stable "true want" waiting to be worded; the protocol's questions *build* the want. Design the elicitation knowing it is construction equipment.

**Environmental value-action / knowledge-action gap — CONFIRMED as a field, no single model.** Kollmuss & Agyeman (2002, https://doi.org/10.1080/13504620220145401, 8,809 citations) integrative model; Frederiks, Stenner & Hobman (2014, https://doi.org/10.1016/j.rser.2014.09.026, 937 citations): sizeable discrepancy between self-reported knowledge/values/intentions and observable behaviour; Kraus (1995) boundary conditions via https://en.wikipedia.org/wiki/Attitude-behavior_consistency.
→ Values talk ("I care about X") predicts behaviour only when stable, accessible, and grounded in direct experience. Otherwise it is identity talk.

## 4. Emotional granularity and alexithymia

**Granularity is construction of precise context-specific emotions, and it predicts better regulation — LIKELY (single authoritative tertiary + one primary, field nascent).** Barrett program: phenomenon first reported 1995 as "differentiation", term coined 2004; current position is construction, not mere verbal labelling (https://en.wikipedia.org/wiki/Emotional_granularity, extracted; refs Hoemann et al. 2021 scoping review). "High granularity aids coping" is hedged on the same page ("limited amount of research… fairly nascent"). Nuance from a live primary: differentiation tracks *effectiveness* of regulation strategies, not *selection* (experience-sampling Ns 200+101; full URL not captured — DOI lookup needed). Background: Barrett et al. (2007, https://doi.org/10.1146/annurev.psych.58.110405.085709, 1,393 cites), Barrett (2017, https://doi.org/10.1093/scan/nsw154), Garfinkel/Critchley interoception review (https://doi.org/10.1016/j.copsyc.2017.04.020). A "Smidt & Wager meta-analysis" of granularity→regulation was sought and NOT found — UNVERIFIED, do not cite.
→ Users who describe wants in fine-grained terms ("I want *relief from* X, not *more of* Y") likely regulate follow-through better — but the evidence is young. Do not gate protocol strictness on it.

**Alexithymia: single-digit general rate, TAS-20 cut-offs, interoception link — CONFIRMED.** Prevalence ~5% point estimate, <10% high-alexithymia cut-off; elevated in autism (~50%), PTSD (42%), cancer (37%) — https://en.wikipedia.org/wiki/Alexithymia (extracted). TAS-20: 20 items, 3 factors (DIF items 1,3,6,7,9,13,14; DDF 2,4,11,12,17; EOT 5,8,10,15,16,18,19,20), 5-point Likert, cut-offs ≤51 none / 52–60 possible / ≥61 present — https://en.wikipedia.org/wiki/Toronto_Alexithymia_Scale (extracted) plus Goerlich (2018, https://doi.org/10.3389/fpsyg.2018.01614). Interoception: Brewer, Cook & Bird (2016, https://doi.org/10.1098/rsos.150664, 405 cites) general interoception failure; Shah et al. (2016, https://doi.org/10.1016/j.cortex.2016.03.021, 331 cites) deficit tracks alexithymia not autism. Outcomes: autism-group alexithymia 49.93% vs NT 4.89% (https://doi.org/10.1016/j.eurpsy.2018.09.004); suicide ideation r=0.54, behaviour r=0.25 (https://doi.org/10.1016/j.jad.2019.05.013). Caveat (same TAS-20 page): EOT subscale low reliability; PAQ now psychometrically stronger.
→ Expect ~1 in 10–20 users to be unable to label what they feel about an option — structurally, not stubbornly. For them, verbal elicitation is the wrong modality.

## 5. What actually improves self-knowledge

**Feedback helps on average but often backfires; content and target matter — LIKELY (original FIT numbers UNVERIFIED this run).** Kluger & DeNisi (1996) feedback-intervention theory ("38% backfire" family) returned EMPTY from SearXNG (all engines suspended) — do NOT cite the numbers. Live modern meta: Wisniewski, Zierer & Hattie (2020, https://doi.org/10.3389/fpsyg.2019.03087, k=994, N>61,000): overall d=0.48 but highly heterogeneous — content matters, stronger for cognitive/motor than motivational/behavioural outcomes. Coaching with multisource feedback: Jones et al. (2015, https://doi.org/10.1111/joop.12119) overall δ=0.36 — but multisource-feedback use gave *smaller* positive effects, internal coaches beat external. 360-degree feedback (https://en.wikipedia.org/wiki/360-degree_feedback, extracted): >1/3 of US companies, 90% of Fortune 500; gains appear years 2–4 not year 1; self-ratings systematically exceed others'; accuracy falls with acquaintance; gaming under appraisal framing — LIKELY.
→ Feedback aimed at the *task* ("this plan fails test X") calibrates; feedback aimed at the *self* ("you are unclear") risks the backfire pattern. Keep every protocol mirror task-level, anonymous where possible, development-framed.

**Expressive writing / journaling: paradigm CONFIRMED, benefits minor-to-null — CONFIRMED.** Paradigm (https://en.wikipedia.org/wiki/Expressive_writing, extracted): 15 min × 4 days, deepest thoughts/feelings vs neutral-factual control; original fewer-physician-visits claim. Updates: Mogk et al. (42 RCTs, 30 in meta, https://pubmed.ncbi.nlm.nih.gov/19742069): no significant somatic/psychological effects except one very small exploratory effect; cancer-only meta (16 RCTs, https://doi.org/10.1002/pon.3802): psychological g=0.04, physical g=0.08, QoL g=0.09, all ns. Frattaroli (2006) d≈0.075 NOT live-retrieved — UNVERIFIED here. Smyth (1998) d=0.47 survives only as cited-inside-abstract prior.
→ Do not prescribe journaling as a self-knowledge lever. Structured, cued reflection may differ — but that claim is UNVERIFIED (no live retrieval; Vazire/Houde queries failed).

**Implementation intentions (if-then plans) work given a strong goal — LIKELY (mechanism CONFIRMED, headline d UNVERIFIED).** Mechanism (https://en.wikipedia.org/wiki/Implementation_intention, extracted): "when situation X arises, I will perform Y" → strategic automaticity, heightened cue perception; boundary condition — requires strong superordinate goal intention (Sheeran, Webb & Gollwitzer 2005, https://doi.org/10.1177/0146167204271308; Achtziger et al. 2008, https://doi.org/10.1177/0146167207311201). Domain meta: healthy eating (https://doi.org/10.1016/j.appet.2010.10.012). The famous Gollwitzer & Sheeran (2006) headline (94 studies, d=0.65) NOT returned live — UNVERIFIED for the number.
→ Convert every "want" into an if-then before trusting it: no cue-action link, no usable want.

## 6. When the user cannot articulate what they want

**Recognition beats recall — CONFIRMED.** Nielsen Norman Group (https://www.nngroup.com/articles/recognition-and-recall/, extracted): recognition (familiarity judgment with full cues) is easier than recall (detail retrieval with few cues) via spreading activation; therefore multiple-choice, examples, and galleries beat open-ended "what do you want?"
→ Never open with a blank "what do you want?" Show candidates and let the user recognise. This is the single most actionable finding in the sweep.

**Repertory grid surfaces the user's own dimensions — CONFIRMED (method + live applications).** Kelly (1955) personal-construct theory: elements × bipolar constructs via triadic elicitation ("how are two alike, the third different?"), 5/7-point ratings, 6–16 constructs typical (https://en.wikipedia.org/wiki/Repertory_grid, extracted). Applications: search-engine mental models (https://doi.org/10.1108/00220410710737213 — grid + laddering → ease/efficiency/effort/effectiveness layers).
→ Triadic comparison ("of these three directions, which two belong together?") extracts criteria the user could never have listed.

**Laddering / means-end chains climb from attributes to values — LIKELY.** Reynolds & Gutman (1988) via Veludo-de-Oliveira (https://doi.org/10.46743/2160-3715/2006.1651): one-on-one interviews mapping attributes → consequences → personal values; needs trained interviewers, homogeneous samples. Applied example: Pike destination positioning (https://doi.org/10.1016/j.tourman.2011.02.008).
→ Repeated "why does that matter?" (3–5 rungs) reaches the load-bearing value — but see §1: stop before reason-analysis degrades the choice; ladder the *consequence*, not the *justification*.

**Forced choice / pairwise comparison / AHP: mechanics LIKELY, superiority UNVERIFIED.** Saaty (1970s) Analytic Hierarchy Process: decompose goal → criteria → alternatives, pairwise relative-importance judgments → numeric weights (https://en.wikipedia.org/wiki/Analytic_hierarchy_process, extracted). Effectiveness-vs-direct-rating numbers NOT retrieved (query failed) — UNVERIFIED.
→ Pairwise "A or B?" is usable now as mechanics; do not claim validated superiority without a new search.

**Information avoidance is strategic, not apathy — LIKELY–CONFIRMED.** Golman, Hagmann & Loewenstein (2017, https://doi.org/10.1257/jel.20151245, 845 cites): people avoid free, useful, non-strategic information when it threatens beliefs/preferences or enables unwanted action. Primaries: Taber et al. (2015, https://doi.org/10.1007/s12160-014-9679-7 — avoidance predicts declining genetic results; self-affirmation attenuates), Melnyk & Shepperd (2012, https://doi.org/10.1007/s12160-012-9382-5 — controllability framing reduces avoidance).
→ Never infer "no preference" from non-engagement. Offer controllability framing and opt-in granularity before concluding indifference.

**Reactions to artifacts/prototypes; structured reflection; HCI preference-elicitation as a body — UNVERIFIED (no live retrieval).** Consistent with practice but uncited here; needs a dedicated pass (Houde & Hill, Wizard-of-Oz, technology probes) after the rate limit clears.
→ Usable as practitioner prior only. Do not present as evidence-backed.

## 7. What this licenses / what it rules out

Strong enough to design against (CONFIRMED): introspected reasons are theories, not observations (§1); stated intensity overstates real commitment 1.3–3× and intention explains ~20% of behaviour variance, ~1–3% causally (§3); recognition beats recall (§6); planning-fallacy multipliers on user estimates (§2); ~1-in-10–20 users structurally cannot label feelings — give them non-verbal modalities (§4); journaling is not a self-knowledge lever (§5).

Usable with care (LIKELY): SOKA asymmetry for weighting self vs outside views (§2); granularity-as-regulation (§4); feedback-at-task-level and if-then conversion (§5); laddering, rep-grid triads, pairwise choice, controllability framing (§6).

Ruled out: trusting assent to a restated want; trusting confidence statements as calibration; trusting self-scored expertise; asking "why" to improve a hard-to-verbalise choice; citing Zell-2020, Connelly-Ones-2010, Sheeran-2002/Webb-Sheeran-2006, Kluger-DeNisi-38%, Frattaroli-0.075, Gollwitzer-Sheeran-0.65, or Smidt-Wager numbers as fact — none verified in this sweep.

Against the premise (honest counterweight): preferences are *constructed* by elicitation (§3: Tversky/Thaler, Galizzi) — so "self-knowledge" is not a stable trait to be measured but a capacity exercised in the interaction, which cuts against a fixed spectrum model; affective-forecast accuracy is high for the about-question, so the deficit is narrower than "people don't know themselves" (§2: Levine); reason-analysis *harms* some decisions (§1), so more self-knowledge work can produce worse wants; information avoidance (§6) means low-signal users may be strategically opaque rather than low-capacity — a spectrum misreads motive as ability.

## 8. Open gaps

1. Connelly & Ones (2010) agreement coefficients and incremental-validity numbers — no usable record; single-query retry.
2. Vazire (2010) original asymmetry correlations; SOKA boundary conditions — original unreachable.
3. Zell et al. (2020) better-than-average meta-analytic d — unreachable.
4. Wilson & Schooler (1991) jam/poster originals; exact decision-impairment sizes — unreachable.
5. Sheeran (2002) / Webb & Sheeran (2006) / McEachan et al. (2011) canonical intention–behaviour numbers — cited second-hand only.
6. Kluger & DeNisi (1996) FIT backfire rate; Frattaroli (2006) writing d; Gollwitzer & Sheeran (2006) if-then d — headline numbers unverified.
7. Smidt/Wager granularity meta — no record found; may not exist under that name.
8. Prototype/artifact superiority (Houde & Hill, technology probes) and structured-reflection trials — entirely unretrieved.
9. UX-specific say-do gap and conjoint-vs-direct academic literature — returned glossary noise only.
10. SearXNG rate limits capped late-sweep verification; a follow-up pass with single queries after cooldown (or institutional access for paywalled primaries: Nisbett-Wilson stimulus %-tables, Lichtenstein & Slovic book, Ajzen/McEachan PDFs) would upgrade most LIKELYs and UNVERIFIEDs above.

## Sources

- Nisbett & Wilson (1977), Telling more than we can know, Psych Rev. https://doi.apa.org/doi/10.1037/0033-295X.84.3.231
- Berger et al. (2016) revisit. https://doi.org/10.1521/soco.2016.34.3.167
- Introspection illusion — overview. https://en.wikipedia.org/wiki/Introspection_illusion (fetched full text)
- Moore & Haggard (2006) commentary. https://linkinghub.elsevier.com/retrieve/pii/S1053810006000948
- Bortolotti & Sullivan-Bissett (2019) choice-blindness review. https://doi.org/10.1007/s11229-019-02414-3
- Ganapini (2019) confabulation. https://doi.org/10.1007/s11245-018-09629-y
- Cheung et al. (2015) consumer choice blindness. https://doi.org/10.1016/j.appet.2015.09.022
- Choice-blindness record. https://philpapers.org/rec/MOOHSC
- Illusory superiority — overview. https://en.wikipedia.org/wiki/Illusory_superiority (fetched full text)
- Dunning–Kruger effect — overview. https://en.wikipedia.org/wiki/Dunning%E2%80%93Kruger_effect (fetched full text)
- Magnus & Peresetsky (2022) artifact model. https://doi.org/10.3389/fpsyg.2022.840180
- Mazor & Fleming (2021) revisit. https://doi.org/10.1038/s41562-021-01101-z
- Hofer et al. (2022) generality test. https://doi.org/10.3390/jintelligence10010010
- Coutinho et al. (2021) CRT miscalibration. https://doi.org/10.3389/fpsyg.2021.603225
- Krajc & Ortmann alternative explanation. https://doi.org/10.1016/j.joep.2007.12.006 (record only)
- Verbal overshadowing — overview. https://en.wikipedia.org/wiki/Verbal_overshadowing (fetched full text)
- Chin & Schooler (2008) review. https://doi.org/10.1080/09541440701728623
- Melcher & Schooler (2004). https://doi.org/10.3758/bf03195853
- Bollich, Johannet & Vazire (2011). https://doi.org/10.3389/fpsyg.2011.00312
- Neubauer & Hofer SOKA chapter. https://doi.org/10.1017/9781108770422.050
- Affective forecasting — overview. https://en.wikipedia.org/wiki/Affective_forecasting (fetched full text)
- Levine et al. (2012). https://doi.org/10.1037/a0029544
- Levine et al. (2013). https://doi.org/10.1037/a0034340
- Halpern & Arnold (2008). https://doi.org/10.1007/s11606-008-0719-5
- Planning fallacy — overview with Buehler numbers. https://en.wikipedia.org/wiki/Planning_fallacy (fetched full text)
- Fleming review proposal. https://doi.org/10.1146/annurev-psych-022423-032425 (record only)
- Fleming, Dolan & Frith. https://doi.org/10.1098/rstb.2012.0021
- Self-knowledge (psychology) — overview. https://en.wikipedia.org/wiki/Self-knowledge_(psychology) (fetched full text)
- Revealed preference — overview. https://en.wikipedia.org/wiki/Revealed_preference (fetched full text)
- Mas-Colell revealed-preference lineage. https://link.springer.com/10.1007/978-94-009-7377-0_3 (record only)
- Contingent valuation — overview. https://en.wikipedia.org/wiki/Contingent_valuation (fetched full text)
- Carson (2012). https://doi.org/10.1257/jep.26.4.27 (record only)
- Loomis (2011) hypothetical bias. https://doi.org/10.1111/j.1467-6419.2010.00675.x
- Murphy & Stevens (2004). https://doi.org/10.1017/s1068280500005761
- Infant-health WTP case. https://doi.org/10.1017/s1074070800003163
- Schmidt & Bijmolt (2019) WTP meta. https://doi.org/10.1007/s11747-019-00666-6
- Haghani et al. (2021) choice-experiment bias. https://doi.org/10.1016/j.jocm.2021.100322
- Kollmuss & Agyeman (2002). https://doi.org/10.1080/13504620220145401
- Frederiks, Stenner & Hobman (2014). https://doi.org/10.1016/j.rser.2014.09.026
- Attitude–behavior consistency — overview. https://en.wikipedia.org/wiki/Attitude-behavior_consistency (fetched full text)
- Conner & Norman (2022) intention–behaviour gap. https://doi.org/10.3389/fpsyg.2022.923464 (fetched full text)
- Ajzen (2011) TPB reflections. https://doi.org/10.1080/08870446.2011.613995
- Rise, Sheeran & Hukkelberg (2010). https://doi.org/10.1111/j.1559-1816.2010.00611.x
- Theory of planned behavior — overview. https://en.wikipedia.org/wiki/Theory_of_planned_behavior (fetched full text)
- Francis et al. TPB manual. https://openaccess.city.ac.uk/id/eprint/1735/1/TPB%20Manual%20FINAL%20May2004.pdf
- Tversky & Thaler (1990) preference reversals. https://doi.org/10.1257/jep.4.2.201
- Galizzi & Navarro-Martinez (2019) lab-field failure. https://doi.org/10.1287/mnsc.2017.2908
- Pfister & Böhm (2008) emotion in construction. https://doi.org/10.1017/s1930297500000127
- Emotional granularity — overview. https://en.wikipedia.org/wiki/Emotional_granularity (fetched full text)
- Barrett et al. (2007). https://doi.org/10.1146/annurev.psych.58.110405.085709
- Barrett (2017) constructed emotion. https://doi.org/10.1093/scan/nsw154
- Koole (2009) review. https://doi.org/10.1080/02699930802619031
- Garfinkel/Critchley interoception review. https://doi.org/10.1016/j.copsyc.2017.04.020
- Alexithymia — overview. https://en.wikipedia.org/wiki/Alexithymia (fetched full text)
- Toronto Alexithymia Scale — overview. https://en.wikipedia.org/wiki/Toronto_Alexithymia_Scale (fetched full text)
- Goerlich (2018) TAS-20 vs BVAQ. https://doi.org/10.3389/fpsyg.2018.01614
- Brewer, Cook & Bird (2016) interoception. https://doi.org/10.1098/rsos.150664
- Shah et al. (2016). https://doi.org/10.1016/j.cortex.2016.03.021
- Autism alexithymia meta. https://doi.org/10.1016/j.eurpsy.2018.09.004
- Suicide/alexithymia meta. https://doi.org/10.1016/j.jad.2019.05.013
- Wisniewski, Zierer & Hattie (2020) feedback meta. https://doi.org/10.3389/fpsyg.2019.03087
- Jones et al. (2015) coaching. https://doi.org/10.1111/joop.12119
- 360-degree feedback — overview. https://en.wikipedia.org/wiki/360-degree_feedback (fetched full text)
- Expressive writing — overview. https://en.wikipedia.org/wiki/Expressive_writing (fetched full text)
- Mogk et al. writing update. https://pubmed.ncbi.nlm.nih.gov/19742069
- Cancer writing meta. https://doi.org/10.1002/pon.3802
- Implementation intention — overview. https://en.wikipedia.org/wiki/Implementation_intention (fetched full text)
- Sheeran, Webb & Gollwitzer (2005). https://doi.org/10.1177/0146167204271308
- Achtziger et al. (2008). https://doi.org/10.1177/0146167207311201
- Healthy-eating implementations meta. https://doi.org/10.1016/j.appet.2010.10.012
- Analytic hierarchy process — overview. https://en.wikipedia.org/wiki/Analytic_hierarchy_process (fetched full text)
- Laddering method. https://doi.org/10.46743/2160-3715/2006.1651
- Pike laddering application. https://doi.org/10.1016/j.tourman.2011.02.008
- Repertory grid — overview. https://en.wikipedia.org/wiki/Repertory_grid (fetched full text)
- Grid + laddering application. https://doi.org/10.1108/00220410710737213
- Golman, Hagmann & Loewenstein (2017) information avoidance. https://doi.org/10.1257/jel.20151245
- Taber et al. (2015). https://doi.org/10.1007/s12160-014-9679-7
- Melnyk & Shepperd (2012). https://doi.org/10.1007/s12160-012-9382-5
- Recognition over recall (NN/g). https://www.nngroup.com/articles/recognition-and-recall/ (fetched full text)

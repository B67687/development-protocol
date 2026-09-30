# Dynamic Guidance & Pacing: ZPD, Scaffolding, Fading, Desirable Difficulties

- **Date:** 2026-09-30
- **Scope:** Development Protocol T-051 to T-054 — guidance/teaching principle, "understanding built on prior", "learning by building", pacing control, plain-words teaching links, dynamic growth, ZPD slightly above ability.
- **Method:** Authoritative search — SearXNG `categories=science` sweep (6 queries: ZPD, Wood/Bruner/Ross fading, Bjork difficulties, expertise reversal, progressive disclosure, guidance fading) + full-page extraction (`web_url_read`, Trafilatura-sidecar equivalent) of load-bearing sources. Claims labelled CONFIRMED / LIKELY / UNVERIFIED.
- **Status:** Research sweep only. No protocol text changed. No INDEX.md edit (row suggested in §8).

## 1. Context — what T-051..T-054 is asking

- THOUGHT_LOG T-051..T-054 asks one pacing question four ways: how fast the protocol should build knowledge, who controls pacing, how teaching links stay in simple words, and how growth stays dynamic with tasks kept slightly above current ability.
- Plain-words gloss (for the protocol itself): teach the next small thing the learner can almost do, help while it is hard, remove help as soon as it works, and make practice effortful but still winnable.
- This sweep answers three things: what licenses a pacing law (§8, encode-ready), what rules out front-loading (N1–N5), and what stays open (G1–G5).
- Structure note: this file mirrors `docs/research/prompt-engineering-science.md` — sections, confidence labels, effect sizes, citations — but that reference file was absent from the workspace at write time, so the shape is reconstructed from the task spec.

## 2. Findings at a glance

| # | Finding | Confidence | Effect size / magnitude |
|---|---------|------------|-------------------------|
| F1 | Scaffolding (contingent + faded + transferred) improves learning outcomes online | LIKELY | Large, significant (meta of 18 articles / 64 ES; exact g not extracted) |
| F2 | Instructional support for diagnostic competences helps overall | CONFIRMED | g = 0.39, 95% CI [0.22, 0.56] (35-study meta) |
| F3 | Worked examples beat open problem-solving for novices; advantage reverses with expertise | CONFIRMED (direction) | Large for novices; shrinks/reverses (no single number) |
| F4 | Retrieval practice beats restudy for long-term retention | CONFIRMED (direction) / LIKELY (size) | d ≈ 0.5–0.7 in meta-literature (not re-extracted; verify before freezing) |
| F5 | Spacing beats massing for retention, with short-term practice cost | LIKELY | Small-to-moderate, heterogeneous across STEM courses |
| F6 | Adaptive fading beats fixed fading beats pure problem-solving | LIKELY | Ordinal only (Salden et al. tutor studies) |
| F7 | ZPD / contingency / progressive disclosure are validated constructs without causal effect sizes | CONFIRMED (construct) | No ES — design frameworks, not interventions |

## 3. Zone of proximal development (Vygotsky)

- ZPD = gap between unaided performance and performance achievable with a more knowledgeable other; today's assisted performance becomes tomorrow's independent performance **(CONFIRMED)**. Sources: https://en.wikipedia.org/wiki/Zone_of_proximal_development ; https://en.wikipedia.org/wiki/Instructional_scaffolding
- Vygotsky introduced but never completed the construct (died 1934, last-three-years work); there is no canonical ZPD meter **(CONFIRMED)**. Source: https://en.wikipedia.org/wiki/Zone_of_proximal_development (Origins/Challenges).
- Operational proxy is dynamic assessment: test → assist → retest, measuring responsiveness to help rather than static score **(CONFIRMED as procedure)**. Source: https://doi.org/10.5539/elt.v3n4p237.
- "Hardest task doable with scaffolding yields largest gains" (Wass & Golding, cited on ZPD page) is a teaching heuristic, not a measured dose-response curve **(LIKELY, no effect size)**.
- Adult/asymmetric peer extension: helpers need not be more capable in general, only relative to the target task **(LIKELY)**. Source: ZPD page (L2 section).
- Plain-words gloss: work on what the learner cannot do alone but can do with help — never far above, never already-mastered.
- Pacing implication: ZPD licenses "slightly above demonstrated ability + assist", never a fixed difficulty number; any numeric step-size is a stipulation (see G1).

## 4. Scaffolding — contingency, fading, transfer (Wood / Bruner / Ross → van de Pol)

- Origin: Wood, Bruner & Ross tutoring studies (1976) coined "scaffolding"; Vygotsky himself never used the term **(CONFIRMED)**. Sources: https://en.wikipedia.org/wiki/Zone_of_proximal_development (Scaffolding); https://en.wikipedia.org/wiki/Instructional_scaffolding (Theory).
- Van de Pol et al. decade review distils three non-negotiable features — contingency (adapt to diagnosed level), fading (withdraw), transfer of responsibility — plus a means × intentions coding framework **(CONFIRMED)**. Source: https://doi.org/10.1007/s10648-010-9127-6.
- Online higher-ed meta-analysis (18 articles, 64 effect sizes, 8 countries, 2010–2019): large, statistically significant positive effect; metacognitive scaffolding most studied; computer-provided scaffolding prevalent **(LIKELY large; exact g not extracted)**. Source: http://www.irrodl.org/index.php/irrodl/article/view/4638.
- Classroom experiment (30 teachers, 768 pupils, 5-lesson EU project): contingency interacts with independent working time — low-contingency/frequent help wins when solo time is short; high-contingency help wins when solo time is long; higher contingency raises appreciation of support **(LIKELY)**. Source: https://doi.org/10.1007/s11251-015-9351-z.
- Uptake study (35 lessons, multilevel mediation): students who apply teacher support in later group work answer more accurately; contingent-then-gradually-faded support most effective, and untimely fading breaks uptake **(LIKELY)**. Source: https://doi.org/10.1080/10508406.2018.1522258.
- Measurement is the field's main challenge: few effectiveness studies, coding contingency/fading reliably is hard **(CONFIRMED as field consensus)**. Source: van de Pol review, ibid.
- Plain-words gloss: check what the learner gets now, help exactly there, then hand the job over bit by bit.

## 5. Guidance fading and the expertise reversal effect (Sweller / Kalyuga / Renkl / Atkinson)

- Minimal-guidance formats (pure discovery / problem-based / experiential for novices) lose to guided instruction across half a century of comparisons **(CONFIRMED direction)**. Source: https://doi.org/10.1207/s15326985ep4102_1.
- Worked-example effect: studying worked steps beats open solving for novices on high-element-interactivity material; the advantage is eliminated or reversed as expertise grows **(CONFIRMED)**. Sources: https://doi.org/10.1037/edu0000018 (5 experiments, geometry + trigonometry, immediate + delayed); https://doi.org/10.1007/s10648-019-09465-5.
- Expertise reversal defined: techniques that help low-knowledge learners (worked examples, integrated text+diagram, segmentation) become neutral or harmful for high-knowledge learners via redundant processing load **(CONFIRMED)**. Sources: https://en.wikipedia.org/wiki/Expertise_reversal_effect (cites Kalyuga 2007; Kalyuga et al. 2003); https://doi.org/10.1111/j.1365-2923.2009.03498.x.
- Caveats travel with the effect: many studies use subjective effort ratings of disputed validity, and motivational (not load) accounts remain viable **(CONFIRMED as open dispute)**. Source: expertise-reversal page (Cognitive load theory section).
- Completion/fading ladder (Renkl & Atkinson): study full example → complete partial example → solve independently; structures the worked-to-problem transition **(CONFIRMED as procedure)**. Sources: https://en.wikipedia.org/wiki/Expertise_reversal_effect (Adaptive fading); https://link.springer.com/10.1023/B:TRUC.0000021815.74806.f6.
- Adaptive fading (fade keyed to demonstrated competence, e.g. Salden et al. cognitive-tutor studies) beats fixed-schedule fading, which beats pure problem-solving **(LIKELY)**. Source: expertise-reversal page (cites Salden et al. 2008).
- Moderator number: diagnostic-competence meta (35 studies) — support g = 0.39 [0.22, 0.56]; high-guidance scaffolds suit low-knowledge, self-regulation scaffolds suit advanced learners **(CONFIRMED)**. Source: https://doi.org/10.1007/s10648-019-09492-2.
- Plain-words gloss: show how first, then leave blanks, then let go — and let go earlier when the learner already knows a lot.

## 6. Desirable difficulties (Bjork) — effort that pays, bounded by reachability

- Coined by R. Bjork (1994): effortful conditions that slow encoding but raise retention/transfer; only some difficulties qualify **(CONFIRMED)**. Sources: https://en.wikipedia.org/wiki/Desirable_difficulty ; https://bjorklab.psych.ucla.edu/wp-content/uploads/sites/13/2016/04/EBjork_RBjork_2011.pdf.
- Qualifying rule (challenge-point reading): desirable only if (a) encoding practice matches retrieval demands and (b) success stays reachable — unreachable difficulty dissuades and blocks processing **(CONFIRMED as framework)**. Source: https://en.wikipedia.org/wiki/Desirable_difficulty (Requirements).
- Retrieval practice (testing effect): testing beats restudy across materials/ages/test formats; feedback required; larger flashcard piles raise difficulty and final scores (Kornell 2009) **(CONFIRMED direction; size LIKELY d ≈ 0.5–0.7 — verify)**. Sources: https://doi.org/10.1146/annurev-psych-010419-051019 ; https://doi.org/10.5688/ajpe7324.
- Spacing/interleaving: spaced quizzes lower practice scores but raise end-of-semester retention (calculus replication); single-paper STEM meta finds a significant overall spacing benefit driven by few courses, heterogeneous elsewhere **(LIKELY small-to-moderate)**. Sources: https://doi.org/10.1007/s10648-022-09677-2 ; https://doi.org/10.1186/s40594-024-00468-5 ; https://doi.org/10.1007/s10459-023-10274-3.
- Combined routines (e.g. Read/Recite/Review) stack retrieval + feedback-seeking and beat rereading **(LIKELY)**. Source: desirable-difficulty page (Combined techniques).
- Boundary with §5: generation/self-explanation wins on low-element-interactivity material but loses to worked examples on high-interactivity material for novices (Chen et al., five experiments, immediate + delayed) **(LIKELY)**. Source: https://doi.org/10.1037/edu0000018.
- Long-horizon caveat: most trials span hours–days; permastore-scale evidence (Bahrick Spanish, Landauer/Ainslie testing) is suggestive, not protocol-grade **(LIKELY)**. Source: desirable-difficulty page (Implications).
- Plain-words gloss: practice remembering (not rereading), spread it out, mix topics — hard enough to think, easy enough to win.

## 7. Progressive and staged disclosure — the presentation layer (Nielsen; Carroll & Rosson)

- Nielsen (2006): show core options first, defer rare/advanced ones behind a clearly labelled secondary step; improves learnability, efficiency, error rate; more than 2 levels usually backfires **(LIKELY; practitioner evidence, no RCT effect size)**. Source: https://www.nngroup.com/articles/progressive-disclosure/.
- Two things must be right: correct core/secondary split (task analysis + frequency data + usability testing) and obvious mechanics + information scent for progressing **(LIKELY)**. Source: ibid. (Usability Criteria).
- Training-wheels lineage (Carroll & Rosson, minimal manual): a cut-down functional subset teaches the system better, not worse — answers the "limiting mental model" worry **(LIKELY)**. Source: ibid. (cites training-wheels research).
- Staged (wizard) vs progressive distinction: staged forces a linear sequence every user traverses; progressive keeps most users on the core screen with rare excursions **(CONFIRMED as design taxonomy)**. Source: ibid. (comparison table).
- Protocol mapping: disclosure governs how much guidance is *visible*; fading (§5) governs how much guidance *exists* — use both, never disclosure alone as a learning theory **(author synthesis, UNVERIFIED as empirical claim)**.
- Plain-words gloss: show a little, hide the rest behind one clear door, open it only when asked.

## 8. What this licenses, what it rules out, and open gaps

**Licenses a pacing law (encode-ready candidates):**

1. **P1 — Contingent step-up:** diagnose current level, then assign the hardest task still completable with assistance; re-diagnose after each step. (ZPD §3 + contingency §4.)
2. **P2 — Fade by demonstration, not by clock:** withdraw worked steps only on shown competence via study → complete → solve. (Salden/Renkl §5.)
3. **P3 — Desirable difficulty inside reach:** prefer retrieval/spacing/varied practice over restudy/summary, gated on retrievability — if recall fails outright, drop difficulty before repeating. (Bjork §6 + reversal §5.)

**Rules out — do NOT encode:**

- N1 — Front-loading: dumping full guidance/maps/manuals up front. Ruled out by redundancy effects and retrieval/spacing superiority over restudy.
- N2 — Fixed-schedule fading ("reduce help every N steps"). Ruled out by adaptive > fixed fading and the contingency × solo-time interaction.
- N3 — Pure discovery for novices. Ruled out by the Kirschner/Sweller/Clark review (§5).
- N4 — Permanent scaffolding (help that never fades or transfers). Ruled out by fading/transfer requirement (§4) and expertise reversal (§5).
- N5 — Difficulty for its own sake (unreachable challenge, unguaranteed delayed feedback, >2 disclosure levels). Ruled out by Bjork reachability and Nielsen depth limit.

**Open gaps (for T-051..T-054 drafting, not blockers):**

- G1 — No calibrated "slightly above" metric: ZPD has no agreed unit; dynamic assessment is a procedure, not a threshold. Any numeric step-size is a stipulation — mark UNVERIFIED.
- G2 — Exact meta-analytic numbers for testing/spacing/worked-example effects were scoped out of extraction; re-pull Adesope/Cepeda/Sweller meta tables before freezing quantitative claims.
- G3 — Scaffolding measurement problem (van de Pol): coding contingency/fading reliably is hard, so protocol compliance checks need behavioural anchors.
- G4 — Affect/frustration control (Wood lists managing frustration as scaffolding) is under-specified for an agent protocol — needs its own micro-principle.
- G5 — Progressive disclosure has no learning-outcome effect sizes; treat as presentation guidance only.

**Suggested INDEX.md row (do NOT apply — author to confirm):**

- `| guidance-dynamic-2026-09-30 | ZPD, scaffolding/fading, expertise reversal, desirable difficulties, progressive disclosure → pacing law (T-051..T-054) | 2026-09-30 |`

**Key references:** Vygotsky / Mind in Society (via ZPD page); Wood, Bruner & Ross 1976 (via scaffolding pages); van de Pol et al. 2010 https://doi.org/10.1007/s10648-010-9127-6 ; classroom study https://doi.org/10.1007/s11251-015-9351-z ; uptake study https://doi.org/10.1080/10508406.2018.1522258 ; Kirschner/Sweller/Clark 2006 https://doi.org/10.1207/s15326985ep4102_1 ; Sweller et al. 2019 https://doi.org/10.1007/s10648-019-09465-5 ; Kalyuga et al. 2003/2007 (via https://en.wikipedia.org/wiki/Expertise_reversal_effect); Chen et al. https://doi.org/10.1037/edu0000018 ; Bjork & Bjork https://bjorklab.psych.ucla.edu/wp-content/uploads/sites/13/2016/04/EBjork_RBjork_2011.pdf ; Karpicke/Roediger review https://doi.org/10.1146/annurev-psych-010419-051019 ; Doo et al. 2020 http://www.irrodl.org/index.php/irrodl/article/view/4638 ; Nielsen 2006 https://www.nngroup.com/articles/progressive-disclosure/ ; diagnostic-competence meta https://doi.org/10.1007/s10648-019-09492-2 (g = 0.39); calculus spacing https://doi.org/10.1007/s10648-022-09677-2 ; STEM spacing meta https://doi.org/10.1186/s40594-024-00468-5.

## Appendix A. Search log (SearXNG science sweep)

- Instance: http://127.0.0.1:8888 (`categories=science`; engines include arxiv, crossref, pubmed, openalex, semantic scholar, wikipedia/wikimedia).
- Q1 `Vygotsky zone of proximal development definition scaffolding` → ZPD page, van de Pol 2010 review, ZPTD/DA papers.
- Q2 `Wood Bruner Ross 1976 scaffolding tutoring fading contingency` → van de Pol decade review; scaffolding-condition classroom experiment (N=768); uptake/mediation study.
- Q3 `Bjork desirable difficulties retrieval practice spacing effect size meta-analysis` → Bjork lab review; Annual Review retrieval (Karpicke/Roediger); calculus spacing replication; health-professions systematic review (56 articles, 63 experiments, 43 positive).
- Q4 `expertise reversal effect Kalyuga worked example guidance fading cognitive load` → Kalyuga/IGI chapter; CLT health-professions guideline review (15 guidelines); Sweller 20-year CLT update.
- Q5 `progressive disclosure HCI minimal manual Carroll Rosson training wheels` → weak science-category recall; pivoted to direct fetch of Nielsen 2006 (authoritative practitioner source).
- Q6 `guidance fading effect Renkl Atkinson completion problems cognitive load meta-analysis` → guidance-fading encyclopedia entry; Renkl/Atkinson transition paper; diagnostic-competence meta (g = 0.39).
- Q7–Q10 (second wave, effect-size targeting): scaffolding online meta (Doo et al. 2020); testing-effect field studies; worked-example vs generation boundary (Chen et al.); Nielsen usability criteria.
- Extraction: full-page reads of ZPD, Desirable difficulty, Expertise reversal, Instructional scaffolding (Wikipedia) + Nielsen progressive disclosure; DOI sources via snippet + abstract metadata (paywalled full texts not extracted — flagged in G2).

## Appendix B. Source authority table

| Source | Tier | Used for |
|---|---|---|
| en.wikipedia ZPD / scaffolding / desirable difficulty / expertise reversal | authoritative (tertiary) | construct definitions, reference lists |
| van de Pol et al. 2010 (Educ. Psych. Rev.) | authoritative (peer-reviewed review) | scaffolding trinity, measurement problem |
| Kirschner/Sweller/Clark 2006 (Educ. Psychologist) | authoritative | minimal-guidance verdict |
| Sweller et al. 2019 (Educ. Psych. Rev.) | authoritative | CLT update, worked-example/fading |
| Chen et al. (J. Educ. Psych., element interactivity) | authoritative | worked-example/generation boundary |
| Karpicke/Roediger Annual Review; Bjork lab PDFs | authoritative | retrieval/testing effect |
| Doo et al. 2020 (IRRODL meta) | authoritative | scaffolding online effect |
| Diagnostic-competence meta 2019 (Frontline Learning Res.) | authoritative | g = 0.39 moderator |
| Nielsen 2006 (NN/g) | community-authoritative (practitioner) | progressive disclosure |
| SearXNG snippets for paywalled DOIs | leads only | direction claims, marked LIKELY |

## Appendix C. Plain-words glossary (protocol-ready phrasing)

- ZPD: the work you can do with help but not alone.
- Contingency: help that fits what you just showed.
- Fading: help that shrinks as you succeed.
- Transfer: you doing it alone at the end.
- Worked example: a shown solution you study before solving.
- Completion problem: a half-solved problem you finish.
- Expertise reversal: help that used to help now gets in the way.
- Desirable difficulty: harder practice that pays off later — only if you can still succeed.
- Retrieval practice: closing the book and recalling.
- Spacing: spreading practice over days, mixing topics.
- Progressive disclosure: showing the basics now, the rest behind one clear button.
- Dynamic assessment: test, help, retest — measuring how you respond to help.

## Appendix D. Pacing-law sketch (stipulation, UNVERIFIED numbers)

What the evidence above authorises, in operational form for T-051..T-054 drafters:

1. Diagnose before teaching: one probe task, no help; record unaided level.
2. Step up once: assign the smallest task the learner fails alone but passes with help.
3. Help contingently: model → hint → question, stopping at the least help that works.
4. Fade on proof: remove one worked step per demonstrated success (study → complete → solve).
5. Drill with difficulty: recall-first practice, spaced and interleaved, feedback always.
6. Gate difficulty on reach: failed recall → step down before repeating; never drill failure.
7. Show little: core guidance visible, advanced guidance behind one labelled step.
8. Transfer explicitly: end each cycle with an unaided rep; assistance score must trend to zero.

Calibration warnings (why numbers stay UNVERIFIED):

- 'Slightly above' has no unit in ZPD literature; step 2's 'smallest failing task' is a procedure, not a constant.
- Fade rate (step 4) interacts with solo working time (§4, N=768 study) — no universal schedule exists.
- Retrieval/spacing dosages come from semester-scale classroom trials, not agent dialogue turns; transfer across settings is assumed, not shown.
- If the protocol needs numbers (e.g. hint budgets, retry caps), treat them as local conventions with review dates, not empirical constants.

## Appendix E. Threats and limits of this sweep

- Paywalled full texts (van de Pol, Kalyuga, Renkl/Atkinson, Chen) were judged from abstracts, snippets, and tertiary summaries — direction CONFIRMED, magnitudes LIKELY at best.
- SearXNG science recall missed HCI sources (Q5); Nielsen is practitioner-grade, included transparently as such.
- Reference shape (`prompt-engineering-science.md`) was absent from the workspace; sectioning follows the task spec, not a verified template.
- No git commit made; the new file is currently git-ignored (`/*` root ignore) — surfacing it (allow-listing, INDEX row) is the author's call.

_End of sweep. Hand to T-051..T-054 drafters: encode P1–P3, refuse N1–N5, resolve G1–G5 before freezing numbers._
_End of sweep. Hand to T-051..T-054 drafters: encode P1–P3, refuse N1–N5, resolve G1–G5 before freezing numbers._

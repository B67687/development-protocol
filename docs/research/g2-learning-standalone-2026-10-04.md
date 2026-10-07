# LANDSCAPE Research: Human Learning Standalone Sweep (THOUGHTS T-015 + T-018)

- **Date:** 2026-10-04
- **Thoughts:** T-015 — must the protocol make human learning cheap (science of learning), not produce learned helplessness? · T-018 — full research on human learning in general
- **Core pattern:** F-017 shipped learning deltas as inline citations (testing effect, self-explanation, generation effect, productive failure, expertise reversal, metacognitive decoupling) without a saved synthesis file. This sweep is that file: the durable evidence base behind F-017 plus everything the inline citations miss.
- **Method:** SearXNG self-hosted (127.0.0.1:8888, categories=science + general follow-up), page extraction attempted (bot-blocked on 2/2 targets; snippets only), confidence labels per claim.
- **Confidence scheme:** CONFIRMED = replicated/meta-analytic or canonical primary source; LIKELY = single-study or strong convergent theory; CONTESTED = live literature dispute with evidence both sides; UNVERIFIED = plausible, needs verification.

## 1. Problem Statement

T-015 asks whether the protocol makes learning cheap or manufactures learned helplessness (the Primeagen worry: the agent does everything, the human learns nothing and stops trying). T-018 asks for full general research on human learning. Both were BUILT without a saved sweep: F-017 cites Roediger & Karpicke 2006 (testing effect), Chi 1989 (self-explanation), the generation effect (Wikipedia-verified), then three sweep deltas (c981891) — metacognitive decoupling under LLM use (arXiv:2603.29681), expertise-reversal guidance calibration (CLT), productive failure (Kapur) — plus guidance meta-analysis d=0.50–0.71 and a "interleaving/deliberate-practice already covered, no change" note. The research question for this standalone file: what does that inline set miss that a saved synthesis would hold — effect sizes with numbers, boundary conditions, the contested zones (deliberate practice), the feedback literature, and the sequencing principle that ties F-017's mechanisms together?

## 2. Field Landscape

Eight sub-fields surveyed; retrieval/spacing strong, deliberate-practice contested, AI-sequencing nascent (itself a finding).

| # | Sub-field | Key anchor | Strength |
|---|-----------|-----------|----------|
| 1 | Desirable difficulties framework | Bjork & Bjork; DDF vs CLT integration model (QJE 2025, https://doi.org/10.1177/17470218241308143) | CONFIRMED (theory), LIKELY (boundary numbers) |
| 2 | Retrieval / testing effect | Roediger & Karpicke 2006; Karpicke & Roediger 2008 Science; Rowland review (https://doi.org/10.1146/annurev-psych-010419-051019) | CONFIRMED |
| 3 | Spacing / distributed practice | Cepeda et al. 2008 (N>1,350, gaps to 3.5mo, test to 1yr); classroom meta 2025 d=0.54 (https://doi.org/10.3390/bs15060771); calculus spaced-retrieval (https://doi.org/10.1007/s10648-022-09677-2) | CONFIRMED |
| 4 | Interleaving vs blocking | Rohrer school; spacing-vs-interleaving dissociation review (https://doi.org/10.1007/s10648-021-09613-w) | LIKELY (moderators active) |
| 5 | Elaboration (self-explanation, elaborative interrogation) | Chi 1989; Dunlosky et al. 2013 taxonomy; tutorial review six strategies (https://doi.org/10.1186/s41235-017-0087-y) | CONFIRMED (taxonomy), LIKELY (sizes) |
| 6 | Deliberate practice | Ericsson; music meta rc=0.61 (https://doi.org/10.3389/fpsyg.2014.00646); falsifiability critique (https://doi.org/10.3389/fpsyg.2020.01134); individualization rebuttal (https://doi.org/10.1007/s12144-021-02326-x) | CONTESTED |
| 7 | Feedback | Hattie & Timperley 2007 (canonical, numbers UNVERIFIED here); text-feedback meta g+=0.35, N=6,124 (https://doi.org/10.1016/j.edurev.2019.100296); HE systematic review (https://doi.org/10.1002/rev3.3292) | CONFIRMED (direction), LIKELY (sizes) |
| 8 | Learned helplessness / AI cognitive offloading | Seligman (general search only); RIF retrieval-interruption framework 2025 (https://doi.org/10.3390/educsci16081179) | LIKELY (RIF as synthesis), UNVERIFIED (helplessness transfer) |

## 3. Top Findings (with confidence + effect sizes)

**F1. Retrieval practice beats restudy for long-term retention, across materials, ages, and test formats. (CONFIRMED, medium-to-large)**
Annual-review synthesis (search-returned, https://doi.org/10.1146/annurev-psych-010419-051019): practicing retrieval shortly after learning slows forgetting across material types, ages, learner abilities, and test types, and transfers to classrooms. The F-017 inline citation (Roediger & Karpicke 2006) is therefore the tip of a replicated mass, not a single study. Exact pooled g NOT snippet-verified in this sweep — left UNVERIFIED rather than asserted (see G2). Protocol corollary: explain-back at ratification is a retrieval rep, and echo-fails-ledger-checked is exactly the "retrieval with feedback" condition the literature endorses.

**F2. Spacing wins over massing in real classrooms too: d=0.54 [0.31, 0.77]. (CONFIRMED, applied meta)**
2025 classroom meta (22 reports, 31 effect sizes, N>3,000; https://doi.org/10.3390/bs15060771): moderate effect favoring distributed over massed practice; larger with longer retention intervals, higher education levels, fewer re-exposures. Convergent mechanism study: spaced calculus practice raised end-of-semester retention while *lowering* practice-quiz scores (https://doi.org/10.1007/s10648-022-09677-2) — the desirable-difficulty signature (worse now, better later). Protocol corollary: per-cycle learning counts (REFLECT Q9) should be read across cycles, never within one; a cycle that feels fluent is massing, not learning.

**F3. The optimal gap scales with the retention interval: ~20–40% of a 1-week delay, ~5–10% of a 1-year delay. (CONFIRMED single large study, N>1,350)**
Cepeda et al. 2008 (https://journals.sagepub.com/doi/10.1111/j.1467-9280.2008.02209.x): interstudy gap first raised then reduced final performance at every test delay; optimal gap grew in absolute terms but shrank proportionally. Implication: most educational scheduling is highly inefficient (authors' own conclusion). Protocol corollary: continuation-seed spacing (REFLECT Q9 → next INBOX) should widen with the horizon the knowledge must survive — same-cycle recap is near-useless for durable retention.

**F4. The strategy taxonomy is settled at the coarse grain: testing + distributed practice = high utility; elaborative interrogation + self-explanation + interleaving = moderate; rereading/highlighting = low. (CONFIRMED taxonomy, LIKELY sizes)**
Tutorial review of six evidence-backed strategies — spaced practice, interleaving, retrieval practice, elaboration, concrete examples, dual coding (https://doi.org/10.1186/s41235-017-0087-y) — converges with the Dunlosky et al. 2013 utility ratings. Nothing in F-017 contradicts this; what F-017 misses is the *low-utility* half: the protocol currently never tells the user what NOT to do (reread the spec, highlight the plan). The cheap-learning move is often subtractive.

**F5. Feedback helps learning from text at g+=0.35 — and elaboration matters more than correctness alone. (LIKELY, 104 contrasts, N=6,124)**
Meta-analysis (https://doi.org/10.1016/j.edurev.2019.100296): feedback after reading beats no feedback at g+=0.35; elaborate and knowledge-of-correct-response feedback beat mere knowledge-of-response when given after reading; computer-delivered beats non-computer-delivered. HE systematic review concurs that low-stakes quizzing is the most powerful tested approach (https://doi.org/10.1002/rev3.3292). Hattie-scale numbers (d≈0.7) are canonical but NOT snippet-verified in this sweep — UNVERIFIED here (see G3). Protocol corollary: the abdication tripwire's "pointed question" should come *with* elaborate feedback after the answer, not as a bare correctness check.

**F6. Deliberate practice is real, large in-domain (music rc=0.61 [0.54,0.67], N=788), but its scope is CONTESTED — individualization decides how large. (CONTESTED)**
For: first music-domain meta-analysis finds rc=0.61 (https://doi.org/10.3389/fpsyg.2014.00646). Against: 25-year review documents shifting definitions and unfalsifiable edges in Ericsson's program (https://doi.org/10.3389/fpsyg.2020.01134). Reconciliation (LIKELY): chess study (N=178) finds structured practice predicts 3× better at high individualization/quality than at average (https://doi.org/10.1007/s12144-021-02326-x) — prior metas underestimated DP by pooling non-individualized practice. Protocol corollary: F-017's "interleaving/deliberate-practice already covered, no change" note (c981891) is the one inline claim this sweep REFINES — deliberate practice is not covered by anything in the protocol, and its active ingredient (individualized, feedback-rich reps at the edge) is exactly what per-cycle learning counts do not yet enforce.

**F7. AI assistance must preserve retrieval before displacing it: learner-first sequencing is the integrative principle. (LIKELY, conceptual synthesis)**
The Retrieval Interruption Framework (2025; https://doi.org/10.3390/educsci16081179) synthesizes retrieval practice, productive failure, scaffolding, cognitive load/offloading, metacognition, expertise reversal, and the assistance dilemma into one prediction: retrieval-*displacing* assistance (answer before attempt) boosts immediate performance but weakens delayed recall, explanation quality, transfer, and calibration — especially for conceptually demanding tasks and low-prior-knowledge learners; early guidance stays productive only when it cuts extraneous load, fades, and is followed by independent performance. This single framework retro-justifies three separate F-017 mechanisms (attempt-before-structure, fading, explain-back) as one sequencing rule, and it is the missing theoretical roof the inline citations never name.

**Supporting (LIKELY/UNVERIFIED):**
- S1. DDF↔CLT integration: raise difficulty for low-element-interactivity material, lower it for high-interactivity material; calibrate by expertise (https://doi.org/10.1177/17470218241308143) — LIKELY; this is the rule that unifies desirable difficulties (F1–F4) with expertise reversal (already in INBOX).
- S2. Spaced and interleaved practice need distinct theories (working-memory depletion/recovery vs discriminative contrast; https://doi.org/10.1007/s10648-021-09613-w) — LIKELY; do not treat interleaving as "spacing with variety" when designing reps.
- S3. Learned-helplessness transfer (Seligman dogs → LLM-era passivity) — UNVERIFIED mechanism; the protocol's tripwire fires on behavior (arXiv:2603.29681), which is the correct operationalization regardless of whether "helplessness" is the right name.

## 4. Design Principles for T-015/T-018 (encode these)

**P1. Attempt before answer — retrieval-preserving sequencing is the roof rule.**
RIF (F7) subsumes the three inline mechanisms: every agent explanation, structure, or solution must follow a learner attempt (recall, self-explanation, representation, generation). EXTRACTION's attempt-before-structure (Kapur) is one instance; extend the rule to STRATEGY ratification and EXECUTOR outputs: no new concept arrives explained-first.

**P2. Space the reps across cycles; never trust within-cycle fluency.**
F2's desirable-difficulty signature (worse practice scores, better retention) plus F3's gap scaling: schedule the same retrieval (standing decisions, explain-back) across cycles with widening gaps, and read learning counts only as cross-cycle slopes. Same-cycle recap is massing theater.

**P3. Feedback after every rep: elaborated, immediate, and machine-delivered.**
F5 (g+=0.35): follow each tripwire question and explain-back with *why* the answer is right/wrong, immediately, in the channel itself. Bare correctness ("noted") wastes most of the effect.

**P4. Calibrate difficulty to expertise — desirable inside reach, support outside it.**
S1 + expertise reversal: novices get worked examples and full structure; experts get blanks and light touch (excess guidance backfires). The INBOX Knowledge-cell calibration already does this for guidance; extend it to learning reps — the same question is desirable difficulty for one stance row and overload for another.

**P5. Teach subtraction: name the low-utility strategies as anti-patterns.**
F4's settled half names what fails (rereading, highlighting, massed recap). The protocol should say so explicitly at least once (e.g., REFLECT: "rereading this record is not studying it — close it and restate what changed").

## 5. Not-to-Encode (explicit non-goals)

- **N1. Do not build testing theater.** Forced quizzes on every gate would convert retrieval into compliance ritual; reps attach to commitment points (ratification, tripwire, Q9), not to every turn.
- **N2. Do not encode the 10,000-hour myth or any fixed practice quantity.** F6 is contested precisely on dose claims; encode individualization and feedback-richness, never hours.
- **N3. Do not let feedback become praise or grades.** The HE review finds mixed evidence for praise/grading (F5 sources); feedback here means elaborated correctness information, not encouragement or scores.
- **N4. Do not diagnose helplessness, depression, or any trait from passivity.** The tripwire fires on behavior counts (3+ passive cycles), never on a label — S3 stays UNVERIFIED and must not leak into gate language.
- **N5. Do not re-litigate guidance scaffolding.** Scaffolding/fading/expertise-reversal mechanics live in guidance-dynamic-2026-09-30.md; this file owns reps, spacing, feedback, and sequencing only.

## 6. Open Gaps

- **G1.** Dunlosky et al. 2013 full text unretrieved — utility ratings above rest on the tutorial-review convergence (F4), not on the primary. Retrieve via institutional access before quoting moderator details.
- **G2.** Testing-effect pooled numbers unverified — Rowland review page blocked; no pooled g asserted here. Targeted retrieval: Adesope et al. meta-analytic g.
- **G3.** Hattie feedback numbers unverified — the famous d≈0.7 is deliberately NOT asserted; verify against Hattie & Timperley 2007 / Visible Learning before any quantitative claim ships.
- **G4.** Deliberate-practice adjudication open — Macnamara-side variance-explained numbers vs individualization rebuttal (F6) needs a primary decisão; until then the protocol claims only "individualized reps," never a dose–performance function.
- **G5.** Interleaving transfer to protocol work unmeasured — S2 warns spacing≠interleaving; whether interleaved *project topics* within a run helps or just adds load is untested.
- **G6.** Adult professional (non-student) samples thin — F2's classroom meta skews school/HE; protocol users are working adults. Treat education-level moderation as a humility flag on every size claim.

## 7. Sources (search-returned URLs only — no invented links)

1. https://doi.org/10.1146/annurev-psych-010419-051019 — Retrieval practice annual review (F1).
2. https://doi.org/10.3390/bs15060771 — Distributed-practice classroom meta, d=0.54 (F2).
3. https://doi.org/10.1007/s10648-022-09677-2 — Spaced retrieval imposes desirable difficulty in calculus (F2).
4. https://journals.sagepub.com/doi/10.1111/j.1467-9280.2008.02209.x — Cepeda spacing-gap × retention-interval (F3).
5. https://doi.org/10.1186/s41235-017-0087-y — Six learning strategies tutorial review (F4).
6. https://doi.org/10.1016/j.edurev.2019.100296 — Feedback-on-text meta, g+=0.35 (F5).
7. https://doi.org/10.1002/rev3.3292 — HE formative-assessment/feedback systematic review (F5).
8. https://doi.org/10.3389/fpsyg.2014.00646 — Deliberate practice in music meta, rc=0.61 (F6).
9. https://doi.org/10.3389/fpsyg.2020.01134 — Deliberate-practice falsifiability critique (F6).
10. https://doi.org/10.1007/s12144-021-02326-x — Individualization rebuttal, chess N=178 (F6).
11. https://doi.org/10.3390/educsci16081179 — Retrieval Interruption Framework (F7; snippet-verified, page blocked).
12. https://doi.org/10.1177/17470218241308143 — DDF vs CLT difficulty integration model (S1).
13. https://doi.org/10.1007/s10648-021-09613-w — Spacing/interleaving distinct-bases review (S2).
14. https://doi.apa.org/doi/10.1016/j.jarmac.2020.09.003 — Desirable difficulties in theory and practice (landscape anchor).
15. https://doi.org/10.3758/s13421-014-0499-6 — Retrieval + spacing in young/older adults (landscape anchor).

## 8. Provenance & Next Step

- **Engines:** SearXNG @127.0.0.1:8888 (categories=science, then general); page extraction attempted on 2/2 targets, hard-blocked (bot detection / JS shell). All DOIs above are search-returned, none invented.
- **INDEX.md:** untouched per instructions. No git operations performed.
- **F-017 delta:** this file CONFIRMS the inline citations (testing, self-explanation, generation, productive failure, expertise reversal, decoupling all survive) and REFINES one note — deliberate practice is not "already covered" (F6); its individualized-rep ingredient is unenforced. It ADDS the RIF sequencing roof (F7), five numbered sizes (F2/F3/F5/F6), the subtraction list (F4), and six gaps.
- **Next:** P2b fold decision — encode P1–P5 + N1–N5 against F-017's four mechanism sites (SERIOUSNESS tripwire, STRATEGY standing-decisions + explain-back, REFLECT Q9 counts); close G1–G3 via institutional access before any numeric effect size enters protocol text.

> **Proposed INDEX row (exact text, do not insert without owner go):**
> `| g2-learning-standalone-2026-10-04.md | 2026-10-04 | T-015+T-018 standalone human-learning sweep: retrieval CONFIRMED, spacing d=0.54, gap-scaling rule, feedback g+=0.35, deliberate practice CONTESTED, RIF sequencing roof; P1-P5 + N1-N5, 6 gaps. |`

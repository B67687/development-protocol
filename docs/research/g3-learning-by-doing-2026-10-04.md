# G3: Learning-by-Doing Spine — What Evidence Governs Keeping a Build Inside Every Learning Loop?

**Date:** 2026-10-04
**Slot:** G3 (learning-by-doing spine)
**Question:** What evidence governs keeping a build inside every learning loop?
**Method:** SearXNG `science` + `general` categories; full-text extraction via Trafilatura sidecar `POST 127.0.0.1:8081/extract`.
**Retro-inventory note:** Searched repo for "retro-inventory G3" (`grep -ril` over `.omo/`, `docs/`; listed `.omo/goal/`, `.omo/archive/sweeps/INDEX.md`, `docs/research/`, `.omo/plans/`). No file literally named retro-inventory G3 was found; `docs/research/` is empty. This brief therefore answers the G3 spine question directly from the literature and ties it to the repo's learning-track build (Dijkstra from zero, reviewed 2026-10-06).

---

## 1. Question and scope

G3 asks whether **every learning loop must contain a build** — a tangible artifact the learner makes, runs, traces, or fixes — rather than ending at reading, watching, or re-studying. "Build" here is deliberately broad: a coded implementation, a hand-trace table, a worked derivation, a small experiment. The scope is individual concept learning (the learning-track unit: one algorithm from zero), not whole-course curriculum design. The verdict sought is not "building feels good" but **what causal evidence, with what boundary conditions, justifies making the build non-optional** in each loop.

## 2. TL;DR verdict — keep the build, but govern it

The literature converges: **doing with reflection reliably beats passive study, attempting before instruction reliably beats instruction-then-practice on conceptual understanding and transfer, and retrieval through production beats re-study on retention** — but each effect has documented failure modes (unproductive failure without scaffolding/contrast, nulls outside STEM, engagement-dependent testing effects). So the rule is: **keep a build in every loop, keep it small, attempt-first, reflect-after, and gate progress on the artifact** (trace-before-run, byte-match, assert). The build earns its place only when it forces generation, surfaces a gap, and is followed by instruction/feedback — a build without reflection is just activity.

## 3. Constructionism: learning is strongest when the learner constructs a meaningful product

Seymour Papert's constructionism (proposal to NSF; *Mindstorms*, 1980; Papert & Harel, *Situating Constructionism*) extends Piagetian constructivism with one operational claim: learning is most effective when it is part of an activity the learner experiences as **constructing a meaningful product** — a tangible object in the world, not just a mental model. The extracted definition is explicit: students learn through participation in project-based making, connecting ideas, with the teacher as coach rather than lecturer (extracted via sidecar from `https://en.wikipedia.org/wiki/Constructionism_(learning_theory)`).

What this governs for G3:

- Every loop needs a **public artifact** (code + trace + timing table), because the artifact is what makes thinking inspectable — by the learner and by the loop's own checks.
- The product must be **personally meaningful and runnable**, not a fill-in-the-blank: the repo's Dijkstra track (own `dijkstra.py`, own 8-pop trace, own timing) satisfies this; a copied cell does not.
- Papert and Harel warn constructionism is "much richer… than any learning-by-making formula" — the build alone is insufficient; coaching, sharing, and debugging the artifact are part of the mechanism. A build step with no review pass is cargo-cult constructionism.

## 4. Experiential learning cycle: doing without reflection does not count

Dewey → Lewin → Piaget → Kolb (1970s–80s) formalize the loop as **concrete experience → reflective observation → abstract conceptualization → active experimentation**. The extracted definition draws the line sharply: experiential learning is "learning through reflection on doing"; hands-on activity that never reflects on its product is *not* experiential learning (extracted via sidecar from `https://en.wikipedia.org/wiki/Experiential_learning`). Aristotle's axiom ("we learn by doing them," *Nicomachean Ethics* ~350 BC) is the ancient version of the same claim.

What this governs for G3:

- The loop shape is **do → reflect → conceptualize → re-test**, and the reflection step must be written, not mental: the track's Ch5 gate ("reconcile trace vs. run before §6") is exactly reflective observation made procedural.
- Senge's gloss (*The Fifth Discipline*) adds the motivation constraint: the learner must want the knowledge, so each loop must show the direction (why this build matters) before demanding the build.
- Failure mode: activity without debrief. Any loop whose build has no compare-and-explain step (expected vs. actual, pushes/pops accounting) should be scored as incomplete.

## 5. Active learning: doing beats listening at scale (Freeman et al. 2014 and successors)

The large-scale STEM meta-analysis (Freeman et al., *PNAS* 2014, widely cited as "active learning increases performance / cuts failure rates") found students in active-learning sections outperform lecture sections on exams and concept inventories, with substantially lower failure rates. SearXNG `general` results echo the headline numbers practitioners quote (~+6% grades, large failure-rate reductions; e.g. UWTSD case study citing Freeman 2014). Bonwell & Eison's canonical definition ("doing something besides passively listening"), Hanson & Moser on outcomes, and the KSA/taxonomy framing (read, write, discuss, solve — analysis/synthesis/evaluation, not just listening) all point the same way (extracted via sidecar from `https://en.wikipedia.org/wiki/Active_learning`).

What this governs for G3:

- The **null hypothesis is lecture-then-quiz**; the build loop must beat it, not merely accompany it. Keep the build as the primary contact with the idea, with prose as the sidecar — the track's "plain-Jupyter, stdlib-only, no video" constraint is consistent with this.
- Effect sizes come from **higher-order tasks** (analyze, synthesize, evaluate), so trivial builds (retype this code) do not inherit Freeman's warrant; the build must require a decision (representation choice, invariant, complexity accounting).
- Cost note: students and teachers both report adaptation difficulty. Loops must stay small (one concept, one fixture, one gate) or the active load becomes the bottleneck — the 12-page / 7-chapter budget is a load control, not just a format.

## 6. Productive failure / PS-I: attempt first, then instruct — with boundary conditions

Manu Kapur's productive failure (PF) / problem-solving-before-instruction (PS-I) program is the most direct warrant for "build before explanation": learners first attempt a complex problem (and typically fail), then receive instruction, and outperform direct-instruction-first groups on **conceptual understanding and transfer** (Kapur; `doi:10.1080/23735082.2015.1002195`; `doi:10.1080/00461520.2016.1155457`; Sinha & Kapur meta-analysis; process-engineering LH vs. HL replication, `doi:10.1111/bjet.12492`; childhood-programming tinkering study, `doi:10.1145/3615430.3615432`). A recent comparative meta-analysis of preparatory approaches (118 comparisons, 33 articles; `https://osf.io/83p7e_v1`) finds PS-I favored over alternative-sensemaking preparation, with Bayesian support for at least a moderate advantage at meaningful probability, and larger bias-adjusted effects — while finding essentially no difference between unscaffolded and scaffolded PS-I variants.

Boundary conditions found in the same sweep (these govern the "keep" rule):

- **Domain transfer is not guaranteed.** A two-study quasi-experiment (N=212, N=152, 10th graders) on social-science research methods found PF did *not* outperform direct instruction — a clean non-replication outside STEM (`doi:10.1007/s11251-020-09525-2`). For algorithm learning (STEM, well-defined representations) the warrant holds; do not generalize it to opinion or methods-writing loops without new evidence.
- **Failure must be productive, not merely experienced.** Kapur's conditions (multiple representations attempted, contrast across student solutions, instruction that builds on those attempts) are load-bearing. Unproductive failure (attempt → correct answer posted, no contrast) shows weak or no benefit.
- **Sequence is the intervention.** The process-engineering study isolates sequence (low-to-high vs. high-to-low) as the only difference and still finds higher conceptual/transfer gains — so G3's "trace-by-hand-BEFORE-run" ordering is not decoration; it is the mechanism.

## 7. Retrieval, generation, and deliberate practice: the build as a memory and skill mechanism

Three adjacent literatures explain *why* the build works when it works:

- **Testing / retrieval practice (Roediger et al.; forward effect, `10.3389/fpsyg.2014.00286`).** Retrieving through testing beats re-studying on delayed retention, and successful retrieval potentiates subsequent new learning. A build that requires recalling the invariant (e.g., "pops are final") is a retrieval event; re-reading the chapter is not. Caveat from a 2026 Prolific replication pair (`10.3389/fpsyg.2026.1727423`): the testing effect vanishes under low-engagement crowdsourced conditions despite feedback and delayed tests — engagement is a moderator, so unsupervised loops need attention checks (gates), not just content.
- **Generation effect and ICAP (Chi).** Constructive/interactive modes (explain, generate, contrast) outperform active/passive modes. Copying a trace is active; generating it from the code and then explaining each stale-skip is constructive. The loop must demand generation, not transcription.
- **Deliberate practice (Ericsson) and Kolb/Miller lenses on clinical training (`10.3389/fmed.2026.1890078`).** Competence grows through repeated supervised practice with specific, timely feedback — WBAs, timely supervisor feedback, problem/scenario-based learning — while simulation/technology "may support learning… but cannot substitute" for the real performance. Translated: notebook cells and PDFs support the build; they do not substitute for running, breaking, and fixing the build with feedback (asserts, stats `pushes == 8, pops == 8`).

## 8. Design rules for learning loops + sources and method

**Keep-rules (each is falsifiable):**

1. **One loop, one build, one gate.** No chapter closes without its artifact (code runs, trace matches, asserts pass). The Dijkstra track's triple equality (`EXPECTED_DIST` == PDF table == Cell 5 == runtime `[0, 3, 1, 4, 7]`, 8 pushes / 8 pops) is the template.
2. **Attempt before exposition inside the loop.** Hand-trace before run; predict pop order before reading the answer (Ch5 gate). Sequence is the treatment (§6).
3. **Reflection is written.** Every build ends with a contrast line: what I predicted vs. what ran, and why (stale skips, heap order). No contrast, no credit (§4).
4. **Feedback is immediate and specific.** Asserts, counts, and byte-match — not "looks right." Supervisor-equivalent: the fixture plus `stats` check (§7).
5. **Load-controlled.** One fixture (5-node, E=7, max out-degree 2), stdlib-only, fixed notation (`dist[]`, 0-indexed, `∞`). If Ch5 needs 2 pages, waive the sub-cap explicitly rather than bloating every loop (per 2026-10-06 review FAIL 2.8).
6. **Cite-plus-quote definitions.** The review's MEDIUM gap (no lecture-section citations, Ch1–Ch4) stays open: each loop's definitions need one authoritative cite with section number and a one-line quote, or definition drift returns.

**Drop-rules (when to remove or shrink a build):** outside STEM-concept terrain without fresh PF evidence; when the build is transcription rather than generation; when engagement cannot be verified (then add a gate before adding content).

**Sources consulted (SearXNG science+general; extractor 127.0.0.1:8081):**

- Constructionism definition + Papert/Harel "richer than learning-by-making" — sidecar extract of `https://en.wikipedia.org/wiki/Constructionism_(learning_theory)`; Smart Learning constructionism survey — `http://arxiv.org/abs/2501.07486v1`.
- Experiential learning definition + Kolb/Dewey/Lewin/Piaget lineage + reflection-on-doing criterion — sidecar extract of `https://en.wikipedia.org/wiki/Experiential_learning`; Kolb overview — `https://learning-theories.com/experiential-learning-kolb.html`.
- Active learning definition + Bonwell & Eison / Hanson & Moser / KSA framing — sidecar extract of `https://en.wikipedia.org/wiki/Active_learning`; Freeman-2014-citing case — `https://idns.co.uk/case-studies/university-of-wales-trinity-st-david/`.
- Productive failure core — `http://www.tandfonline.com/doi/full/10.1080/23735082.2015.1002195`; four-way PF/PS framework — `http://www.tandfonline.com/doi/full/10.1080/00461520.2016.1155457`; vicarious vs. direct PF — `http://www.tandfonline.com/doi/abs/10.1080/10508406.2013.819000`; PF book — `https://onlinelibrary.wiley.com/doi/book/10.1002/9781394308712`; tinkering/PF in programming — `https://dl.acm.org/doi/10.1145/3615430.3615432`; PF sequence in engineering — `https://doi.org/10.1111/bjet.12492`; PF-in-STEM application review — `http://dx.doi.org/10.22158/jecs.v8n4p86`.
- Non-STEM null — `https://doi.org/10.1007/s11251-020-09525-2`; preparatory-approaches meta-analysis (118 comparisons) — `https://osf.io/83p7e_v1`.
- Retrieval/forward testing — `http://journal.frontiersin.org/article/10.3389/fpsyg.2014.00286/abstract`; testing-effect null under low engagement — `https://www.frontiersin.org/articles/10.3389/fpsyg.2026.1727423/full`; deliberate-practice/experiential/feedback synthesis — `https://doi.org/10.3389/fmed.2026.1890078`.
- Repo-internal anchors: `.omo/reviews/review-2026-10-06-learning-track.md` (42/45 PASS; FAILs 2.8/Ch5 length, 4.2/V2 notebook execution environment, 5.5/lecture citations); `learning-track.pdf` / `learning-track.ipynb` / `dijkstra.py` triple-equality evidence.

**Method note:** SearXNG instance `http://127.0.0.1:8888` (science + general categories, EN); extraction `POST http://127.0.0.1:8081/extract` verified working for Wikipedia pages during this session. Paywalled DOI pages were cited from SearXNG snippets/abstracts, not full-text extracted. No `INDEX.md` touched; no git operations performed.

# LANDSCAPE Research: Just-Right over Perfection + Never-Perfection Balancing (THOUGHTS T-062 / T-064)

- **Date:** 2026-10-03
- **Thoughts:** T-062 — just-right over perfection; T-064 — never perfection / balancing
- **Core pattern:** A thinking agent must satisfice, not optimize: stop at "just-right" (good-enough for the actual goal) rather than polishing toward perfection, and treat "felt over-do" (the subjective sense of gilding, gold-plating, second-guessing past sufficiency) as a stop-signal. Balancing = holding quality against cost on every iteration, not maximizing one side.
- **Method:** SearXNG self-hosted (127.0.0.1:8888, categories=science first, then general follow-ups), Trafilatura extractor sidecar (127.0.0.1:8081) probes on 4 targets (2 full-verified, 2 partial), confidence labels per claim.
- **Confidence scheme:** CONFIRMED = replicated/meta-analytic or canonical primary source; LIKELY = single-study or strong convergent theory; UNVERIFIED = plausible, needs verification.

## 1. Problem Statement

T-062 observes a failure mode in agent (and human) work: past the point of sufficiency, extra effort goes into polishing rather than into the goal — longer answers, extra features, extra revisions that nobody asked for and nothing downstream consumes. T-064 is its balancing twin: quality and cost must be held against each other continuously ("never perfection" as a standing policy, not a one-time choice), because every unit of over-do is taken from somewhere else (other thoughts, other users, latency, attention). The research question: what does the literature say about satisficing vs optimizing, over-engineering, perfectionism, simplicity doctrines (YAGNI/KISS), and is there any empirical basis for a "just-right via felt over-do" stop rule?

## 2. Field Landscape

Six sub-fields surveyed; decision-science core strong, software-doctrine and stop-rule per se thin (itself a finding).

| # | Sub-field | Key anchor | Strength |
|---|-----------|-----------|----------|
| 1 | Bounded rationality / satisficing | Simon 1956 / Administrative Behavior 1947; Nobel speech; Gigerenzer moral satisficing | CONFIRMED (as canonical theory) |
| 2 | Maximizing vs satisficing & well-being | Schwartz et al. 2002; Röckl et al. cross-cultural (US N=307, W.Europe N=263, China N=218); Slovak two-component studies (n≈480–514) | CONFIRMED (effect exists), LIKELY (boundary conditions) |
| 3 | Choice overload | Chernev et al. 2015 conceptual review + meta-analysis (99 obs, N=7,202) | CONFIRMED (moderated effect) |
| 4 | Effort–accuracy / adaptive strategy selection | Payne, Bettman & Johnson Adaptive Decision Maker; Creyer et al. accuracy/effort-feedback study | LIKELY (theory CONFIRMED as framework; numeric ES snippet-only) |
| 5 | Perfectionism (clinical/personality) | Frost MPS; Hewitt & Flett MPS; Stoeber & Damian partialling (strivings vs concerns) | CONFIRMED (constructs/measures), LIKELY (differential adaptiveness) |
| 6 | Simplicity doctrines (engineering) | KISS (IxDF canon); YAGNI/DRY practitioner canon (ExceptionNotFound, acronym-fatigue essay) | LIKELY as community constructs, UNVERIFIED empirically |

## 3. Top Findings (with confidence + effect sizes)

**F1. Satisficing is the rational strategy under intractability or missing information — not a character flaw. (CONFIRMED, theory; no numeric ES)**
Simon (1956; concept in Administrative Behavior 1947; Nobel speech): decision makers satisfice by searching until an acceptability threshold is met, because many natural problems are computationally intractable or information-poor, precluding optimization. Nobel-speech formulation (extractor-verified from https://en.wikipedia.org/wiki/Satisficing): "decision makers can satisfice either by finding optimum solutions for a simplified world, or by finding satisfactory solutions for a more realistic world." Gigerenzer's moral-satisficing extension (https://doi.org/10.1111/j.1756-8765.2010.01094.x) adds the environment-relativity clause: heuristics are good/bad only relative to the environment. T-062/T-064 corollary: an agent with finite context, latency budget, and an underspecified goal is exactly in Simon's intractable/information-poor regime — satisficing with an explicit threshold is the correct policy, and "optimize" is the category error.

**F2. Maximizing as a *strategy* (endless alternative search) is maladaptive; maximizing as a *goal* (high standards) is neutral. Only the strategy needs a stop rule. (LIKELY, 2 converging single studies, direction-only ES)**
Slovak general-population studies (https://doi.org/10.1017/S1930297500007932, extractor-verified intro; preprint twin https://osf.io/zav9t_v1): alternative-search strategy positively related to depression and negatively to happiness, while high-standards goal showed no maladaptive relation (no relation with well-being at all); components differentially associated with personality. Numeric effect sizes not present in snippet/extract — direction only. T-062 corollary: the pathology is not "caring about quality" (high standards are fine) but *unbounded search past sufficiency*. The felt-over-do signal maps onto the strategy component, not the goal component — so the stop rule should target search continuation, never lower the standard itself.

**F3. Maximizing costs well-being via regret, but only where choice is culturally load-bearing. (CONFIRMED direction; cross-cultural N=788; mediation, no numeric ES in retrieved text)**
Röckl et al. (https://doi.org/10.1017/S1930297500003247, extractor-verified): in choice-abundant societies (US, Western Europe) maximizers reported less well-being than satisficers, mediated by experienced regret; in China maximizing was unrelated to well-being even though it still predicted regret. T-064 corollary: regret-over-imperfect-output is the affective tax on over-do, and it is culturally conditioned — a balancing policy cannot assume every user (or every task) prices regret the same. Encode regret-cost as a variable, not a constant.

**F4. "More options/effort helps" reverses under four moderators — the just-right point is conditional, not fixed. (CONFIRMED, meta-analysis 99 obs N=7,202; moderator significance reported, numeric pooled ES not in snippet)**
Chernev et al. choice-overload meta-analysis (https://myscp.onlinelibrary.wiley.com/doi/10.1016/j.jcps.2014.08.002): higher decision-task difficulty, choice-set complexity, preference uncertainty, and effort-minimizing goal each reliably facilitate overload; with moderators modeled, the overall assortment-size effect is significant (counter to the earlier null meta-analytic report). T-062 corollary: there is no universal "right amount" of thoroughness — the just-right threshold must move with task difficulty, option complexity, goal uncertainty, and whether the user wants effort minimized. A fixed verbosity/depth default will systematically over-do hard-to-judge cases and under-do easy ones.

**F5. People adapt strategy to effort–accuracy goals — so an explicit "accuracy vs effort" goal statement changes processing, while effort feedback alone does almost nothing. (LIKELY, single-study + canonical framework)**
Payne, Bettman & Johnson framework (http://www.dtic.mil/docs/citations/ADA205750 — record located, full text not extracted, UNVERIFIED details); Creyer et al. (https://onlinelibrary.wiley.com/doi/10.1002/bdm.3960030102, snippet): emphasizing accuracy produces more normative-like processing, emphasizing effort produces less extensive/more selective processing; explicit effort feedback had almost no impact. T-064 corollary: telling the agent (and the user) the current goal — "this pass is accuracy-weighted" vs "this pass is effort-weighted" — is the highest-leverage balancing intervention; passive cost meters (token counts, elapsed time) without a goal frame should be expected to change little.

**Supporting (LIKELY/UNVERIFIED):**
- S1. Perfectionistic strivings vs concerns must be partialled — concerns carry the harm signal (Stoeber & Damian https://doi.org/10.1016/j.paid.2016.08.039, snippet-only — LIKELY; twin https://linkinghub.elsevier.com/retrieve/pii/S0191886916309473). Supports F2's strategy/goal split at the trait level.
- S2. KISS as canon: "designs and/or systems should be as simple as possible… simplicity guarantees the greatest levels of user acceptance" (https://www.interaction-design.org/literature/topics/keep-it-simple-stupid, extractor-verified — LIKELY as doctrine, UNVERIFIED as causal claim; "guarantees" is canon-talk, not evidence-talk).
- S3. YAGNI/DRY/KISS practitioner triad ("You Aren't Gonna Need It" as the over-engineering brake; https://www.exceptionnotfound.net/kiss-dry-yagni-good-code-basic-training/; https://devz.cl/posts/acronym-fatigue-series-dry-kiss-yagni/ — CONFIRMED as stable community constructs, UNVERIFIED empirically).
- S4. Machine-checked satisficing (FFSD, Lean 4, threshold ε<1/2 uniqueness; http://arxiv.org/abs/2507.07052v1 — PREPRINT, capped at LIKELY): tolerance-threshold formalization exists and is mechanizable — a future stop-rule could be stated as a threshold parameter, not prose.
- S5. MPS instruments (Frost https://doi.apa.org/doi/10.1037/t05051-000; Hewitt–Flett https://doi.apa.org/doi/10.1037/t04592-000; FMPS Spanish validation n=582 https://pubmed.ncbi.nlm.nih.gov/21266154 — CONFIRMED as measurement infrastructure, not as findings).

## 4. Design Principles for T-062 / T-064 (encode these)

**P1. Satisfice against a written threshold, stated before the work starts (F1 + F5).**
Every non-trivial task opens with one sentence: what "good enough" means here and whether this pass is accuracy-weighted or effort-weighted. The goal frame — not a cost meter — drives processing (F5). No threshold, no start; the threshold is Simon's acceptability level made explicit.

**P2. Police the search strategy, never the standard (F2 + S1).**
High standards stay. What gets stopped is continued alternative-search past the threshold: extra options surveyed, extra revisions, extra features. Operationalize felt over-do as a strategy-stop question asked at fixed checkpoints ("am I still searching, or am I now gilding?"), because effort feedback alone does not self-correct (F5).

**P3. Move the just-right point with the four moderators (F4), and price regret per context (F3).**
Threshold-setting reads: task difficulty, option complexity, goal uncertainty, effort-minimization pressure. High on any → smaller scope per pass, earlier stop, explicit deferral list instead of continued grinding. Regret-pricing is contextual (F3): do not spend the same anti-regret effort on a throwaway draft as on a published decision.

## 5. Not-to-Encode (explicit non-goals)

- **N1. Do not encode "low standards" or anti-craft.** F2 is explicit: high-standards goals are not maladaptive. T-062 must never become permission for sloppy first passes — it is a stop rule for search continuation, not a quality ceiling.
- **N2. Do not fix the just-right point.** A universal depth/verbosity/length default violates F4 (moderated overload). Any constant (e.g., "always 3 options") will misfire on both tails.
- **N3. Do not diagnose perfectionism.** S1/MPS material is trait psychology (S5 instruments need validated administration); never label a user a perfectionist from chat text. Regret language only, no trait claims.
- **N4. Do not cite KISS/YAGNI as evidence.** S2/S3 are community doctrines (UNVERIFIED causally); use them as shorthands in prose, never as justifications in design rationale. Justifications cite F1–F5.
- **N5. Do not rely on cost meters alone.** F5: explicit effort feedback barely moves processing. A token counter without a goal frame is decoration.

## 6. Open Gaps

- **G1.** No direct empirical literature on a "felt over-do" stop signal as a named phenomenon — the construct is synthesized here from F2 (strategy) + F5 (goal framing); needs targeted search ("stopping rules", "information search termination", "satisficing threshold calibration") or primary protocol work.
- **G2.** Numeric effect sizes for F2/F3/F5 not recovered (snippets + partial extracts only) — retrieve full texts (Cambridge DOIs closed-access; Creyer et al. paywalled) via institutional access or ~/papers ingest before quoting numbers.
- **G3.** Chernev pooled ES numeric value unverified (Wiley page snippet-only; extraction blocked) — re-run via 127.0.0.1:8081 extractor or institutional access before promoting F4 beyond moderator-direction CONFIRMED.
- **G4.** Connector coverage: full 14-connector fan-out (Self-Hosted-Search measure.connectors + discovery) not executed from this sandbox; science-category SearXNG engines (arxiv, pubmed, crossref, semantic scholar, openalex) served as proxy. Re-run academic-strategy fan-out before any claim above LIKELY is promoted.
- **G5.** Perfectionism-concerns differential ES (Stoeber & Damian) snippet-only — full-text retrieval pending; do not encode trait-level guidance until read.
- **G6.** Agent-transfer gap: all F-evidence is human decision-makers; LLM-agent satisficing-threshold compliance unmeasured. Candidate: threshold-adherence A/B (explicit vs implicit stop rule) on revision-count and user-satisfaction.

## 7. Sources (search-returned URLs only — no invented links)

1. https://en.wikipedia.org/wiki/Satisficing — Simon 1956/1947, bounded rationality, Nobel-speech quote (F1; extractor-verified).
2. https://doi.org/10.1017/S1930297500003247 — Tyranny of choice cross-cultural maximizing–well-being, regret mediation (F3; extractor-verified intro).
3. https://www.cambridge.org/core/product/identifier/S1930297500003247/type/journal_article — same article landing twin (F3).
4. https://doi.org/10.1017/S1930297500007932 — Two-component maximizing/satisficing, strategy maladaptive vs goal neutral (F2; extractor-verified intro).
5. https://osf.io/zav9t_v1 — same study preprint twin (F2).
6. https://myscp.onlinelibrary.wiley.com/doi/10.1016/j.jcps.2014.08.002 — Choice overload conceptual review + meta-analysis, 99 obs N=7,202 (F4; snippet).
7. http://www.dtic.mil/docs/citations/ADA205750 — Payne/Bettman/Johnson Adaptive Decision Maker record (F5 framework; snippet).
8. https://onlinelibrary.wiley.com/doi/10.1002/bdm.3960030102 — Accuracy/effort feedback and goals, strategy-shift evidence (F5; snippet).
9. https://doi.org/10.1016/j.paid.2016.08.039 — Partialling strivings vs concerns (S1; snippet).
10. https://linkinghub.elsevier.com/retrieve/pii/S0191886916309473 — same article twin (S1; snippet).
11. https://www.interaction-design.org/literature/topics/keep-it-simple-stupid — KISS canon (S2; extractor-verified).
12. https://www.exceptionnotfound.net/kiss-dry-yagni-good-code-basic-training/ — KISS/DRY/YAGNI practitioner triad (S3; snippet).
13. https://devz.cl/posts/acronym-fatigue-series-dry-kiss-yagni/ — acronym-fatigue counter-view (S3; snippet).
14. https://doi.org/10.1111/j.1756-8765.2010.01094.x — Moral satisficing / bounded rationality (F1 support; snippet).
15. https://direct.mit.edu/books/book/4286/chapter/182861 — Models of Bounded Rationality record (F1 support; snippet).
16. http://arxiv.org/abs/2507.07052v1 — FFSD Lean-4 satisficing formalization, PREPRINT (S4).
17. https://doi.apa.org/doi/10.1037/t05051-000 — MPS instrument record (S5; snippet).
18. https://doi.apa.org/doi/10.1037/t04592-000 — MPS instrument record twin (S5; snippet).
19. https://pubmed.ncbi.nlm.nih.gov/21266154 — FMPS Spanish validation (S5; snippet).
20. https://doi.apa.org/doi/10.1037/e549982013-019 — Tyranny-of-choice APA record twin (F3; snippet).

## 8. Provenance & Next Step

- **Engines:** SearXNG @127.0.0.1:8888 (categories=science primary: Simon/satisficing, maximizing–well-being, perfectionism, choice overload, effort–accuracy; general follow-ups: YAGNI/KISS). Science-category engines incl. arxiv, pubmed, crossref, semantic scholar, openalex, plus Bing/Brave/Dogpile families. Extraction via 127.0.0.1:8081 sidecar: 4 probes (2 full-verified: Satisficing wiki + Röckl DOI; 2 partial: IxDF KISS + two-component DOI), remainder snippet-only.
- **Connectors (14):** full Self-Hosted-Search measure.connectors + discovery fan-out not executed from this sandbox (see G4); science-category SearXNG engines served as proxy coverage for arxiv/pubmed/crossref/semantic-scholar/openalex. No URLs invented — §7 lists search-returned URLs only.
- **INDEX.md:** untouched per instructions (no INDEX.md exists in repo; proposed row text below for the owner to insert).
- **Next:** T-062/T-064 design — encode P1–P3 + N1–N5 as stop-rule policy (threshold-first, strategy-stop checkpoints, moderator-moved just-right point); close G2/G3/G5 before quoting any numeric effect size.

> **Proposed INDEX row (exact text, do not insert without owner go):**
> `| just-right-perfection-2026-10-03 | T-062 just-right over perfection + T-064 never-perfection/balancing (satisficing, over-do stop rule) | docs/research/just-right-perfection-2026-10-03.md |`

# LANDSCAPE Research: Max-Planning vs Min-Execution — Is Max-Plan/Min-Exec the Rule? (THOUGHT T-063)

- **Date:** 2026-10-03
- **Thought:** T-063 — max planning vs min execution
- **Core pattern:** A thinking agent must decide how much to plan before acting: front-load everything into the plan (max-plan/min-exec) or keep planning thin and discover in execution. The naive rule ("plan maximally, then execute mechanically") assumes late change is always expensive and early commitment always cheap — both halves are contested.
- **Method:** SearXNG self-hosted (127.0.0.1:8888, categories=science first, then general follow-ups), page verification via URL reads (6 full-verified, 2 bot-blocked), confidence labels per claim.
- **Confidence scheme:** CONFIRMED = replicated/meta-analytic or canonical primary source; LIKELY = single-study or strong convergent theory; UNVERIFIED = plausible, needs verification.

## 1. Problem Statement

T-063 asks whether max-plan/min-exec should be the standing policy for agent work: invest maximally in planning (specs, designs, decision logs) so execution becomes minimal, mechanical, error-free. The case for it rests on the cost-of-change curve (late fixes cost exponentially more, so shift every decision left) and on front-loading evidence (early problem-solving improves downstream performance). The case against it rests on three counter-forces: (a) early commitments made under uncertainty are frequently wrong and expensive to unwind; (b) planning has diminishing and then negative returns (analysis paralysis, stale plans, planning fallacy); (c) iterative execution itself generates the information the plan was missing. The research question: what does the literature say about front-loading, cost-of-change, set-based vs point-based design, deferred commitment, and where is the boundary at which more planning harms?

## 2. Field Landscape

Six sub-fields surveyed; software-economics canon strong, numeric effect sizes thin (itself a finding — the most-cited curves are the least-measured in retrieved text).

| # | Sub-field | Key anchor | Strength |
|---|-----------|-----------|----------|
| 1 | Cost-of-change economics | Boehm curve (Software Engineering Economics) vs Beck flattened curve (XP Explained) | CONFIRMED (as canonical constructs); UNVERIFIED (numeric ratios) |
| 2 | Lean PD front-loading & knowledge value stream | Toyota PD System (Morgan & Liker); Raman & D'Souza knowledge-value-stream framework; LAI waste / PDVSM | LIKELY |
| 3 | Set-based vs point-based design | Sobek/Ward/Schmidtke Toyota SBCE paradigm; Cranfield SBCE thesis; SBCE capabilities in 29 Norwegian firms | LIKELY (paradigm CONFIRMED as Toyota record, effects LIKELY) |
| 4 | Deferred commitment (last responsible moment) | Poppendieck LRM via Atwood 2006; Wirfs-Brock 2011 "most responsible moment" dissent | LIKELY (doctrine, extractor-verified both sides) |
| 5 | Over-planning harms | Paralysis-by-analysis canon; planning-fallacy distributional-information work; deliberative-planning × innovation-newness study | LIKELY–UNVERIFIED (snippets/blocked) |
| 6 | Agile-vs-waterfall success evidence | Serrador & Pinto 2015; SACJ 2020 SA survey (617 projects); agile-success lit review (arXiv 1711.06851); Procedia selection decision model | LIKELY (direction), UNVERIFIED (pooled ES) |

## 3. Top Findings (with confidence + effect sizes)

**F1. The Boehm curve is canon, but its numbers never surfaced — and Beck's flattened curve is the live alternative. (CONFIRMED direction/canonicity; UNVERIFIED magnitude)**
Nilsson's practitioner exposition (extractor-verified, https://jimmynilsson.com/blog/posts/Chronicle5.htm) states both curves cleanly: Boehm's curve (from Software Engineering Economics) says correction cost rises exponentially over project time, implying "all decisions must be taken very early"; Beck's curve (from Extreme Programming Explained) says cost levels out instead — conditional on tiny, beautiful, controlled code with continuous cleanup, redesign, automated tests and deployment. Nilsson's sting: if you believe you are careful yet still live on the Boehm curve, "unfortunately it's probably the case that the code isn't all that great." T-063 corollary: max-plan/min-exec is mandatory *only on the Boehm curve*; on the Beck curve it is optional overhead. No numeric ratio (no 1:10:100) was recovered in any retrieved text — any number quoted for T-063 design today would be invented. The curve you are on is a property of your engineering discipline (tests, deployability, reversibility), not a law of nature.

**F2. Front-load knowledge, not decisions: lean PD pays for early learning, SBCE pays for late elimination. (LIKELY, convergent records)**
The Toyota Product Development System record (Morgan & Liker; chapter-level records https://www.taylorfrancis.com/books/9781482293746/chapters/10.4324/9780367805159-22 and https://www.taylorfrancis.com/books/9781482293746, snippet-only) frames PD as a knowledge value stream, not a decision-freezing pipeline. Raman & D'Souza's framework (extractor-verified abstract, http://arxiv.org/abs/2105.07444v1) makes the mechanism explicit: raw concepts flow into mature knowledge through knowledge cadence and learning cycles that manage uncertainty and variability before optimal decisions are taken. LAI waste work (http://hdl.handle.net/1721.1/79838) and the PDVSM manual (http://hdl.handle.net/1721.1/81908) supply the waste vocabulary. T-063 corollary: the evidence supports max-*learning*/min-*rework*, not max-*plan*/min-*exec* — early effort should produce tested knowledge (prototypes, trade-off curves, limit diagrams), and every early decision that is not knowledge-backed is waste, not diligence.

**F3. Under uncertainty, carry sets and eliminate late; single-point iteration burns the rework SBCE avoids. (LIKELY; paradigm CONFIRMED as Toyota record)**
Sobek/Ward/Schmidtke's SBCE paradigm papers (ASME records https://asmedigitalcollection.asme.org/IDETC-CIE/proceedings/DETC94/12822/79/1103708 and https://asmedigitalcollection.asme.org/IDETC-CIE/proceedings/DETC-CIE96/97607/Irvine,%20California,%20USA/1100805, snippet-only) document the Toyota paradox: no collocated dedicated teams, relatively unstructured process — yet superior outcomes via 11 SBCE principles (delay commitment, narrow sets with data). The Cranfield SBCE transformation thesis (http://dspace.lib.cranfield.ac.uk/handle/1826/8434, snippet) and the 29-firm Norwegian SBCE-capability study (https://doi.org/10.1016/j.procir.2019.04.276, snippet) extend the paradigm beyond Toyota, with shipbuilding application (http://hdl.handle.net/11250/2390450) as a second industry datapoint. T-063 corollary: the plan artifact should be a *set of candidate approaches with kill criteria*, narrowed by evidence during execution — a single frozen plan is point-based design, the exact failure mode SBCE exists to prevent.

**F4. The last responsible moment is real, but practitioners correct it to the "most responsible moment" — LRM misread as procrastination is the failure mode. (LIKELY, extractor-verified both sides)**
The canonical LRM formulation is Poppendieck via Atwood 2006 (extractor-verified, https://blog.codinghorror.com/the-last-responsible-moment/): delay commitment until "the moment at which failing to make a decision eliminates an important alternative"; early decisions risk throwaway work and "crippling and unavoidable consequences"; the decision rule is "make decisions as late as you can responsibly wait because that is the point at which you have the most information." The essential dissent is Wirfs-Brock 2011 (extractor-verified in full, https://wirfs-brock.com/rebecca/blog/2011/01/18/agile-architecture-myths-2-architecture-decisions-should-be-made-at-the-last-responsible-moment/): dissemination takes time, apparently-local decisions ripple, default decisions accrete in fast-moving code, and nobody can reliably identify the last moment in advance — so she decides at the "most responsible moment": early when the decision unblocks others, late when more information is genuinely incoming, and never revisiting without cause. T-063 corollary: encode LRM-with-Wirfs-Brock-correction, never bare LRM — bare LRM drifts into decision debt and collaborator-blocking.

**F5. Agile ≥ Waterfall on success, but modestly and conditionally — planning weight is a contingency choice, not a moral one. (LIKELY direction; no pooled ES recovered)**
Khoza & Marnewick 2020 (extractor-verified abstract, https://doi.org/10.18489/sacj.v32i1.683): 617 South African IS projects, success on a five-level continuum (not just triple constraint) — Agile projects more successful than Waterfall "to some extent," with remaining concerns. Convergent: Serrador & Pinto 2015 quantitative analysis (https://linkinghub.elsevier.com/retrieve/pii/S0263786315000071, snippet-only) and an agile-success literature review (extractor-verified abstract, http://arxiv.org/abs/1711.06851v1). The contingency mechanism is explicit in the Procedia decision model (https://doi.org/10.1016/j.procs.2021.01.227, snippet): 15 criteria (scope, time, costs, org context, team) from literature plus 15 German expert interviews select the procedural model per project. T-063 corollary: max-plan/min-exec is not the rule — it is one cell of a selection matrix, correct where scope is stable, requirements knowable, and change genuinely expensive; elsewhere it underperforms.

**Supporting (LIKELY/UNVERIFIED):**
- S1. Standish CHAOS statistics as practitioner backdrop (https://www.infoq.com/articles/standish-chaos-2015/ — page blocked 405, UNVERIFIED; https://www.standishgroup.com/; http://idsemergencymanagement.com/2021/07/02/what-is-the-standish-group-chaos-report/): CONFIRMED as widely-cited industry surveys, UNVERIFIED as numbers here, and methodologically contested in general — never quote a Standish percentage in T-063 rationale.
- S2. Planning-fallacy mitigation via distributional information (https://www.ssrn.com/abstract=5015318, snippet-only — LIKELY direction): outside-view data beats inside-view plan detail; supports capping plan elaboration in favor of reference-class checks.
- S3. Deliberative planning × innovation-newness interaction (https://espace.library.uq.edu.au/view/UQ:4b4d465 — page blocked 403, UNVERIFIED): the value of planning effort is moderated by newness — the newest work benefits least from heavy upfront planning.
- S4. Paralysis-by-analysis canon (https://linkinghub.elsevier.com/retrieve/pii/0024630195942949, snippet-only — LIKELY as longstanding construct, UNVERIFIED details): over-rational planning systems degrade decisions; the named failure mode for max-plan pathology.
- S5. Lean value-in-PD review (https://doi.org/10.1002/sys.21299, snippet-only — LIKELY): value-generation framing for complex PD; supports F2's knowledge-over-decisions reading.

## 4. Design Principles for T-063 (encode these)

**P1. Front-load learning, defer locking (F1 + F2 + F4).**
Early effort goes into knowledge-producing work (spikes, prototypes, trade-off checks, limit tests), each time-boxed with a kill/continue criterion. No decision is frozen before its knowledge threshold is met; no learning is extended past it. Which curve you are on (F1) determines how much locking lateness you can afford — widen the affordable-lateness window by investing in reversibility (tests, small steps, easy undo) rather than in thicker plans.

**P2. Plan as a narrowing set with kill criteria, never as a single frozen path (F3).**
Every non-trivial plan opens with 2–3 candidate approaches plus the evidence that would eliminate each. Execution narrows the set; the plan artifact is updated by elimination, not rewritten from scratch. A plan with no listed alternatives and no kill criteria is point-based design — reject it at review.

**P3. Set planning weight by uncertainty × irreversibility, and re-select per phase (F5 + S2/S3).**
Read scope stability, requirement knowability, decision irreversibility, and team/org context (Procedia criteria, F5); stable + irreversible → heavier upfront plan; novel + reversible → thin plan, fast first execution, outside-view sanity check (S2) instead of elaboration. Revisit the weight when the phase changes — planning weight is per-decision, per-phase, never per-project-fixed.

## 5. Not-to-Encode (explicit non-goals)

- **N1. Do not encode "always max-plan" or "always min-plan."** F5 is explicit: the winner is contingent. Either blanket rule misfires on half the matrix.
- **N2. Do not cite numeric cost-of-change ratios.** F1: no ratio (1:10:100 or any variant) was recovered from any source. Quoting one would be invention — cite the curve shape conditionally, never a number.
- **N3. Do not cite Standish percentages.** S1: Chaos numbers are contested practitioner-survey figures, unverified here. Use SACJ/Serrador direction (LIKELY) for justification, never Standish digits.
- **N4. Do not encode bare LRM as permission to dawdle.** F4 (Wirfs-Brock): delayed decisions block collaborators, accrete by default, and misjudge ripple effects. LRM without the most-responsible-moment correction and without dissemination time is decision debt, not leanness.

## 6. Open Gaps

- **G1.** Boehm primary numbers unrecovered — retrieve Software Engineering Economics cost-escalation table (and any modern replication) via institutional access before any quantitative planning-weight model ships.
- **G2.** Thomke & Fujimoto front-loading performance evidence not directly retrieved — targeted search ("front-loading problem-solving new product performance") or manual-paper ingest still pending; F2 currently leans on Toyota-system records plus the knowledge-value-stream framework.
- **G3.** SBCE primary full texts unverified (both ASME records snippet-only; Cranfield thesis and Norwegian 29-firm study snippet-only) — retrieve via institutional access before promoting F3 above LIKELY.
- **G4.** Serrador & Pinto 2015 effect size unrecovered (Elsevier page snippet-only) — retrieve before quoting any agile-advantage magnitude.
- **G5.** Connector coverage: full 14-connector fan-out (Self-Hosted-Search measure.connectors + discovery) not executed from this sandbox; science-category SearXNG engines (arxiv, pubmed, crossref, semantic scholar, openalex, plus Bing/Brave/Dogpile families) served as proxy. Re-run academic-strategy fan-out before promoting any LIKELY above its current cap.
- **G6.** Agent-transfer gap: all F-evidence is human teams/projects; agent planning-weight compliance and set-based-plan adherence unmeasured. Candidate: plan-weight A/B (elaborate vs thin plan) on rework-count and task success; set-based vs single-path plan comparison on novel tasks.
- **G7.** Blocked extractions: InfoQ Standish page (405) and UQ eSpace record (403) bot-blocked — re-run via the 127.0.0.1:8081 extractor sidecar or institutional access before using S1/S3 quantitatively.

## 7. Sources (search-returned URLs only — no invented links)

1. https://jimmynilsson.com/blog/posts/Chronicle5.htm — Boehm vs Beck curves exposition (F1; extractor-verified).
2. https://blog.codinghorror.com/the-last-responsible-moment/ — LRM canonical practitioner statement, Poppendieck quote (F4; extractor-verified).
3. https://wirfs-brock.com/rebecca/blog/2011/01/18/agile-architecture-myths-2-architecture-decisions-should-be-made-at-the-last-responsible-moment/ — "most responsible moment" dissent (F4; extractor-verified).
4. http://arxiv.org/abs/2105.07444v1 — Knowledge value stream framework, Raman & D'Souza (F2; extractor-verified abstract).
5. http://arxiv.org/abs/1711.06851v1 — Agile project-success literature review (F5; extractor-verified abstract).
6. https://doi.org/10.18489/sacj.v32i1.683 — Khoza & Marnewick 2020, 617-project Agile vs Waterfall SA survey (F5; extractor-verified abstract).
7. https://linkinghub.elsevier.com/retrieve/pii/S0263786315000071 — Serrador & Pinto "Does Agile work?" quantitative analysis (F5; snippet).
8. https://doi.org/10.1016/j.procs.2021.01.227 — Agile-vs-Waterfall decision model, 15 criteria + 15 interviews (F5; snippet).
9. https://doi.org/10.1002/sys.21299 — Value and lean in complex PD review (S5; snippet).
10. http://hdl.handle.net/1721.1/79838 — LAI waste in lean PD (F2; snippet).
11. http://hdl.handle.net/1721.1/81908 — Product Development Value Stream Mapping manual (F2; snippet).
12. https://asmedigitalcollection.asme.org/IDETC-CIE/proceedings/DETC94/12822/79/1103708 — Set-based concurrent engineering and Toyota (F3; snippet).
13. https://asmedigitalcollection.asme.org/IDETC-CIE/proceedings/DETC-CIE96/97607/Irvine,%20California,%20USA/1100805 — Toyota SBCE 11 principles (F3; snippet).
14. http://dspace.lib.cranfield.ac.uk/handle/1826/8434 — SBCE applications thesis (F3; snippet).
15. https://doi.org/10.1016/j.procir.2019.04.276 — SBCE capabilities, 29 Norwegian firms (F3; snippet).
16. http://hdl.handle.net/11250/2390450 — SBCE in shipbuilding thesis (F3; snippet).
17. https://academic.oup.com/book/52162/chapter/421073318 — Toyota concurrent engineering paradox record (F3; snippet).
18. https://www.taylorfrancis.com/books/9781482293746/chapters/10.4324/9780367805159-22 — Toyota PD System, PD value stream (F2; snippet).
19. https://www.taylorfrancis.com/books/9781482293746 — Toyota PD System book record (F2; snippet).
20. https://www.ssrn.com/abstract=5015318 — Distributional information vs planning fallacy (S2; snippet).
21. https://espace.library.uq.edu.au/view/UQ:4b4d465 — Deliberative planning × innovation-newness (S3; snippet, page blocked).
22. https://linkinghub.elsevier.com/retrieve/pii/0024630195942949 — Paralysis-by-analysis / extinction-by-instinct (S4; snippet).
23. https://www.infoq.com/articles/standish-chaos-2015/ — Standish Chaos Q&A record (S1; search-returned, page blocked).
24. https://www.standishgroup.com/ — Standish Group CHAOS record (S1; search-returned).
25. http://idsemergencymanagement.com/2021/07/02/what-is-the-standish-group-chaos-report/ — CHAOS summary record (S1; search-returned).

## 8. Provenance & Next Step

- **Engines:** SearXNG @127.0.0.1:8888 (categories=science primary: cost-of-change, SBCE, lean PD, LRM, planning harms, agile-vs-waterfall; general follow-ups: Boehm ratios, LRM, front-loading, Standish). Science-category engines incl. arxiv, pubmed, crossref, semantic scholar, openalex, plus Bing/Brave/Dogpile families. Extractor sidecar @127.0.0.1:8081 verified reachable (agent-search-extractor v1.3.0; endpoints /extract, /search, /tavily/*); page verification via URL reads: 6 full-verified (Nilsson, Atwood/CodingHorror, Wirfs-Brock, arXiv 2105.07444, arXiv 1711.06851, SACJ DOI), 2 bot-blocked (InfoQ 405, UQ eSpace 403). Remainder snippet-only.
- **Connectors (14):** full Self-Hosted-Search measure.connectors + discovery fan-out not executed from this sandbox (see G5); science-category SearXNG engines served as proxy coverage for arxiv/pubmed/crossref/semantic-scholar/openalex. No URLs invented — §7 lists search-returned URLs only.
- **INDEX.md:** untouched per instructions (proposed row text below for the owner to insert).
- **Next:** T-063 design — encode P1–P3 + N1–N4 as planning-weight policy (learn-first front-loading, set-based plans with kill criteria, per-phase weight selection); close G1–G4 before quoting any numeric ratio or effect size.

> **Proposed INDEX row (exact text, do not insert without owner go):**
> `| max-plan-min-exec-2026-10-03 | T-063 max-planning vs min-execution (front-loading, cost-of-change, set-based, last responsible moment) | docs/research/max-plan-min-exec-2026-10-03.md |`

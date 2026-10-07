# AGILE/SPRINT Research: Sprint Cadence vs Single-Project Protocol, Iterative Evidence, Protocol Dynamism (THOUGHT T-071)

- **Date:** 2026-10-03
- **Thought:** T-071 — dev protocol isn't quite AGILE or SPRINT oriented? Is it for single serious project or more dynamic? Sprint cadence vs protocol phases, when protocol fits sprint and when it doesn't.
- **Core pattern:** The protocol is a single-project gated pipeline (INBOX → … → SHIP) with event-driven iteration (Gate Restart on discovery), not a calendar-driven sprint cadence; sprints belong INSIDE the EXECUTOR as a delivery rhythm, never as a replacement for the SERIOUSNESS/VALIDATION kill gates — because the evidence favours iterative delivery over waterfall, but also shows fixed sprint cadence breaking down (Scrum→Kanban/hybrid drift) where the protocol's gates hold.
- **Method:** SearXNG self-hosted (127.0.0.1:8888, categories=science first, then general follow-ups), Trafilatura extractor sidecar (127.0.0.1:8081) probes on 5 targets (3 full-verified, 1 partial, 1 failed-blocked), confidence labels per claim.
- **Confidence scheme:** CONFIRMED = replicated/meta-analytic or canonical primary source; LIKELY = single-study or strong convergent theory; UNVERIFIED = plausible, needs verification.

## 1. Problem Statement

T-071 observes a category question: the Development Protocol reads as a single-serious-project pipeline — one cluster selected at INBOX, one X extracted, SERIOUSNESS DROP/COMMIT, AMBITION locks scope, SPECIFICATION writes a 16-section spec, EXECUTOR builds, REVIEW/REFLECT gate SHIP — while agile/sprint practice reads as a recurring cadence: fixed-length sprints, a product backlog that never empties, planning/review/retro every cycle, velocity as progress. Is the protocol agile? Sprint-oriented? For one serious project or for dynamic multi-project flow?

The research question: what does the literature say about (a) what agile canonically requires (Manifesto values + 12 principles as extractor-verified anchor), (b) what sprint cadence canonically requires (Scrum Guide timebox ≤1 month + per-sprint ceremonies), (c) whether iterative/incremental beats waterfall empirically (IEEE systematic-review record), (d) where sprint cadence breaks down in practice (Scrum→Kanban/hybrid drift, large-scale friction, Water-Scrum-Fall), and (e) what iteration mechanism the protocol already has (Gate Restart, VALIDATION loops, REFLECT→EXTRACTION) — and what must therefore be encoded so the protocol stays iterative without becoming sprint-shaped?

## 2. Field Landscape

Six sub-fields surveyed; agile canon extractor-verified at the top, sprint-breakdown evidence convergent but snippet-only, iterative-vs-waterfall magnitudes paywalled (itself a finding — do not quote numbers we could not read).

| # | Sub-field | Key anchor | Strength |
|---|-----------|-----------|----------|
| 1 | Agile canon (values + principles) | Agile Manifesto 4 values + 12 principles (extractor-verified) | CONFIRMED (as doctrine) |
| 2 | Sprint cadence definition | Scrum Guide 2020 (extractor-partial: purpose/definition verified, timebox numbers via snippet); Scrum ceremony-adherence SLR (~87% agile teams use Scrum practice) | CONFIRMED (timebox ≤1 month exists), LIKELY (adherence figures) |
| 3 | Iterative/incremental vs waterfall | IEEE systematic review: cost/duration/quality waterfall vs iterative-incremental (record only, paywalled); productivity-trends incremental/iterative record; Water-Scrum-Fall thesis | LIKELY (direction favours iterative), UNVERIFIED (magnitudes) |
| 4 | Sprint-breakdown drift | Scrum-vs-Kanban SLR (38 studies: transitions nearly all Scrum→Kanban/hybrid, flow instead of sprint); Agile-challenges SLR (8 themes, 46 subthemes) | LIKELY (convergent single-reviews, snippet-only) |
| 5 | Scale-up friction | Large-scale agile SLR (SAFe, LeSS, Scrum-at-Scale, DAD, Spotify; TSE 2021): practices-over-principles, custom methods, challenges/success factors | LIKELY (snippet-only) |
| 6 | Agile classification + teamwork | Abrahamsson et al. review/analysis (definition + 10-method comparison); ATEM teamwork-effectiveness model (shared leadership, team orientation, redundancy, adaptability, peer feedback) | LIKELY (snippet-only) |

## 3. Top Findings (with confidence + effect sizes)

**F1. Agile canon demands frequent working software and reflection — cadence-agnostic. (CONFIRMED as doctrine, extractor-verified primary)**

The Manifesto (https://agilemanifesto.org/, extractor-verified: "uncovering better ways… Individuals and interactions over processes and tools; Working software over comprehensive documentation; Customer collaboration over contract negotiation; Responding to change over following a plan") plus 12 principles (https://agilemanifesto.org/principles.html, extractor-verified: "early and continuous delivery"; "deliver working software frequently, from a couple of weeks to a couple of months, preference to the shorter timescale"; "working software is the primary measure of progress"; "sponsors, developers, users maintain constant pace indefinitely"; "at regular intervals, the team reflects… then tunes and adjusts"). T-071 corollary: nothing in the canon mandates a fixed sprint length, a backlog tool, or velocity — it mandates working-software frequency + reflection intervals. The protocol's EXECUTOR increments + REVIEW/REFLECT already satisfy the letter; the sprint is one scheduling choice among several.

**F2. Sprint cadence is a fixed timebox ≤1 month with per-sprint ceremonies — a scheduling device, not a project shape. (CONFIRMED that the timebox exists; LIKELY on prevalence, extractor-partial + snippet-only)**

Scrum Guide 2020 (https://scrumguides.org/scrum-guide.html, extractor-partial: purpose/definition verified — "Each element of the framework serves a specific purpose… Changing the core design… covers up problems"; timebox detail via snippet + Scrum Alliance corroboration https://resources.scrumalliance.org/Article/sprint-review: review timeboxed 4h for a one-month sprint, shorter sprints shorter timeboxes). Ceremony-adherence SLR (https://www.authorea.com/doi/full/10.22541/au.176183673.34271999/v1, snippet): ~87% of agile teams employ Scrum practice; prescribed ceremonies are sprint planning, daily stand-up, sprint review, sprint retrospective; "Scrum But" modification/omission is the modal deviation. T-071 corollary: sprint answers "how do we schedule the next N weeks" — it does not answer "should this project exist" (SERIOUSNESS) or "is the plan feasible" (VALIDATION). A protocol that replaced its gates with sprints would schedule efficiently toward possibly-wrong goals.

**F3. Iterative/incremental beats waterfall on cost, duration, and quality — direction established, magnitudes unread. (LIKELY direction; UNVERIFIED magnitudes, record-only)**

IEEE systematic review (http://ieeexplore.ieee.org/document/5314228/, extractor-failed: paywall/JS shell only, title record confirms the comparison exists — cost, duration, quality, waterfall vs iterative-incremental). Convergent: productivity-trends in incremental/iterative development (http://ieeexplore.ieee.org/document/5316044/, record-only); iterative-requirements case study inside a waterfall enterprise (http://rave.ohiolink.edu/etdc/view?acc_num=osu1345447033, snippet): expanded iterative scope → regular feedback into decisions, "commit to planning" (planning as constant process) over "planning to commit", frequent coordinated releases → higher business value, reduced risk. T-071 corollary: the protocol's iterative machinery (VALIDATION loops, Gate Restart, REFLECT→EXTRACTION new cycle) is evidence-aligned; no finding licenses removing gates in the name of speed — iteration in the evidence means feedback-driven replanning, which is what Gate Restart is.

**F4. Fixed sprint cadence breaks down in practice: teams drift Scrum→Kanban/hybrid, "flow instead of a sprint". (LIKELY, convergent single-reviews, snippet-only)**

Scrum-vs-Kanban SLR, 38 studies (https://doi.org/10.15439/2022f143, snippet): nearly all transitioning organisations move Scrum→Kanban or →hybrid; problems experienced with Scrum are prominent among transition reasons; Kanban stands out positively; almost all combining teams use flow instead of a sprint. Challenges-of-Agile-Scrum SLR (https://www.preprints.org/manuscript/202506.0080/v1, snippet): 8 main themes + 46 subthemes — dependency issues, Agile-difficult-to-implement, multi-team environment, requirements-engineering challenges, knowledge issues, resistance to change, org structure/boundaries, QA challenges; most-mentioned first. T-071 corollary: this is the direct answer to "when protocol fits sprint and when it doesn't" — sprint cadence fits single-team greenfield product work with decomposable backlog; it strains under dependencies, multi-team coupling, and requirements-discovery load — exactly the conditions the protocol's LANDSCAPE/STRATEGY/VALIDATION gates exist for. Encode: sprint as optional EXECUTOR rhythm, never as gate substitute.

**F5. At scale, organisations customise; commercial frameworks' practices outrun their principles. (LIKELY, single-review, snippet-only)**

Large-scale agile SLR (https://doi.org/10.1109/tse.2021.3069039, snippet): first standardised comparison of SAFe, LeSS, Scrum-at-Scale, DAD, Spotify model (+ Nokia/Ericsson custom builds) across principles, practices, tools, metrics; findings — literature emphasises commercial-framework practices at the expense of underlying principles and of custom methods; challenges + success factors identified per method. Abrahamsson et al. classification (http://arxiv.org/abs/1709.08439v1, snippet): definition + classification of agile approaches, 10 methods analysed against criteria, similarities/differences + future-research needs. T-071 corollary: "more dynamic / multi-project" does not mean "adopt a scaling framework" — the evidence pattern is principles-first, custom-method-per-context. The protocol's answer to dynamism is PRIORITIZE (2–10 ideas, What-Matters bet) + REFLECT→EXTRACTION next-cycle seed, not a SAFe prescription.

**Supporting (LIKELY/UNVERIFIED):**
- S1. ATEM teamwork-effectiveness model (https://doi.org/10.1007/s10664-021-10115-0, snippet): shared leadership, team orientation, redundancy, adaptability, peer feedback + coordinating mechanisms (shared mental models, communication, mutual trust); extensions for small/multi-team/distributed/safety-critical. Supports mapping sprint rituals to protocol gates without inventing new roles.
- S2. Software-cost-estimation-in-agile survey (https://doi.org/10.25103/jestr.104.08, snippet): estimation must run throughout the lifecycle in ASD ("live and dynamic nature"). Supports AMBITION pacing-track + continuous replanning over upfront commitment.
- S3. AI-for-Agile meta-analysis (http://arxiv.org/abs/2305.08093v1, snippet): specialised socio-technical expertise still the bottleneck. Cautions against "agents make cadence free" reasoning.

## 4. Design Principles for T-071 (encode these)

**P1. Keep the single-project gated spine; sprints live INSIDE the EXECUTOR as an optional delivery rhythm (F1 + F2).**
INBOX → … → SHIP with SERIOUSNESS/VALIDATION/REVIEW/REFLECT gates is the project shape; a sprint (fixed timebox ≤1 month, planning/review/retro per cycle) is one way to schedule EXECUTOR increments, alongside flow/Kanban. No sprint may skip, compress, or timebox-away a gate. Spec records the chosen EXECUTOR rhythm and its timebox explicitly.

**P2. Iteration is event-driven (Gate Restart on discovery), with optional calendar cadence layered inside execution only (F3 + F4).**
VALIDATION→AMBITION/EXTRACTION/LANDSCAPE loops, EXECUTOR→SPECIFICATION midpoint check, REVIEW→EXECUTOR rework, REFLECT→EXTRACTION next cycle — these fire on evidence (failed assumptions, prototype surprises, review findings), not on calendar dates. Sprint ceremonies that surface such evidence (review/retro) feed the same loop-backs; the calendar never itself authorises scope or kill decisions.

**P3. Map sprint rituals to protocol gates 1:1; preserve the gates sprint has no equivalent for (F2 + F5).**
Sprint planning → AMBITION/SPEC slices; sprint review → REVIEW (working-software demo); sprint retrospective → REFLECT. SERIOUSNESS (DROP/COMMIT) and VALIDATION (KILL/PIVOT/COMMIT) have no sprint equivalent and must survive any "agile" adaptation. Principles-first, custom-rhythm-per-context (F5): the protocol never prescribes SAFe/LeSS/velocity — teams choose EXECUTOR rhythm and record why.

## 5. Not-to-Encode (explicit non-goals)

- **N1. Do not encode a mandatory sprint length.** No "all work runs in 2-week sprints" rule (F1: canon is cadence-agnostic; F4: fixed cadence is precisely what breaks). Timeboxes are chosen per EXECUTOR run and recorded.
- **N2. Do not encode velocity as progress.** Working software + gate passage are the measures (F1: "working software is the primary measure of progress"). Story-points-per-sprint velocity is at most an EXECUTOR-local planning aid, never a protocol metric.
- **N3. Do not permit Scrum-But gate-dropping without record.** F2's modal deviation (modify/omit ceremonies) has a protocol analogue — skipping SERIOUSNESS/VALIDATION/REVIEW. Any skipped gate is a logged Gate Restart-class decision with rationale, never silent.
- **N4. Do not prescribe a scaling framework.** F5's practices-over-principles finding forbids it: no SAFe/LeSS/Spotify-model text in protocol steps. Multi-project dynamism goes through PRIORITIZE + REFLECT→EXTRACTION, not framework adoption.
- **N5. Do not encode "agile = no planning, no documentation".** F1's "responding to change over following a plan" values the left *more*, not *only*; F3's "commit to planning" means planning is continuous. SPECIFICATION's 16-section spec and the source manifest stand.

## 6. Open Gaps

- **G1.** IEEE waterfall-vs-iterative magnitudes unread (paywalled, extractor-failed) — retrieve full text (institutional access) before citing any cost/duration/quality number; until then direction-only.
- **G2.** ~87% Scrum-adoption figure snippet-only (Authorea preprint SLR) — verify against Digital.ai State of Agile primary before quoting in rationale.
- **G3.** Scrum→Kanban drift magnitudes snippet-only (38-study SLR) — retrieve transition counts/rates before citing as more than directional.
- **G4.** Sprint-length optimality unaddressed — no evidence surveyed on 1- vs 2- vs 4-week timebox performance; any EXECUTOR default length is a stipulation, not an evidence claim.
- **G5.** Agent-transfer gap: all F-evidence is human-team software practice; whether gated-pipeline + optional-sprint-rhythm improves agent fix/ship success vs pure-sprint emulation is unmeasured. Candidate: A/B (gate-spine vs sprint-emulation) on ship success, rework rate, wrong-problem rate.
- **G6.** Connector coverage: full 14-connector fan-out not executed from this sandbox; science-category SearXNG engines (arxiv, pubmed, crossref, semantic scholar, openalex, plus Bing/Brave/Dogpile families) served as proxy. Re-run academic-strategy fan-out before promoting any LIKELY above its current cap.
- **G7.** Prior-work boundary: guidance-dynamic (ZPD/pacing), power-verification, just-right-perfection, max-plan-min-exec, protocol-transcript sweeps nearby — this sweep cites none of their claims and shares no encoded principle; keep T-071 encoding additive (EXECUTOR-rhythm + gate-mapping only).

## 7. Sources (search-returned URLs only — no invented links)

1. https://agilemanifesto.org/ — Agile Manifesto 4 values (F1; extractor-verified).
2. https://agilemanifesto.org/principles.html — 12 principles: frequent delivery, working-software measure, sustainable pace, reflect-and-tune (F1; extractor-verified).
3. https://scrumguides.org/scrum-guide.html — Scrum Guide 2020 definition/purpose (F2; extractor-partial, timebox via snippet).
4. https://resources.scrumalliance.org/Article/sprint-review — Sprint review timebox 4h per one-month sprint (F2; snippet).
5. https://www.authorea.com/doi/full/10.22541/au.176183673.34271999/v1 — Scrum ceremony-adherence SLR, ~87% usage, Scrum-But deviations (F2; snippet).
6. http://ieeexplore.ieee.org/document/5314228/ — Waterfall vs iterative-incremental systematic review, cost/duration/quality (F3; record-only, extractor paywall-blocked).
7. http://ieeexplore.ieee.org/document/5316044/ — Productivity trends in incremental/iterative development (F3; record-only).
8. http://rave.ohiolink.edu/etdc/view?acc_num=osu1345447033 — Water-Scrum-Fall thesis: iterative scoping-to-deployment, "commit to planning" (F3; snippet).
9. https://doi.org/10.15439/2022f143 — Scrum/Kanban/hybrid SLR, 38 studies, flow-instead-of-sprint drift (F4; snippet).
10. https://www.preprints.org/manuscript/202506.0080/v1 — Challenges of Agile Scrum SLR, 8 themes/46 subthemes (F4; snippet).
11. https://doi.org/10.1109/tse.2021.3069039 — Large-scale agile methods comparison SAFe/LeSS/Scrum-at-Scale/DAD/Spotify (F5; snippet).
12. http://arxiv.org/abs/1709.08439v1 — Abrahamsson et al. agile methods review/analysis, 10-method comparison (F5; snippet).
13. https://doi.org/10.1007/s10664-021-10115-0 — ATEM agile teamwork-effectiveness model (S1; snippet).
14. https://doi.org/10.25103/jestr.104.08 — Cost estimation in agile survey, estimation-throughout-lifecycle (S2; snippet).
15. http://arxiv.org/abs/2305.08093v1 — AI-for-Agile meta-analysis (S3; snippet).
16. https://www.scrum.org/resources/what-is-a-sprint-in-scrum — Sprint explainer (F2; extractor thin-JS, snippet-equivalent).
17. https://softwareengineeringauthority.com/agile-methodology/ — Agile principles/practice overview (background; snippet).

## 8. Provenance & Next Step

- **Engines:** SearXNG @127.0.0.1:8888 (categories=science primary: agile effectiveness review, sprint/velocity SLR, iterative-vs-waterfall comparison; general follow-ups: Agile Manifesto principles, State-of-Agile/sprint statistics, Scrum Guide cadence). Science-category engines incl. arxiv, pubmed, crossref, semantic scholar, openalex, plus Bing/Brave/Dogpile families. General-category returns were thin (relevance 0.3); canonical extractor probes compensated.
- **Extractor probes (5):** full-verified — https://agilemanifesto.org/ (4 values), https://agilemanifesto.org/principles.html (12 principles incl. frequent delivery + reflect-and-tune); partial — https://scrumguides.org/scrum-guide.html (purpose/definition verified, timebox section truncated); failed-blocked — http://ieeexplore.ieee.org/document/5314228/ (paywall/JS shell, title record only); thin-JS — https://www.scrum.org/resources/what-is-a-sprint-in-scrum (nav shell, no body content). Remainder snippet-only.
- **Connectors (14):** full Self-Hosted-Search measure.connectors + discovery fan-out not executed from this sandbox (see G6); science-category SearXNG engines served as proxy coverage. No URLs invented — §7 lists search-returned URLs only.
- **INDEX.md:** untouched per instructions (proposed row text below for the owner to insert).
- **Next:** T-071 design — encode P1–P3 + N1–N5 as EXECUTOR-rhythm policy (optional sprint/flow rhythm recorded per run; ritual→gate mapping; SERIOUSNESS/VALIDATION un-skippable); close G1–G3 before quoting any numeric magnitude in rationale.

> **Proposed INDEX row (exact text, do not insert without owner go):**
> `| [agile-sprint-protocol-2026-10-03.md](agile-sprint-protocol-2026-10-03.md) | 2026-10-03 | T-071 sprint cadence vs single-project protocol: agile canon (extractor-verified), Scrum timebox, iterative-vs-waterfall direction, Scrum→Kanban drift, scale-up friction; sprints-inside-EXECUTOR + event-driven iteration principles. |`

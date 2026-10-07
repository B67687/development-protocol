# G7 Sprint-Level Architecture Evolution — Continuous Top-Down/Bottom-Up Refactor Without Regress (2026-10-04)

- **Date:** 2026-10-04
- **Retro:** G7 — T-058. Continuous architecture view: refactor top-down/bottom-up every sprint, no regress, triage old features? OPEN (no sprint-level architecture-evolution gate beyond L1 descent + REVIEW diffusion-debt). Cluster: Continuity.
- **Downstream question:** what sprint-cadence mechanism keeps the architecture view current — descending from the whole-system picture to the changed parts (top-down) and ascending from code/test signals to structural decisions (bottom-up) — without regressing shipped behavior or accumulating untriaged legacy?
- **Core pattern:** The literature confirms both halves separately but never as one sprint gate: continuous-architecting evidence says postponement compounds (interest on debt); comprehension evidence says experts interleave top-down inference with bottom-up execution traces while novices stay bottom-up; conformance evidence says automated dependency checks catch erosion at commit time. The missing piece is the *cadence binding* — what runs every sprint, in what order, with what stop rule.
- **Method:** SearXNG self-hosted (`127.0.0.1:8888`, `categories=science` primary across 5 queries, one `general` pass for feature-deprecation triage — thin recall, marked accordingly), Trafilatura extractor sidecar (`127.0.0.1:8081` POST `/extract`) — 5 probes, 5 full-verified (see §8).
- **Confidence scheme:** CONFIRMED = replicated/meta-analytic or canonical primary source; LIKELY = single-study or strong convergent theory; UNVERIFIED = plausible/practitioner, needs verification. Snippet-only numbers stay LIKELY max and never enter design rationale (§5 N4).
- **Proposed INDEX row (exact text, do not insert without owner go):** `| g7-arch-evolution-2026-10-04.md | 2026-10-04 | G7 sweep for T-058: continuous-architecting + erosion/conformance + top-down/bottom-up comprehension + regression-in-CI + temporal-discounting evidence, 5 extractor-verified anchors, per-sprint evolution-gate shape (P1–P4) with N1–N5 guards. |`
- **Note:** per sweep instructions, no INDEX.md writes and no git operations were performed; row above is proposal only.

---

## 1. Problem Statement

T-058 asks for a continuous architecture view with three coupled demands: (a) **refactor top-down/bottom-up every sprint** — the whole-system picture must descend to the changed parts and the code-level signals must ascend to structural decisions, each cycle; (b) **no regress** — evolution must not break shipped behavior; (c) **triage old features** — legacy surface must shrink or be explicitly retained, never silently accumulate.

What exists today covers fragments: L1 descent gives the big→small entry (bounded descent per run) and REVIEW's diffusion-debt check catches cross-file propagation after the fact. Neither runs on sprint cadence, neither binds the two directions into one pass, and neither owns the old-feature triage decision (keep / degrade-honestly / deprecate / remove).

This brief answers: what evidence constrains a sprint-level evolution gate — what to check top-down, what to lift bottom-up, what regression instrument guards the refactor, what counter-bias the triage needs, and what must stay out of the gate — so the T-058 fold can proceed without re-sweeping shared ground.

## 2. Field Landscape

Seven sub-fields; strength column records today's verification status.

| # | Sub-field | Key anchor | Strength |
|---|-----------|-----------|----------|
| 1 | Agile × architecture coexistence | Yang et al. systematic mapping, 54 studies 2001–2014 (costs/failure stories under-described; 20 challenges, 29 factors, 25 lessons); agile-architecture grounded theory, 44 participants / 36 orgs, 6 forces + 5 up-front-effort strategies | LIKELY (mapping structure CONFIRMED as published fact, effect claims snippet) |
| 2 | Continuous architecting + debt interest | Lenarduzzi et al. microservices CA study (http://arxiv.org/abs/1810.10855v1, extractor full-verified today): postponed activities accrue interest; deep re-architecting (API gateway / message bus) raises fault risk | LIKELY (preliminary/WIP single SME, verified abstract) |
| 3 | Erosion + conformance checking | DepCoL dependency-constraint language + Eclipse plugin, immediate violation feedback (http://arxiv.org/abs/1510.08510v1, extractor full-verified today); architecture-erosion cause/consequence survey (https://doi.org/10.21275/ms2012134218, snippet) | LIKELY (mechanism CONFIRMED as built artifact, erosion-rate numbers absent) |
| 4 | Top-down / bottom-up comprehension | Burkhardt et al. OO comprehension: experts top-down inference-driven + multiple guidance, novices execution-based + less top-down (http://arxiv.org/abs/cs/0702002v1, extractor full-verified today); XP comprehension-assessment framework, 5 practices incl. refactoring + evolutionary design (https://ir.cwi.nl/pub/4343, snippet) | LIKELY (convergent lab result, small-N lineage) |
| 5 | Regression-in-CI guard | Continuous-regression-testing formalization, build-chain model (http://arxiv.org/abs/2511.02810v1, snippet); RefBot interactive NSGA-II + CI pull-request bot thesis (https://hdl.handle.net/2027.42/154775, snippet); CI-refactoring survey, 31 developers, quality-gate-triggered refactor (https://doi.org/10.1109/icsme.2018.00068, snippet) | LIKELY (formalism + survey convergence, numbers snippet) |
| 6 | Debt prioritization + temporal bias | Practitioner survey 184 responses BR/FI/NZ: debt lives in legacy, instances hard to conceptualize, structure-clarity practices help most (http://arxiv.org/abs/2104.14761v1, extractor full-verified today); temporal-discounting experiment 33 developers / 2 companies, widespread discounting of architectural investment (http://arxiv.org/abs/1901.07024v2, extractor full-verified today) | LIKELY (survey CONFIRMED as conducted fact; discounting single-experiment) |
| 7 | Old-feature triage / deprecation | Vendor deprecation-schedule practice (general-pass hits only — NVIDIA/MSDN/WordPress notices); no empirical deprecation-triage study retrieved | UNVERIFIED (practice existence CONFIRMED, no evidence base — itself a finding) |

Already-covered ground (cited, not re-argued): L1 bounded descent (c5c7633), REVIEW diffusion-debt, second-system guard in REFLECT Q9 (6038450), per-cycle learning signal F-017 (9293199), `scales-analysis-2026-10-03.md` F3 (higher levels constrain lower — the big→small licence), `max-plan-min-exec-2026-10-03.md` (front-load knowledge not decisions, last-responsible-moment).

## 3. Top Findings (with confidence + effect sizes)

**F1. Postponed architecture work accrues interest — continuous beats catch-up. (LIKELY, extractor full-verified today)**
Lenarduzzi et al. (http://arxiv.org/abs/1810.10855v1): SME migrating monolith→microservices under Continuous Architecture; simplified initial patterns (direct monolith links, no bus) forced later deep re-architecting (gateway / lightweight bus), each round raising new-fault risk. The mechanism is interest-like: deferred structural decisions return larger and fault-prone. Transfer: a protocol with no sprint-level evolution gate is the initial-simplification strategy — cheap this cycle, deep-refactor debt next. The gate's job is to convert lump-sum re-architecture into per-sprint installments. *Fold owner: T-058 gate cadence.*

**F2. Experts interleave directions; novices stay bottom-up — the gate must enforce the missing direction. (LIKELY, extractor full-verified today)**
Burkhardt et al. (http://arxiv.org/abs/cs/0702002v1): experts show strong top-down inference-driven behavior plus multiple guidance and consult more files; novices lean execution-based with less top-down use and exploit inheritance/composition poorly. The XP-comprehension analysis (https://ir.cwi.nl/pub/4343, snippet) independently routes refactoring + evolutionary design through comprehension support. Transfer: an agent defaulting to local fixes behaves as the novice — bottom-up only, static relationships underused. The sprint gate must *force* the top-down pass (whole-view → changed parts: which invariants, which dependents) because the bottom-up pass happens anyway. Top-down is the scarce direction; gate time spends there. *Fold owner: T-058 gate order.*

**F3. Erosion is checked cheapest at commit time by declared constraints — not by periodic review. (LIKELY, extractor full-verified today)**
DepCoL (http://arxiv.org/abs/1510.08510v1): dependency constraints over plugin groups + Eclipse plugin checking consistency *during development*, immediate violation feedback, counteracting erosion incrementally. The erosion survey (https://doi.org/10.21275/ms2012134218, snippet) frames the alternative: decades-long lifespans with upgrade phases where implementation diverges from intent, up to inoperability. Transfer: REVIEW-time diffusion-debt detection is the periodic-review pole — necessary, late. The sprint gate needs the DepCoL pole too: a small declared-constraint set (dependency direction, layering, interface ownership) checked at change time, so erosion never waits for review to be noticed. *Fold owner: T-058 constraint ledger.*

**F4. Agile practices that verify structure reduce debt; vague debt instances resist management. (LIKELY, extractor full-verified today)**
Practitioner survey, n=184 across Brazil/Finland/NZ (http://arxiv.org/abs/2104.14761v1): practitioners aware of debt but under-utilize the concept; debt concentrates in legacy yet concrete instances are hard to conceptualize (hence hard to manage); queried agile practices help reduce debt — particularly techniques verifying and maintaining structure and clarity of artifacts; competing stakeholder interests remain a concern. Transfer, two parts: (i) the gate's bottom-up instrument is settled — structure-verifying practices (tests, static checks, clarity passes) are the highest-yield debt reducers, so the gate wires them rather than inventing new ones; (ii) the conceptualization gap ("hard to name instances") is exactly what a per-sprint triage ledger fixes — unnamed debt is unmanageable debt. *Fold owner: T-058 triage ledger.*

**F5. Developers systematically discount future architecture value — triage needs a counter-bias instrument. (LIKELY, extractor full-verified today)**
Temporal-discounting experiment, 33 developers / 2 companies (http://arxiv.org/abs/1901.07024v2): widespread discounting of longer-term architectural investment versus near-term feature work, with notable individual differences; first empirical intertemporal-choice study in SE. Transfer: sprint triage ("refactor now vs ship feature") is an intertemporal choice run under discounting — without a counterweight the gate votes feature every time and the architecture view decays by revealed preference, not by decision. The gate must price the future explicitly (interest estimate, blast-radius if deferred) or discounting decides silently. *Fold owner: T-058 triage economics.*

**F6. The regression guard is a build-chain window, and refactor-in-CI is gated by quality thresholds — but perceived barriers dominate. (LIKELY, snippet convergence)**
Formalization (http://arxiv.org/abs/2511.02810v1): continuous regression testing as a time-ordered build chain with a limited test window per build — the no-regress guarantee is window-bounded, never absolute. RefBot thesis (https://hdl.handle.net/2027.42/154775, snippet): interactive NSGA-II refactor recommendation minimizing deviation from initial design + CI bot surfacing refactor sequences as pull requests mined from developer profiles. CI-refactoring survey, 31 developers (https://doi.org/10.1109/icsme.2018.00068, snippet): quality gates (e.g., SonarQube thresholds) trigger refactor; prior work shows refactoring often not done — CI changes perception but barriers persist. Joint transfer: the no-regress half of T-058 is three instruments, not one — (i) window-bounded regression suite per change, (ii) deviation-minimizing refactor preference (smallest structural delta that pays the debt), (iii) threshold-triggered rather than volunteer-triggered refactor. "No regress" never means proof; it means bounded window + small deltas + gate thresholds. *Fold owner: T-058 regress guard.*

**F7. Old-feature triage has practice but no evidence base — deprecation is the protocol's unowned decision. (UNVERIFIED as evidence; practice existence CONFIRMED)**
The general pass returned only vendor notices (NVIDIA Fermi deprecation schedule, Microsoft/WordPress deprecation pages) — real practice, zero retrieved empirical studies on how teams decide keep/degrade/deprecate/remove. Business-driven TD prioritization (http://arxiv.org/abs/2010.09711v2, five-month industrial case, snippet) is the nearest neighbor: aligning business + technical stakeholders on eight business factors drove debt affecting high-value assets downward — triage works when business value joins the room. Transfer: the protocol's triage ledger needs the business-value column (which user value does this old feature still carry?) because the technical side alone cannot decide removal; and every retained-legacy entry needs an explicit price (maintenance cost + constraint on new work) or retention is free by default. *Fold owner: T-058 triage decision.*

**Supporting (LIKELY/UNVERIFIED):**
- S1. Mapping-study context (https://doi.org/10.1016/j.jss.2015.09.028, snippet): 54 studies, no dominant architecting approach, costs/failures under-reported — the field has no canonical sprint-evolution method; any gate we design is greenfield, not adoption. (LIKELY as publication fact)
- S2. Refactoring-vs-regression-testing impact study (https://ieeexplore.ieee.org/document/6405293/, snippet, paywalled): existence confirms the F6 coupling is a recognized empirical question; numbers unrecovered. (UNVERIFIED details)
- S3. Agile-architecture grounded theory (https://doi.org/10.26686/wgtn.17007694.v1, snippet): 6 forces + 5 up-front-effort strategies from 44 practitioners — the up-front vs continuous calibration this gate inherits; unextracted. (LIKELY as thesis fact)

## 4. Design Principles for G7 (encode these; fold owner T-058 throughout)

**P1. One sprint pass, two directions, top-down first.**
Every sprint runs the evolution pass in fixed order: (i) top-down — from the current architecture view to this sprint's changed parts (which invariants touched? which dependents? which constraints fired?); (ii) bottom-up — from code/test signals to structural findings (new smells, new coupling, new constraint candidates). Order is load-bearing (F2): bottom-up happens without enforcement, top-down does not. Empty findings in either direction are recorded as explicit "none observed," never skipped silently — the none-record is what makes cadence auditable.

**P2. Constraints declared, checked at change time — review catches the remainder.**
Maintain a small declared-constraint set (dependency direction, layering, interface ownership — DepCoL-shaped, F3) checked when code changes, not when review convenes. REVIEW diffusion-debt stays as the second net for what declarations miss. Constraint set itself evolves bottom-up: recurring review findings graduate into declared constraints. No constraint lives only in prose.

**P3. Refactor by smallest-deviation installments behind a triple regress guard.**
No-regress = (i) window-bounded regression suite per change (F6: bounded window, never absolute proof), (ii) smallest structural delta that pays the debt (RefBot deviation-minimization), (iii) threshold-triggered refactor (quality-gate breach fires work; volunteering is bonus, not plan). Lump-sum re-architecture is the failure mode F1 prices — installments are the policy.

**P4. Triage every old feature on a dated ledger with priced retention.**
Ledger columns: feature · user value still carried (F7 business column) · maintenance price + constraint-on-new-work (F5 counter-bias: future priced explicitly) · verdict (keep / degrade-honestly / deprecate-with-date / remove) · next review sprint. Retention without a price is forbidden — unpriced retention is how discounting (F5) and unconceptualized debt (F4) win silently. Deprecation always carries a date; dateless deprecation is retention.

## 5. Not-to-Encode (explicit non-goals)

- **N1. Do not encode a canonical sprint-evolution method as adopted practice.** S1: the mapping study finds no dominant approach and under-reported costs/failures — the gate is greenfield design (P1–P4), not field adoption. Cite structure, never authority.
- **N2. Do not promise proof of no-regress.** F6: the guarantee is window-bounded (suite + window + thresholds). Any gate wording claiming refactor-safety absolutely is the eval-illusion failure — state the window, not the warranty.
- **N3. Do not quote snippet-only numbers as findings.** CI-survey 31-developer proportions, RefBot effectiveness, erosion-survey rates, grounded-theory force counts, business-prioritization deltas — single-study priors (LIKELY max), usable for sizing bets, never as design-rationale citations. Rationale cites F1–F7 qualitative structure only.
- **N4. Do not re-encode L1 descent, REVIEW diffusion-debt, or scale-analysis F3 ground.** Bounded descent, propagation checking, and the big→small licence stand; this sweep adds cadence binding (P1), change-time constraints (P2), installment policy (P3), and priced triage (P4) — deltas only.
- **N5. Do not let the triage ledger become a second backlog.** The ledger's verdict + next-review-sprint columns are load-bearing: every row resolves (verdict) and recurs (date) or it is backlog by another name. Rows without both are rejected at gate time.
- **N6. Do not treat top-down and bottom-up as interchangeable or order-free.** F2 fixes the order (top-down first) because scarcity differs by direction. A gate running bottom-up-only is the novice profile with a checklist.

## 6. Open Gaps

- **G1.** Constraint-set seed unbuilt: which dependency/layering/ownership constraints does the protocol declare first, and in what syntax? Small design task, high leverage — blocks P2 wiring. Candidate: mine last N REVIEW diffusion-debt findings for recurring violations as the seed set.
- **G2.** Regression-window specification unbuilt: which suite subset runs per change, what window budget, what breach threshold triggers P3 refactor? Pairs with P3 instrument (iii); needs harness reality (CI minutes available), not literature.
- **G3.** Triage pricing instrument unbuilt: how is "maintenance price + constraint-on-new-work" estimated at gate time (T-shirt, blast-radius count, debt-interest sketch)? F5 demands explicit pricing; the unit is undecided. Joint with any future calibration-layer work.
- **G4.** Cadence scope unscoped: does the evolution pass run per protocol cycle, per N cycles, or per calendar sprint — and what is the minimum viable pass when the sprint changed nothing structural? P1 needs a skip rule (explicit none-record) so empty sprints cost minutes, not hours.
- **G5.** Deprecation-date policy unbuilt: what sets the removal date (usage evidence? maintenance cost? downstream contract?) and who can extend it? F7's nearest neighbor (business-driven prioritization) suggests joint business+technical sign-off; the protocol's sign-off shape is undesigned.
- **G6.** Agent-transfer gap: all F-evidence is human comprehension/industrial corpora; agent-executed sprint evolution (does a forced top-down pass improve agent refactor quality vs bottom-up-only?) unmeasured. Candidate: seeded-erosion A/B with constraint-violation recall + regression-pass endpoints.

## 7. Sources (search-returned URLs only — no invented links)

Load-bearing (extractor-verified abstracts, 2026-10-04):

- http://arxiv.org/abs/1810.10855v1 — Lenarduzzi et al.: Continuous Architecture + microservices migration, postponed-activity interest, deep-refactor fault risk, single-SME WIP (F1; §2.2)
- http://arxiv.org/abs/2104.14761v1 — Besker et al.: TD + agile practices practitioner survey, n=184 BR/FI/NZ, legacy concentration + conceptualization gap, structure-verifying practices help most (F4; §2.6)
- http://arxiv.org/abs/1901.07024v2 — Fagerholm et al.: temporal discounting in TD, 33 developers / 2 companies, widespread discounting, first intertemporal-choice study in SE (F5; §2.6)
- http://arxiv.org/abs/cs/0702002v1 — Burkhardt/Detienne et al.: OO comprehension expertise × top-down/bottom-up, experts inference-driven + multi-guidance, novices execution-based (F2; §2.4)
- http://arxiv.org/abs/1510.08510v1 — DepCoL: dependency-constraint language + Eclipse change-time checking against erosion (F3; §2.3)

Supporting (SearXNG snippet-level, not extractor-verified — LIKELY max):

- https://doi.org/10.1016/j.jss.2015.09.028 — Yang et al. mapping study: 54 studies 2001–2014, 20 challenges / 29 factors / 25 lessons, costs+failures gap (S1; §2.1)
- https://doi.org/10.26686/wgtn.17007694.v1 — Agile-architecture grounded theory: 44 participants / 36 orgs, 6 forces + 5 strategies (S3; §2.1)
- https://doi.org/10.21275/ms2012134218 — Architecture-erosion cause/consequence + maturity-model direction (F3 context; §2.3)
- https://ir.cwi.nl/pub/4343 — XP comprehension: 5 practices (pairing, testing, refactoring, evolutionary design, planning) through reverse-engineering lens (F2 context; §2.4)
- http://arxiv.org/abs/2511.02810v1 — Continuous regression-testing formalization: build-chain + window model (F6; §2.5)
- https://hdl.handle.net/2027.42/154775 — RefBot thesis: interactive NSGA-II refactor + CI pull-request bot (F6; §2.5)
- https://doi.org/10.1109/icsme.2018.00068 — Continuous refactoring in CI survey, 31 developers, quality-gate triggers + barriers (F6; §2.5)
- http://arxiv.org/abs/2010.09711v2 — Business-driven TD prioritization: 5-month industrial case, 8 business factors, high-value-asset debt down (F7 neighbor; §2.7)
- https://ieeexplore.ieee.org/document/6405293/ — Refactoring × regression-testing empirical study (S2; paywalled, existence only)
- https://forums.developer.nvidia.com/t/unix-graphics-feature-deprecation-schedule/60588 — Vendor deprecation-schedule practice example (F7; general pass, thin)

Protocol-internal (cited, not re-argued):

- `retro-inventory-2026-10-03.md` §2A T-058 row + §6 G7 assignment (this sweep's charter)
- L1 bounded descent (c5c7633) + REVIEW diffusion-debt (current fragmentary coverage)
- `scales-analysis-2026-10-03.md` F3 (higher constrains lower — big→small licence); `max-plan-min-exec-2026-10-03.md` (front-load knowledge, last-responsible-moment); REFLECT Q9 second-system guard (6038450)

## 8. Provenance & Next Step

- **Engines:** SearXNG @127.0.0.1:8888 (6 queries, 2026-10-04): "continuous refactoring agile sprint architecture evolution empirical study" (science); "software architecture erosion decay technical debt sprint agile prevention" (science); "top-down bottom-up program comprehension architecture recovery refactoring" (science); "regression testing continuous integration refactoring without regression evidence" (science); "Lehman laws software evolution architecture conformance checking empirical" (science); "feature deprecation triage removal old features software product management" (general — vendor-notice recall only, marked thin accordingly). No Tavily fallback needed (science returned throughout). No URLs invented — §7 lists search-returned URLs only.
- **Extraction (127.0.0.1:8081 POST /extract):** 5 probes — full-verified: Lenarduzzi CA/microservices abstract (F1), Besker 184-practitioner survey abstract (F4), Fagerholm temporal-discounting abstract (F5), Burkhardt OO-comprehension abstract (F2), DepCoL abstract (F3). Remainder snippet-level, confidence-graded inline (§3) and labeled in §7.
- **INDEX.md:** untouched per instructions (proposed row text in header; no git operations performed).
- **Next:** single T-058 fold per §4 owners — seed the P2 constraint set from recent REVIEW diffusion-debt findings (G1), specify the P3 regression window against real CI budgets (G2), adopt the P4 ledger with priced retention on the current feature surface as its first rows, and write the P1 skip rule (G4) so empty sprints stay cheap. Close G3/G5 (pricing unit + deprecation-date policy) before any gate wording claims economic authority.

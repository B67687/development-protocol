# Foundation vs Overlay: Natural Development Trajectories and the Protocol Spine

- **Date:** 2026-10-07
- **Status:** Research (pre-decision — backs overlay ordering, settles no gate)
- **Scope:** Which natural/foundation trajectory the protocol's overlay spine (INBOX→…→HANDOVER) follows, when fuzzy vs clear is optimal, and what the cost-of-change evidence actually licenses.
- **Synthesized from:** `/tmp/research-inventory.md` (2026-10-09), `/tmp/natural-models.md` (2026-10-09), `/tmp/cost-change.md` (2026-10-09). No git. No new searches.

## 1. Problem

The protocol spine (INBOX → EXTRACTION → SERIOUSNESS → FUNDAMENTALS → AMBITION → LANDSCAPE → STRATEGY → VALIDATION → SPECIFICATION → EXECUTOR → REVIEW → REFLECT → HANDOVER) grew by accretion: each gate was backed by its own sweep, but no artifact asked whether the *order itself* matches any natural efficient trajectory — or whether the spine imposes ceremony where a lighter path would do. Two failure modes bracket the question: running heavyweight gate governance on a fuzzy front end (regime error), and skipping clarification/validation on genuinely ambiguous work (cheap-now-expensive-later). This document maps foundation models onto the overlay spine, states when each regime is optimal, and records what cost-of-change evidence does and does not license.

## 2. Landscape

Nine foundation families were swept in `/tmp/natural-models.md`, each with a fuzzy-vs-clear signature:

| Family | Fuzzy or clear | Core mechanism |
|---|---|---|
| Stage-Gate (Cooper) + Agile-Stage-Gate hybrids | Clear-gate control | Strict gates for portfolio/audit coordination; rigidity is the documented pain; ASG nests sprints inside stages |
| Lean Startup / BML (Ries, Blank) | Fuzzy-lean | MVP = max validated learning per effort; pivot/persevere; fails in regulated hardware and coach-authority conflicts |
| Agile: Scrum vs Kanban / Scrumban | Clear cadence vs fuzzy-pull | Scrum −22% defects via roles/ceremonies but brittle to change; Kanban −35% lead time under daily volatility iff strict WIP (both UNVERIFIED as population parameters) |
| Effectuation (Sarasvathy) | Fuzzy-effectual | Means-driven, affordable loss, crazy-quilt co-creation; experts effectual, novices causal; culture moderates (SV vs Munich vs Singapore) |
| Bricolage (Baker & Nelson) | Fuzzy-make-do | Recombine at-hand resources; wins when penurious, risks second-best lock-in; optimization wins when munificent |
| Real Options | Staged-clear | Stage/defer/abandon/expand as compound options; standard NPV undervalues flexibility; unmeasurable volatility breaks formal pricing |
| Set-Based Concurrent Engineering (Toyota) | Fuzzy-then-sharp | Parallel sets, trade-off curves, delayed convergence, integrate by intersection; kills late rework in coupled systems |
| Fuzzy Front End (Khurana–Rosenthal; Smith–Reinertsen) | Fuzzy-then-clear, different regimes | Holistic front end (strategy→concept→resourcing); early uncertainty reduction + draft planning → R² 0.17 efficiency / 0.24 effectiveness (n=497 SEM, LIKELY); standardized front ends best for incremental |
| Exploration–Exploitation (March) + ambidexterity | Dynamic balance | Refinement beats search short-run, self-destructs long-run (CONFIRMED mechanism); explore and exploit each predict performance strongly, ambidexterity per se weaker than focus (LIKELY, counter-consensus); advantage from *shifting* balance + agility |

Cost-of-change backdrop (`/tmp/cost-change.md`): Boehm 1:5:10:50:100 (1:10:100 simplified) is LIKELY as a *1970s large-waterfall historical baseline*, but universal DIE is CONFLICTED — Menzies et al. 2017 (N=171 projects, 2006–2014, ESE peer-reviewed, LIKELY) found no consistent late-phase penalty. Spiral is CONFIRMED definitionally, UNVERIFIED as a superiority claim. No portable "agile X% cheaper," CHAOS-delta, or TD-interest-rate constant survived extraction.

## 3. Top Findings

1. **The natural efficient trajectory is fuzzy-open → fuzzy-narrow → clear-commit → clear-scale → re-fuzz.** Generate sets and probe cheaply (effectuate/bricoler/BML) → narrow via trade-offs and option screening (SBCE, FFE screening, pivot/persevere) → commit via draft plan, gate, sprint/flow with WIP+DoD → scale via Stage-Gate launch discipline and real-options expansion → re-fuzz on discontinuity via dynamic rebalance, *not* static ambidexterity.
2. **The overlay spine already follows this trajectory** (research-inventory §2): INBOX/EXTRACTION = fuzzy-open (raw-first, readings enumeration, revealed>stated); SERIOUSNESS/FUNDAMENTALS/AMBITION = fuzzy-narrow with AMBITION deliberately late (evidence-scaffolded); LANDSCAPE/STRATEGY/VALIDATION = clear-commit (2–3 candidates + kill criteria, most-responsible-moment, heavy rule-checked SPEC gate); SPECIFICATION/EXECUTOR = clear-scale (sane-default rows, Ralph loop, window-bounded no-regress); REFLECT/HANDOVER = re-fuzz hook (calibration ledger, PARKED-with-trigger). The spine is a regime-correct overlay, not an arbitrary sequence.
3. **Fuzzy-vs-clear rule (backed):** fuzzy/ambiguous/high-stakes → full spine with late AMBITION, set-based candidates, heavy VALIDATION; clear/small/care-L1 → Light-SPEC + single pass allowed, every skip logged with reason (CONSTITUTION skippability). Stay fuzzy while uncertainty is Knightian, probes are cheaper than errors, late change is expensive, and co-creators pre-commit; go clear once uncertainty reduces to risk, coordination/audit cost dominates, or WIP+DoD is enforceable.
4. **Boehm with the Menzies caveat:** quote 1:10:100 ratios only as "100× in 1970s waterfall mega-projects; no consistent delay penalty in 171 modern projects." The protocol's fix-in-planning stance survives — but on SBCE/front-loading grounds (late rework in coupled systems is expensive), not on a universal cost curve. Never use 1:10:100 ratios or Standish percentages as decision inputs (already a not-to-encode in max-plan-min-exec).
5. **Ambidexterity warning:** treating explore+exploit balance as a steady state is refuted-as-universal; the protocol's phase structure (explore early, exploit late, REFLECT-triggered re-fuzz) matches the dynamic-balancing evidence better than a permanently dual-mode design would.

## 4. Principles

- **P1 — Regime fit over ceremony:** run fuzzy governance on fuzzy work (sets, probes, affordable-loss bets) and clear governance on clear work (gates, sprints, DoD). Regime error in either direction is the primary waste.
- **P2 — Front-load knowledge, not decisions:** generate 2–3 candidates + kill criteria, narrow on trade-off evidence, commit at the most responsible moment. (Re-affirms max-plan-min-exec; adds SBCE/FFE grounding.)
- **P3 — Late AMBITION, early probe:** competence-before-ambition; bounded provisional want early, full AMBITION post-evidence. Falsifier: if early ambition increases downstream rework questions, the ordering fails.
- **P4 — Calibrate cost claims to context:** cite Boehm only with Menzies; cite defect/lead-time deltas only with N and context; every efficiency claim ships with its falsifier and gate-collision prediction.

## 5. Not-to-encode

- **N1 — No 1:10:100 ratios or Standish percentages as decision inputs** (ratios are historical, not universal; CHAOS methodology contested).
- **N2 — No mainstream gate governance on the fuzzy front end** (Khurana/TU/e regime separation).
- **N3 — No static ambidexterity** (weaker than focus; dynamic shifting + agility instead).
- **N4 — No Black-Scholes precision on Knightian uncertainty** (use affordable-loss + staging heuristics).
- **N5 — No single-point + test-fix when sets are affordable** (late-fix trap).
- **N6 — No copying one culture's effectuation/causation mix into another without adjustment** (SV/Munich/Singapore boundary conditions).
- **N7 — No universal TD interest rate, "agile X% cheaper," NIST $59.5B, or MVP-validates-cleanly claims** (all UNVERIFIED in-sweep).

## 6. Open Gaps

1. Reorder evidence (g9): evidence-before-AMBITION is protocol-internal; needs reorder A/B on downstream rework questions.
2. Per-model calibration dataset (g6/power-verification): support + teaching ledgers specified but N=1.
3. Bench spine: only 2 draft pilots (A14, A20), neither frozen/preregistered; no VALIDATION- or SERIOUSNESS-gate bench.
4. Master + checker (retro-t003): rebuild-from-design unproven; manifest + rendering-diff checker unbuilt.
5. Provenance machinery (g8): sidecar location, sensitivity lattice, signer unsigned.
6. Light-SPEC generality: folded on N=1; SPEC chore still highest friction protocol-wide.
7. Taste T2 threshold: no count for "demonstrated preference"; homogenisation +10.7% unaddressed.
8. Transcript numbers (90→14d knee, 10–20% slice, 44% prune) are priors; A20 cold-reader ≤5min untested.
9. Read et al. effectuation–performance meta numeric r; Henard & Szymanski 2001 full text; Smith–Reinertsen peer-reviewed test; IEEE 5314228 effect sizes — all unretrieved.
10. Menzies DIE replication on local repo history (introduction-phase vs fix-effort) before adopting a flattened-curve policy.

## 7. Sources

Natural models (`/tmp/natural-models.md`, SearXNG `categories=science`, 9 sweeps + 2 follow-ups, 2026-10-09): March 1991 10.1287/orsc.2.1.71; Evanschitzky et al. 2012 10.1111/j.1540-5885.2012.00964.x; Khurana & Rosenthal 10.1111/1540-5885.1510057 + S0737678298900984; Verworn/Herstatt SEM econstor 684384019; ambidexterity meta 10.1108/s1479-067x20140000014009; SBCE-Toyota ASME DETC94 + OUP ch.421073318 + 10.1016/j.procir.2019.04.276; effectuation UTwente 10.3990/1.9789036545211 + 10.1007/s11187-021-00531-3 + SEJ 10.1002/sej.1201/1413; bricolage 10.1002/sej.1150 + 10.1016/j.jretconser.2023.103495; Scrum/Kanban 10.15439/2022f143; LSC 10.1080/00472778.2019.1659677; IT-ambidexterity 10.1111/deci.12399; Cooper & Sommer 10.1080/08956308.2016.1117317; IEEE TEM 10.1109/tem.2023.3282269. Cost-of-change (`/tmp/cost-change.md`, 8 science + 4 general queries + 8 extractions, 2026-10-09): Boehm 1981 + Boehm & Basili 10.1007/3-540-27662-9_26; Menzies et al. ESE 2017 / arxiv:1609.04886; Boehm 1988 spiral + 10.1109/mc.2020.2971402; SRI 10.11591/ijeecs.v21.i1.pp524-537; TD principal–interest 10.1016/j.infsof.2020.106391 + TD-BP 10.1145/3194164.3194175; NFR-rework 10.1109/re48521.2020.00021; PAM 10.1007/s10664-023-10331-w; startup prototyping 10.1007/978-3-319-57633-6_2; MVP/pivot arxiv:1808.05630 + arxiv:1710.04037. Inventory (`/tmp/research-inventory.md`, 2026-10-09): 45-artifact map, natural-trajectory diagram, 10-item gap list.

## 8. Provenance

- **Method:** synthesis only — no new searches, no git. Confidence labels inherited from source sweeps (CONFIRMED ≥2 independent peer-reviewed; LIKELY = 1 peer-reviewed + 1 community or strong single; UNVERIFIED = single/community; CONFLICTED = disagreeing authorities). arXiv = PREPRINT (never CONFIRMED alone); paywalled DOI = snippet-only (LIKELY max); Wiki = tertiary (supports LIKELY only with peer-reviewed partner).
- **Limits:** DOI landings mostly paywalled → abstract + converging-source discipline; effect magnitudes for Scrum −22% / Kanban −35%, LSC "strong and robust," and TD correlations are directionally usable but not population parameters — do not quote as constants. Smith–Reinertsen front-end loading is practitioner heuristic, not independently verified.
- **RESEARCH_BASIS row suggestion:** add to the basis file backing the spine-ordering claim — `foundation-vs-overlay-2026-10-07.md | trajectory fit (fuzzy-open→…→re-fuzz), fuzzy/clear decision rule, Boehm+Menzies, overlay mapping | natural-models + cost-change + inventory sweeps 2026-10-09 | spine order regime-correct; 10 gaps open`.
- **INDEX row:** `| [foundation-vs-overlay-2026-10-07.md](foundation-vs-overlay-2026-10-07.md) | 2026-10-07 | Foundation-vs-overlay: natural trajectory fuzzy-open→fuzzy-narrow→clear-commit→clear-scale→re-fuzz, when each regime is optimal, Boehm with Menzies caveat, overlay mapping to INBOX→…→HANDOVER, 10 gaps. |`

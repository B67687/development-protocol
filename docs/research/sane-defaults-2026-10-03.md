# LANDSCAPE Research: Sane Defaults — Convention over Configuration, Sensible Defaults, Defaults in Specs (THOUGHT T-065)

- **Date:** 2026-10-03
- **Thought:** T-065 — sane defaults must be found and in specs
- **Core pattern:** A thinking agent must ship usable behaviour without interrogating the user on every knob: pick the default the common case wants (convention over configuration, sensible defaults), write that default into the spec (visible, reasoned, overridable), and use choice architecture deliberately — because whatever is pre-selected will be selected, whether intended or not.
- **Method:** SearXNG self-hosted (127.0.0.1:8888, categories=science first, then general follow-ups), Trafilatura extractor sidecar (127.0.0.1:8081) probes on 5 targets (2 full-verified, 1 partial, 2 failed/blocked), confidence labels per claim.
- **Confidence scheme:** CONFIRMED = replicated/meta-analytic or canonical primary source; LIKELY = single-study or strong convergent theory; UNVERIFIED = plausible, needs verification.

## 1. Problem Statement

T-065 observes a specification failure mode: defaults that are absent (every knob forced on the user), hidden (magic behaviour nowhere in the spec), or arbitrary (a value picked because something had to fill the slot). The user pays three times: decision fatigue up front, surprise when the hidden default fires, and rework when the arbitrary default turns out wrong. The research question: what does the literature say about (a) convention-over-configuration as an engineering doctrine for suppressing needless decisions, (b) sensible defaults as a usability/API-design practice, (c) defaults as choice-architecture interventions (nudge theory) with measured effect sizes and moderators, and (d) what must therefore be written into specs so a default is sane rather than merely present?

## 2. Field Landscape

Six sub-fields surveyed; behavioural-science core strong with pooled effect sizes, software-doctrine side canonical-but-thin (itself a finding).

| # | Sub-field | Key anchor | Strength |
|---|-----------|-----------|----------|
| 1 | Convention over configuration (engineering) | Rails CoC/DRY/MVC thesis; Esfinge metadata CoC model (Wiley) + Authorea preprint twin | CONFIRMED (as stable doctrine), LIKELY (lab effect) |
| 2 | Choice-architecture effectiveness | Mertens et al. PNAS 2022 meta-analysis (+ correction record) | CONFIRMED (pooled ES, snippet-only) |
| 3 | Default-effect mechanisms | Jachimowicz et al. BPP 2018 meta-analysis (58 studies, n=73,675) | CONFIRMED (pooled ES, extractor-verified intro) |
| 4 | Retirement-savings auto-enrollment | Madrian & Shea via Jachimowicz intro; HRS older-American study; NBER match study; Form 5500 724-plan panel | CONFIRMED (participation direction), LIKELY (contribution nuances) |
| 5 | Organ-donation opt-in vs opt-out | Johnson & Goldstein classic gap vs 2024 five-country longitudinal null; presumed-consent ethics review | LIKELY (classic gap UNVERIFIED primary; null LIKELY snippet-only) |
| 6 | Defaults-at-the-boundary (privacy/dark patterns) | UOOM privacy-expectations paper; dark-patterns chapter; CCPA opt-out study; CMP non-compliance measurement | LIKELY |

## 3. Top Findings (with confidence + effect sizes)

**F1. Choice architecture works on average (d ≈ 0.45), and restructuring the choice beats redescribing it. (CONFIRMED, meta-analytic; numeric ES snippet-only)**
Mertens et al. PNAS meta-analysis (https://doi.org/10.1073/pnas.2107346118, snippet): pooled effect ≈ 0.45, 95% CI [0.39, 0.52]; interventions targeting decision structure (organisation of alternatives — which includes defaults) consistently outperform decision-information and decision-assistance interventions; food choices up to 2.5× more responsive than other domains; moderate publication bias toward positive results. Correction record exists (https://doi.org/10.1073/pnas.2204059119). Full text not extractor-verified (empty response) — pooled number capped at snippet trust. T-065 corollary: the spec's highest-leverage default work is structural (what is pre-selected, how many alternatives, in what order), not explanatory (better help text around a bad default).

**F2. Defaults per se carry d ≈ 0.68, but effectiveness is moderated by domain and mechanism — endorsement and endowment win. (CONFIRMED, meta-analysis 58 studies n=73,675; extractor-verified intro)**
Jachimowicz et al. (https://doi.org/10.1017/bpp.2018.43, extractor-verified intro): pooled d = 0.68, 95% CI 0.53–0.83; most studies positive, several null, two negative; consumer-domain defaults more effective, environmental-domain less; defaults operating through endorsement (seen as the architect's recommendation) or endowment (seen as status quo) more effective. Also recovered: axiomatic default-bias characterisation (https://www.ssrn.com/abstract=3133552, snippet) and "Why Default Nudge Works" record (https://www.aeaweb.org/doi/10.1257/rct.12458-1.0, snippet). T-065 corollary: a sane default must declare its mechanism — is it a recommendation ("we think you should") or a status quo ("this is where you start")? — because the two persuade differently, and neither works everywhere.

**F3. Auto-enrollment moves participation dramatically but not contribution depth — defaults get people in the door, not up the stairs. (CONFIRMED direction; LIKELY magnitude details)**
Via Jachimowicz intro (extractor-verified): employees 50% more likely to participate when enrollment is the default (Madrian & Shea 2001). Convergent: HRS 2008/2010 waves (https://doi.org/10.7249/wr1117, snippet) — auto-enrollment associated with large participation increase, especially lower-income/less-educated/minority, but not with longer-run contribution status; opt-outs resemble voluntary non-participants (financially unprepared). NBER match study (https://doi.org/10.3386/w13352; PDF https://www.nber.org/papers/w13352.pdf, snippet): match moves opt-out only modestly (5–11 pp). Form 5500 panel, 724 plans 2017–2021 (https://doi.org/10.17918/00011419, snippet): auto-enrollment associated with lower loan usage but alone does not raise contribution amounts; admin attentiveness correlates with higher contributions. T-065 corollary: spec every default with its level — participation-default (get started) vs intensity-default (how much/how far) — and never assume the first implies the second; the contribution rate needs its own default and its own escalation rule.

**F4. The organ-donation showcase gap (opt-out 90s% vs opt-in 10s%) does not survive longitudinal testing — defaults without infrastructure do not move bodies. (LIKELY caution; classic figure UNVERIFIED primary, null LIKELY snippet-only)**
Classic Johnson & Goldstein gap as relayed in Jachimowicz intro (extractor-verified relay, primary UNVERIFIED here): one-word framing difference, high-90s vs tens signup. Counter-evidence: five-country longitudinal analysis Argentina/Chile/Sweden/Uruguay/Wales (https://doi.org/10.1016/j.puhe.2024.08.009, snippet): switching opt-in→opt-out produced no average increase, no gradual slope change; COVID reduced odds everywhere. Backdrop: procurement-system review (http://arxiv.org/abs/1203.4289v5, snippet), presumed-consent ethics review under principlism (https://doi.org/10.15385/jbfp.2017.3.1.5, snippet), practitioner explainer (https://theconversation.com/organ-donation-does-presumed-consent-work-49478, snippet). T-065 corollary: never spec a default on the strength of a cross-sectional showcase alone; the organ case is the standing warning that family veto, logistics, and awareness dominate the default switch. Any default whose fulfilment depends on downstream infrastructure must spec the infrastructure, not just the pre-selection.

**F5. Convention over configuration is a mature doctrine with one lab win — and its failure mode is named ("explicit is better than implicit"). (CONFIRMED as doctrine; LIKELY as lab effect)**
Rails thesis record (http://www.theseus.fi/handle/10024/152418, snippet): CoC + DRY + MVC as the framework's distinctive principles, suppressing XML configuration files. Esfinge metadata-CoC model (https://onlinelibrary.wiley.com/doi/10.1002/smr.70028, snippet; preprint twin https://www.authorea.com/doi/full/10.22541/au.168067455.55373151/v1): conventions decoupled from framework logic; lab result 75% of participants implemented faster than with raw Reflection API, fewer lines, higher Reflection failure rate. Sensible-defaults existence proof in the wild: musclesyneRgies R package ships "sensible defaults" as a design claim (https://joss.theoj.org/papers/10.21105/joss.04439, snippet). Canonical dissent relayed via extractor-verified CoC exposition: Zen of Python "explicit is better than implicit" + DSL/limited-hook critique. T-065 corollary: CoC earns its keep only when the convention matches the common case and the escape hatch is one line; otherwise convention becomes magic. Spec rule: every convention names its override.

**Supporting (LIKELY/UNVERIFIED):**
- S1. Donation defaults are self-canceling: 8 studies, 11,508 participants, 2,423 decisions (https://doi.org/10.1509/jmr.15.0001, snippet) — low defaults raise rate but lower amount (lower-bar + scale-back), any defaults distract from other cues, net revenue rose in field. Supports F3's level-separation: rate-default and amount-default fight each other.
- S2. Regulatory choice screens break default inertia measurably: DMA browser switching 8.5%→13.2%, alt-browser share 19.6%→24.5% (https://journal.idscipub.com/legalis/article/view/1126, snippet-only — LIKELY direction, UNVERIFIED causally). Supports mandating active choice where defaults entrench power.
- S3. Nudge ethics is settled only at the abstract level: Sunstein's seven propositions (http://nrs.harvard.edu/urn-3:HUL.InstRepos:17915544, snippet) — architecture unavoidable, object to tokens not types, transparency + legitimate ends required; manipulation critique (https://doi.org/10.1017/s1867299x00002762, snippet) supplies the four-type framework. Supports N-governance below.
- S4. Nudge book canon record (https://openlibrary.org/books/OL23681251M/Nudge, extractor-verified partial): libertarian paternalism as middle path between command and neutrality. Use as shorthand, never as evidence.
- S5. Personalized choice architecture via ML (http://arxiv.org/abs/1907.02100v1, snippet) — population nudges generalise weakly to individuals; supports per-context default tuning (G-gaps).
- S6. Peer-information can backfire oppositionally in 401(k) (https://doi.org/10.1111/jofi.12258, snippet) — supports pairing every default with non-comparative justification.

## 4. Design Principles for T-065 (encode these)

**P1. Default the common case, and write the default into the spec with rationale + override (F5 + F2).**
Every default appears in the spec as one row: value, who it serves (the common case, named), mechanism (endorsement vs endowment/status-quo), and the one-line override. No spec row, no default — undocumented magic is a bug. Rationale cites the common-case evidence, never "CoC says so."

**P2. Separate the participation-default from the intensity-default, and make both reversible at the cost of one step (F3 + S1).**
Ship two defaults where F3 applies: the get-started default (enrollment, enablement) and the how-much default (rate, depth, verbosity), because the first never implies the second and S1 shows they can cancel out. Both must be reversible in one obvious step; reversibility cost is itself spec'd (what happens to data already created under the old default).

**P3. Condition the default on domain and infrastructure, and trigger re-review when either moves (F1 + F4).**
Decision-structure defaults first (F1: restructure before rewording); each default carries its scope conditions (domain responsiveness per F2, downstream infrastructure per F4 — family veto, logistics, awareness). Spec lists the invalidation trigger ("revisit if X changes"), so the organ-donation error — a default surviving the world that defeated it — cannot recur.

## 5. Not-to-Encode (explicit non-goals)

- **N1. Do not encode hidden defaults.** Any behaviour the user cannot discover from the spec is forbidden, however "sensible" (F5's explicit-over-implicit dissent; privacy S-archives show hidden defaults read as deception).
- **N2. Do not cite CoC, KISS, or Nudge as evidence.** Doctrines and book records (S4, F5) are shorthands, never justifications. Justifications cite F1–F4 effect sizes and moderators.
- **N3. Do not assume opt-out always works.** F4's five-country null forbids the "just flip to opt-out" reflex. Any opt-out proposal must name the infrastructure it depends on or be rejected at review.
- **N4. Do not manipulate via defaults (no dark defaults).** Pre-selection that benefits the architect at the user's expense — obstruction, misdirection, non-consensual data capture (S-archives: CCPA opt-out hurdles http://arxiv.org/abs/2409.09222v1; CMP non-compliance http://arxiv.org/abs/2202.00885v3; privacy-expectations analysis http://arxiv.org/abs/2603.15705v1) — is out of scope even when legal. Transparency + legitimate ends (S3) is the floor.
- **N5. Do not fix defaults permanently.** No default ships without a re-review trigger (P3). A default with no expiry or invalidation condition is a future incident report.

## 6. Open Gaps

- **G1.** Mertens pooled ES (0.45) and publication-bias-adjusted ES unverified beyond snippet — PNAS extractor probe returned empty; retrieve full text via institutional access before quoting 0.45 in any spec rationale.
- **G2.** Jachimowicz moderator magnitudes (endorsement vs endowment; consumer vs environmental gaps) direction-verified, numerically snippet-only — full-text retrieval pending before encoding differential default strength.
- **G3.** Organ-donation five-country null (Public Health 2024) snippet-only — retrieve before citing against any opt-out proposal; confirm model (Bayesian aggregated binomial) and per-country slopes.
- **G4.** Madrian & Shea 2001 primary unrecovered (relayed via Jachimowicz intro) — retrieve before quoting "50% more likely" as a number; HRS/NBER/Form-5500 nuances all snippet-only.
- **G5.** Esfinge 75%-faster lab result single-study, snippet-only — replicate-or-discount before using CoC productivity claims in framework guidance; no agent-spec-default compliance study exists at all.
- **G6.** Connector coverage: full 14-connector fan-out (Self-Hosted-Search measure.connectors + discovery) not executed from this sandbox; science-category SearXNG engines (arxiv, pubmed, crossref, semantic scholar, openalex, plus Bing/Brave/Dogpile families) served as proxy. Re-run academic-strategy fan-out before promoting any LIKELY above its current cap.
- **G7.** Agent-transfer gap: all F-evidence is human decision-makers; agent-set defaults (what the agent pre-selects on the user's behalf) and user override-rate under agent defaults unmeasured. Candidate: default-adherence A/B (spec'd default vs forced choice) on task success, override rate, and regret.
- **G8.** Privacy-default jurisprudence moving fast (UOOM defaults, CPA pre-installed-software dispute) — legal review required before any data-collection default ships; do not generalise from S-archives without counsel.

## 7. Sources (search-returned URLs only — no invented links)

1. https://doi.org/10.1073/pnas.2107346118 — Mertens et al. choice-architecture meta-analysis, pooled ≈0.45 (F1; snippet).
2. https://doi.org/10.1073/pnas.2204059119 — same meta-analysis correction record (F1; snippet).
3. https://pnas.org/doi/full/10.1073/pnas.2204059119 — correction twin (F1; snippet).
4. https://doi.org/10.1017/bpp.2018.43 — Jachimowicz et al. default-effects meta-analysis, d=0.68 n=73,675 (F2/F3/F4 relays; extractor-verified intro).
5. https://www.ssrn.com/abstract=3133552 — Default bias axiomatic characterisation (F2 support; snippet).
6. https://www.aeaweb.org/doi/10.1257/rct.12458-1.0 — Why Default Nudge Works record (F2 support; snippet).
7. https://doi.org/10.1509/jmr.15.0001 — Donation defaults, 8 studies 11,508 participants (S1; snippet).
8. https://doi.org/10.17918/00011419 — 401(k) Form 5500 panel, 724 plans 2017–2021 (F3; snippet).
9. https://doi.org/10.7249/wr1117 — Auto-enrollment HRS older Americans (F3; snippet).
10. https://doi.org/10.3386/w13352 — Employer match under auto-enrollment (F3; snippet).
11. http://www.nber.org/papers/w13352.pdf — same study PDF twin (F3; snippet).
12. https://doi.org/10.1111/jofi.12258 — Peer-information oppositional effect in 401(k) (S6; snippet).
13. https://doi.org/10.1016/j.puhe.2024.08.009 — Opt-out defaults do not increase organ donation, 5-country longitudinal (F4; snippet).
14. http://arxiv.org/abs/1203.4289v5 — Organ procurement presumed vs explicit consent (F4 backdrop; snippet).
15. https://doi.org/10.15385/jbfp.2017.3.1.5 — Presumed consent principlism review (F4; snippet).
16. https://theconversation.com/organ-donation-does-presumed-consent-work-49478 — Presumed-consent practitioner explainer (F4; snippet).
17. http://www.theseus.fi/handle/10024/152418 — Rails CoC/DRY/MVC thesis (F5; snippet).
18. https://onlinelibrary.wiley.com/doi/10.1002/smr.70028 — Esfinge metadata CoC model, 75% faster (F5; snippet).
19. https://www.authorea.com/doi/full/10.22541/au.168067455.55373151/v1 — Esfinge preprint twin (F5; snippet).
20. https://joss.theoj.org/papers/10.21105/joss.04439 — Sensible-defaults package claim (F5; snippet).
21. http://nrs.harvard.edu/urn-3:HUL.InstRepos:17915544 — Sunstein nudging/choice-architecture ethics, 7 propositions (S3; snippet).
22. https://doi.org/10.1017/s1867299x00002762 — Nudge and manipulation of choice (S3; snippet).
23. https://openlibrary.org/books/OL23681251M/Nudge — Nudge book record, libertarian paternalism (S4; extractor-verified partial).
24. http://arxiv.org/abs/1907.02100v1 — ML + behavioural economics personalised choice architecture (S5; snippet).
25. https://journal.idscipub.com/legalis/article/view/1126 — DMA choice screens, browser switching 8.5%→13.2% (S2; snippet).
26. https://doi.org/10.1590/1982-7849rac2022220098.en — Nudging and choice architecture perspectives (snippet).
27. https://doi.org/10.1111/acfi.12471 — Nudge theory financial-markets review (snippet).
28. http://arxiv.org/abs/2603.15705v1 — Privacy expectations vs default opt-out UOOM (N4; snippet).
29. https://doi.org/10.1007/978-3-031-28643-8_9 — Dark patterns hows-and-whys, privacy chapter (N4; snippet).
30. http://arxiv.org/abs/2409.09222v1 — Dark patterns in CCPA opt-out (N4; snippet).
31. http://arxiv.org/abs/2202.00885v3 — Opted-out yet tracked, CMP non-compliance (N4; snippet).

## 8. Provenance & Next Step

- **Engines:** SearXNG @127.0.0.1:8888 (categories=science primary: CoC, sensible defaults, choice architecture, default bias, auto-enrollment, organ-donation defaults, privacy defaults; general follow-ups: Rails doctrine, Johnson–Goldstein, Madrian–Shea, libertarian paternalism). Science-category engines incl. arxiv, pubmed, crossref, semantic scholar, openalex, plus Bing/Brave/Dogpile families. Extractor sidecar @127.0.0.1:8081 verified reachable (FastAPI agent-search-extractor, Swagger at /docs, POST /extract): 5 probes — 2 full-verified (CoC exposition; Jachimowicz BPP intro with Johnson–Goldstein/Madrian–Shea relays), 1 partial (OpenLibrary Nudge record), 2 failed (PNAS 2107346118 empty-response; organ-donation DOI resolve 404). Remainder snippet-only.
- **Connectors (14):** full Self-Hosted-Search measure.connectors + discovery fan-out not executed from this sandbox (see G6); science-category SearXNG engines served as proxy coverage for arxiv/pubmed/crossref/semantic-scholar/openalex. No URLs invented — §7 lists search-returned URLs only.
- **INDEX.md:** untouched per instructions (no INDEX.md exists in repo; proposed row text below for the owner to insert).
- **Next:** T-065 design — encode P1–P3 + N1–N5 as defaults-in-specs policy (spec'd default row, participation/intensity separation, conditional defaults with re-review triggers); close G1–G4 before quoting any numeric effect size in rationale.

> **Proposed INDEX row (exact text, do not insert without owner go):**
> `| sane-defaults-2026-10-03 | T-065 sane defaults (convention over configuration, sensible defaults, defaults in specs) | docs/research/sane-defaults-2026-10-03.md |`

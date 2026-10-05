# Thought-Log Handling — Ideation Capture, Synthesis & Disposition (2026-10-03)

> How to handle thought logs / ideation capture correctly: verbatim append-only capture, a disposition state machine with research as mandatory gate, traceability back to source, and anti-patterns. Last updated: 2026-10-03. Local protocol grounding observed in `development-protocol-local/` (THOUGHT_LOG.md T-001..T-068, BACKLOG.md, HANDOVER.md) and `development-protocol/steps/` (RULES.md, INBOX.md, LANDSCAPE.md, EXTRACTION.md).

---

## 1. Executive Summary

**Key finding:** the literature supports every half of the proposed loop and warns against the other half being skipped. Writing ideas down externally reliably improves performance on demanding memory tasks (choice-to-offload beats memory-alone, η²G = 0.105 **[LIKELY]**); a concrete if-then disposition ("if T-### researched, then fold/park/drop with reason") reliably moves goal attainment with small-to-medium effects (g ≈ 0.31–0.34 **[CONFIRMED]**); linking raw thoughts to downstream artifacts improves maintenance accuracy (+86% in one trial **[LIKELY]**); and single-pass unstructured search is provably insufficient for synthesis-grade retrieval (92.9% recall at 0.13% precision **[CONFIRMED as measurement]**). The contested point is *verbatim-vs-paraphrase*: verbatim capture aids factual fidelity but raw transcription without reframing is associated with shallower processing — the effect is conditional, not universal **[CONFIRMED as contested]**.

| # | Finding | Confidence | Effect size |
|---|---------|-----------|-------------|
| 1 | External capture (writing down) beats internal memory; advantage grows with load | LIKELY | η²G = 0.105 (condition), 0.085 (condition × load) |
| 2 | If-then disposition plans reliably raise goal attainment | CONFIRMED | g = 0.336 [0.229, 0.443]; experimenter-led g = 0.465 vs document-only g = 0.277 |
| 3 | Verbatim-vs-paraphrase effect is conditional, not universal | CONFIRMED (as contested) | Direction varies by replication; no single number |
| 4 | Traceability links improve maintenance/change accuracy; link upkeep is the main cost | LIKELY | +86.06% task accuracy (one trial); 63-study mapping agrees on benefit/cost split |
| 5 | Single-engine unstructured search cannot carry synthesis | CONFIRMED (measurement) | Recall 92.9% at precision 0.13% (n = 396 refs, 14 reviews) |

Method: SearXNG (`categories=science`) first-pass, Trafilatura sidecar (`127.0.0.1:8081/extract`) full-text for load-bearing claims, connector fan-out via `measure/connectors.py` (degraded — stubs only, see §8). Confidence per the house scheme: ≥2 independent peer-reviewed sources = CONFIRMED; 1 peer-reviewed + 1 community = LIKELY; preprint-only = LIKELY max; single/snippet-only = UNVERIFIED.

---

## 2. Ideation Capture: Verbatim, Append-Only

**Rule the evidence supports: capture first, in the originator's own words, append-only; corrections are new entries that reference, never edits.** This matches the existing local rule (THOUGHT_LOG header: append-only, corrections via new referencing entry; VERBATIM vs RECALLED honesty flag — LOCAL-CONFIRMED, observed in file).

- **Offloading works, and load is the moderator.** A peer-reviewed registered report (N = 114) replicating Risko & Dunn (2015) found letting participants write items down beat memory-alone (F(1,113) = 96.539, p < .001, η²G = 0.105), with a condition × load interaction (η²G = 0.085): the heavier the load, the larger the payoff **[LIKELY]** (https://doi.org/10.1186/s41235-019-0201-4). A second independent study found offloading improved delayed-intention fulfilment in both instructed and spontaneous groups **[LIKELY]** (https://doi.org/10.1186/s41235-019-0195-y). Design implication: the log pays off exactly when the idea backlog is large — i.e. always, at T-068 scale.
- **Capture is driven by felt uncertainty, not actual ability.** The same programme shows people offload more when *less confident*, regardless of actual memory ability, and that the confidence→offloading link holds for spontaneously generated strategies too **[LIKELY]** (https://doi.org/10.1186/s41235-019-0195-y). Design implication: make capture frictionless and unconditional (INBOX.md's "one sentence, no paraphrase" checkpoint) because the agent/human will not reliably judge which thoughts are "worth" logging.
- **Uncaptured intentions intrude; a made plan quiets them.** Masicampo & Baumeister (2011, JPSP: "Consider it done! Plan making can eliminate the cognitive effects of unfulfilled goals") report that forming a specific plan eliminates the working-memory intrusion of unfulfilled goals **[LIKELY — abstract/record-level; APA full text paywalled, see §8]** (https://doi.org/10.1037/a0024192). Converging field evidence: unfinished afternoon tasks predict evening affective rumination, which mediates next-morning vitality loss **[LIKELY — abstract-level]** (https://doi.org/10.3389/fpsyg.2022.935775). Design implication: LOGGED alone does not quiet the loop — only a *plan with a next step* (the RAW→disposition transition) does. A log that only accumulates is a rumination engine.
- **The cost of paraphrase is real but conditional.** Mueller & Oppenheimer (2014) found laptop note-takers transcribed verbatim, processed shallowly, and did worse on conceptual questions **[LIKELY — abstract-level; publisher page blocked extraction, see §8]** (https://journals.sagepub.com/doi/10.1177/0956797614524581). But a classroom replication found the *opposite* locus (factual-recall benefit, no conceptual deficit), and follow-ups show verbatim notes aid factual review **[CONFIRMED as contested]** (https://doi.org/10.17705/1atrr.00041; https://scholar.colorado.edu/concern/undergraduate_honors_theses/00000047w). Design implication, resolved: **verbatim at capture, paraphrase at synthesis.** The log entry stays verbatim (fidelity); the LANDSCAPE/SYNTHESIS step does the reframing (depth). Collapsing both into one step is what the literature punishes.

---

## 3. The Disposition State Machine

**Rule the evidence supports: every logged idea carries an explicit if-then disposition, and "no decision" is a state with an owner and a revisit date, not the absence of one.** The existing local Disposition key (BUILT/PARKED/DROPPED/OPEN) is the right skeleton; what is missing is the *gate* between states.

Proposed states (superset of local key, mapped):

| State | Meaning | Exit condition |
|-------|---------|----------------|
| LOGGED | Verbatim captured, source-flagged (VERBATIM/RECALLED) | Triage within one session: → RAW |
| RAW | Acknowledged, not yet understood; restated problem in own words | Research question framed → LANDSCAPE |
| LANDSCAPE | Evidence being gathered per §6 | Synthesis written → SYNTHESIS |
| SYNTHESIS | Verdict + confidence + what-would-change-my-mind recorded | Fold, park, or drop decided |
| FOLDED | Folded into artifact with backlink (ex-BUILT) | Terminal; link both directions |
| PARKED | Deferred with home + revisit trigger (date or event) | Terminal until trigger fires → RAW |
| DROPPED | Rejected with reason (one line suffices) | Terminal |

- **If-then format is the load-bearing mechanism.** Gollwitzer & Sheeran's meta-analysis is commonly cited at d = 0.65 across ~94 tests **[LIKELY — primary paywalled (https://linkinghub.elsevier.com/retrieve/pii/S0065260106380021); figure converges across secondary reviews, e.g. https://osf.io/u4znb_v1]**. Two fully extracted meta-analyses pin the modern range: MCII (mental contrasting + implementation intentions), k = 24, N = 15,907, g = 0.336, 95% CI [0.229, 0.443], I² = 59.3% **[CONFIRMED]** (https://doi.org/10.3389/fpsyg.2021.565202); implementation intentions for substance-use reduction, g = 0.31 for alcohol and tobacco **[LIKELY — abstract-level]** (https://doi.org/10.1016/j.drugalcdep.2020.108120).
- **The moderator that matters for protocol design: experimenter-led (g = 0.465) beats document-only (g = 0.277)** — same MCII meta-analysis **[CONFIRMED as measured]**. A disposition written *to* a checklist nobody enforces is the document condition. The gate must therefore be *machine-checked* (lint Rule 11 pattern: session-close 1:1 coverage, line-count monotonic — LOCAL-CONFIRMED) rather than advisory.
- **Commitment is a precondition, not a bonus.** Implementation intentions fail without goal commitment, and work only when the superordinate goal is active **[LIKELY]** (https://en.wikipedia.org/wiki/Implementation_intention; https://doi.org/10.1177/0146167204271308 — abstract-level). Design implication: the SYNTHESIS→FOLDED transition must record *which goal* the idea serves; an idea with no goal home is PARKED or DROPPED, never left OPEN indefinitely.
- **Practitioner convergence (not science):** GTD's capture→clarify→organize→review loop (Allen, practitioner literature — no URL claimed) and the existing RULES.md Thought-log/Interruption gates (LOCAL-CONFIRMED) independently converged on the same shape: capture verbatim now, decide explicitly later, review on a cadence. Treat as **[UNVERIFIED]** scientifically, but as two independent field replications of the design.

---

## 4. Research Synthesis Before Fold

**Rule the evidence supports: no FOLDED transition without a written synthesis that states a verdict, a confidence label, and what would change the verdict.** Research is the mandatory gate between LANDSCAPE and FOLDED — not a decoration applied after the decision.

- **Small, cheap, structured interventions move outcomes.** Beyond the g ≈ 0.3–0.47 planning effects above, even low-intensity "consider-the-opposite" prompts measurably mitigate anchoring bias in a survey experiment on 1,221 public managers **[LIKELY — abstract-level]** (https://doi.org/10.1111/puar.13211), and a pre-registered study tests elaborative vs answer-feedback against confirmation bias **[UNVERIFIED — record-level only]** (https://doi.org/10.1016/j.cedpsych.2020.101844). Design implication: the synthesis template should *require* one considered opposite ("strongest case against folding this") — the cheapest debiasing intervention with experimental support.
- **The synthesis must precede the edit, structurally.** Existing protocol analogues already do this: EXTRACTION's readings gate (enumerate+pin+confirm before use) and LANDSCAPE's P2b entry gate (no entry unless SERIOUSNESS=COMMIT) — LOCAL-CONFIRMED. The thought-log gate should mirror their wording: *no FOLDED unless SYNTHESIS exists with verdict + confidence + falsifier*.
- **What a sufficient synthesis contains (minimum):** (a) the research question the idea raised; (b) sources consulted with per-claim confidence (the CONFIRMED/LIKELY/UNVERIFIED scheme); (c) the verdict (fold/park/drop); (d) what would change the verdict; (e) backlink to the verbatim T-entry. Anything less is research-as-decoration (§7).

---

## 5. Traceability Back to Verbatim Source

**Rule the evidence supports: every folded idea keeps a bidirectional link (artifact → T-entry and T-entry disposition → artifact ref); link maintenance is budgeted as a first-class cost, not assumed free.**

- **Links pay: +86% maintenance accuracy in a controlled trial.** An empirical study (28 industry/academia subjects, 5 maintenance tasks, TraceLink prototype vs no-link control) found traceability links improved task accuracy by 86.06% **[LIKELY — abstract-level; IEEE full text not extracted, see §8]** (https://doi.org/10.1109/access.2013.2286822).
- **A 63-study mapping agrees on the benefit/cost split.** A systematic mapping study (63 studies, 2000–2020) finds traceability supports 11 maintenance/evolution activities (change management most frequent), easing change management is the main benefit, and establishing/maintaining links is the main cost — while explicitly calling for stronger industrial evidence **[LIKELY — preprint cap; arXiv abstract-page extraction]** (https://arxiv.org/abs/2108.02133). Design implication: the "Disposition → refs" field already exists locally (LOCAL-CONFIRMED); add the reverse link (artifact cites T-NNN) and *budget* link repair at session close, because the literature says link rot is the dominant cost.
- **Pre-RS traceability is the direct analogue.** A 2023 systematic review (77 articles, 1992–2022) studies exactly our case: linking downstream artifacts back to their *origin* (stakeholder utterances, meeting protocols) rather than only forward to code **[LIKELY — abstract-level]** (https://doi.org/10.1007/s00766-023-00412-z). A thought log is pre-requirements traceability for a single-stakeholder project: T-entries are the stakeholder utterances. Fold without backlink = post-RS traceability only = the configuration the literature identifies as the researched-and-found-wanting half.

---

## 6. What Good LANDSCAPE Looks Like

**Rule the evidence supports: fan-out across independent engines, full-text extraction before citing, archive fallback when blocked, per-claim confidence, and an explicit statement that one engine is never enough.**

- **One engine is provably insufficient.** Translating 14 Cochrane systematic-review strategies to Google Scholar retrieved 291,190 hits for 396 gold-standard references: overall relative recall 92.9%, overall precision 0.13% (range 0.05–0.92%), with hard interface ceilings (no query history, no bulk export, 20-per-page cap) — "not ready as a professional searching tool for tasks where structured retrieval methodology is necessary" **[CONFIRMED as reported measurement]** (https://doi.org/10.1186/1471-2288-13-131). Design implication: LANDSCAPE must fan out (SearXNG multi-engine + structured connectors) and must treat any single-search result set as a lead list, never as coverage.
- **Thesaurus + free-text, then translate across databases.** The replicable method for exhaustive search preparation: draft single-line strategies with field codes/Boolean syntax, combine controlled thesaurus terms with free-text synonyms, optimize by comparing thesaurus-retrieved vs free-text-retrieved sets to surface missing terms, and macro-translate syntax between databases **[LIKELY]** (https://doi.org/10.5195/jmla.2018.283). Design implication: a good LANDSCAPE section shows its search terms and its engine list, so the next session can re-derive rather than trust.
- **Extract, don't snippet-quote; archive when blocked.** The authoritative-search discipline (SearXNG lead → Trafilatura `/extract` full text → Wayback fallback → degraded snippet+UNVERIFIED) is itself the LANDSCAPE procedure: during this study, 7 of 8 extractions succeeded and 1 failed hard (SAGE 502), which is exactly why the M&O mechanism claim stays LIKELY rather than CONFIRMED. A LANDSCAPE that cannot show extraction status per source is research-as-decoration.
- **Per-claim confidence with a CONFLICTED slot.** When authoritative sources disagree (as with verbatim-vs-paraphrase, §2), the correct output is *both sides with no verdict*, not a winner. The LANDSCAPE template needs a CONFLICTED label alongside CONFIRMED/LIKELY/UNVERIFIED.

---

## 7. Anti-Patterns

| # | Anti-pattern | What it looks like | Why the evidence forbids it |
|---|--------------|-------------------|----------------------------|
| 1 | **Log-as-done** | T-entry written, disposition stays OPEN forever; log grows, nothing folds | Masicampo & Baumeister: only a *made plan* quiets intrusion — accumulation without disposition is a rumination engine **[LIKELY]** (https://doi.org/10.1037/a0024192) |
| 2 | **Research-as-decoration** | Decision made, sources cited afterward; synthesis has no falsifier | Consider-the-opposite / confirmation-bias literature: verdict-first synthesis preserves the bias it claims to check **[LIKELY]** (https://doi.org/10.1111/puar.13211) |
| 3 | **Paraphrase-at-capture** | Agent "helpfully" restates the user's words in the log | Violates fidelity at the only stage where fidelity is the product; depth belongs to synthesis (§2). VERBATIM/RECALLED flag exists precisely to catch this (LOCAL-CONFIRMED) |
| 4 | **Edit-the-log** | Fixing old entries in place instead of appending referencing entries | Destroys the pre-RS traceability chain (§5); append-only with referencing correction is the existing local rule for this reason (LOCAL-CONFIRMED) |
| 5 | **Fold-without-backlink** | Idea implemented, T-entry never updated / artifact never cites T-NNN | Post-RS-only traceability: keeps the +86% accuracy benefit's precondition (the link) absent **[LIKELY]** (https://doi.org/10.1109/access.2013.2286822) |
| 6 | **Park-without-trigger** | PARKED with no home and no revisit date/event | OPEN by another name; commitment precondition fails and the if-then plan has no cue **[LIKELY]** (§3) |
| 7 | **Single-search synthesis** | One query, top hits cited, verdict rendered | 92.9%/0.13% recall/precision demonstration: unstructured single-engine retrieval is coverage theatre **[CONFIRMED]** (https://doi.org/10.1186/1471-2288-13-131) |

---

## 8. Confidence Ledger, Gaps & Implementation Notes

**Confidence ledger (every load-bearing claim, one row each):**

| Claim | Label | Basis |
|-------|-------|-------|
| Offloading beats memory-alone, grows with load (η²G = 0.105 / 0.085) | LIKELY | 1 extracted peer-reviewed replication + 1 extracted peer-reviewed converging study (https://doi.org/10.1186/s41235-019-0201-4, https://doi.org/10.1186/s41235-019-0195-y) |
| Capture follows felt uncertainty, not ability | LIKELY | Extracted peer-reviewed (https://doi.org/10.1186/s41235-019-0195-y) + cited Gilbert 2015b lineage within |
| Plan-making quiets unfulfilled-goal intrusion | LIKELY | 1 peer-reviewed record (https://doi.org/10.1037/a0024192) + 1 converging abstract (https://doi.org/10.3389/fpsyg.2022.935775); full texts unextracted |
| Verbatim-transcription → shallower processing (M&O mechanism) | LIKELY | Primary abstract-level (https://journals.sagepub.com/doi/10.1177/0956797614524581); publisher blocked extraction |
| Verbatim effect is conditional/contested | CONFIRMED | 2+ independent peer-reviewed sources disagree on locus/direction (https://doi.org/10.17705/1atrr.00041, https://scholar.colorado.edu/concern/undergraduate_honors_theses/00000047w, vs M&O 2014) |
| If-then plans raise goal attainment, g ≈ 0.31–0.34 | CONFIRMED | 2 independent extracted peer-reviewed meta-analyses (https://doi.org/10.3389/fpsyg.2021.565202, https://doi.org/10.1016/j.drugalcdep.2020.108120 — second abstract-level) |
| G&S 2006 d = 0.65 figure specifically | LIKELY | Convergent secondary citation (https://osf.io/u4znb_v1, https://en.wikipedia.org/wiki/Implementation_intention); primary paywalled (https://linkinghub.elsevier.com/retrieve/pii/S0065260106380021) |
| Experimenter-led (0.465) > document-only (0.277) | CONFIRMED | Measured moderator within extracted meta-analysis (https://doi.org/10.3389/fpsyg.2021.565202) |
| Traceability improves maintenance accuracy; upkeep is main cost | LIKELY | 1 abstract-level trial +86.06% (https://doi.org/10.1109/access.2013.2286822) + 1 preprint mapping, 63 studies (https://arxiv.org/abs/2108.02133) |
| Single-engine search insufficient for synthesis | CONFIRMED | Reported measurement, n = 396 refs (https://doi.org/10.1186/1471-2288-13-131) |
| Exhaustive-search method (thesaurus + free-text + translate) | LIKELY | 1 extracted peer-reviewed methods paper (https://doi.org/10.5195/jmla.2018.283) |
| Consider-the-opposite debiases | LIKELY | 1 abstract-level experiment, N = 1,221 (https://doi.org/10.1111/puar.13211) |
| GTD capture→clarify shape; ADR decision-log shape | UNVERIFIED | Practitioner literature, no URLs claimed, no extraction attempted |
| Local schema/gate observations (THOUGHT_LOG, RULES.md, INBOX.md…) | LOCAL-CONFIRMED | Direct file observation by survey agent, not literature |

**Open gaps (do not encode beyond these):**

1. **M&B 2011 full text** — APA paywall; the "plan quiets intrusion" mechanism rests on record + converging abstract. Re-verify via institutional access before hardening session-close wording around it.
2. **M&O 2014 full text** — SAGE returned 502 via sidecar *and* Jina fallback; mechanism detail (process vs multitasking confound) unverified. Retry extraction or Wayback before citing mechanism specifics.
3. **G&S 2006 primary** — Elsevier closed; d = 0.65 is convergent-secondary only. The g ≈ 0.3 modern range is the number to encode, not 0.65.
4. **IEEE TraceLink full text** — abstract-level only; the +86.06% figure needs full-text verification (task design, variance, transfer to solo-agent setting) before use as a quantified target.
5. **Connector layer degraded** — `measure/connectors.py::search_all` returned only low-value stubs in this environment (keyless APIs unreachable); the "14 connectors" leg ran but contributed nothing. Fan-out claims for LANDSCAPE rest on SearXNG + sidecar only.
6. **No direct literature on idea-disposition state machines** — the LOGGED→…→FOLDED/DROPPED/PARKED machine is a synthesis of GTD (practitioner), pre-RS traceability, and implementation-intention mechanics. No paper validates this exact machine; encode it as *derived design*, not as science.
7. **House-format references did not exist** — neither `docs/research/prompt-engineering-science.md` nor `docs/research/INDEX.md` exists in this repo; format was derived from `HARNESS_BENCHMARKS.md` (numbered sections, magnitude tables, bold findings) instead.

**Implementation notes (for the INBOX.md / RULES.md gateporter — derived, not encoded here):** pick `steps/RULES.md` §Thought-log as the normative home (Scope/Counterexample/Check pattern already there) and `steps/INBOX.md` checkpoint as point-of-use; add the SYNTHESIS template (verdict + confidence + falsifier + backlink); extend the lint with a parked-without-trigger and folded-without-backlink check; encode g ≈ 0.3 (not d = 0.65) as the expected planning-effect prior.

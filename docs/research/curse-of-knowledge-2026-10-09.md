# Curse of Knowledge / Expert–Novice Gap: Synthesis

- **Date:** 2026-10-09
- **Status:** Research (pre-decision — backs existing gates, settles no new gate)
- **Scope:** What curse-of-knowledge, perspective-taking, hindsight-bias, Feynman-technique, and plain-language evidence licenses inside a gated protocol: term origin, Heath/Newton framing, egocentrism mechanisms, plausibility moderation, hindsight taxonomy, Feynman-component mapping, plain-language direction, debiasing survivors.
- **Synthesized from:** `/tmp/curse-knowledge.md` (2026-10-09). No git. No new searches.

## 1. Problem

The protocol assumes a human can state a want and judge a gate outcome, but an informed party systematically overestimates how understandable its own knowledge is to a less-informed party — and cannot reconstruct its own pre-knowledge judgment once the outcome is known. This document records what the sweep licenses: where the curse bites gate design (instructions, defaults, reviews), which debiasing moves survive evidence, and what must not be encoded — without adding any gate.

## 2. Landscape

Eight claim families were swept:

| Family | Verdict | Core mechanism |
|---|---|---|
| Term origin: Camerer, Loewenstein & Weber (1989) | LIKELY | Informed agents cannot ignore private information even with money at stake; paradoxically dampens exploitation of asymmetry |
| Heath & Heath framing + Newton (1990) tappers–listeners | LIKELY (numbers); CONFIRMED (book treats curse) | ~50% predicted vs ~2.5% actual recognition (~20× overestimation); no standardized d located |
| Perspective-taking / egocentrism (Keysar lineage) | LIKELY (mechanism pair); UNVERIFIED (specific year/effect) | Inhibitory-control failure + fluency misattribution (ease of retrieval mistaken for sharedness) |
| Birch & Bloom plausibility moderator | LIKELY | Outcome knowledge biases false-belief reasoning; weaker when planted location implausible; 2014 replication <½ original size |
| Hindsight bias: Fischhoff (1975) → Roese & Vohs (2012) | LIKELY | 3 levels (memory distortion, inevitability, foreseeability); inputs cognitive + metacognitive + motivational; consequences myopic narratives + overconfidence |
| Feynman technique vs evidence | UNVERIFIED (technique as such); LIKELY (component mapping, snippet-level) | No Wikipedia article (popularizer label); components map to Dunlosky practice testing HIGH + elaborative interrogation / self-explanation MODERATE |
| Plain-language effectiveness | LIKELY (direction); no pooled size extracted | Lower health literacy ↔ poorer comprehension/processing, poorer QoL and care experience; institutional response = review services + mandates |
| Correcting the bias | Mixed (see §3.8) | Telling/incentives ineffective (UNVERIFIED single-source); alternative-outcomes consideration helps (UNVERIFIED); epistemological treatment helps in one paradigm (LIKELY); teaching-clarity case (UNVERIFIED) |

No gate-specific curse-of-knowledge RCT was found; all gate mappings below are theory-grounded hypotheses, not empirical gate findings.

## 3. Top Findings

1. **The term is experimental-economics canon with a welfare paradox (LIKELY).** Camerer et al. 1989 (*JPE*) tested whether better-informed agents anticipate less-informed judgments under asymmetry; informed subjects lost money. The curse can raise welfare by dampening exploitation (e.g. underwriting/pricing). Capped at LIKELY: JPE full text not extractable; rests on 1 peer-reviewed secondary (Frontiers 2023 intro) + encyclopedia. [§1]
2. **The 50%-vs-2.5% tapping gap is the canonical intuition pump (LIKELY numbers; CONFIRMED book).** Newton 1990 via Heath & Heath 2007: tappers predicted ~50% recognition, actual ~2.5% (3/120 in common retelling), ~20× overestimation. No primary thesis fetched; general-category search returned junk (documented engine gap). The Heath quote ("forget what it is like not to know") is community-verified. [§2]
3. **Two mechanisms carry the explanatory load (LIKELY).** Failure of inhibitory control (cannot disregard own knowledge) + fluency misattribution (ease of retrieval mistaken for sharedness), via Birch & Bloom 2007 / Ryskin & Brown-Schmidt 2014 / Birch et al. 2017 chain (peer-reviewed, extracted) consistent with encyclopedia. Specific "Keysar et al." year/effect UNVERIFIED; Psychonomic DOI 10.3758/s13421-022-01382-3 is the follow-up lead (references window only). [§3]
4. **Plausibility moderates the curse, but the moderation is smaller than first reported (LIKELY).** Birch & Bloom 2007 violin (Vicki/Denise) + 2014 replication (7 experiments, large N): effect <½ original — "small enough that its impact on real-life perspective-taking may need reevaluation." Peer-reviewed lineage confirmed (Psych Sci 14:283–286; 18:382–386; Cognition 166:447–458); replication detail single-sourced. [§4]
5. **Hindsight has a stable taxonomy and a thin numbers base in-sweep (LIKELY).** Fischhoff 1975 (4-outcome vignettes inflate rated likelihood; cannot reconstruct pre-outcome judgment) + Beyth & Fischhoff Nixon-trip predecessor; Roese & Vohs 2012 three levels with cognitive/metacognitive/motivational inputs. SAGE full text NOT extracted (E1); one preregistered replication datapoint (Brazil, n=431, d=0.12, 95% CI [0.07; 0.37], n.s.) is preprint + snippet-only → UNVERIFIED lead. [§5]
6. **"Feynman technique" is packaging, not a validated construct (UNVERIFIED as such; LIKELY component mapping).** Confirmed redlink (no Wikipedia article). Best reading: retrieval practice + elaboration/self-explanation per Dunlosky et al. 2013 (testing HIGH; interrogation/self-explanation MODERATE) — but the Dunlosky monograph itself was not fetched (snippet-level relatives only), so the mapping stays LIKELY pending Table 1 extraction. [§6]
7. **Plain language direction is supported; magnitudes are not in-sweep (LIKELY direction).** Definitions converge (first-time comprehension; "read, understand, confidently act"); PLOS ONE mixed-studies review (66 papers/60 studies, PROSPERO CRD42020166454) supports lower-literacy → poorer comprehension direction. No Cochrane pooled estimate fetched — explicitly not claimed. [§7]
8. **Most debiasing moves fail; two survive narrowly.** Telling people about the curse / perspective-taking prompts / cash incentives: ineffective (single-source, UNVERIFIED, needs primary). Considering alternative outcomes/explanations: reduces hindsight bias (UNVERIFIED lead). Epistemological treatment: improved disagreement ratings in one political-polarization paradigm (peer-reviewed abstract → LIKELY there only). Teaching clarity (clear instructions, reflective cycle): reduced classroom curse in one Saudi EFL case (snippet-only → UNVERIFIED). [§8]

## 4. Principles

- **P1 — Write every gate card for first contact:** assume the reader does not share the author's context; state goal, human part, and close condition in plain words. Falsifier: first-time readers cannot restate what the gate asks without re-reading.
- **P2 — Show, don't assume, shared vocabulary:** define terms at first use; prefer examples and defaults over abstract labels. Falsifier: term-misuse rate flat after adding definitions.
- **P3 — Pre-register kill/commit criteria before evidence arrives:** hindsight makes outcomes feel inevitable; decide what would change the verdict while still uncertain. Falsifier: post-outcome "we knew it" narratives persist despite criteria.
- **P4 — Debias by alternatives, not by warning:** prompt "what else could explain this" rather than "remember the curse of knowledge." Falsifier: warnings alone move no rating; alternative-prompts do.
- **P5 — Teach-back over tick-box:** retrieval (explain-back in own words) is the Feynman-component with HIGH utility; ticking "understood" is fluency illusion. Falsifier: tick-gated knowledge retains no worse at 7 days than explain-back (then simplify).
- **P6 — Keep plausibility in the model:** perspective-taking failures shrink when the alternative is implausible and grow when it is plausible; calibrate review effort to plausibility, not to confidence. Falsifier: review catches no more misses on plausible-alternative items than on implausible ones.

## 5. Not-to-encode

- **N1 — No new gates from this document** (synthesis only; no gate-RCT meta found).
- **N2 — No Camerer/JPE numbers without primary extraction** (welfare-paradox direction usable; magnitudes withheld).
- **N3 — No Newton 50%/2.5% as a precise constant** (LIKELY retelling; quote as "~50 vs ~2.5, ~20×" with the caveat, not as lab canon).
- **N4 — No Keysar-year/effect-size citations** (mechanism pair usable; specific primary unfetched).
- **N5 — No Birch-plausibility magnitude without the 2014 primary** (direction + "<½" single-sourced; needs fetch before CONFIRMED).
- **N6 — No Roese & Vohs level wording as verbatim canon** (SAGE full text unextracted; paraphrase only).
- **N7 — No hindsight d values** (d=0.12 preprint snippet is a lead, not a license).
- **N8 — No "Feynman technique (HIGH utility)" claims** (packaging, not construct; cite Dunlosky components only after Table 1 extraction).
- **N9 — No plain-language pooled effects** (no Cochrane estimate fetched; direction only).
- **N10 — No "warnings/incentives fix the curse" policy** (single-source ineffective signal; do not build on it either way until primaries fetched).

## 6. Open Gaps

1. Camerer et al. 1989 primary via library/Wayback or `~/papers` drop (needed to lift §1 LIKELY → CONFIRMED).
2. Roese & Vohs 2012 full text + Pohl/Guilbault hindsight meta-analysis for pooled effects.
3. Dunlosky et al. 2013 Table 1 (utility ratings) to ground the Feynman-component claim.
4. Keysar primary (Keysar, Barr, Balin & Brauner 2000; Epley, Keysar, Van Boven & Gilovich 2004) + Birch et al. 2017 for mechanism effect sizes.
5. Cochrane/plain-language pooled estimates for comprehension interventions.
6. Gate-specific curse RCT (instruction-clarity → gate-completion/rework; local baselines instead until then).
7. Local explain-back retention check (7-day; tick-box vs teach-back).
8. Pre-registered kill-criteria field test (does pre-commitment reduce hindsight narratives in gate reviews).
9. Plausibility-calibrated review replication (does effort-by-plausibility beat effort-by-confidence).
10. Retraction/contradiction watch: none encountered in-sweep; re-check on each follow-up fetch.

## 7. Sources

Curse sweep (`/tmp/curse-knowledge.md`, SearXNG `categories=science` + general fallback + Trafilatura `127.0.0.1:8081`, 2026-10-09): `10.1086/261651` (JPE DOI record, extractor E1); Wikipedia Curse of knowledge (full-text: Camerer aim, Newton 1990, Birch & Bloom + 2014 replication, mechanisms, correcting-the-bias); `10.3389/fpsyg.2023.1200627` (Frontiers 2023 intro, Camerer summary + debiasing experiment); DecisionLab curse-of-knowledge (Heath quote, Hogarth coinage note); `10.1111/j.1540-5885.2007.00285.x` (Made to Stick review) + `academic.oup.com/ieam/article/5/2/360/7746252` (book existence); `10.3758/s13421-022-01382-3` (references window: Birch & Bloom 2003/2007, Birch et al. 2017); Wikipedia Hindsight bias (Beyth & Fischhoff Nixon, Fischhoff 1975 method, LaBine, Pezzo); `10.1177/1745691612454303` (Roese & Vohs record, extractor E1); `10.31219/osf.io/x394e_v2/v3` (hindsight replication preprint snippet, d=0.12); `en.wikipedia.org/wiki/Feynman_technique` (confirmed redlink); `10.3389/fpsyg.2019.00707`, `10.3389/fpsyg.2018.01653`, `10.38140/obp1-2024-20` ch. 20 (elaborative-interrogation relatives, snippet-only); Wikipedia Plain language (definitions, guidance); `10.1371/journal.pone.0259815` (PLOS ONE 66-paper review, snippet); `10.1093/heapro/dax015` (n=7 community interventions, snippet lead); `10.5296/gjes.v5i2.15224` (Saudi EFL case, snippet).

## 8. Provenance

- **Method:** synthesis only — no new searches, no git. Confidence labels inherited from the source sweep (CONFIRMED ≥2 independent peer-reviewed; LIKELY = 1 peer-reviewed + 1 community, single meta via abstract, or preprint-capped; UNVERIFIED = single/community/snippet-only/extrapolation; CONFLICTED = disagreeing authorities). DOI pages at snippet level stay capped at LIKELY; JPE and SAGE E1 failures and the Feynman redlink noted in the sweep degrade those claims as stated. General-category SearXNG fallback returned junk throughout — excluded per anti-pattern rule.
- **Limits:** all gate mappings are synthesis (UNVERIFIED as empirical gate claims). Withheld rather than reconstructed: Camerer magnitudes, Newton primary numbers, Keysar effect sizes, Birch replication magnitude, Roese & Vohs verbatim taxonomy, hindsight pooled d, Dunlosky Table 1 values, plain-language pooled effects. Do not quote any of these without new primary extraction.
- **INDEX row:** `| [curse-of-knowledge-2026-10-09.md](curse-of-knowledge-2026-10-09.md) | 2026-10-09 | Curse-of-knowledge / expert–novice gap: Camerer origin, Newton 50-vs-2.5, egocentrism mechanisms, Birch plausibility, hindsight taxonomy, Feynman-component mapping, plain-language direction, 6 principles, 10 not-to-encode (no new gates), 10 gaps. |`

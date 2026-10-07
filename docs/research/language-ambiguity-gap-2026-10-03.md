# LANGUAGE-AMBIGUITY-GAP Research: What the Readings Gate Still Misses (Retro G1 — T-007 Standalone)

- **Date:** 2026-10-03
- **Retro:** G1 — T-007 (+T-006 remainder, T-009 full sanitizer, T-036/T-037/T-041 follow-ups). First standalone saved sweep on language ambiguity; T-007 was BUILT (d31128a) but never saved as a file.
- **Downstream question:** which ambiguities do the readings-gate + context-declaration + anaphora rules still miss?
- **Core pattern:** The current gates enumerate *term readings* — what a word could mean. Nearly every remaining gap is *non-lexical*: scope relations between clear terms, smuggled presuppositions, bridging reference with no antecedent at all, illocutionary force mistaken for content, and vagueness with no enumerable readings. The gate asks "which meaning?"; the gaps are cases where no meaning-choice is on offer.
- **Method:** SearXNG self-hosted (127.0.0.1:8888, categories=science primary, then general), Trafilatura extractor sidecar (127.0.0.1:8081 POST /extract) probes on 3 content targets (2 full-verified, 1 partial-verified), confidence labels per claim.
- **Confidence scheme:** CONFIRMED = replicated/meta-analytic or canonical primary source; LIKELY = single-study or strong convergent theory; UNVERIFIED = plausible/practitioner, needs verification.

## 1. Problem Statement

Four protocol mechanisms already police ambiguity: the readings gate (enumerate live readings, pin domain, confirm, culture-flag, grounding criterion, flat-consensus escalation; d31128a: SAT reanalysis cost, high-context probing, C3MOD 78%-vs-71%), the context-declaration rule (term · domain · definition · prerequisites · contrast · why-here; REVIEW 4.13), the pronoun-antecedent rule (one live antecedent or FAIL; T-037), and the chain/connector extensions (skipped links → prerequisites field; dropped because/so-that/unless → flagged-terms table; T-041). The T-009 conflation handling covers the wrong-sentence-right-topic mode. This sweep is scoped to what those mechanisms *cannot see* — deliberately not re-covering d31128a ground. The research question: which ambiguity classes pass all four mechanisms untouched, and what does the literature say about detecting and handling each?

## 2. Field Landscape

Seven sub-fields surveyed; requirements-engineering and QA-clarification cores strong, scope/vagueness mid-strength from snippets, presupposition/implicature thin (itself a finding — the pragmatic layer most relevant to agent requirements-gathering has the least directly retrievable evidence).

| # | Sub-field | Key anchor | Strength |
|---|-----------|-----------|----------|
| 1 | Ambiguous QA at scale (AmbigQA/AmbigNQ) | Min et al. EMNLP 2020 (extractor-verified abstract) | CONFIRMED (dataset fact) |
| 2 | Selective clarification (CLAM) | Kuhn et al. 2022/2023 (extractor full-verified) | LIKELY (single framework paper) |
| 3 | RE anaphoric ambiguity automation | TOSEM-style industrial study, ~1,350 reqs (snippet) | LIKELY (single study, snippet-only) |
| 4 | RE smell detection (Paska/Rimay) | TSE 2024, 2,725 industrial reqs (abstract-verified) | LIKELY (single industrial study) |
| 5 | Quantifier/negation scope | Feiman & Snedeker lineage; German scrambling/scope; L2 scope (snippets) | LIKELY (convergent multi-study) |
| 6 | Syntactic attachment + lingering misinterpretation | Garden-path/reanalysis tradition (snippets) | LIKELY (canonical tradition, snippet-only here) |
| 7 | Vagueness vs ambiguity; nocuous ambiguity; bridging/deixis/ellipsis | Chantree nocuous (IEEE, paywalled); Vogt/Maggiore vagueness; CODI-CRAC bridging (snippets) | LIKELY–UNVERIFIED (titles/abstracts only) |

Already-covered (d31128a + gates — cited, not re-argued): SAT +250ms reanalysis cost, Hall high-context probing asymmetry, AAE→SAE silent normalization, C3MOD 78%-vs-71% annotation-plus-moderation, Clark & Brennan grounding, stakes test, flat-consensus escalation.

## 3. Top Findings (with confidence + effect sizes)

**F1. Over half of real open-domain questions are ambiguous, in four structural kinds the gate does not enumerate. (CONFIRMED, dataset fact, extractor-verified)**
AmbigQA/AmbigNQ (https://doi.org/10.18653/v1/2020.emnlp-main.466): 14,042 NQ-OPEN questions, *over half* ambiguous — event references 39% (which marriage — informal season 5 vs legal season 7), properties 27% (count including vs excluding the OVA), entity references 23% (Clay Matthews Jr. vs III), answer types 16% (group vs lead singer). None is a "term with 2+ readings": they are *which-event / which-entity / which-counting-rule / which-question* ambiguities. The readings gate fires on flagged *terms*; a user asking "how many episodes in season 2?" has no flagged term and sails through with a silently assumed counting rule. Corollary: the gate needs an *event/entity/counting-rule* trigger alongside its term trigger.

**F2. Models default to answering, not clarifying — selective clarification beats both always-ask and never-ask. (LIKELY, single framework paper, extractor full-verified)**
CLAM (http://arxiv.org/abs/2212.07769v2): "current language models rarely ask users to clarify ambiguous questions and instead provide incorrect answers"; framework = detect ambiguity → generate clarifying question → answer after clarification, with simulated users holding privileged information for evaluation; "significantly improves accuracy on mixed ambiguous and unambiguous questions." Two transfers: (i) the readings gate's stakes test is exactly the right selectivity criterion (clarify iff load-bearing) — CLAM is independent corroboration of *selective* over *universal* clarification; (ii) CLAM's simulated-user method (privileged-information roleplay) is a ready-made test harness for the gate itself — run gate-vs-no-gate with a simulated user who knows the intended reading.

**F3. Scope ambiguity is invisible to term enumeration: every term clear, relation between terms undetermined. (LIKELY, convergent snippets)**
"Every bear approached a tent" — same-tent vs different-tent; each/every/all scope alike-or-not (https://osf.io/jgcxy); inverse scope is hard even for natives with individual variation, mediated by working memory and inhibitory control (https://doi.org/10.1017/langcog.2025.10040); adults *strongly avoid* ambiguous negative-quantified sentences in production and judge them poor alternatives (https://doi.org/10.1207/s15327817la1302_5). The last point is the operational tell: when a user writes "all nodes must not restart" or "every service gets a retry", the form itself signals likely-unintentional ambiguity *because competent producers avoid it*. The gate enumerates readings of "restart"/"retry" but never asks *which scope*. New rule owed: any all/every/each + negation (or two quantifiers) in a requirement triggers a scope pin ("all-not = none-may, or not-all?"), logged like a reading.

**F4. Anaphora beyond pronouns: bridging, demonstrative-to-proposition, and notional agreement have no antecedent to check. (LIKELY, RE study snippet + ACL shared-task existence)**
The T-037 rule ("exactly one live antecedent") assumes the antecedent is *present*. Three violations: (a) *bridging/associative* reference — "deploy the service; the port is already open" (which port? never introduced); CODI-CRAC 2021 ran a dedicated bridging/discourse-deixis shared task (https://aclanthology.org/2021.codi-sharedtask.8) precisely because standard coreference misses it. (b) *Demonstrative-to-proposition* — "this is too risky" refers to a whole preceding plan, not a noun phrase; demonstratives in technical text routinely bind propositions (UNC Cystic Fibrosis corpus study). (c) *Notional anaphora* — "the team … they" (singular form, plural construal; genre- and position-dependent, http://arxiv.org/abs/1804.07375v1). RE automation evidence: best anaphoric-ambiguity detection P≈60%/R=100%, best resolution ≈98% on ~1,350 industrial requirements (https://doi.org/10.1145/3510003.3510157, snippet-only → numbers LIKELY). Corollary: recall-100%/precision-60% is the honest calibration for any automated flag — the gate should *over-flag* anaphora (human discards false positives) and must add a bridging check: every definite ("the X") whose X was never introduced is a flag.

**F5. Vagueness is not ambiguity and the gate's machinery misfires on it. (LIKELY, theoretical convergence, abstract-level)**
Ambiguity offers enumerable readings; vagueness (fast, scalable, robust, done) offers *borderline cases with no readings to list* — enumeration, confirmation-pick, and discarded-readings logging all assume a discrete candidate set that does not exist. Italian/German corpus work finds intentional vagueness *avoids* ambiguous elements (https://benjamins.com/catalog/pbns.347.03vog) — they are different phenomena with different functions, and diachronic work treats them separately (https://benjamins.com/catalog/pbns.347.02mag). Requirements transfer (Chantree: https://ieeexplore.ieee.org/document/1704049, paywalled → claim LIKELY): only *nocuous* ambiguity — ambiguity that actually induces misunderstanding — deserves handling; unnocuous ambiguity is correctly ignored. The stakes test already approximates this, but vagueness needs its own instrument: not "which reading?" but a *threshold probe* ("what would count as fast enough to fail?") — a boundary case, not a candidate list.

**F6. Presuppositions and indirect speech acts bypass term-level checks entirely. (LIKELY theory; UNVERIFIED numbers — thinnest retrieval in this sweep)**
"The retry broke staging again" presupposes a prior retry-incident (existence + recurrence) that the gate never examines — accommodation is silent by default (Romoli & Sauerland handbook chapter, https://www.taylorfrancis.com/chapters/edit/10.4324/9781315668925-21/presupposition-accommodation-jacopo-romoli-uli-sauerland; snippet-only). "Can you check the logs?" is syntactically a question about ability, illocutionarily a request — the gate confirms the reading of "check"/"logs" and misses that the *sentence type* is the ambiguity. Pragmatic-ambiguity RE work now simulates novice/intermediate/expert stakeholders with retrieval-augmented knowledge bases and finds interpretation discrepancies by expertise (https://arxiv.org/abs/2607.04436; GPT-4o-mini recall 0.75/F2 0.75, snippet-only) — the multi-expertise simulation is directly reusable as a gate test: would a novice and an expert read this requirement differently? New rule owed: definite descriptions ("the X" never introduced) and factive/recurrence triggers ("again", "still", "stop", "continue") get a presupposition flag; directive-form questions ("can/could/would you…") get a force check.

**F7. Attachment ambiguity + lingering misinterpretation: confirmation can produce false grounding. (LIKELY, canonical tradition, snippet-only here)**
PP-attachment ("deploy with the new config" — accompaniment vs instrument), garden-path reanalysis costs, and the underspecification literature (https://doi.org/10.3758/mc.36.1.201) converge: comprehenders commit early and *lingering misinterpretations survive reanalysis* (the JML 2013 competing-representations line). The gate's grounding criterion ("both sides state the reading back") is vulnerable exactly here: a user who misparsed the agent's paraphrase confirms a reading neither side holds distinctly. The T-041 connector rule covers dropped *because/so-that/unless*; attachment is the remaining structural case. Owed: paraphrase-back in *different syntactic form* (if the requirement says it with a with-phrase, confirm with a relative clause) so agreement cannot ride on shared misparsing.

**Supporting (LIKELY/UNVERIFIED):**
- S1. Paska/Rimay industrial result — 89% precision and recall on smell detection, 96%P/94%R on pattern recommendations, 2,725 requirements across 13 financial systems (http://arxiv.org/abs/2305.07097, abstract-verified): controlled-language patterns are the proven fix direction for whatever the gate flags — flag-to-pattern, not flag-to-free-text.
- S2. Ellipsis/gapping + deixis ("same for staging", "here/now/you" shifting across runs and agents) unretrieved in depth — listed as G-gap, not finding; the DRT/focusing ellipsis tradition (http://arxiv.org/abs/cmp-lg/9411016v1) marks the trailhead.
- S3. Strategic/intentional ambiguity (user vague on purpose — face-saving, unformed intent, SUE-adjacent): enumerating readings of a deliberately vague term forces premature precision; no direct source retrieved — G-gap with T-009-sanitizer linkage.

## 4. Design Principles for G1 (encode these)

**P1. Enumerate relations, not just readings: every flag names term-readings AND scope AND attachment (F1 + F3 + F7).**
A flagged ambiguity ships with three slots — what each term could mean (existing), what scope each quantifier/negation takes (new: all-not vs not-all pin), what each modifier attaches to (new: paraphrase-back in altered syntactic form). Empty slots must be affirmatively marked empty ("scope: single quantifier, N/A"), never silently skipped — the N/A mark is what makes the check auditable.

**P2. No antecedent in text = flag, not pass: definite-without-introduction and bridging get the same ledger row as overloaded pronouns (F4 + F6).**
Extend the T-037 one-antecedent rule to zero-antecedent definites: any "the X" whose X was never introduced, any recurrence/factive trigger ("again/still/stop/continue"), any demonstrative binding a proposition ("this approach is risky" — which approach, how many sentences back?). Calibrate to the RE number: recall 100% / precision ~60% is the honest operating point — over-flag and let the human discard; a missed bridging reference is a wrong requirement, a false flag is one question.

**P3. Vagueness gets thresholds, not candidate lists; force gets a force check (F5 + F6).**
When no discrete readings exist, the gate switches instruments: threshold probe ("what concrete case would fail this?") instead of enumeration; directive-form questions ("can/could/would you…") confirmed as requests vs questions before any term work begins. Presupposition flags ride the existing flagged-terms table (new trigger column), so no new ledger machinery is owed.

## 5. Not-to-Encode (explicit non-goals)

- **N1. Do not re-encode d31128a ground.** SAT reanalysis cost, high-context probing, dialect normalization, C3MOD moderation, Clark & Brennan grounding, stakes test, flat-consensus escalation stay as built — this sweep adds triggers and instruments around them, never restatements.
- **N2. Do not demand full disambiguation of everything.** Chantree's nocuous/unnocuous line plus the stakes test: unnocuous ambiguity is correctly ignored. A gate that fires on all vagueness is the over-constraint failure the prompt standard warns about.
- **N3. Do not force precision on strategically vague input.** Where the user is vague on purpose (unformed intent, face-saving), enumeration coerces a premature commitment that later reads as a requirement — route to the T-009 sanitizer (clarify vs auto-understand), not to harder enumeration.
- **N4. Do not quote snippet-only numbers as findings.** RE anaphora 60%/100%/98%, Paska 89%/96%/94%, pragmatic-ambiguity 0.75 recall/F2 are single-study priors (LIKELY) — usable for sizing bets, never as design-rationale citations. Rationale cites F1–F7 only.
- **N5. Do not treat confirmation as comprehension.** F7: shared misparsing survives mutual read-back. Grounding requires paraphrase in altered form, not repetition.

## 6. Open Gaps

- **G1.** Numeric effect sizes unrecovered for scope-cost, attachment-cost, and presupposition-accommodation rates (all snippet-only) — retrieve full texts via institutional access before quoting numbers.
- **G2.** Ellipsis/gapping + cross-run deixis ("same for staging"; here/now/you across agents and sessions) unsearched in depth — candidate next sweep; DRT/focusing tradition is the trailhead.
- **G3.** Strategic/intentional vagueness (deliberate, face-saving, SUE-adjacent) — no direct source retrieved; joint sweep with the T-009 full sanitizer (conflated-concepts detection) recommended.
- **G4.** Gate test harness unbuilt: CLAM-style simulated-user evaluation (privileged-information roleplay, gate-vs-no-gate on mixed ambiguous/unambiguous requirements) plus novice-vs-expert discrepancy simulation (F6 method transfer). Proposed metrics: flag recall on seeded ambiguities, false-flag rate, downstream requirement-revision rate.
- **G5.** Presupposition-trigger inventory for protocol English unbuilt (definite-without-introduction, factives, recurrence adverbs, clefts) — compile from linguistics, wire as flagger column on the existing table.
- **G6.** Agent-transfer gap: all F-evidence is human comprehension or RE corpora; agent-to-human requirements dialogue (where the "interviewer" has infinite patience and the user knows it) unmeasured. Candidate: seeded-ambiguity A/B with flag-rate + revision-rate endpoints.

## 7. Sources (search-returned URLs only — no invented links)

1. https://doi.org/10.18653/v1/2020.emnlp-main.466 — AmbigQA: >50% of NQ-OPEN ambiguous; event 39% / property 27% / entity 23% / answer-type 16% (F1; extractor-verified abstract).
2. http://arxiv.org/abs/2212.07769v2 — CLAM: LMs rarely clarify; selective clarification improves mixed accuracy (F2; extractor full-verified).
3. https://osf.io/jgcxy — Each/every/all scope priming, Dutch replication (F3; snippet).
4. https://doi.org/10.1017/langcog.2025.10040 — L2 scope: inverse scope hard, WM/IC mediation (F3; snippet).
5. https://doi.org/10.1207/s15327817la1302_5 — Adults avoid ambiguous negative-quantified sentences in production (F3; snippet).
6. https://doi.org/10.5070/g6011.6627 — Scrambling vs quantifier-scope shared-source test (F3; snippet).
7. https://doi.org/10.1145/3510003.3510157 — RE anaphoric ambiguity: detection P≈60%/R=100%, resolution ≈98%, ~1,350 reqs (F4; snippet).
8. https://aclanthology.org/2021.codi-sharedtask.8 — Bridging + discourse-deixis shared task (F4; snippet).
9. https://cdr.lib.unc.edu/record/uuid:9627ec0b-580d-47ac-806f-be2d14929a9a — Demonstrative-to-proposition anaphora (F4; snippet).
10. http://arxiv.org/abs/1804.07375v1 — Notional anaphora prediction, OntoNotes (F4; snippet).
11. https://benjamins.com/catalog/pbns.347.03vog — Intentional vagueness avoids ambiguous elements (F5; snippet).
12. https://benjamins.com/catalog/pbns.347.02mag — Ambiguity vs vagueness in language change (F5; snippet).
13. https://ieeexplore.ieee.org/document/1704049 — Nocuous ambiguities in NL requirements (F5; title/paywalled, LIKELY).
14. http://link.springer.com/10.1007/s11168-008-9058-2 — Automatic identification of nocuous ambiguity (F5; snippet).
15. https://www.taylorfrancis.com/chapters/edit/10.4324/9781315668925-21/presupposition-accommodation-jacopo-romoli-uli-sauerland — Presupposition & accommodation handbook (F6; snippet).
16. https://arxiv.org/abs/2607.04436 — RAG pragmatic-ambiguity detection, recall 0.75/F2 0.75 (F6; snippet).
17. https://doi.org/10.3758/mc.36.1.201 — Underspecification of syntactic ambiguities (F7; snippet).
18. http://arxiv.org/abs/2305.07097 — Paska: 89% P/R smell detection, 96%P/94%R recommendations, 2,725 reqs (S1; abstract-verified).
19. http://arxiv.org/abs/cmp-lg/9411016v1 — DRT + focusing for anaphora/ellipsis (S2; snippet).
20. Internal: steps/EXTRACTION.md readings gate + context declaration; steps/INBOX.md flagged-terms table; steps/REVIEW.md 4.13; commits d31128a (T-007 deltas), 1d8bccd (gate), f4cee56 (stakes test), 96cf053 + 3805ac3 (T-036/T-037/T-041).

## 8. Provenance & Next Step

- **Engines:** SearXNG @127.0.0.1:8888 (categories=science primary: lexical/scope/anaphora/vagueness/RE-ambiguity; general: presupposition/implicature/strategic-ambiguity queries mostly empty → thin F6/S3 marked accordingly). No Tavily fallback needed (science returned throughout). No URLs invented — §7 lists search-returned URLs only.
- **Extraction (127.0.0.1:8081 POST /extract):** 4 probes — full-verified: CLAM arXiv abstract (F2), AmbigQA ACL bib+abstract (F1); partial-verified: Paska arXiv abstract (S1, cut at length limit); endpoint-format probe confirmed `{"url"}` schema. Remainder snippet-only, labeled accordingly.
- **INDEX.md:** untouched per instructions (proposed row text below for the owner to insert; no git operations performed).
- **Next:** T-007 fold — encode P1–P3 (scope pin + attachment paraphrase-back; zero-antecedent flags; threshold-probe instrument) with N1–N5 guards; build the G4 simulated-user harness before hardening trigger wording; close G1/G5 before quoting numeric effect sizes.

> **Proposed INDEX row (exact text, do not insert without owner go):**
> `| language-ambiguity-gap-2026-10-03.md | 2026-10-03 | T-007 standalone ambiguity sweep: non-lexical gaps the readings gate misses — scope, bridging anaphora, vagueness-vs-ambiguity, presupposition, attachment; AmbigQA >50% CONFIRMED, selective clarification LIKELY, 3 principles + guards. |`

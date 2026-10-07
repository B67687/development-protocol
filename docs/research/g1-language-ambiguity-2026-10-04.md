# G1 Language-Ambiguity Sweep — Fold Brief for T-007 / T-006 Remainder / T-009 / T-036 / T-037 / T-041 (2026-10-04)

- **Date:** 2026-10-04
- **Retro:** G1 — T-007 (+T-006 remainder, T-009 full sanitizer, T-036/T-037/T-041 follow-ups). Standalone saved sweep; complements `language-ambiguity-gap-2026-10-03.md` (the 10-03 evidence sweep, F1–F7) with a fold-oriented brief: what each gap owes which thought's follow-up.
- **Downstream question:** which ambiguities do the readings-gate + declaration + anaphora rules still miss, and which thought owns each fix?
- **Core pattern:** The 10-03 sweep stands — remaining gaps are *non-lexical* (scope, bridging, vagueness, presupposition, attachment). This brief re-verifies the three load-bearing anchors through the extractor sidecar today, adds deltas retrieved since (clarification-question datasets, scope-priming replications, garden-path LLM alignment, discourse-deixis survey), and routes each gap to its owning follow-up so T-007/T-006/T-009/T-036/T-037/T-041 work never re-sweeps shared ground.
- **Method:** SearXNG self-hosted (`127.0.0.1:8888`, `categories=science` primary, one `general` pass for presupposition/attachment), Trafilatura extractor sidecar (`127.0.0.1:8081` POST `/extract`) — 4 probes, 3 full-verified, 1 failed hard (see §8).
- **Confidence scheme:** CONFIRMED = replicated/meta-analytic or canonical primary source; LIKELY = single-study or strong convergent theory; UNVERIFIED = plausible/practitioner, needs verification. Snippet-only numbers stay LIKELY max and never enter design rationale (§5 N4).

---

## 1. Problem Statement

Four protocol mechanisms police ambiguity today: the readings gate (enumerate live readings, pin domain, confirm, grounding criterion, flat-consensus escalation), the context-declaration rule (term · domain · definition · prerequisites · contrast · why-here; T-036), the pronoun-antecedent rule (one live antecedent or FAIL; T-037), and the chain/connector extensions (T-041) plus the T-009 conflation checks. The T-006 remainder asks where the *meaning gate* lives for non-term ambiguity; T-009 asks for the full intent sanitizer behind the current conflation checks; T-036/T-037/T-041 each have a follow-up edge (declaration completeness, zero-antecedent reference, structural attachment). This brief answers: which ambiguity classes pass all four mechanisms untouched, with what evidence, and which thought's follow-up owns the fix — so the six downstream folds can proceed in parallel without re-researching each other.

## 2. Field Landscape

Seven sub-fields, same partition as the 10-03 sweep; strength column records *today's* verification status, deltas marked ★.

| # | Sub-field | Key anchor | Strength |
|---|-----------|-----------|----------|
| 1 | Ambiguous QA at scale | AmbigQA/AmbigNQ, 14,042 NQ-OPEN questions (extractor full-verified today) | CONFIRMED (dataset fact) |
| 2 | Selective clarification ★ | CLAM (extractor full-verified today) + CAMBIGNQ 5,654 clarification-question dataset + CLARINET retrieval-gain result | LIKELY (convergent frameworks, single-group each) |
| 3 | Scope ambiguity ★ | Feiman & Snedeker verb-priming lineage + Dutch replication (verb-specific priming) + heritage-scope simplification + German scrambling/scope | LIKELY (convergent multi-study, snippet-level today) |
| 4 | Zero-antecedent anaphora ★ | CODI-CRAC bridging/deixis task + COLI discourse-deixis survey (probe failed → snippet-only) + notional-anaphora OntoNotes model | LIKELY–UNVERIFIED (existence CONFIRMED, numbers snippet) |
| 5 | Vagueness vs ambiguity; nocuous line | Chantree nocuous (paywalled) + nocuous-anaphora extension + underspecification hypothesis (cmp-lg/9505034) | LIKELY (theory convergence) |
| 6 | Presupposition & force | Romoli & Sauerland handbook (snippet) + pragmatic-ambiguity RAG study (recall 0.75/F2 0.75, snippet) | LIKELY theory / UNVERIFIED numbers |
| 7 | Attachment + lingering misparse ★ | Garden-path reanalysis tradition + LLM garden-path alignment study (arXiv 2405.16042) + Paska industrial fix direction (extractor full-verified today: 89%/89%, 96%/94%) | LIKELY (canonical + convergent) |

Already-covered ground (cited, not re-argued): d31128a deltas (SAT reanalysis cost, high-context probing, C3MOD 78%-vs-71%), Clark & Brennan grounding, stakes test, flat-consensus escalation, and the full 10-03 F1–F7 evidence — this file adds verification + deltas + fold routing only.

## 3. Top Findings (with confidence + effect sizes)

**F1. Over half of real open-domain questions are ambiguous in four structural kinds the gate does not enumerate. (CONFIRMED, extractor full-verified today)**
AmbigQA/AmbigNQ (https://doi.org/10.18653/v1/2020.emnlp-main.466): 14,042 NQ-OPEN questions, *over half* ambiguous — event references 39%, properties 27%, entity references 23%, answer types 16%. None is a "term with 2+ readings": they are *which-event / which-entity / which-counting-rule / which-question* ambiguities. A user asking "how many episodes in season 2?" carries no flagged term and sails through with a silently assumed counting rule. *Fold owner: T-007* — the gate needs an event/entity/counting-rule trigger alongside its term trigger.

**F2. Models default to answering, not clarifying — selective clarification beats both always-ask and never-ask. (LIKELY, extractor full-verified today)**
CLAM (http://arxiv.org/abs/2212.07769v2): "current language models rarely ask users to clarify ambiguous questions and instead provide incorrect answers"; pipeline = detect ambiguity → generate clarifying question → answer after clarification, with simulated users holding privileged information; "significantly improves accuracy on mixed ambiguous and unambiguous questions." ★ Deltas deepen the corroboration: CAMBIGNQ adds 5,654 ambiguous questions with passages, answers, and clarification questions — baselines 61.3 F1 ambiguity detection, 40.5 F1 clarification-based QA (http://arxiv.org/abs/2305.13808v2, snippet → numbers LIKELY max); CLARINET (retrieval setting) beats information-gain heuristics by 17% and vanilla-prompted LLMs by 39% relative (http://arxiv.org/abs/2405.15784v1, snippet); Tree of Clarifications recursively disambiguates via few-shot + external knowledge and tops ASQA few-shot on all metrics (https://doi.org/10.18653/v1/2023.emnlp-main.63, snippet). Two transfers stand: (i) the stakes test is exactly the right selectivity criterion — independent corroboration of *selective* over *universal* clarification; (ii) CLAM's simulated-user method is the ready-made gate test harness (gate-vs-no-gate, privileged-information roleplay). *Fold owner: T-006 remainder* — selectivity criterion + harness live here.

**F3. Scope ambiguity is invisible to term enumeration — and priming evidence now says representations are verb-specific, not abstract. (LIKELY, convergent snippets)**
"Every hiker climbed a hill" — same-hill vs different-hill; competent producers *avoid* ambiguous negative-quantified forms, so the form itself signals likely-unintentional ambiguity (https://doi.org/10.1207/s15327817la1302_5). ★ Deltas: a Dutch close replication of Feiman & Snedeker finds *no* verb-independent scope priming — priming is stronger with same-verb prime/target pairs, i.e. logical representations integrate compositional structure with meaning features (https://escholarship.org/uc/item/5bn3t780, snippet); heritage Mandarin/English speakers lack inverse scope in *both* languages — pressure to simplify the scope grammar when two systems meet (https://doi.org/10.5334/gjgl.198, snippet); German scrambling/inverse-scope self-paced-reading replicates processing costs for both with no shared-source connection at the individual level (https://doi.org/10.5070/g6011.6627, snippet). Design consequence: scope pins cannot be generic ("check quantifiers") — they must be *verb-anchored* ("who did what to whom, same-X or each-own-X?"). *Fold owner: T-007* (scope-pin rule) with the verb-anchored wording as the new detail.

**F4. Anaphora beyond pronouns: bridging, demonstrative-to-proposition, and notional agreement have no antecedent to check. (LIKELY; survey-existence CONFIRMED, numbers snippet)**
The T-037 rule ("exactly one live antecedent") assumes the antecedent is *present*. Three violations: (a) *bridging* — "deploy the service; the port is already open" (never introduced); (b) *discourse deixis* — "this is too risky" binds a whole plan, not a noun phrase; ★ the COLI survey on non-nominal-antecedent anaphora confirms this as a recognized open area (events/facts/propositions as antecedents; annotation + resolution surveyed — https://doi.org/10.1162/coli_a_00327, extraction failed hard, snippet-only → UNVERIFIED details, existence LIKELY); (c) *notional anaphora* — "the team … they", genre- and position-dependent (http://arxiv.org/abs/1804.07375v1, snippet). RE automation prior (snippet-only → LIKELY): detection P≈60%/R=100%, resolution ≈98% on ~1,350 industrial requirements (https://doi.org/10.1145/3510003.3510157). Honest calibration for any automated flag: recall 100% / precision ~60% — over-flag, human discards. *Fold owner: T-037 follow-up* — extend the one-antecedent rule to zero-antecedent definites (every "the X" never introduced is a flag).

**F5. Vagueness is not ambiguity; only nocuous ambiguity deserves handling. (LIKELY, theoretical convergence)**
Ambiguity offers enumerable readings; vagueness (*fast, scalable, robust, done*) offers borderline cases with no readings to list — enumeration, confirmation-pick, and discarded-readings logging all assume a discrete candidate set that does not exist. ★ Deltas: the underspecification hypothesis (http://arxiv.org/abs/cmp-lg/9505034v1, snippet) gives the formal frame — one representation denoting a *set of senses* with defeasible reasoning to resolve; the nocuous-ambiguity programme extends from coordination to anaphora (http://ieeexplore.ieee.org/document/5636921/, snippet) and offers automatic nocuous/anodyne classification (http://link.springer.com/10.1007/s11168-008-9058-2, snippet). The stakes test already approximates the nocuous line; vagueness needs its own instrument — a *threshold probe* ("what concrete case would fail this?"), not a candidate list. *Fold owner: T-036 follow-up* — declaration completeness for gradable adjectives is a threshold, not a definition.

**F6. Presuppositions and indirect speech acts bypass term-level checks entirely. (LIKELY theory; UNVERIFIED numbers — thinnest retrieval, itself a finding)**
"The retry broke staging again" presupposes a prior retry-incident never examined — accommodation is silent by default (https://www.taylorfrancis.com/chapters/edit/10.4324/9781315668925-21/presupposition-accommodation-jacopo-romoli-uli-sauerland, snippet). "Can you check the logs?" is syntactically a question, illocutionarily a request — the gate confirms readings of "check"/"logs" and misses that the *sentence type* is the ambiguity. New rule owed: definite-without-introduction + factive/recurrence triggers ("again", "still", "stop", "continue") get a presupposition flag; directive-form questions ("can/could/would you…") get a force check. Reusable gate test: would a novice and an expert read this requirement differently (method transfer from https://arxiv.org/abs/2607.04436, snippet: GPT-4o-mini recall 0.75/F2 0.75)? *Fold owner: T-041 follow-up* (structural triggers ride the existing flagged-terms table, new trigger column) — presupposition flags are structural like connectors.

**F7. Attachment ambiguity + lingering misinterpretation: confirmation can produce false grounding. (LIKELY, canonical tradition + ★ LLM-alignment delta)**
PP-attachment ("deploy with the new config" — accompaniment vs instrument) plus the finding that misinterpretations *linger past disambiguation*. ★ Delta: four LLMs (GPT-2, LLaMA-2, Flan-T5, RoBERTa) on 24 garden-path sentences show human–LLM alignment in processing and lingering misinterpretation, especially with extra-syntactic cues (http://arxiv.org/abs/2405.16042v1, snippet) — the gate's paraphrase-back step inherits the same vulnerability when both sides share the misparse. Owed: paraphrase-back in *altered syntactic form* (with-phrase confirmed via relative clause) so agreement cannot ride on shared misparsing. ★ Fix direction is industrially proven: Paska detects requirement smells at 89% precision *and* recall and recommends Rimay controlled-pattern rewrites at 96%P/94%R over 2,725 requirements across 13 financial systems (http://arxiv.org/abs/2305.07097v2 — extractor full-verified today) — flag-to-pattern, not flag-to-free-text. *Fold owner: T-009 full sanitizer* — attachment + lingering-misparse handling belongs to the sanitizer's clarify-vs-auto-understand decision, with Paska's pattern-recommendation shape as the proven template.

**Supporting (LIKELY/UNVERIFIED):**
- S1. Strategic/intentional vagueness (user vague on purpose — face-saving, unformed intent): enumerating readings forces premature precision; no direct source retrieved — joint work with the T-009 sanitizer, not with enumeration. (G-gap, UNVERIFIED)
- S2. Ellipsis/gapping + cross-run deixis ("same for staging"; here/now/you across agents and sessions): DRT/focusing tradition marks the trailhead (http://arxiv.org/abs/cmp-lg/9411016v1, snippet) — G-gap, not finding. (UNVERIFIED)

## 4. Design Principles for G1 (encode these; fold owners in brackets)

**P1. Enumerate relations, not just readings. [T-007]**
Every flagged ambiguity ships with three slots — term readings (existing), quantifier/negation scope (new, verb-anchored pin: same-X or each-own-X), modifier attachment (new, paraphrase-back in altered form). Empty slots are affirmatively marked N/A, never silently skipped — the N/A mark is what makes the check auditable.

**P2. No antecedent in text = flag, not pass. [T-037 follow-up]**
Extend the one-antecedent rule to zero-antecedent definites: any "the X" never introduced, any recurrence/factive trigger, any proposition-binding demonstrative gets the same ledger row as an overloaded pronoun. Operate at recall-100%/precision-~60% honesty: over-flag, human discards.

**P3. Vagueness gets thresholds, force gets a force check. [T-036 + T-041 follow-ups]**
No discrete readings → switch instruments: threshold probe for gradable adjectives; presupposition-flag column + directive-force check for sentence-level smuggling. Both ride existing machinery (declaration rule, flagged-terms table) — no new ledger owed.

**P4. The sanitizer decides clarify-vs-absorb; patterns, not free text, are the fix. [T-009 + T-006 remainder]**
Attachment/lingering-misparse and strategic-vagueness cases route to the T-009 sanitizer (clarify vs auto-understand), whose output shape follows Paska's proven pattern-recommendation template; the T-006 remainder owns the selectivity criterion (stakes test, CLAM-corroborated) and the simulated-user harness.

## 5. Not-to-Encode (explicit non-goals)

- **N1. Do not re-encode d31128a or 10-03 F1–F7 ground.** SAT cost, high-context probing, dialect normalization, C3MOD moderation, grounding, stakes test, escalation stay as built; this brief adds triggers, instruments, and fold routing only.
- **N2. Do not demand full disambiguation.** The nocuous/unnocuous line plus the stakes test: unnocuous ambiguity is correctly ignored. A gate firing on all vagueness is the over-constraint failure the prompt standard warns about.
- **N3. Do not force precision on strategically vague input.** Deliberate vagueness routes to the T-009 sanitizer, never to harder enumeration — enumeration coerces premature commitments that later read as requirements.
- **N4. Do not quote snippet-only numbers as findings.** CAMBIGNQ 61.3/40.5, CLARINET +17%/+39%, RE anaphora 60%/100%/98%, pragmatic-ambiguity 0.75 — single-study priors (LIKELY max), usable for sizing bets, never as design-rationale citations. Rationale cites F1–F7 only.
- **N5. Do not treat confirmation as comprehension.** Shared misparsing survives mutual read-back (F7). Grounding requires paraphrase in altered form, not repetition.

## 6. Open Gaps

- **G1.** Numeric effect sizes unrecovered for scope-cost, attachment-cost, presupposition-accommodation rates (all snippet-only) — institutional-access retrieval before quoting numbers.
- **G2.** Ellipsis/gapping + cross-run deixis unsearched in depth — candidate next sweep; DRT/focusing trailhead recorded (S2).
- **G3.** Strategic/intentional vagueness — no direct source; joint sweep with the T-009 full sanitizer recommended (S1).
- **G4.** Gate test harness unbuilt: CLAM-style simulated-user evaluation (gate-vs-no-gate on mixed requirements) + novice-vs-expert discrepancy simulation. Proposed metrics: flag recall on seeded ambiguities, false-flag rate, downstream requirement-revision rate. [T-006 remainder]
- **G5.** Presupposition-trigger inventory for protocol English unbuilt (definites-without-introduction, factives, recurrence adverbs, clefts) — compile from linguistics, wire as flagger column. [T-041 follow-up]
- **G6.** Agent-transfer gap: all F-evidence is human comprehension or RE corpora; agent-to-human requirements dialogue (interviewer with infinite patience) unmeasured. Candidate: seeded-ambiguity A/B with flag-rate + revision-rate endpoints.

## 7. Sources (search-returned URLs only — no invented links)

1. https://doi.org/10.18653/v1/2020.emnlp-main.466 — AmbigQA: >50% NQ-OPEN ambiguous; event 39% / property 27% / entity 23% / answer-type 16% (F1; extractor full-verified 2026-10-04).
2. http://arxiv.org/abs/2212.07769v2 — CLAM: LMs rarely clarify; selective clarification improves mixed accuracy (F2; extractor full-verified 2026-10-04).
3. http://arxiv.org/abs/2305.07097v2 — Paska: 89% P/R smell detection, 96%P/94%R recommendations, 2,725 reqs / 13 systems (F7 fix direction; extractor full-verified 2026-10-04).
4. http://arxiv.org/abs/2305.13808v2 — CAMBIGNQ: 5,654 ambiguous questions + clarification questions; 61.3 F1 detection / 40.5 F1 clarification-QA baselines (F2 delta; snippet).
5. http://arxiv.org/abs/2405.15784v1 — CLARINET: +17% over heuristics, +39% relative over vanilla LLMs (F2 delta; snippet).
6. https://doi.org/10.18653/v1/2023.emnlp-main.63 — Tree of Clarifications: recursive disambiguation, ASQA few-shot best (F2 delta; snippet).
7. https://escholarship.org/uc/item/5bn3t780 — Dutch scope-priming replication: verb-specific, not abstract (F3 delta; snippet).
8. https://doi.org/10.5334/gjgl.198 — Heritage scope simplification: no inverse scope in either language (F3 delta; snippet).
9. https://doi.org/10.5070/g6011.6627 — Scrambling vs quantifier-scope shared-source test, no individual-level link (F3; snippet).
10. https://doi.org/10.1207/s15327817la1302_5 — Producers avoid ambiguous negative-quantified forms (F3; snippet).
11. https://doi.org/10.1162/coli_a_00327 — Discourse-deixis / non-nominal-anaphora survey (F4; extraction failed, snippet-only).
12. http://arxiv.org/abs/1804.07375v1 — Notional anaphora prediction, OntoNotes (F4; snippet).
13. https://doi.org/10.1145/3510003.3510157 — RE anaphoric ambiguity: P≈60%/R=100%, resolution ≈98% (F4; snippet).
14. http://arxiv.org/abs/cmp-lg/9505034v1 — Underspecification hypothesis: sets of senses + defeasible reasoning (F5; snippet).
15. https://ieeexplore.ieee.org/document/1704049 — Nocuous ambiguities in requirements (F5; paywalled, LIKELY).
16. http://link.springer.com/10.1007/s11168-008-9058-2 — Automatic nocuous-ambiguity identification (F5; snippet).
17. http://ieeexplore.ieee.org/document/5636921/ — Nocuous-ambiguity analysis extended to anaphora (F5; snippet).
18. https://www.taylorfrancis.com/chapters/edit/10.4324/9781315668925-21/presupposition-accommodation-jacopo-romoli-uli-sauerland — Presupposition & accommodation handbook (F6; snippet).
19. https://arxiv.org/abs/2607.04436 — RAG pragmatic-ambiguity detection, recall 0.75/F2 0.75 (F6; snippet).
20. http://arxiv.org/abs/2405.16042v1 — Garden-path LLM alignment: lingering misinterpretation in 4 LLMs (F7 delta; snippet).
21. http://arxiv.org/abs/cmp-lg/9411016v1 — DRT + focusing for anaphora/ellipsis (S2; snippet).
22. Internal: `language-ambiguity-gap-2026-10-03.md` (10-03 evidence sweep, F1–F7 — prior art, not re-argued); steps readings gate + context declaration; INBOX flagged-terms table; REVIEW 4.13; commits d31128a, 1d8bccd, f4cee56, 96cf053, 3805ac3.

## 8. Provenance & Next Step

- **Engines:** SearXNG @127.0.0.1:8888 (`categories=science` primary across 4 queries; `general` 1 query — presupposition/attachment returned one handbook hit, marked thin accordingly). No Tavily fallback needed (science returned throughout). No URLs invented — §7 lists search-returned URLs only.
- **Extraction (127.0.0.1:8081 POST /extract):** 4 probes — full-verified: CLAM arXiv abstract (F2), AmbigQA ACL bib+abstract (F1), Paska arXiv abstract (F7 fix direction); failed hard: MIT Press COLI survey DOI (empty response even with fallback → F4 survey details stay snippet-only/UNVERIFIED).
- **INDEX.md:** untouched per instructions (proposed row text below; no git operations performed).
- **Next:** six parallel folds per §4 owners — T-007 (P1 scope pin, verb-anchored), T-006 remainder (P4 selectivity + G4 harness), T-009 (P4 sanitizer with Paska template + F7 attachment), T-036 follow-up (P3 thresholds), T-037 follow-up (P2 zero-antecedent flags), T-041 follow-up (P3 presupposition column + G5 inventory). N1–N5 guard all six. Close G1/G5 before quoting numeric effect sizes in gate wording.

> **Proposed INDEX row (exact text, do not insert without owner go):**
> `| g1-language-ambiguity-2026-10-04.md | 2026-10-04 | G1 fold brief for T-007/T-006 remainder/T-009/T-036/T-037/T-041: re-verified AmbigQA/CLAM/Paska anchors, clarification-dataset + scope-priming + garden-path-LLM deltas, per-gap fold owners (P1–P4) with N1–N5 guards. |`

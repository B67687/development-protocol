# Changelog

All notable changes to the Development Protocol. Follows Keep a Changelog; versions are squash markers on the public mirror.

## [Unreleased]

### Added

- Raw-first rule: attend to the user's words with no framework before the protocol engages (`steps/INBOX.md`, always-on `dev-defaults` gate). The protocol serves thinking, not replaces it.
- User-model layer: 7-dimension Stance Read in `INBOX.md` (expertise, assertiveness, attachment, patience, reflectiveness, stopping, gravity) with vibe mode + one tripwire.
- Dissent Log in `SERIOUSNESS.md` (D-IDs, Right/Wrong-call tracking, recalibration every 10 overrides); ratification carries the dissent line in `STRATEGY.md`.
- Scope ceiling: ratified proposals state proposal + explicit exclusions + less-wins-ties; Exclusion Log in `KILL_LOG.md`; scope-fidelity check in `REVIEW.md` (unlisted scope = FAIL unless new cycle).
- `docs/standards/README.md` and `docs/adr/README.md` indexes; `docs/archive/` for superseded material.
- Stance table grounded in literature: per-dimension observable signals + 6-source cite line (novice reliance, sycophancy/assertiveness, forcing/NfC, face-preservation); Attachment + Stopping marked practitioner heuristics.
- Truth-elicitation in `EXTRACTION.md` bouncing: narrative-before-checking, contradiction-as-curiosity with explanation-request (SUE-C/E), face-saving framing, concrete-over-abstract, open verification (Bond & DePaulo 54%; SUE meta-analysis; 2026 SDB review; Brenner & DeLamater; Luke et al.). Serious pass adds: open narration first (cognitive interview: Fisher & Geiselman), checkable-details probe (verifiability approach: Nahari, Vrij & Fisher), one unanticipated angle (cognitive-load tradition), no-pressure boundary (false-confession literature). Trigger (e) for guarded answers.
- Cross-cycle layer: REFLECT Q9 next-cycle seed (ambition delta + carry-forward + second-system caution per Brooks — max 3 deferred items forward, one scale-step up per cycle) feeds the next INBOX as first capture item; upfront-extraction-default doctrine in `EXTRACTION.md`; X-002 excludes numbered cycle tracking.
- Stance table re-verified against all later passes (item #6, last): Assertiveness → EXTRACTION elicitation cross-ref; Attachment re-grounded in face-saving lit (no longer heuristic); Reflectiveness gains no-early-template rule (Einstellung); Stopping gains ambition cap → REFLECT Q9 (Brooks); only Patience remains heuristic.
- FEATURES registry F-009..F-016: every recent layer registered with Origin block (verbatim user query + research basis + commits) — stance read, dissent log, scope ceiling, truth-elicitation, cross-cycle, raw-first, SWE appendixes, OSS metric (proposed). Lint Rule 8 enforces registry hygiene (valid State + File per entry); 8/8 PASS.
- Learning anti-abdication (F-017): abdication tripwire in SERIOUSNESS (3+ passive cycles → pointed retrieval question before COMMIT); standing-decisions display in STRATEGY ratification + every EXECUTOR output; learned-this-cycle in REFLECT Q9; explain-back anti-gaming at ratification (echo fails, ledger-checked). Research: testing effect (Roediger & Karpicke 2006), self-explanation effect (Chi 1989), generation effect (Wikipedia-verified). Sweep: metacognitive decoupling under LLM use (arXiv:2603.29681, fires on behavior not self-report); expertise-reversal guidance calibration in INBOX Knowledge cell (CLT); productive-failure grounding for attempt-before-structure in EXTRACTION (Kapur).

### Changed

- 20 protocol docs moved root → `steps/` (history preserved via `git mv`); all cross-refs re-pointed. `AGENTS.md` stays at root (tool auto-loading).
- `.gitignore` back to standard track-by-default; private material lives in the `-Local` sibling folder.
- README: plain-language hero, 4-checks intro, Archify pipeline diagram (SVG + interactive HTML), ithmb-style header with badges, `CREDITS.md` aligned to ithmb format.
- Scope ceiling grounded in evidence: feature/scope-creep overrun mechanics (Wikipedia), maintenance-share + cost-of-change + Gall's law; the '64% unused features' figure cited with its true scope (4 internal apps, XP 2002) instead of as universal fact.
- Raw-pass grounded: diverge-before-converge (Guilford), Einstellung mechanized-set (Luchins), design fixation on first-shown examples (Jansson & Smith) — the protocol's own template is such an example; in-house trajectory anchor agrees.
- Open-source metric: cold-start stranger-test now FAIL (was UNPROVEN) — the :8081 extractor surface and measure harness are undocumented in README, no curl examples, 3 meilisearch files of unclear status, 7 unexplained results blobs. Fix = extractor API docs + examples + dedupe (queued, not built).
- Research redo with fixed engine (science routing live): 3 surgical deltas — STRATEGY gains evaluability-bias + fairness-effect counter (Lee, Keil & Park 2019, JAIS 20(12): make alternatives evaluable, flag reward-coupled advocacy); EXTRACTION gains rule (10) adapt-don't-script (O&W 2020 small-study caution + practitioner adaptive-use); INBOX Knowledge cell gains trust-calibration (literacy doesn't fix over-reliance — arXiv:2604.01114v3; expert under-reliance via metacognition — SSRN N=2,799).

### Removed

- `docs/SPEC_SYNC.md` stub (content long merged into `REVIEW.md` fidelity check).

### Fixed

- 20+ broken internal links: new `LICENSE` (MIT, ithmb text); `docs/PHILOSOPHY.md` repoints → `steps/QUICKSTART.md`; new `docs/STANDING_PRINCIPLES.md`; appendix/trace/test-signal/AGENTS/standards/traces path prefixes; removed `ledger-check.py` tree line (file never existed). Sweep-verified; scaffold-target, archive, `.omo`, and cross-repo refs intentionally untouched.

## [2026-09-09] — Public mirror synced

- Thematic squash to public: SWE appendixes (P2b early + P4 late), agnostic search backend, PoP altitude gates, lived traces, README overhaul. Origin and public trees identical.

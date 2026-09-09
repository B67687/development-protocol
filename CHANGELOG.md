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
- Truth-elicitation in `EXTRACTION.md` bouncing: narrative-before-checking, contradiction-as-curiosity with explanation-request (SUE-C/E), face-saving framing, concrete-over-abstract, open verification (Bond & DePaulo 54%; SUE meta-analysis; 2026 SDB review; Brenner & DeLamater; Luke et al.). Trigger (e) for guarded answers.
- Cross-cycle layer: REFLECT Q9 next-cycle seed (ambition delta + carry-forward) feeds the next INBOX as first capture item; upfront-extraction-default doctrine in `EXTRACTION.md` (extract early by default, seeding covers the rest); X-002 excludes numbered cycle tracking.

### Changed

- 20 protocol docs moved root → `steps/` (history preserved via `git mv`); all cross-refs re-pointed. `AGENTS.md` stays at root (tool auto-loading).
- `.gitignore` back to standard track-by-default; private material lives in the `-Local` sibling folder.
- README: plain-language hero, 4-checks intro, Archify pipeline diagram (SVG + interactive HTML), ithmb-style header with badges, `CREDITS.md` aligned to ithmb format.

### Removed

- `docs/SPEC_SYNC.md` stub (content long merged into `REVIEW.md` fidelity check).

### Fixed

- 20+ broken internal links: new `LICENSE` (MIT, ithmb text); `docs/PHILOSOPHY.md` repoints → `steps/QUICKSTART.md`; new `docs/STANDING_PRINCIPLES.md`; appendix/trace/test-signal/AGENTS/standards/traces path prefixes; removed `ledger-check.py` tree line (file never existed). Sweep-verified; scaffold-target, archive, `.omo`, and cross-repo refs intentionally untouched.

## [2026-09-09] — Public mirror synced

- Thematic squash to public: SWE appendixes (P2b early + P4 late), agnostic search backend, PoP altitude gates, lived traces, README overhaul. Origin and public trees identical.

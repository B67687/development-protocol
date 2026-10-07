# Pilot A20 full run (2026-10-07)

Frozen-spec run of `docs/bench/pilot-a20-multi-session-epic.md` on
`fixtures/bench/a20-epic/`. Tier: **Reimplement**. Two sessions with forced
kill + gap-injected store-path delta; session 2 started from HANDOVER only.

## Environment (deviation logged)

Spec pins `python:3.11.9-slim` + Node 20.12.2 with digests at freeze. This
run used the host toolchain (**Python 3.14.4**) — no frozen image was
available. CLI is stdlib-only, so results are portable-evidence, not
container-frozen data. Runtime declared at session-1 start: Python 3 stdlib
only (`argparse`, `json`, `csv`, `os`, `sys`); no lockfile needed.

## Session log

- Session 1 (ends `b4c0565`): scaffold (`EPIC.md`, `HANDOVER.template.md`) +
  M1 (`add`/`list`/`done`/`rm`, store `./data.json`, counter-backed stable
  ids, atomic tmp+rename writes). Schema fix pre-freeze: bare max+1 reused
  ids after rm-of-max, so the store became `{next_id, todos}` (EPIC.md
  updated before any oracle existed). Kill drill: background churn loop
  SIGKILLed mid-write — store reloaded valid, 34 todos, next_id=35, no dups,
  no torn write, no orphan `.tmp`. Closed with `HANDOVER.md`.
- Gap: delta delivered (store `./data.json` → `./store/todos.json`).
- Session 2: ran the HANDOVER next-trigger first (`add "smoke"` → `1`,
  `list` → `1 [ ] smoke`, green — M1 survived). Disposition logged in
  `session-2-log.md`: capture (delta is new info, HANDOVER marked it
  UNKNOWN) → re-research (bare default swap would orphan legacy stores) →
  refold (default moves; legacy read-fallback; first write promotes to new
  path; old file never deleted). Then M2 (`search`, `list --filter`) + M3
  (`export`, global `--dry-run`, `README.md`).

## Predicted vs actual

- **Predicted failure (pilot §5):** PARKED-without-trigger — successor
  re-derives everything; falsifier = HANDOVER <30/40 on H2/H4.
- **Actual:** oracle **12/12** + HANDOVER rubric **35/40** (H1 10 · H2 10 ·
  H3 10 · H4 5 — no timed human cold-reader spot-check was available, so H4
  takes partial credit: the trigger was concrete and executed in seconds).
  The falsifier did **not** trigger — a finding against the canonical
  prediction at N=1, same shape as the A14 run. No A17-mode silent
  continuation either: the delta refold is logged step by step.

## Oracle results

- Hidden oracle `oracle/hidden.a20.test.sh` (T01–T04 M1 incl. stable ids,
  T05–T08 M2 incl. validation side-effect-freedom, T09–T10 M3 export +
  dry-run, T11 relocated default path, T12 legacy migration): **12/12**.
- Seal (sha256 at green time):
  - oracle `c6ca3464a455257cfc0d921e7650a2f57606fcc36378f40a8731b3bccf5a06d3`
  - CLI `cb95e7be2fe280ffe56b843194f15cd2871adbc457dd5180060ce446d332200b`
- HANDOVER rubric (self-graded from session-1 artifact): H1 10/10 (exact
  commit `4e44545`, files, D1–D4 per-criterion) · H2 10/10 (exact smoke
  command with pass/fail branches) · H3 10/10 (R1/R2 + delta UNKNOWN) ·
  H4 5/10 (no human timed check). **Total 35/40 (threshold ≥30/40).**

## Caveats

Self-authored oracle + self-graded rubric (same circularity as the A14 run);
seal hashes above freeze the oracle for independent re-grading. A14
fixtures untouched. Lint 17/17 PASS (protocol-lint sub-checks).

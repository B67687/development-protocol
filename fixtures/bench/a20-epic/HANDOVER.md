# HANDOVER — A20 session 1 → session 2 (only cross-session channel)

## 1. State (exact)

- Commit: `4e44545` ("Freeze A20 epic scaffold + M1 todo CLI (session 1)").
  Working tree at stop: clean except this file (untracked until committed).
- Files: `fixtures/bench/a20-epic/EPIC.md` (milestones D1–D10 + delta slot),
  `HANDOVER.template.md`, `src/todo.py` (M1 only: add/list/done/rm).
  Runtime: Python 3 stdlib only. Store default: `./data.json`
  (`{"next_id": int, "todos": [...]}`; ids never reused; atomic tmp+rename).
- Per done-criterion: D1 pass (add persists, prints id); D2 pass (list shows
  ids + done-state); D3 pass (done/rm work; rm-then-add drew a fresh id —
  verified ids 1,3 with next_id=4); D4 pass (missing store → empty; corrupt
  store → stderr + exit 2; validated live).
- Kill drill (forced-stop rehearsal): background churn loop SIGKILLed
  (`kill -9`) mid-write; store reloaded as valid JSON, 34 todos, next_id=35,
  no dup ids, no torn write, no orphan `.tmp`. Position-index survives.
- Last verified content at stop: demo `data.json` removed; store starts empty
  on next run. M2/M3 NOT started: no `search`, no `--filter`, no `export`,
  no `--dry-run`, no `README.md`.

## 2. Next trigger (concrete resume condition, not "continue")

- Run `cd fixtures/bench/a20-epic && rm -f data.json && python3 src/todo.py
  add "smoke" && python3 src/todo.py list`: if it prints `1` then
  `1 [ ] smoke`, M1 survived the gap — start M2 (`search` + `--filter`).
  If it fails, stop: the handover is stale, re-verify D1–D4 before writing
  any M2 code.

## 3. Open risks (≥2, unknowns marked unknown)

- R1: `argparse` subcommand surface will grow (search/filter/export/dry-run);
  keep one runtime (stdlib only) — any new dependency must be risk-logged
  per pilot §2, and none is expected to be needed.
- R2: id monotonicity depends on `next_id` inside the store file; concurrent
  writers could interleave read-modify-write (no locking). Single-writer use
  only — do not add background writers without a lock.
- Store-path delta: UNKNOWN at forced stop (no gap notice had arrived).
  If the gap delivers a path change, migrate — never drop — the old store.

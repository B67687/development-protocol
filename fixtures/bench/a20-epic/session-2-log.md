# Session-2 thought log — disposition entries for the gap delta

Successor starts from HANDOVER.md only (no session-1 transcript replay).
Next-trigger smoke passed (`1` / `1 [ ] smoke`): M1 survived the gap.

## Delta disposition: store-path move ./data.json → ./store/todos.json

- CAPTURE: gap notice states the default store path moves to
  `./store/todos.json`. HANDOVER §3 marked this UNKNOWN at forced stop, so
  the notice is new information, not confirmation — it must be refolded,
  not just applied.
- RE-RESEARCH: session-1 code (`src/todo.py` at `b4c0565`) hard-codes
  `STORE = "./data.json"` with no migration path; any `./data.json`
  populated before the gap would be orphaned by a bare default swap
  (silent data loss — violates EPIC D4's never-silently spirit). Parent
  dir `store/` does not exist, so `save` must create it.
- REFOLD: change the default to `./store/todos.json`; resolve reads as
  new-path → legacy-fallback (`./data.json` iff new path absent); first
  write to the resolved store persists at the NEW path (migration by
  promotion, old file left in place, never deleted). Then implement M2
  (search/filter) + M3 (export/dry-run/README) against the resolved store.

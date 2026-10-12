# EPIC.md — A20 todo-list CLI (frozen task material)

Runtime (declared at session-1 start): **Python 3 stdlib only** (`argparse`,
`json`, `csv`, `os`, `sys`). No dependencies. Entry point: `src/todo.py`.

Store: JSON object `{"next_id": int, "todos": [{"id": int, "title": str, "done": bool}]}`.
`id` is a stable position-index drawn from the persistent `next_id` counter,
never reused after `rm` (gap survival depends on it). All writes are atomic
(tmp file + rename).

## M1 — ingest + add/list (session 1 must stop here)

Done-criteria:
- D1 `add <title>` persists a todo and prints its id.
- D2 `list` shows all todos with ids and done-state.
- D3 `done <id>` marks a todo done; `rm <id>` removes one (ids never reused).
- D4 Store defaults to `./data.json`; corrupt/missing store degrades loudly
  (stderr + non-zero exit), never silently.

## M2 — search/filter + validation errors (session 2)

Done-criteria:
- D5 `search <query>` lists todos whose title contains the query (substring).
- D6 `list --filter open|done` filters by state; invalid filter value errors.
- D7 Validation errors (empty title, unknown id) exit non-zero with a message
  on stderr and leave the store untouched.

## M3 — export-csv + --dry-run + README (session 2)

Done-criteria:
- D8 `export <file.csv>` writes `id,title,done` rows for all todos.
- D9 `--dry-run` on any mutating command prints the plan and changes nothing
  (store byte-identical afterwards).
- D10 `README.md` documents usage for every command with one example each.

## Gap-injected delta (delivered in the gap, session 2 only)

> Store file moves from `./data.json` to `./store/todos.json`.
> Session 2 must capture → re-research → refold: default path changes, and a
> legacy `./data.json` (if present, no new-path file) is migrated on first
> write, never silently dropped.

# todo CLI — usage

Python 3 stdlib only. Store defaults to `./store/todos.json`; a legacy
`./data.json` (no new-path file) is read and promoted on first write.

## Commands

- Add: `python3 src/todo.py add "buy milk"` → prints the new id (`1`).
- List: `python3 src/todo.py list` → `1 [ ] buy milk` (`[x]` when done).
- Filter: `python3 src/todo.py list --filter open` (or `--filter done`).
- Search: `python3 src/todo.py search milk` → substring matches.
- Mark done: `python3 src/todo.py done 1` → prints `1`.
- Remove: `python3 src/todo.py rm 1` → prints `1` (ids are never reused).
- Export: `python3 src/todo.py export todos.csv` → `id,title,done` rows.
- Dry run: `python3 src/todo.py --dry-run add "ghost"` → prints
  `would add 4: ghost`, store untouched. Works on add/done/rm/export.
- Custom store: `python3 src/todo.py --store /tmp/mine.json list`.

Validation errors (empty title, unknown id, bad `--filter`) exit 2 with a
stderr message and leave the store untouched.

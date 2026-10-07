#!/usr/bin/env python3
"""A20 todo-list CLI — session 2: M1+M2+M3, store ./store/todos.json.

Gap delta refolded: default path moved from ./data.json (legacy fallback,
promoted on first write). Writes are atomic (tmp file + os.replace).
Ids are stable position-indices from a persistent counter, never reused.
"""
import argparse
import csv
import json
import os
import sys

STORE = './store/todos.json'
LEGACY = './data.json'


def fail(msg):
    print(f'todo: error: {msg}', file=sys.stderr)
    raise SystemExit(2)


def resolve_paths(requested, explicit):
    """Return (read_path, write_path). Legacy store is read but never
    written: the first write promotes it to the new default path."""
    if explicit or os.path.exists(requested) or not os.path.exists(LEGACY):
        return requested, requested
    return LEGACY, requested


def load(path):
    state = {'next_id': 1, 'todos': []}
    if os.path.exists(path):
        try:
            with open(path, encoding='utf-8') as f:
                raw = json.load(f)
        except (json.JSONDecodeError, OSError) as e:
            fail(f'store unreadable ({path}): {e}')
        if isinstance(raw, list):  # pre-counter schema: adopt, keep ids
            raw = {'next_id': max((t['id'] for t in raw), default=0) + 1, 'todos': raw}
        if not isinstance(raw, dict) or not isinstance(raw.get('todos'), list):
            fail(f'store corrupt ({path}): expected next_id/todos object')
        state = raw
    return state


def save(path, data):
    parent = os.path.dirname(path)
    try:
        if parent:
            os.makedirs(parent, exist_ok=True)
        tmp = path + '.tmp'
        with open(tmp, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2)
            f.write('\n')
            f.flush()
            os.fsync(f.fileno())
        os.replace(tmp, path)
    except OSError as e:
        fail(f'store write failed ({path}): {e}')


def take_id(state):
    tid = state['next_id']
    state['next_id'] = tid + 1
    return tid


def find(todos, tid):
    for t in todos:
        if t['id'] == tid:
            return t
    return None


def show(t):
    mark = 'x' if t['done'] else ' '
    print(f'{t["id"]} [{mark}] {t["title"]}')


def cmd_add(args):
    if not args.title.strip():
        fail('title must not be empty')
    state = load(args.read_path)
    tid = take_id(state)
    if args.dry_run:
        print(f'would add {tid}: {args.title}')
        return
    state['todos'].append({'id': tid, 'title': args.title, 'done': False})
    save(args.store, state)
    print(tid)


def cmd_list(args):
    filt = args.filter
    if filt not in ('all', 'open', 'done'):
        fail(f'invalid filter: {filt} (expected open|done)')
    for t in load(args.read_path)['todos']:
        if filt == 'open' and t['done']:
            continue
        if filt == 'done' and not t['done']:
            continue
        show(t)


def cmd_search(args):
    query = args.query
    if not query.strip():
        fail('query must not be empty')
    for t in load(args.read_path)['todos']:
        if query in t['title']:
            show(t)


def cmd_done(args):
    state = load(args.read_path)
    t = find(state['todos'], args.id)
    if t is None:
        fail(f'unknown id: {args.id}')
    if args.dry_run:
        print(f'would mark done {args.id}')
        return
    t['done'] = True
    save(args.store, state)
    print(args.id)


def cmd_rm(args):
    state = load(args.read_path)
    if find(state['todos'], args.id) is None:
        fail(f'unknown id: {args.id}')
    if args.dry_run:
        print(f'would remove {args.id}')
        return
    state['todos'] = [t for t in state['todos'] if t['id'] != args.id]
    save(args.store, state)
    print(args.id)


def cmd_export(args):
    todos = load(args.read_path)['todos']
    if args.dry_run:
        print(f'would export {len(todos)} rows to {args.dest}')
        return
    try:
        with open(args.dest, 'w', encoding='utf-8', newline='') as f:
            w = csv.writer(f)
            w.writerow(['id', 'title', 'done'])
            for t in todos:
                w.writerow([t['id'], t['title'], 'true' if t['done'] else 'false'])
    except OSError as e:
        fail(f'export failed ({args.dest}): {e}')
    print(f'exported {len(todos)} rows to {args.dest}')


def main(argv=None):
    p = argparse.ArgumentParser(prog='todo', description='tiny todo-list CLI')
    p.add_argument('--store', default=None,
                   help=f'store path (default {STORE}; legacy {LEGACY} migrated on write)')
    p.add_argument('--dry-run', action='store_true',
                   help='print the plan, change nothing')
    sub = p.add_subparsers(dest='cmd', required=True)
    a = sub.add_parser('add')
    a.add_argument('title')
    a.set_defaults(fn=cmd_add)
    b = sub.add_parser('list')
    b.add_argument('--filter', default='all')
    b.set_defaults(fn=cmd_list)
    s = sub.add_parser('search')
    s.add_argument('query')
    s.set_defaults(fn=cmd_search)
    c = sub.add_parser('done')
    c.add_argument('id', type=int)
    c.set_defaults(fn=cmd_done)
    d = sub.add_parser('rm')
    d.add_argument('id', type=int)
    d.set_defaults(fn=cmd_rm)
    e = sub.add_parser('export')
    e.add_argument('dest')
    e.set_defaults(fn=cmd_export)
    args = p.parse_args(argv)
    requested = args.store or STORE
    args.read_path, args.store = resolve_paths(requested, args.store is not None)
    args.fn(args)


if __name__ == '__main__':
    main()

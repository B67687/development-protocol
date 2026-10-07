#!/usr/bin/env bash
# oracle/hidden.a20.test.sh — sealed continuation oracle, 12 tests.
# M1–M3 behaviour incl. relocated store path + --dry-run side-effect-freedom.
# Threshold: 12/12 (M1-only state scores <=4).
set -u
CLI="$(cd "$(dirname "$0")/../src" && pwd)/todo.py"
WORK="$(mktemp -d)"
PASS=0
FAIL=0

ok() { PASS=$((PASS + 1)); echo "ok: $1"; }
no() { FAIL=$((FAIL + 1)); echo "FAIL: $1"; }
t() { python3 "$CLI" --store "$WORK/s.json" "$@"; }

# T01 add+list roundtrip
[ "$(t add 'buy milk')" = "1" ] && t list | grep -q '1 \[ \] buy milk' \
  && ok T01-add-list || no T01-add-list
# T02 ids never reused after rm
t add 'tmp' >/dev/null && t rm 2 >/dev/null && [ "$(t add 'fresh')" = "3" ] \
  && ok T02-stable-ids || no T02-stable-ids
# T03 done marks state
t done 1 >/dev/null && t list | grep -q '1 \[x\] buy milk' \
  && ok T03-done || no T03-done
# T04 rm removes
t rm 3 >/dev/null && ! t list | grep -q 'fresh' \
  && ok T04-rm || no T04-rm
# T05 search substring
t add 'milk run' >/dev/null && [ "$(t search milk | wc -l)" = "2" ] \
  && ok T05-search || no T05-search
# T06 filter open/done
t list --filter open | grep -q 'milk run' \
  && t list --filter done | grep -q 'buy milk' \
  && ! t list --filter open | grep -q 'buy milk' \
  && ok T06-filter || no T06-filter
# T07 empty title: non-zero + store untouched
H0=$(sha256sum "$WORK/s.json"); t add '' 2>/dev/null; E=$?
[ $E -ne 0 ] && [ "$(sha256sum "$WORK/s.json")" = "$H0" ] \
  && ok T07-empty-title || no T07-empty-title
# T08 unknown id: non-zero + store untouched
t done 99 2>/dev/null; E=$?
[ $E -ne 0 ] && [ "$(sha256sum "$WORK/s.json")" = "$H0" ] \
  && ok T08-unknown-id || no T08-unknown-id
# T09 export csv content
t export "$WORK/out.csv" >/dev/null \
  && head -1 "$WORK/out.csv" | grep -q '^id,title,done' \
  && grep -q 'buy milk' "$WORK/out.csv" \
  && ok T09-export || no T09-export
# T10 dry-run side-effect-free
t --dry-run add 'ghost' >/dev/null && t --dry-run rm 1 >/dev/null \
  && t --dry-run export "$WORK/dry.csv" >/dev/null \
  && [ "$(sha256sum "$WORK/s.json")" = "$H0" ] && [ ! -e "$WORK/dry.csv" ] \
  && ! t list | grep -q 'ghost' && ok T10-dry-run || no T10-dry-run
# T11 relocated default store path (no --store flag)
D11="$(mktemp -d)"; (cd "$D11" && python3 "$CLI" add 'gap' >/dev/null \
  && [ -f store/todos.json ] && [ ! -f data.json ]) \
  && ok T11-relocated-path || no T11-relocated-path
# T12 legacy data.json migrated on write, never dropped
D12="$(mktemp -d)"
printf '%s' '{"next_id": 8, "todos": [{"id": 7, "title": "legacy", "done": false}]}' \
  > "$D12/data.json"
(cd "$D12" && python3 "$CLI" list | grep -q 'legacy' \
  && python3 "$CLI" add 'new' | grep -q '8' \
  && grep -q 'legacy' store/todos.json && grep -q '"id": 7' data.json) \
  && ok T12-legacy-migration || no T12-legacy-migration

echo "---"
echo "Result: $PASS/12 pass, $FAIL fail"
[ "$FAIL" = "0" ] && [ "$PASS" = "12" ]

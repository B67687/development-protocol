#!/usr/bin/env bash
# Local verification for the Development Protocol repo.
# Local counterpart to template/ci.yml.tmpl — run this when CI isn't configured.
# Usage: ./scripts/check.sh  (from anywhere in the repo)
set -euo pipefail
cd "$(dirname "$0")/.."

fail() { echo "FAIL: $1"; exit 1; }
ok()   { echo "  ok: $1"; }

# 1. Phase compliance — mirrors the phase check in template/ci.yml.tmpl. Accepts both the
#    template form (`**Current:** \`PHASE\``) and the bootstrapped form (`> **Current: PHASE**`).
if ! grep -Eq '^>? ?\*\*Current:(\*\*)? ?\`?(DISCOVER|WORK|ITERATE|PERFECT|DISTRIBUTE)\`?(\*\*)?' RULES.md; then
  fail "RULES.md phase not set (expected '**Current:** \`PHASE\`')."
fi
ok "RULES.md phase set"

# 2. CLI contract — the cli crate greps RULES.md for these headings at runtime
#    (cli/src/main.rs ~601-628). Mirror has_heading: level-2 `## ` heading,
#    optional "N. " number prefix, substring match.
headings=(
  'Phase'
  'Constitution'
  'V1 Scope|Scope'
  'Stop Rules'
  'AI Persona|Persona'
  'Verification Gates'
  'Test Philosophy'
)
for h in "${headings[@]}"; do
  if ! grep -Eq '^## [^#]*('"$h"')' RULES.md; then
    fail "RULES.md missing heading: $h"
  fi
done
ok "RULES.md CLI-contract headings present"

# 3. cli crate: build + tests.
(cd cli && cargo check --quiet) || fail "cargo check failed in cli/"
ok "cargo check (cli/)"
(cd cli && cargo test --quiet) || fail "cargo test failed in cli/"
ok "cargo test (cli/)"

echo "All checks passed."

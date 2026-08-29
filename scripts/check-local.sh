#!/usr/bin/env bash
# check-local.sh — Local verification gates for Development-Protocol
# 4 gates: markdown lint, ADR existence, registry check, .omo leak
# Run before every push to origin (Dev)

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PASS=0
FAIL=0

pass() { echo "✅ $1"; PASS=$((PASS + 1)); }
fail() { echo "❌ $1"; FAIL=$((FAIL + 1)); }

echo "=== Development-Protocol Local Check ==="
echo ""

# Gate 1: Markdown lint — critical files exist and are non-empty
echo "--- Gate 1: Critical file presence ---"
for f in README.md RULES.md STANDARDS.md SPECIFICATION.md docs/FEATURES.md docs/SE_ARTIFACT_REGISTRY.md docs/TECH_DEBT_AUDIT.md AGENTS.md; do
  if [[ -f "$REPO_ROOT/$f" ]] && [[ -s "$REPO_ROOT/$f" ]]; then
    pass "$f exists and non-empty"
  else
    fail "$f missing or empty"
  fi
done
echo ""

# Gate 2: ADR existence — at least 3 ADRs in docs/adr/
echo "--- Gate 2: ADR existence ---"
ADR_COUNT=$(find "$REPO_ROOT/docs/adr" -name "*.md" 2>/dev/null | wc -l)
if [[ "$ADR_COUNT" -ge 3 ]]; then
  pass "ADR count: $ADR_COUNT (≥3 required)"
else
  fail "ADR count: $ADR_COUNT (≥3 required)"
fi
echo ""

# Gate 3: Registry self-consistency — Development-Protocol row exists
echo "--- Gate 3: Registry self-consistency ---"
if grep -q "Development-Protocol" "$REPO_ROOT/docs/SE_ARTIFACT_REGISTRY.md" 2>/dev/null; then
  pass "SE_ARTIFACT_REGISTRY tracks Development-Protocol"
else
  fail "SE_ARTIFACT_REGISTRY missing Development-Protocol row"
fi
echo ""

# Gate 4: .omo leak — no .omo files tracked by git
echo "--- Gate 4: .omo leak detection ---"
OMO_TRACKED=$(git -C "$REPO_ROOT" ls-files .omo/ 2>/dev/null | wc -l)
if [[ "$OMO_TRACKED" -eq 0 ]]; then
  pass "No .omo/ files tracked by git"
else
  fail "$OMO_TRACKED .omo/ files tracked by git (should be 0)"
fi
echo ""

# Summary
echo "=== Results: $PASS passed, $FAIL failed ==="
if [[ "$FAIL" -gt 0 ]]; then
  echo "❌ CHECK FAILED"
  exit 1
else
  echo "✅ ALL CHECKS PASSED"
  exit 0
fi

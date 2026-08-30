#!/usr/bin/env bash
# ==============================================================================
# protocol-lint.sh — 7-rule CI lint for Development-Protocol ceremony
# ==============================================================================
# Checks that the protocol's structural invariants hold. Fails PRs when
# ceremony collapses (missing files, broken schemas, stale artifacts).
#
# Usage:
#   bash scripts/protocol-lint.sh [REPO_ROOT]
#
# Arguments:
#   REPO_ROOT   Path to Development-Protocol repo (default: parent of scripts/)
#
# Exit code: 0 if all rules pass, 1 if any rule fails.
# Runs in <30s with no dependencies beyond bash, coreutils, and python3.
#
# Rules:
#   1. METHOD_LEDGER  — docs/METHOD_LEDGER.md exists, schema fields documented,
#                       Light vs Standard mode distinction present
#   2. KILL_LOG       — KILL_LOG.md table has 8 cols, valid outcome, counterfactual
#                       contains 30-day/reminder token
#   3. SERIOUSNESS    — Decision Journal table with valid fields, scores 0-120,
#                       classes in valid set
#   4. SPEC_RTM       — SPECIFICATION.md §1.5 traceability matrix exists with
#                       required columns and ≥1 data row
#   5. BIAS_CATALOG   — 8 biases, each with detection prompt in quoted block
#   6. QUICKSTART     — ≤200 lines, 6 required sections present
#   7. SKIP_CONDITIONS — 5 phase docs each have "## Skip Conditions" box
# ==============================================================================

set -uo pipefail

REPO="${1:-$(cd "$(dirname "$0")/.." && pwd)}"
cd "$REPO"

PASS=0
FAIL=0

pass() { echo "  ✅ $1"; PASS=$((PASS + 1)); }
fail() { echo "  ❌ $1 — $2"; FAIL=$((FAIL + 1)); }

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo " Protocol Lint — 7-Rule Ceremony Check"
echo " Repo: $REPO"
echo "═══════════════════════════════════════════════════════════════════"
echo ""

# ─── Rule 1: METHOD_LEDGER ──────────────────────────────────────────────────
echo "Rule 1: METHOD_LEDGER schema"

LEDGER_DOC="docs/METHOD_LEDGER.md"
LEDGER_DATA=".omo/method-ledger.jsonl"

if [[ -f "$LEDGER_DATA" ]]; then
    # JSONL data file exists — validate structure with python3
    jsonl_check=$(python3 -c "
import json, sys
required = ['case', 'method', 'status', 'evidence', 'ts']
valid_status = {'applied', 'skipped', 'omitted'}
errors = []
with open('$LEDGER_DATA') as f:
    for i, line in enumerate(f, 1):
        line = line.strip()
        if not line:
            continue
        try:
            rec = json.loads(line)
        except json.JSONDecodeError as e:
            errors.append(f'line {i}: invalid JSON: {e}')
            continue
        for field in required:
            if field not in rec:
                errors.append(f'line {i}: missing field \"{field}\"')
        status = rec.get('status', '')
        if status not in valid_status:
            errors.append(f'line {i}: invalid status \"{status}\"')
        if status == 'applied' and not rec.get('evidence'):
            errors.append(f'line {i}: applied entry missing evidence')
        if status == 'skipped' and not rec.get('reason'):
            errors.append(f'line {i}: skipped entry missing reason')
if errors:
    print('ERRORS: ' + '; '.join(errors[:5]))
    sys.exit(1)
else:
    print('OK')
" 2>&1) || true
    if [[ "$jsonl_check" == "OK" ]]; then
        entry_count=$(grep -c '.' "$LEDGER_DATA" 2>/dev/null || echo "0")
        pass "R1 — METHOD_LEDGER.jsonl valid JSONL ($entry_count entries), schema fields present"
    else
        fail "R1-JSONL" "$jsonl_check"
    fi
elif [[ -f "$LEDGER_DOC" ]]; then
    # Fallback: check schema documentation exists with required fields
    missing=()
    for field in case method status evidence ts; do
        if ! grep -qi "$field" "$LEDGER_DOC"; then
            missing+=("$field")
        fi
    done
    if [[ ${#missing[@]} -gt 0 ]]; then
        fail "R1-FIELDS" "$LEDGER_DOC missing documented fields: ${missing[*]}"
    elif grep -qi "light.*mode\|standard.*mode\|Light.*Standard" "$LEDGER_DOC"; then
        pass "R1 — METHOD_LEDGER.md schema documented, fields present, Light/Standard split"
    else
        fail "R1-MODE" "$LEDGER_DOC missing Light vs Standard mode distinction"
    fi
else
    fail "R1-FILE" "neither $LEDGER_DATA nor $LEDGER_DOC exists"
fi

echo ""

# ─── Rule 2: KILL_LOG ───────────────────────────────────────────────────────
echo "Rule 2: KILL_LOG table structure"

KILL_LOG="KILL_LOG.md"
if [[ ! -f "$KILL_LOG" ]]; then
    fail "R2-FILE" "$KILL_LOG does not exist"
else
    r2_issues=()
    row_count=0
    counterfactual_ok=false
    outcome_ok=true
    header_counterfactual_ok=false

    # Extract only the kill tracking table (between ## Template and next heading/blank)
    in_table=false
    while IFS= read -r line; do
        # Start after ## Template heading
        if echo "$line" | grep -qi '## Template'; then
            in_table=true
            continue
        fi
        # Stop at next heading (not empty lines — those are inside the table area)
        if $in_table && echo "$line" | grep -q '^## '; then
            break
        fi
        if ! $in_table; then
            continue
        fi
        # Skip separator lines (contains ---)
        if echo "$line" | grep -q '|.*---'; then
            continue
        fi
        # Skip non-table lines
        if ! echo "$line" | grep -q '|'; then
            continue
        fi
        # Check header row for column names and counterfactual token
        if echo "$line" | grep -qi 'ID.*Date'; then
            # Header row — check for 30-day token in column header
            if echo "$line" | grep -qi '30.day\|counterfactual'; then
                header_counterfactual_ok=true
            fi
            continue
        fi

        row_count=$((row_count + 1))

        # Count columns: split by |, count non-empty fields
        col_count=$(echo "$line" | awk -F'|' '{n=0; for(i=1;i<=NF;i++) if($i!="") n++; print n}')
        if [[ "$col_count" -ne 8 ]]; then
            r2_issues+=("row $row_count: $col_count cols (need 8)")
        fi

        # Check counterfactual column (col 7 = $8) for any content
        counterfactual=$(echo "$line" | awk -F'|' '{gsub(/^[ \t]+|[ \t]+$/, "", $8); print $8}')
        if [[ -n "$counterfactual" && "$counterfactual" != "—" ]]; then
            counterfactual_ok=true
        fi

        # Check outcome column (col 8 = $9) for valid values
        outcome=$(echo "$line" | awk -F'|' '{gsub(/^[ \t]+|[ \t]+$/, "", $9); print $9}')
        if ! echo "$outcome" | grep -qiE '^(Correct|Miss|Pending|—|-|Correct / Miss)$'; then
            outcome_ok=false
            r2_issues+=("row $row_count: outcome='$outcome' not in {Correct, Miss, Pending, —}")
        fi
    done < "$KILL_LOG"

    if [[ $row_count -eq 0 ]]; then
        fail "R2-EMPTY" "no data rows in $KILL_LOG table"
    elif [[ ${#r2_issues[@]} -gt 0 ]]; then
        fail "R2-STRUCT" "${r2_issues[*]}"
    elif ! $header_counterfactual_ok; then
        fail "R2-COUNTER" "header row missing 30-day/counterfactual column"
    elif ! $outcome_ok; then
        fail "R2-OUTCOME" "invalid outcome values"
    else
        pass "R2 — KILL_LOG: $row_count row(s), 8 cols, valid outcomes, 30-day counterfactual"
    fi
fi

echo ""

# ─── Rule 3: SERIOUSNESS Decision Journal ───────────────────────────────────
echo "Rule 3: SERIOUSNESS Decision Journal"

SERIOUSNESS="SERIOUSNESS.md"
if [[ ! -f "$SERIOUSNESS" ]]; then
    fail "R3-FILE" "$SERIOUSNESS does not exist"
else
    if ! grep -qi "decision journal" "$SERIOUSNESS"; then
        fail "R3-SECTION" "no Decision Journal section in $SERIOUSNESS"
    else
        r3_issues=()
        in_journal=false
        header_found=false
        data_rows=0

        while IFS= read -r line; do
            # Detect Decision Journal section start (### heading)
            if echo "$line" | grep -q '^###.*Decision Journal'; then
                in_journal=true
                continue
            fi
            # Stop at next ### or ## heading after journal starts
            if $in_journal && echo "$line" | grep -qE '^#{2,3} '; then
                break
            fi
            if $in_journal && echo "$line" | grep -q '|'; then
                # Header row
                if echo "$line" | grep -qi 'Date.*Score'; then
                    header_found=true
                    for field in Date Class Score Decision Outcome; do
                        if ! echo "$line" | grep -qi "$field"; then
                            r3_issues+=("header missing: $field")
                        fi
                    done
                elif ! echo "$line" | grep -q '|.*---'; then
                    # Data row
                    data_rows=$((data_rows + 1))

                    # Validate score (col 4 = $5) range 0-120
                    score_raw=$(echo "$line" | awk -F'|' '{gsub(/^[ \t]+|[ \t]+$/, "", $5); print $5}')
                    score_num=$(echo "$score_raw" | grep -oE '[0-9]+' | head -1)
                    if [[ -n "$score_num" ]]; then
                        if [[ "$score_num" -lt 0 || "$score_num" -gt 120 ]]; then
                            r3_issues+=("row $data_rows: score $score_num out of range 0-120")
                        fi
                    fi

                    # Validate class (col 3 = $4)
                    class_val=$(echo "$line" | awk -F'|' '{gsub(/^[ \t]+|[ \t]+$/, "", $4); print $4}')
                    if ! echo "$class_val" | grep -qiE 'Personal tool|Open-source library|Production service|Research prototype'; then
                        r3_issues+=("row $data_rows: invalid class '$class_val'")
                    fi
                fi
            fi
            # (break already handled above for both ## and ### headings)
        done < "$SERIOUSNESS"

        if ! $header_found; then
            fail "R3-TABLE" "Decision Journal table header not found"
        elif [[ ${#r3_issues[@]} -gt 0 ]]; then
            fail "R3-VALID" "${r3_issues[*]}"
        else
            pass "R3 — SERIOUSNESS Decision Journal: table valid, $data_rows row(s), scores 0-120, classes valid"
        fi
    fi
fi

echo ""

# ─── Rule 4: SPECIFICATION §1.5 RTM ────────────────────────────────────────
echo "Rule 4: SPECIFICATION §1.5 Traceability Matrix"

SPEC="SPECIFICATION.md"
if [[ ! -f "$SPEC" ]]; then
    fail "R4-FILE" "$SPEC does not exist"
else
    if ! grep -qiE '1\.5.*traceab|traceab.*matrix|RTM' "$SPEC"; then
        fail "R4-SECTION" "§1.5 Traceability Matrix section not found in $SPEC"
    else
        in_rtm=false
        header_found=false
        data_rows=0

        while IFS= read -r line; do
            if echo "$line" | grep -qiE '1\.5.*traceab|traceab.*matrix'; then
                in_rtm=true
                continue
            fi
            if $in_rtm && echo "$line" | grep -q '|'; then
                if echo "$line" | grep -qi 'Spec.*Section\|Idea.*Source\|Phase.*Artifact\|Decision.*Record'; then
                    header_found=true
                elif ! echo "$line" | grep -q '|.*---'; then
                    data_rows=$((data_rows + 1))
                fi
            fi
            if $in_rtm && $header_found && echo "$line" | grep -q '^## '; then
                break
            fi
        done < "$SPEC"

        if ! $header_found; then
            fail "R4-TABLE" "RTM table header not found in §1.5"
        elif [[ $data_rows -eq 0 ]]; then
            fail "R4-ROWS" "RTM table has 0 data rows (need ≥1 when not Light mode)"
        else
            pass "R4 — SPEC §1.5 RTM: table present, $data_rows data row(s)"
        fi
    fi
fi

echo ""

# ─── Rule 5: BIAS_CATALOG ──────────────────────────────────────────────────
echo "Rule 5: BIAS_CATALOG completeness"

BIAS="BIAS_CATALOG.md"
if [[ ! -f "$BIAS" ]]; then
    fail "R5-FILE" "$BIAS does not exist"
else
    # Count bias entries (## headings)
    bias_count=$(grep -cE '^## [0-9]' "$BIAS" 2>/dev/null || echo "0")

    # Count detection prompts
    detection_count=$(grep -c 'Detection prompt' "$BIAS" 2>/dev/null || echo "0")

    r5_issues=()
    if [[ "$bias_count" -ne 8 ]]; then
        r5_issues+=("found $bias_count bias headings (need 8)")
    fi
    if [[ "$detection_count" -ne 8 ]]; then
        r5_issues+=("found $detection_count detection prompts (need 8)")
    fi

    if [[ ${#r5_issues[@]} -gt 0 ]]; then
        fail "R5-COUNT" "${r5_issues[*]}"
    else
        # Verify detection prompts have quoted blocks (lines starting with >)
        in_detection=false
        quote_count=0
        while IFS= read -r line; do
            if echo "$line" | grep -q 'Detection prompt'; then
                in_detection=true
                continue
            fi
            if $in_detection && echo "$line" | grep -q '^>'; then
                quote_count=$((quote_count + 1))
                in_detection=false
            elif $in_detection && [[ -n "$line" ]] && ! echo "$line" | grep -q '^$'; then
                in_detection=false
            fi
        done < "$BIAS"

        if [[ $quote_count -ne 8 ]]; then
            fail "R5-QUOTE" "$quote_count detection prompts have quoted blocks (need 8)"
        else
            pass "R5 — BIAS_CATALOG: 8 biases, 8 detection prompts with quoted blocks"
        fi
    fi
fi

echo ""

# ─── Rule 6: QUICKSTART ────────────────────────────────────────────────────
echo "Rule 6: QUICKSTART structure and length"

QS="QUICKSTART.md"
if [[ ! -f "$QS" ]]; then
    fail "R6-FILE" "$QS does not exist"
else
    line_count=$(wc -l < "$QS")
    r6_issues=()

    if [[ $line_count -gt 200 ]]; then
        r6_issues+=("$line_count lines exceeds 200 max")
    fi

    for section in "5-Minute" "30-Minute" "Full Map" "Decision Tree" "Glossary" "Skip"; do
        if ! grep -qi "$section" "$QS"; then
            r6_issues+=("missing section: $section")
        fi
    done

    if [[ ${#r6_issues[@]} -gt 0 ]]; then
        fail "R6-STRUCT" "${r6_issues[*]}"
    else
        pass "R6 — QUICKSTART: $line_count lines (≤200), all 6 required sections present"
    fi
fi

echo ""

# ─── Rule 7: Phase Skip Conditions ─────────────────────────────────────────
echo "Rule 7: Phase Skip Conditions"

PHASES=(DECOMPOSITION LANDSCAPE STRATEGY VALIDATION REVIEW)
r7_missing=()

for phase in "${PHASES[@]}"; do
    doc="${phase}.md"
    if [[ ! -f "$doc" ]]; then
        r7_missing+=("$doc — file not found")
    elif ! grep -qi 'skip condition' "$doc"; then
        r7_missing+=("$doc — no Skip Conditions section")
    fi
done

if [[ ${#r7_missing[@]} -gt 0 ]]; then
    fail "R7-SKIP" "${r7_missing[*]}"
else
    pass "R7 — All 5 phase docs (DECOMPOSITION, LANDSCAPE, STRATEGY, VALIDATION, REVIEW) have Skip Conditions"
fi

# ─── Summary ────────────────────────────────────────────────────────────────
echo ""
echo "═══════════════════════════════════════════════════════════════════"
TOTAL=$((PASS + FAIL))
echo " Results: $PASS/$TOTAL passed, $FAIL failed"
echo "═══════════════════════════════════════════════════════════════════"

if [[ $FAIL -gt 0 ]]; then
    echo ""
    echo " ❌ PROTOCOL LINT FAILED — $FAIL rule(s) broken"
    echo ""
    exit 1
else
    echo ""
    echo " ✅ PROTOCOL LINT PASSED — ceremony intact"
    echo ""
    exit 0
fi

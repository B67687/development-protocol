#!/usr/bin/env python3
"""Ledger conformance self-check (Cluster B base + Cluster T Rule 9).

Rule 9 (METHOD_LEDGER.md Emission Rule 9): autonomous-learning decisions
must log a velocity classification; slow-velocity decisions must additionally
log a ratification entry. Missing classification or missing ratification for
a slow decision = omitted = red flag.
"""

import json, os, re, sys

path = sys.argv[1] if len(sys.argv) > 1 else ".omo/method-ledger.jsonl"
bad = 0
omitted = 0
rule9 = 0
steps = {}
qt_re = re.compile(r"^question-tool m\d+")
block_re = re.compile(r"\s+Block\s+[A-Z]+$")
with open(path) as f:
    lines = f.read().strip().split("\n")
for i, line in enumerate(lines, 1):
    try:
        r = json.loads(line)
    except Exception as e:
        print(f"LINE {i}: INVALID JSON: {e}")
        bad += 1
        continue
    missing = [
        k
        for k in ["case", "method", "status", "evidence", "reason", "ts"]
        if k not in r
    ]
    if missing:
        print(f"LINE {i}: missing fields {missing}")
        bad += 1
    if r.get("status") == "omitted":
        omitted += 1
        print(f"LINE {i}: OMITTED RED FLAG: {r.get('method')}")
    ev = r.get("evidence", "")
    if r.get("status") == "applied" and ev:
        for p in ev.split(", "):
            p = p.strip()
            if not p:
                continue
            # Rule 7: dialogue-gated evidence IS the question-tool exchange ref
            if qt_re.match(p):
                continue
            # Section refs: ".omo/protocol-state.md Block M" → path is prefix
            base = block_re.sub("", p)
            if (
                base
                and not os.path.exists(base)
                and not base.startswith("Development-Protocol")
            ):
                print(f"LINE {i}: evidence NOT resolvable: {p}")
                bad += 1
    step = r.get("case", "").split("/")[0]
    steps[step] = steps.get(step, 0) + 1
    # --- Rule 9: trust-boundary conformance (Cluster T) ---
    if r.get("learning") is True:
        v = r.get("velocity")
        if v not in ("fast", "slow", "medium"):
            rule9 += 1
            print(
                f"LINE {i}: RULE 9 RED FLAG: learning entry lacks velocity classification (got {v!r}); fast/slow/medium required"
            )
        elif v == "slow" and not r.get("ratified"):
            rule9 += 1
            print(
                f"LINE {i}: RULE 9 RED FLAG: slow-velocity learning entry lacks ratification entry (need 'ratified' field)"
            )
print("---")
print(
    f"Total entries: {len(lines)}, invalid: {bad}, omitted: {omitted}, rule9: {rule9}"
)
for k in sorted(steps):
    print(f"  {k}: {steps[k]}")
sys.exit(1 if bad or omitted or rule9 else 0)

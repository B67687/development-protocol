#!/usr/bin/env python3
"""
Retrospective Replay — checks if SERIOUSNESS RCF would have killed failed projects.

Usage:
  python replay.py                    # uses fixtures/postmortems.csv
  python replay.py --csv path.csv     # custom CSV

CSV format:
  project,initial_idea,outcome
  MyTool,"I want to build X",shipped
  OldThing,"I want to build Y",failed
  DeadProject,"I want to build Z",killed

RCF base rates (from SERIOUSNESS.md):
  personal_tool: 0.70
  oss_library: 0.40
  production_service: 0.30
  research_prototype: 0.20

RCF adjustment: if initial idea had LOW specificity (vague claims),
RCF penalty applied. The question: "Would RCF have flagged this as
likely to fail before resources were committed?"
"""

import csv
import os
import sys

# RCF base rates by domain
RCF_BASE_RATES = {
    "personal_tool": 0.70,
    "oss_library": 0.40,
    "production_service": 0.30,
    "research_prototype": 0.20,
}

# Vague signals that lower RCF confidence
VAGUE_SIGNALS = [
    "maybe",
    "might",
    "flexible",
    "simple",
    "easy",
    "just",
    "quick",
    "small",
    "prototype",
]

DEFAULT_CSV = os.path.join(
    os.path.dirname(__file__), "..", "..", "fixtures", "postmortems.csv"
)


def classify_domain(project_name: str) -> str:
    """Heuristic domain classification from project name."""
    name_lower = project_name.lower()
    if any(w in name_lower for w in ["api", "service", "pipeline", "deploy", "prod"]):
        return "production_service"
    if any(w in name_lower for w in ["cli", "tool", "lib", "pkg", "npm"]):
        return "oss_library"
    if any(w in name_lower for w in ["research", "proto", "experiment", "lab"]):
        return "research_prototype"
    return "personal_tool"  # default


def rcf_score(idea: str, domain: str) -> dict:
    """
    Compute RCF-style seriousness score.
    Returns dict with base_rate, vagueness_penalty, adjusted_score, would_kill.
    """
    base = RCF_BASE_RATES.get(domain, 0.50)
    vague_count = sum(1 for v in VAGUE_SIGNALS if v in idea.lower())
    penalty = min(vague_count * 0.05, 0.25)  # max 25% penalty
    adjusted = max(0, base - penalty)
    # Kill threshold: < 0.35 means SERIOUSNESS would DROP
    would_kill = adjusted < 0.35
    return {
        "domain": domain,
        "base_rate": base,
        "vagueness_penalty": penalty,
        "adjusted_score": adjusted,
        "would_kill": would_kill,
    }


def replay(csv_path: str) -> list[dict]:
    """Process a postmortems CSV and return replay results."""
    results = []
    with open(csv_path, newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            project = row.get("project", "").strip()
            idea = row.get("initial_idea", "").strip()
            outcome = row.get("outcome", "").strip().lower()
            domain = classify_domain(project)
            rcf = rcf_score(idea, domain)

            # Would RCF have caught this?
            caught = False
            reason = ""
            if outcome in ("failed", "killed"):
                if rcf["would_kill"]:
                    caught = True
                    reason = f"RCF score {rcf['adjusted_score']:.2f} < 0.35 threshold"
                else:
                    reason = f"RCF score {rcf['adjusted_score']:.2f} >= 0.35 — would NOT have killed"
            else:
                reason = f"Shipped — RCF score {rcf['adjusted_score']:.2f}"

            results.append(
                {
                    "project": project,
                    "idea": idea,
                    "outcome": outcome,
                    "domain": domain,
                    "rcf": rcf,
                    "would_have_killed": caught,
                    "reason": reason,
                }
            )
    return results


def print_results(results: list[dict]) -> None:
    """Pretty-print replay results."""
    print("\n" + "=" * 72)
    print("RETROSPECTIVE REPLAY — Would SERIOUSNESS RCF Have Killed?")
    print("=" * 72)
    print(
        f"{'Project':<20} {'Outcome':<10} {'Domain':<20} {'RCF':>6} {'Kill?':>6}  Reason"
    )
    print("-" * 72)
    for r in results:
        rcf = r["rcf"]
        kill_mark = "YES" if r["would_have_killed"] else "NO"
        print(
            f"{r['project']:<20} {r['outcome']:<10} {r['domain']:<20} {rcf['adjusted_score']:>6.2f} {kill_mark:>6}  {r['reason']}"
        )

    # Summary
    failed = [r for r in results if r["outcome"] in ("failed", "killed")]
    caught = [r for r in failed if r["would_have_killed"]]
    missed = [r for r in failed if not r["would_have_killed"]]
    shipped = [r for r in results if r["outcome"] == "shipped"]

    print("\n--- Summary ---")
    print(f"Total projects:     {len(results)}")
    print(f"Failed/Killed:      {len(failed)}")
    print(f"  Would have killed:{len(caught)}")
    print(f"  Would have missed:{len(missed)}")
    print(f"Shipped:            {len(shipped)}")
    if failed:
        recall = len(caught) / len(failed) if failed else 0
        print(f"Kill recall:        {recall:.0%}")
    print("=" * 72)


if __name__ == "__main__":
    csv_path = DEFAULT_CSV
    if len(sys.argv) > 2 and sys.argv[1] == "--csv":
        csv_path = sys.argv[2]

    if not os.path.exists(csv_path):
        print(f"ERROR: CSV not found at {csv_path}")
        print("Create fixtures/postmortems.csv or pass --csv path")
        sys.exit(1)

    results = replay(csv_path)
    print_results(results)

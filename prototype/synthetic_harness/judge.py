#!/usr/bin/env python3
"""
Synthetic Harness Judge — scores output specs on 3 dimensions.

Dims (0-5 each):
  1. Falsifiability: Can the spec be proven wrong? Does it have concrete, testable claims?
  2. One-way door coverage: Does the spec identify irreversible/ costly decisions?
  3. Assumption explicitness: Are assumptions stated vs hidden?

Uses keyword heuristics for mock mode. Real LLM judge is TODO.
"""

import re


# --- Keyword sets for heuristic scoring ---

FALSIFIABLE_SIGNALS = [
    "must",
    "shall",
    "will",
    "measurable",
    "threshold",
    "metric",
    "kpi",
    "target",
    "number",
    "percent",
    "%",
    "days",
    "weeks",
    "before",
    "after",
    "within",
    "success criteria",
    "falsifiable",
    "testable",
    "observable",
    "verifiable",
    "pass/fail",
]

FALSIFIABLE_VAGUE = [
    "maybe",
    "might",
    "could",
    "possibly",
    "hopefully",
    "nice to have",
    "flexible",
    "loosely",
    "vague",
    "unclear",
]

ONEWAYDOOR_SIGNALS = [
    "irreversible",
    "commit",
    "contract",
    "vendor",
    "lock-in",
    "migration",
    "breaking change",
    "backward",
    "permanent",
    "public api",
    "schema",
    "database migration",
    "deploy",
    "one-way",
    "point of no return",
    "can't undo",
    "hard to revert",
]

ASSUMPTION_SIGNALS = [
    "assumption",
    "assuming",
    "belief",
    "hypothesis",
    "we think",
    "we believe",
    "might not",
    "could be wrong",
    "uncertain",
    "if wrong",
    "risk",
    "dependency",
    "depends on",
]

ASSUMPTION_IMPLICIT = [
    "obviously",
    "clearly",
    "of course",
    "everyone knows",
    "it's standard",
    "always",
    "never",
    "impossible",
]


def score_falsifiability(text: str) -> int:
    """Score 0-5: how testable/falsifiable the spec is."""
    lower = text.lower()
    hits = sum(lower.count(s) for s in FALSIFIABLE_SIGNALS)
    vagueness = sum(lower.count(s) for s in FALSIFIABLE_VAGUE)
    # Normalize: count occurrences, cap at 10
    raw = min(hits, 10) * 5 // 10
    penalty = min(vagueness, 3)
    return max(0, min(5, raw - penalty))


def score_onewaydoor(text: str) -> int:
    """Score 0-5: does it identify irreversible decisions?"""
    lower = text.lower()
    hits = sum(lower.count(s) for s in ONEWAYDOOR_SIGNALS)
    # Concrete numbers/deadlines also count as commitment signals
    has_deadline = bool(re.search(r"\b(by|before|after|within)\s+\w+\s+\d", lower))
    has_quant = bool(re.search(r"\d+\s*(hours?|days?|weeks?|months?|%)", lower))
    score = min(hits * 2, 8) + (1 if has_deadline else 0) + (1 if has_quant else 0)
    return min(5, score * 5 // 10)


def score_assumption(text: str) -> int:
    """Score 0-5: how explicitly assumptions are stated."""
    lower = text.lower()
    explicit = sum(lower.count(s) for s in ASSUMPTION_SIGNALS)
    implicit = sum(lower.count(s) for s in ASSUMPTION_IMPLICIT)
    explicit_norm = min(explicit, 10) * 5 // 10
    return max(0, min(5, explicit_norm - implicit))


def judge_spec(text: str) -> dict:
    """
    Score a spec on all 3 dimensions.
    Returns dict with per-dim scores and total.
    """
    f = score_falsifiability(text)
    o = score_onewaydoor(text)
    a = score_assumption(text)
    return {
        "falsifiability": f,
        "one_way_door": o,
        "assumption_explicitness": a,
        "total": f + o + a,  # max 15
    }


def judge_pair(baseline: str, protocol: str) -> dict:
    """Judge both outputs and declare a winner."""
    b_scores = judge_spec(baseline)
    p_scores = judge_spec(protocol)
    winner = "protocol" if p_scores["total"] >= b_scores["total"] else "baseline"
    if p_scores["total"] == b_scores["total"]:
        winner = "tie"
    return {
        "baseline": b_scores,
        "protocol": p_scores,
        "winner": winner,
    }


if __name__ == "__main__":
    # Quick smoke test
    sample_bad = "I want to build something flexible that might help maybe"
    sample_good = "I must ship a deployment tool by Friday that reduces deploy time from 45min to 10min, assuming our AWS config stays the same. This is irreversible once we commit to the schema."
    print("Bad spec:", judge_spec(sample_bad))
    print("Good spec:", judge_spec(sample_good))
    print("Winner:", judge_pair(sample_bad, sample_good))

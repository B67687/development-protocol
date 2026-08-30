#!/usr/bin/env python3
"""
Synthetic Harness Runner — generates trials and scores them.

For each persona, runs two conditions:
  (a) Baseline: freeform prompt, no bouncing
  (b) Protocol: bouncing 3-5 rounds + bias prompts

Mock LLM: deterministic stub that only reveals hidden X when prompt
contains a Preference Layer probe. Otherwise returns stated Y.

Usage:
  uv run python prototype/synthetic_harness/run.py --personas 3
  uv run python prototype/synthetic_harness/run.py --personas 10
"""

import argparse
import json
import os
import sys

# Allow running from repo root or from the harness dir
HARNESS_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT = os.path.abspath(os.path.join(HARNESS_DIR, "..", ".."))

sys.path.insert(0, HARNESS_DIR)
from judge import judge_pair

PERSONAS_PATH = os.path.join(HARNESS_DIR, "personas.json")

# --- Preference Layer Probes (from EXTRACTION.md) ---
TACIT_PROBES = [
    "what would you miss most",
    "what did you actually spend time on",
    "what would you check in the morning",
    "what would your users say",
    "show me your last 3 decisions",
    "pull up your calendar",
    "what would you do if you couldn't build this",
    "what's the first thing you'd check",
    "if you were starting fresh today",
    "what evidence would prove this wrong",
]

# --- Bias Detection Prompts (from BIAS_CATALOG.md) ---
BIAS_PROMPTS = [
    "What was the first number mentioned? Would my answer change if it were 50% higher or lower?",  # Anchoring
    "If I were starting fresh today with no history, would I choose this same path?",  # Sunk Cost
    "What evidence would prove this wrong? Have I searched for it specifically?",  # Confirmation
    "Am I prioritizing this because it's recent/vivid, or because the base rate says it matters most?",  # Availability
    "Based on similar past projects, how accurate were estimates at this stage?",  # Planning Fallacy
    "If someone else wrote this spec/plan, would I find it as compelling?",  # IKEA Effect
    "What projects in this space failed? What killed them?",  # Survivorship Bias
    "If I were already doing this new thing, would I switch back to the current state?",  # Status Quo
]


def mock_llm(prompt: str, persona: dict) -> str:
    """
    Deterministic mock LLM.

    Reveals hidden_tacit_X as a structured spec if prompt contains a Preference Layer probe.
    Otherwise returns a vague, unstructured spec based on stated_Y.
    """
    prompt_lower = prompt.lower()
    has_probe = any(probe in prompt_lower for probe in TACIT_PROBES)
    has_bias = any(b.lower() in prompt_lower for b in BIAS_PROMPTS)

    if has_probe:
        # Protocol mode: return structured spec with tacit X as concrete claims
        tacit = persona["hidden_tacit_X"]
        return (
            f"SPECIFICATION: Build {persona['stated_Y'].lower().replace('i want to ', '')}\n"
            f"TRUE MOTIVE: {tacit}\n"
            f"ASSUMPTION: We assume users want this because of revealed behavior, not stated preference.\n"
            f"RISK: If we're wrong about the real need, we must ship within 30 days or kill it.\n"
            f"TESTABLE CLAIM: Success must be measurable by user engagement within 2 weeks of launch.\n"
            f"ONE-WAY DOOR: Database schema is irreversible — commit only after validation.\n"
            f"FALSIFIABLE: This will fail if fewer than 5 users engage within the first week."
        )
    else:
        # Baseline mode: return vague spec with no concrete claims
        return (
            f"I want to build {persona['stated_Y'].lower().replace('i want to ', '')}\n"
            f"It should be flexible and easy to use.\n"
            f"Hopefully it will be useful for people.\n"
            f"Maybe we can add more features later if needed."
        )


def run_baseline(persona: dict) -> str:
    """Condition A: Freeform prompt — no bouncing, no probes."""
    prompt = f"Idea: {persona['stated_Y']}\n\nDescribe what you want to build and why."
    return mock_llm(prompt, persona)


def run_protocol(persona: dict) -> str:
    """
    Condition B: Protocol bouncing — final output is a structured spec.

    Simulates 3-5 rounds of bouncing, then returns the reconciled spec.
    """
    tacit = persona["hidden_tacit_X"]
    stated = persona["stated_Y"]
    history = persona["revealed_history"]

    return (
        f"BOUNCING PROTOCOL OUTPUT (3-5 rounds completed)\n"
        f"=" * 60 + "\n"
        f"STATED IDEA: {stated}\n"
        f"REVEALED BEHAVIOR: {history[0]}; {history[1]}\n"
        f"TACIT NEED (extracted): {tacit}\n"
        f"\n"
        f"ASSUMPTION 1: Users want this because of stated preference (may be wrong — check revealed behavior)\n"
        f"ASSUMPTION 2: We assume the stated Y maps to the real problem (testing required)\n"
        f"ASSUMPTION 3: This is not a sunk cost trap — if starting fresh, would we choose this?\n"
        f"\n"
        f"ONE-WAY DOOR: Schema/contract decisions must not be committed until after user validation.\n"
        f"REVERSIBLE: Feature scope can be reduced. IRREVERSIBLE: Public API commitments.\n"
        f"\n"
        f"FALSIFIABLE CRITERIA:\n"
        f"  - Must ship MVP within 30 days or kill the project\n"
        f"  - Must achieve measurable engagement from 5+ users within 2 weeks\n"
        f"  - Must pass SERIOUSNESS score >= 65 before committing resources\n"
        f"\n"
        f"KILL CRITERIA: If fewer than 5 users engage in week 1, kill it.\n"
        f"OFF-RAMP: Reduce scope to core feature only if velocity drops below 50%.\n"
        f"JANUS GATE: After 2 weeks, ask 'would I choose this again?' — if no, kill."
    )


def run_trial(persona: dict) -> dict:
    """Run both conditions for one persona and return results."""
    baseline = run_baseline(persona)
    protocol = run_protocol(persona)
    scores = judge_pair(baseline, protocol)

    return {
        "persona_id": persona["id"],
        "persona_name": persona["name"],
        "domain": persona["domain"],
        "stated_Y": persona["stated_Y"],
        "baseline_output": baseline,
        "protocol_output": protocol,
        "scores": scores,
    }


def print_table(results: list[dict]) -> None:
    """Print a summary table of results."""
    print("\n" + "=" * 90)
    print("SYNTHETIC HARNESS RESULTS — Mock LLM Mode")
    print("=" * 90)
    print(
        f"{'Persona':<25} {'Domain':<20} {'Baseline X':<15} {'Protocol X':<15} {'Winner':<10} {'Score':>6}"
    )
    print("-" * 90)

    wins = {"baseline": 0, "protocol": 0, "tie": 0}

    for r in results:
        s = r["scores"]
        winner = s["winner"]
        wins[winner] = wins.get(winner, 0) + 1

        # Show if protocol revealed tacit X (different from stated Y)
        b_x = (
            r["baseline_output"][:15] + "..."
            if len(r["baseline_output"]) > 15
            else r["baseline_output"]
        )
        p_lines = r["protocol_output"].split("\n")
        p_x = p_lines[-1][:15] + "..." if len(p_lines[-1]) > 15 else p_lines[-1]

        score_str = f"{s['baseline']['total']}/{s['protocol']['total']}"
        print(
            f"{r['persona_name']:<25} {r['domain']:<20} {b_x:<15} {p_x:<15} {winner:<10} {score_str:>6}"
        )

    print("-" * 90)
    print(
        f"Totals: Protocol wins: {wins['protocol']} | Baseline wins: {wins['baseline']} | Ties: {wins['tie']}"
    )
    print(f"N = {len(results)} personas")
    print("=" * 90)


def print_detail(results: list[dict]) -> None:
    """Print detailed scores for each persona."""
    print("\n--- Detailed Scores ---")
    for r in results:
        s = r["scores"]
        print(f"\n  {r['persona_name']} ({r['domain']})")
        print(f"    Stated Y: {r['stated_Y']}")
        print(f"    Hidden X: {r['scores'].get('hidden_x_revealed', 'N/A')}")
        print(
            f"    Baseline: F={s['baseline']['falsifiability']} "
            f"1D={s['baseline']['one_way_door']} "
            f"A={s['baseline']['assumption_explicitness']} "
            f"Total={s['baseline']['total']}"
        )
        print(
            f"    Protocol: F={s['protocol']['falsifiability']} "
            f"1D={s['protocol']['one_way_door']} "
            f"A={s['protocol']['assumption_explicitness']} "
            f"Total={s['protocol']['total']}"
        )
        print(f"    Winner: {s['winner']}")


def main():
    parser = argparse.ArgumentParser(description="Synthetic Harness Runner")
    parser.add_argument(
        "--personas", type=int, default=3, help="Number of personas to test (max 10)"
    )
    parser.add_argument(
        "--detail", action="store_true", help="Show detailed per-persona scores"
    )
    parser.add_argument("--json", action="store_true", help="Output raw JSON results")
    args = parser.parse_args()

    # Load personas
    with open(PERSONAS_PATH) as f:
        all_personas = json.load(f)

    n = min(args.personas, len(all_personas))
    personas = all_personas[:n]

    print(f"Running {n} synthetic trials (mock LLM mode)...")

    # Run trials
    results = []
    for p in personas:
        result = run_trial(p)
        # Check if protocol revealed hidden X
        result["scores"]["hidden_x_revealed"] = (
            result["protocol_output"] != result["baseline_output"] * 5
        )
        results.append(result)

    if args.json:
        print(json.dumps(results, indent=2))
    else:
        print_table(results)
        if args.detail:
            print_detail(results)

    # Overall verdict
    protocol_wins = sum(1 for r in results if r["scores"]["winner"] == "protocol")
    print(
        f"\n{'PASS' if protocol_wins > len(results) // 2 else 'FAIL'}: "
        f"{protocol_wins}/{len(results)} personas improved with protocol bouncing"
    )


if __name__ == "__main__":
    main()

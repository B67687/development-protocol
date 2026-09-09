# Lessons Directory

Cross-project failure captures that feed back into the Development Protocol.

## Purpose

This directory stores structured failure captures — documented lessons from real projects that improve the protocol over time. Each capture follows the template in [FAILURE_CAPTURE.md](../steps/FAILURE_CAPTURE.md).

## How It Works

1. **Failure detected** → Create a capture using the template from `../steps/FAILURE_CAPTURE.md`
2. **Save here** → File named `FC-[YYYY]-[###]-[short-name].md`
3. **Feed back** → Match to existing FP-### patterns or add new ones to §11
4. **Session kickoff** → Load recent captures for pattern recognition (§12)

## File Naming Convention

```
FC-[YYYY]-[###]-[short-name].md
```

- **YYYY** — Year the failure was captured
- **###** — Sequential number within the year (001, 002, etc.)
- **short-name** — Brief descriptive slug (e.g., `scope-creep-notifications`, `tautological-tests`)

Examples:

- `FC-2026-001-scope-creep-notifications.md`
- `FC-2026-002-tautological-tests.md`
- `FC-2026-003-phase-drift-distribute.md`

## What Goes Here

- Failures detected during any protocol phase
- Protocol gaps identified through real-world use
- Pattern matches to existing FP-### entries
- New failure patterns not yet in the catalog

## What Does NOT Go Here

- Success stories (those go in `.omo/reflect.md` or project retrospectives)
- Protocol changes (those go in §10 Evolution & Phase Exit)
- Method ledger entries (those go in `.omo/method-ledger.jsonl`)

## Session Kickoff Integration

At session start (§12), the AI loads recent failure captures to check:

- "Have we seen this pattern before?"
- "What happened last time?"
- "Should we apply the prevention strategy?"

This closes the learning loop: failures are not forgotten but reused as pattern recognition inputs.

## Relationship to Other Artifacts

| Artifact         | Tracks                              | Location                      |
| ---------------- | ----------------------------------- | ----------------------------- |
| Method Ledger    | What methods were applied           | `.omo/method-ledger.jsonl`    |
| Failure Captures | What went wrong and what we learned | `lessons/FC-*.md`             |
| Shift Log        | Goalpost shifts and learning        | `.omo/shift-log.md`           |
| Outcome Verdicts | Cluster completion verdicts         | `.omo/outcome-verdicts.jsonl` |

Together, these form the protocol's memory — successes, failures, shifts, and verdicts that make each project cycle stronger than the last.

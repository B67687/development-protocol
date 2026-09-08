# KILL_LOG — Instrumented Kill Tracking

> Every KILL decision is a prediction. This log makes the prediction testable.

## Purpose

When SERIOUSNESS kills an idea (DROP) or VALIDATION kills a path (spike fails), we log it with a counterfactual check. After 10+ decisions, we can calibrate: are we killing the right things?

## Template

| ID    | Date       | Phase Killed | Class              | Score       | Kill Reason                                                       | Counterfactual (30-day check)                                                                         | Outcome        |
| ----- | ---------- | ------------ | ------------------ | ----------- | ----------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- | -------------- |
| K-001 | 2026-08-29 | SERIOUSNESS  | Personal tool      | 28/120 DROP | Energy low, no revealed demand                                    | Did need persist? Did someone else build it?                                                          | Correct / Miss |
| K-002 | 2026-08-29 | VALIDATION   | Production service | spike KILL  | Core assumption falsified                                         | Did assumption become true later?                                                                     | —              |
| K-003 | 2026-09-03 | P2b WHICH-X? | Protocol seam      | Bar 2       | 1 same (narrow 85→100 only) — not chosen; 3 adjacent — not chosen | 30d 2026-10-03: did seam choice hurt transmissibility? Check docs/traces/colour-blind-85-100.md usage | Pending        |

**Field notes:**

- **ID:** Auto-increment `K-NNN`
- **Phase Killed:** SERIOUSNESS / VALIDATION / Preference Kill
- **Class:** Personal tool / Open-source library / Production service / Research prototype (from SERIOUSNESS § Calibration)
- **Score:** SERIOUSNESS score + gate (e.g. `28/120 DROP`)
- **Kill Reason:** One line — which criterion fired (preference or technical matrix)
- **Counterfactual:** Set a 30-day reminder. At 30 days ask: "Did anything happen with this idea? Did someone else build it? Did the need go away?"
- **Outcome:** `Correct` (right to kill), `Miss` (should have kept), `Pending` (not yet checked)

## Counterfactual Tracking

When a project is KILLED, set a calendar reminder at **30 days**. At the check:

1. Did the need persist, grow, or vanish?
2. Did someone else ship what we killed?
3. Would starting it today still fail the same gate?

Record the answer in **Counterfactual** and set **Outcome**.

## Calibration Feedback Loop

After **10+ kill decisions**:

- **>80% correct kills** → thresholds calibrated
- **<70% correct** → thresholds too aggressive (killing good ideas) — loosen by ~10 points
- **>90% correct** → thresholds too conservative (not filtering enough) — tighten

Review at REFLECT Q8 "Kill-Gate Calibration" and update SERIOUSNESS thresholds if needed.

## Integration

- Written by: SERIOUSNESS.md (DROP) and VALIDATION.md (spike KILL / Preference Kill)
- Read by: REFLECT.md Q8 — "Review the kill decisions made this cycle. Were the right things killed?"
- Decision Journal entry in SERIOUSNESS.md § Calibration & Evidence feeds INTO this log.

## Provenance

| Aspect                       | Source                                             |
| ---------------------------- | -------------------------------------------------- |
| Reference-class + base rates | Kahneman & Lovallo (1993) — inside vs outside view |
| Premortem                    | Klein (2007) — prospective hindsight               |
| Calibration target           | Tetlock (2005) — expert judgment calibration       |

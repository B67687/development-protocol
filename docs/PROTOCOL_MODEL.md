# PROTOCOL_MODEL.md — The Development Protocol's Own State Machine

> **Purpose:** The Development Protocol applies its own Project Health rule to itself. This document is the protocol's state machine — every step is a state, every arrow is a valid transition, and the invariants below are what must never change.
>
> **Why this exists:** The protocol mandates that every project document its whole-project state machine (see SPECIFICATION.md §2). This is that mandate applied to the protocol itself. Adding or removing a step means updating this transition table — making the change explicit instead of silently breaking the pipeline's invariants (the orphaned-reference cascade that cost us a full REVIEW cycle).

## States

The protocol's states are its pipeline steps:

```
INBOX → PRIORITIZE(opt) → EXTRACTION → SERIOUSNESS → FUNDAMENTALS (incl. MULTI) → DECOMPOSITION → AMBITION (incl. PACING) → LANDSCAPE → STRATEGY → VALIDATION → SPECIFICATION → EXECUTOR → REVIEW (incl. EXPLAINER) → REFLECT → SHIP
```

| State | Meaning |
|---|---|
| `INBOX` | Run-Shape Selector triage first (Light/Standard/Brownfield — see INBOX.md); raw thoughts captured, clustered, one cluster selected; mode + any reanchor ratification logged to method ledger |
| `PRIORITIZE` (optional) | 2-10 ideas compared on Want/Know/Work/Matters (4-dimension), What-Matters Check picks one bet — AI proposal + user ratification both logged |
| `EXTRACTION` | X (real problem) extracted from Y (stated request) |
| `SERIOUSNESS` | Commitment gate — is X worth pursuing? (D2b proven-earner benchmark fires when the money-tier decision is live) |
| `FUNDAMENTALS` | One-way doors, LLM bias, capability audit, chain analysis + multidisciplinary probes (MULTI folded) |
| `DECOMPOSITION` | MECE tree, Cynefin, Level of Care |
| `AMBITION` | 5-round research-interleaved goal tightening + phase budget allocation (PACING folded) |
| `LANDSCAPE` | Structured research — Declared Coverage declared up front; Intuition-First Route standing mode |
| `STRATEGY` | Strategic ratification gate — AI kernel proposal, human ratify, premortem |
| `VALIDATION` | Prototyping gate — KILL/PIVOT/COMMIT; Sufficiency Checkpoint ratifies ship/defer before COMMIT |
| `SPECIFICATION` | 16-section spec template (§0-15) |
| `EXECUTOR` | Implementation (incl. FINISH gate) |
| `REVIEW` | Independent meta-review (incl. EXPLAINER generation + Spec-to-Code Fidelity Check + Method Conformance Check) |
| `REFLECT` | Protocol retrospective (7 questions) |
| `SHIP` | Delivery |

## Valid Transitions

### Standard path (forward, step-to-next-step)
```
INBOX → PRIORITIZE → EXTRACTION → SERIOUSNESS → FUNDAMENTALS (incl. MULTI) → DECOMPOSITION → AMBITION (incl. PACING) → LANDSCAPE → STRATEGY → VALIDATION → SPECIFICATION → EXECUTOR → REVIEW (incl. EXPLAINER) → REFLECT → SHIP
```

### Valid deviations

| Transition | When valid |
|---|---|
| `INBOX → EXTRACTION` (skip PRIORITIZE) | Single cluster, clear winner, no tie — PRIORITIZE is optional by design |
| Any step → `EXTRACTION` (loop back) | Paradigm Review triggered (3+ assumptions failed), or EXTRACTION found to be incomplete |
| `VALIDATION → AMBITION` | Prototype revealed goal was wrong — ambition must be re-tightened |
| `VALIDATION → EXTRACTION` | Prototype revealed wrong problem — re-extract |
| `VALIDATION → LANDSCAPE` | Prototype showed feasibility unknowns — more research needed |
| `SPECIFICATION → LANDSCAPE` | Spec-writing exposed research gaps |
| `EXECUTOR → SPECIFICATION` | Implementation revealed spec flaws (Midpoint Protocol Check) |
| `EXECUTOR → EXTRACTION` | Paradigm Review reframe path |
| `REVIEW → EXECUTOR` | Review failed — rework implementation |
| `REVIEW → EXTRACTION` | Review found wrong problem extracted |
| `REFLECT → EXTRACTION` | Learning shifts — protocol itself changed, new cycle |
| Any step → `INBOX` | Protocol improvement cycle (recursion) or new intent |
| `EXTRACTION → FUNDAMENTALS` (skip SERIOUSNESS) | Protocol improvement cycle ONLY (per Invariant 4); FUNDAMENTALS (incl. MULTI) still required (per Invariant 5) |
| **Any step → any EARLIER step (Gate Restart)** | Deliberate divergence restart: the user may always jump back to any previous gate and restart from there — no failure trigger required. Divergence is a valid reason to restart from an earlier gate when continuing would compound the divergence. |

### Invalid transitions
```
PRIORITIZE → anything except EXTRACTION
EXTRACTION → anything except SERIOUSNESS (when NOT a protocol-improvement recursion; recursion per Invariant 4)
EXECUTOR → SHIP (must pass through REVIEW, REFLECT)
REVIEW → SHIP (must pass through REFLECT)
SERIOUSNESS → SPECIFICATION (skipping FUNDAMENTALS (incl. MULTI), DECOMPOSITION, AMBITION (incl. PACING), LANDSCAPE, VALIDATION)
```
> These are invariants: **no project ships without passing REVIEW and REFLECT. No project skips from extraction to specification without the middle gates.**

### Gate Restart Procedure

When the user declares a divergence and chooses to restart from an earlier gate:

1. **Name the restart gate** — "restart from AMBITION" (must be an EARLIER gate, never a later one).
2. **Preserve context** — the divergence itself is data. Write a short divergence note (what diverged, why, what the restart gate needs to know) into `.omo/` before restarting.
3. **Invalidate downstream** — artifacts produced after the restart gate are provisional. They stay on disk but are flagged: `[SUPERSEDED by restart from <gate>]`.
4. **Re-enter cleanly** — run the restart gate fresh. If restarting to EXTRACTION or AMBITION, prior conclusions do not bind the new pass.
5. **Log the transition** — add the restart to this file's transition history so the pattern is visible.

> Restart is CHEAPER than continuing with compounded divergence. The cost is re-running one gate (minutes to hours) vs. the compounding cost of building on a wrong foundation (days to weeks). This is the same economics as FUNDAMENTALS' one-way door validation — fail early, restart cheap.

## Invariants (What Must Never Change)

1. **Every project passes through REVIEW and REFLECT before SHIP.** No exceptions, no bypasses.
2. **The prep-sequence is order-preserved** when steps are used: INBOX before EXTRACTION, EXTRACTION before FUNDAMENTALS, FUNDAMENTALS before DECOMPOSITION, DECOMPOSITION before AMBITION, LANDSCAPE before VALIDATION, VALIDATION before SPECIFICATION, SPECIFICATION before EXECUTOR.
3. **PRIORITIZE is always optional** — never mandatory, never blocking.
4. **Recursion exemption is reserved for protocol improvement cycles only** — SERIOUSNESS may be skipped there and nowhere else.
5. **Skipping SERIOUSNESS in protocol-improvement cycles does not skip FUNDAMENTALS (incl. MULTI).**
6. **The pipeline can loop backward (to EXTRACTION, LANDSCAPE, SPECIFICATION) but never jumps forward past a gate.** Run-Shape Light mode may skip heavyweight steps (per INBOX.md) — sanctioned by the front-door triage, not a gate jump; skipped steps are logged with SKIP_CATALOG codes, never silent.
9. **Run-Shape mode is chosen at INBOX and logged** — Light mode skips steps legitimately but NEVER skips ledger obligations, the verification floor (review + targeted tests), or one-way doors (schema/API/data/security force Standard).
10. **Autonomous learning is gated by feedback velocity** — the model may learn-and-apply without ratification ONLY where feedback is machine-checkable (compiler, tests, ledger, grep = `fast`). Where feedback is slow (strategy, taste, ambition, mission) ratification precedes application. Ambiguous cases default to ratification. The gate governs declarative learning only; procedural skill remains user-owned (Dual ZPD). Velocity classification is logged per autonomous-learning decision (Emission Rule 9).

11. **One ratification per run (asymmetric gate default).** The STRATEGY ratification is the single user gate of a run: AMBITION's locked scope + the strategic kernel + the phase budget are ratified together, in one interaction. Every other user confirmation — EXTRACTION X, DECOMPOSITION tree, SERIOUSNESS COMMIT/SCHEDULE, VALIDATION COMMIT and Sufficiency Checkpoint, PRIORITIZE What-Matters — is default-autonomous and ledger-logged; any step automatically re-engages the user gate iff it crosses a one-way door (schema/API/data/security or expensive-to-reverse), and DROP/KILL outcomes always re-engage the user. This invariant governs how many times the user is asked, not whether compliance is checked — the REVIEW Method Conformance Check still catches silent skips. Extends the asymmetric-ratify principle of Invariant 10 to the prep-sequence.

### Design Principle: Sentinels, not just Gates
A gate fires when you ARRIVE at a step; a sentinel fires when you DON'T. Every user-facing gate can be skipped (friction economics) or rubber-stamped (93% automation bias) — so gates are not enforcement. The protocol's real sentinel is the **method-ledger conformance check at REVIEW**: REVIEW blocks SHIP if any step's entry/exit artifact is missing, a transition was not logged, or a skipped step has no SKIP_CATALOG code. An unlogged transition IS an invariant violation. This is the deterministic sentinel that exists today; harness-side hooks (Stop/pre-tool completion gates, deny-and-continue — OpenCode; see harness handoff) upgrade it from review-time to real-time enforcement. Until then, "REVIEW blocks on conformance" is the teeth.

## Adding or Removing a Step (Transition Update Procedure)

When a step is added, removed, renamed, or reordered:

1. Update this file's **States** table
2. Update this file's **Valid Transitions** — add/remove the state's arrows
3. Check the **Invariants** — does the change violate any? (e.g., adding a step after SHIP breaks invariant 1)
4. Update the pipeline diagram in **README.md**
5. Grep all protocol files for stale references to the old step (`grep -rn "OLDSTEP" *.md`)
6. If this is a NEW step: does it need a transition-table test (a checklist in the step file asserting valid entry/exit conditions)?

> The failure mode this prevents: adding PRIORITIZE caused orphaned references across 12 files. With this procedure, step changes become a documented transition, not a regression cascade.

## Test

The transition-table test for the protocol itself:

- [ ] Pipeline diagram in README.md matches the States table here
- [ ] Every step file's Integration section references its correct neighbors
- [ ] No orphaned references to removed/renamed steps anywhere in `*.md`
- [ ] Prep-sequence (README.md) matches this transition table
- [ ] Invariants hold: no path skips REVIEW → REFLECT → SHIP chain
- [ ] Ledger conformance: every transition in this run is logged; unlogged transition = sentinel violation (Design Principle above)

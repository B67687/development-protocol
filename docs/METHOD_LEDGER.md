# METHOD_LEDGER.md — Method Invocation Completeness

> **Purpose:** Close the documented-vs-applied gap. Every documented method in the
> protocol is either **applied** (with an evidence artifact) or **skipped** (with a
> pre-authorized catalog reason). Nothing is silently omitted.
>
> **Why this exists:** The executor (AI) uses a subset of documented methods by
> default. Research (2026-08-01) across checklist discipline (WHO, aviation,
> medicine), skip-with-reason patterns (aviation MEL, NASA waivers), process-mining
> conformance checking, and AI instruction-following (AGENTIF) confirms: the fix
> must be **structural, not exhortational** — a machine-checkable accounting
> system, not a culture pledge.

## The Ledger

A single structured file, `.omo/method-ledger.jsonl`, one JSON record per method
decision. Emitted as a **byproduct of execution** — the executor already reasons;
it just tags. Recorded **at the point of decision** (retrospective recording is
unsafe: the trauma data shows 16% false checks).

```jsonl
{"case":"EXTRACTION/2026-08-01/project-a","method":"Laddering","status":"applied","evidence":".omo/plans/extraction-output.md","ts":"2026-08-01T10:30:00Z"}
{"case":"EXTRACTION/2026-08-01/project-a","method":"Cognitive Interview","status":"skipped","reason":"CAT-EC-10:no-specific-incident","ts":"2026-08-01T10:31:00Z"}
{"case":"EXTRACTION/2026-08-01/project-a","method":"No-Computer-Check","status":"omitted","reason":null,"ts":"2026-08-01T10:32:00Z"}
```

## The Three States

| State | Meaning | Evidence required |
|---|---|---|
| **applied** | Method was used | The actual output artifact (file path). A checkmark alone is a false check — `applied` requires the artifact to exist. |
| **skipped** | Legitimately skipped per a pre-authorized catalog code | The catalog code (e.g., `CAT-EC-10`). Must be in the Skip Catalog. Expires. |
| **omitted** | Skipped with NO catalog code | **This is noncompliance.** The audit's red flag. |

## Emission Rules

1. **Every method decision is logged.** No method is used without a ledger entry.
2. **`applied` requires evidence** — the actual output, not a checkmark (challenge-response, not do-list — Degani/Wiener).
3. **`skipped` requires a catalog code** — from the Skip Catalog's closed list of legitimate conditions. A reason outside the catalog requires a deviation record (NASA waiver template) with independent approval via REVIEW.
4. **`omitted` is a red flag** — surfaces in the conformance check.
5. **Skips expire** (NASA waiver time-limit) — a skip valid for problem A never propagates silently to problem B. Re-justify or remediate.
6. **Skipping must be structurally harder than applying** — if justification effort > execution effort, the honest equilibrium favors applying.

7. **Dialogue-gated methods log BOTH halves** — where a method requires a user ratification (What-Matters Check in PRIORITIZE.md, Sufficiency Checkpoint in VALIDATION.md), the AI proposal AND the user's answer are both logged. Missing ratification entry = omitted state = red flag at REVIEW.

8. **Insertion discipline** — when editing protocol step files, verify insert anchors by reading the region after EVERY edit batch (hashline anchors drift; an insert can clobber an adjacent block). A `cargo check`/grep pass after each batch is the mechanical guard.

9. **Trust-boundary conformance (feedback-velocity gate)** — every autonomous-learning decision (field `learning: true`) logs a velocity classification (`fast`/`slow`/`medium`, per PROTOCOL_MODEL Invariant 10). A `slow`-velocity decision requires a ratification entry (`ratified` field). Missing velocity classification, or missing ratification on a slow decision, = omitted state = red flag at REVIEW. Machine-checked by `ledger-check.py`.

10. **Resolution gate** — every run's lessons get a disposition: `tracked practice change (owner)` | `consciously deferred`. Recording without a resolution gate decays into recurring opinions (Loeffler ESEM 2017) — a lesson with no disposition is `omitted` state. Applying a tracked practice change is an autonomous-learning decision — rule 9 velocity classification and slow-velocity ratification apply at apply time.

11. **Outcome verdict (per cluster)** — a verdict record `{"case", "what_worked": [], "what_didnt": [], "disposition", "owner", "reviewed_ts"}` written to `.omo/outcome-verdicts.jsonl` by the independent REVIEW agent (author + time separation from the run — contamination guard). The run itself records facts only; a self-authored verdict is `omitted` state (reward-hacking surface, METR 2025). Verdicts are TIME-SEPARATED from the eval, and never acted on as a single-cycle delta (small-N regime — aggregate over many cycles).

## The Self-Critique Reconciliation

At each gate (per step completion), the executor writes a mandatory reconciliation
pass — explicitly listing, for each method in scope: **applied-with-evidence /
skipped-catalog-code / omitted**. This is the Constitutional-AI pattern: the
critique *step itself* (not just the revision) drives correction.

## The Conformance Check (runs at REVIEW)

At each REVIEW meta-gate, the ledger is machine-checked:

- **Fitness** — were the prescribed methods invoked with resolvable evidence?
  (Process-mining conformance checking; empty traceability cells per NASA SWE-072.)
- **Skip-rate per reason code** — a single code trending up = catalog too loose
  (the "creeping exceptions" failure mode).
- **Divergence metric** — invocation rate vs outcome quality. Never reward
  checklist completion (specification gaming, Krakovna); divergence is the alarm.
- **Rules 10/11 conformance** — every completed cluster has a verdict record with a disposition (rule 11) and every lesson carries a disposition (rule 10); a deferred lesson has a re-review trigger. Absent verdict or disposition-less lesson = `omitted` state = red flag.

## Integration

The ledger is referenced by:
- **REVIEW.md** — conformance check at the meta-gate
- **README.md** — listed as a standing artifact
- **Every technique-documenting file** — each gains a compact numbered manifest
  (manifest-first, per AGENTIF: >6000 words is an instruction-following failure)

## Provenance

| Element | Source | Evidence |
|---|---|---|
| Checklist effectiveness depends on compliance | WHO checklist (Haynes 2009): mortality 1.5%→0.8%; Ontario (Urbach 2014): no effect when rubber-stamped; van Klei 2012: OR 0.44 full vs 1.16 noncompliance | High (clinical studies) |
| Challenge-response vs do-list | Degani & Wiener, Human Factors 1993; FAA/NASA report | High |
| Skip catalog (pre-authorized conditions) | Aviation MEL (FAA AC 120-125), 50+ years | High |
| Expiring waivers | NASA NPR 1400.1 waiver/deviation process | High |
| Conformance checking | Process mining (van der Aalst); NASA SWE-072 traceability | High |
| Stepwise verification | OpenAI process supervision | Medium-High |
| Manifest-first | AGENTIF (instruction-following beyond 6000 words) | Medium-High |
| Critique-revision loop | Constitutional AI (Bai et al. 2022) | Medium-High |
| Specification gaming warning | Krakovna et al., DeepMind | High |

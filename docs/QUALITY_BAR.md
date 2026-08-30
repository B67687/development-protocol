# QUALITY_BAR.md — Per-Project Quality Contract

> **Purpose:** A finite, checkable quality contract per project. Chosen ONCE at AMBITION (executor-proposed, user-ratified at STRATEGY as part of the scope — never executor-self-selected), verified against at REVIEW.

> **Why:** Every mature domain converges on a small set of NAMED tiers chosen up front by risk judgment, then verified against the chosen tier (ASVS L1/L2/L3, NIST SP 800-53B low/moderate/high, FedRAMP Low/Moderate/High, Microsoft Public/General/Confidential/Highly-Confidential). One-size-fits-all fails (DIHK: 77% report great/extreme effort); a gate with high compliance but no teeth is a rubber stamp (Ontario checklist null result, NEJM 2014 Urbach — 92-98% self-reported compliance, no mortality reduction). **Tier is RISK, not size.** Tier selection is a documented judgment, not a mechanical rule — data-type→tier shortcuts were removed from ASVS v4→v5 because self-selection systematically under-assures.

## The Two Named Profiles

| Dimension | Profile A — Internal / no PII | Profile B — Customer-facing / financial |
|---|---|---|
| Security tier | ASVS L1 | ASVS L3-level (or NIST SP 800-53B Moderate) |
| Privacy tier | General — DPIA screen <2 criteria → documented no-DPIA | Highly Confidential + full DPIA + high-end Art 25 |
| Engineering gate | Short DoD: tests pass · reviewed · diff ≤400 LOC · no self-approve; Google directional bar ("approve once it definitely improves overall code health") | Hard review gate: named accountable reviewer with blocking power, no self-approve, mandatory risk-routing for security diffs, revert telemetry |
| Assurance level | Minimal — internal blast radius, reversible | High — audit trail + external evidence |
| Typical rationale | "No sensitive data, internal blast radius, reversible." | "Financial data, external exposure, regulatory floor." |

## Selection

Profile selection is **executor-proposed, user-ratified at STRATEGY** — never executor-self-selected (ASVS v5 removed data-type shortcuts because self-selection systematically under-assures). The AMBITION Round-5 read-back carries the proposal: "**Quality bar (proposed):** [Profile A | Profile B] — one-paragraph risk rationale." A rationale consisting solely of the template phrases above = RED FLAG at REVIEW.

## Enforcement (hard consequence + verifiable evidence)

- **Hard consequence:** shipping (SHIP gate / merge) is BLOCKED until the bar is met. A gate that cannot fail is theater.
- **Verifiable evidence, not self-report:** self-reported checkmarks are inadmissible (Ontario null result). Evidence: profile + rationale in the AMBITION read-back; diff-size computed from this run's git diff vs the last review; no-self-approve and named-reviewer attested in the ledger.
- **Structural anti-habituation safeguards** (habituation decays review depth: approval 30.1%→36.8% over 7 months, arXiv:2606.22721; 83% of >1000-LOC PRs ship with zero formal review): diff-size caps, named accountable reviewer, risk-routing of security diffs, revert telemetry.

## Definitions

- **Revert telemetry:** a per-change revert/release record — which change went out, when, and whether it was reverted or shipped clean — reviewed at the batched-and-rare review. The signal that catches silently-laundered regressions.
- **Google directional bar:** the one-question review test — "would this change definitely improve overall code health?" Approve only on a definite yes.

## Provenance

| Element | Source |
|---|---|
| Named risk tiers chosen up front | ASVS, NIST SP 800-53B, FedRAMP, Microsoft IL4 |
| Compliance without teeth = null effect | Ontario checklist (Urbach NEJM 2014); Dutch gradient (van Klei 2012) |
| Habituation decays review | arXiv:2606.22721 (400 reviewers, 7 months) |
| Largest diffs least reviewed | CodePulseHQ 2024 (3.4M merged PRs) |
| Data-type tier shortcuts falsified | ASVS v4→v5 (L1=minimal PII removed) |
| Directional bar | Google engineering practice |

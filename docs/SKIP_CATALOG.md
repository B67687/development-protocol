# SKIP_CATALOG.md — Pre-Authorized Skip Conditions

> **Purpose:** The closed list of legitimate conditions under which a documented
> method may be skipped. Modeled on the aviation Minimum Equipment List (MEL):
> an aircraft may fly with inoperative equipment IF the condition is on the
> approved list, WITH documented conditions and time limits.
>
> **Rules:**
> 1. Only codes in this catalog are legitimate `skipped` states.
> 2. Every skip **expires** — re-justified or remediated on next use (NASA waiver pattern).
> 3. A reason NOT in the catalog requires a **deviation record** (requirement ID,
>    scope, justification, compensating action, risk acceptance, one-shot validity,
>    independent approval via REVIEW).
> 4. Uncatalogued omission = **noncompliance** (red flag in the conformance check).

## EXTRACTION Techniques (10)

| Code | Technique | Legitimate skip conditions |
|---|---|---|
| CAT-EC-01 | Goal Climb | X already at strategic mission level; climbing further is provably impossible (chain exhausted) |
| CAT-EC-02 | No-Computer Check | Technology-independent problem; the check adds no information (state why) |
| CAT-EC-03 | Why-Tree | X already validated by 2+ other techniques that converged; tree is redundant |
| CAT-EC-04 | Contextual Probe | No concrete incident available; nothing to probe (record this explicitly) |
| CAT-EC-05 | Mom Question | User's stated Y is already minimal and unambiguous (no hidden layer suspected) |
| CAT-EC-06 | Problem Statement Wall | X was derived cleanly; the wall test is trivially satisfied by prior work |
| CAT-EC-07 | Job/Pain/Gain Map | Non-market domain (internal tool, personal project); customer-profile framing doesn't apply |
| CAT-EC-08 | Laddering | No value-level ambiguity; user's values are already explicit and aligned |
| CAT-EC-09 | Socratic Probe | Extraction converged without contradiction; probing sequence is redundant |
| CAT-EC-10 | Cognitive Interview | No specific incident to recall; nothing to reconstruct (record this explicitly) |

## Other Method Groups

| Code | Method group | Legitimate skip conditions |
|---|---|---|
| CAT-AMB-01 | AMBITION round | Ambition already converged (Schön criteria met early); fewer rounds suffice (record which) |
| CAT-VAL-01 | Pre-mortem | Trivially reversible decision; failure cost below pre-mortem effort (record cost bound) |
| CAT-VAL-02 | Prototype spike | Approach already validated by prior evidence (record the prior validation) |
| CAT-RVW-01 | Independent review | Change is a pure text formatting edit with zero decision impact (record why) |
| CAT-RVW-02 | Conformance check | No methods in scope for the change (record the scope bound) |
| CAT-PLN-01 | FINISH polish checklist | Category genuinely N/A for deliverable type (e.g., no UI → accessibility) — one per category, not blanket |
| CAT-EXPLAINER-01 | EXPLAINER doc | Doc-only change with zero code artifacts to explain (record the scope bound) |

## Deviation Record Template (for reasons NOT in catalog)

```
DEVIATION: <method> (<id>)
- Requirement: <what the protocol requires>
- Why intent is already satisfied OR inapplicable: <justification>
- Compensating action: <what substitutes>
- Risk acceptance: <who accepts, why>
- Validity: <one-shot / time-limited>
- Approver: <independent REVIEW>
```

## Catalog Maintenance

- The catalog is **closed by default** — adding a code requires the same review
  as a protocol change (conformance check: does this code get abused as a lazy out?)
- Skip-rate per code is tracked in the conformance check; a code trending up is
  either (a) a genuinely common condition (catalog stays, rate is expected) or
  (b) a lazy-out being exploited (code removed)

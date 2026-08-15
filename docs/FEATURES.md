# FEATURES.md — Standing Feature & Behavior Inventory

> **Living artifact (mandated).** The project's complete reference of intended + confirmed features and how each is *supposed to behave*. It is the differential against which both **problem-detection** ("what deviated from intent?") and **testing** ("what should we prove?") are measured — one reference, two uses. Seeded at AMBITION (AI-generated `proposed` F-### folded into the STRATEGY ratification — the scope block IS the inventory), hardened to full behavior contracts at SPECIFICATION, maintained through EXECUTOR and every subsequent cycle. Updated in the same PR as the code that changes a feature.

---

## Why this exists (the two uses)

| Use | Question it answers | Mechanism |
| --- | --- | --- |
| **Detection** | "Is this a feature gap, a broken behavior, or an architecture problem?" | Compare the symptom against the feature's contract — if the behavior is *defined* and violated, it's a bug; if there's no entry, it's a gap; if every feature at a boundary misbehaves, it's architecture (escalate — see EXECUTOR Project Health Discipline) |
| **Testing** | "What do we prove, and did we prove it?" | Every feature's contract + acceptance scenarios anchor its tests; a feature with no linked tests is untested intent |

## Lifecycle

Every feature has a status. One status per feature; transitions logged:

```
proposed ──▶ approved ──▶ applied ──▶ archived
                              │
                              └── superseded (→ archived; successor F-###)
```

> **Statuses are exactly these four:** `proposed` / `approved` / `applied` / `archived`. A proposal that fails ratification is simply not entered (or is `archived` with a reason). **`archived` carries a reason taxonomy** — `rejected` (failed vetting, with the evidence that killed it), `superseded` (replaced by a successor F-###, noted in the entry), `clean-slate` (dropped by the betting table, Clean Slate Rule 10) — ledger-logged, zero new user interactions (Invariant 11). A `proposed` entry older than one cycle re-competes at the betting table or is `archived` (Clean Slate Rule 10).

| Status | Meaning | Can be shipped? |
| --- | --- | --- |
| `proposed` | Intended, not yet ratified into V1 | No |
| `approved` | In V1 scope (IN SCOPE, RULES §5) or added via learning shift | No — needs `applied` |
| `applied` | Implemented, tests anchored, spec-synced | Yes |
| `archived` | Removed/superseded; entry kept for history (why it died) | No |

> **画蛇添足 gate:** an addition that does not map to a feature here, or a `proposed` feature that ships anyway, is scope creep. The inventory is the completeness boundary.

## Entry template (one per feature)

````markdown
## F-001: {{feature name}}

- **Status:** {{proposed|approved|applied|archived}}
- **Reviewed:** {{YYYY-MM-DD}} (review cadence: {{N}} months)
- **Supersedes / superseded by:** {{F-### or none}}

### Behavior Contract (what "works" means — design-by-contract)

- **Preconditions:** {{what must be true before this feature's behavior is expected}}
- **Postconditions:** {{what is guaranteed after it runs}}
- **Invariants:** {{what never changes while it is in use}}
- **Error cases:** {{what happens when inputs/state violate the preconditions}}

### Acceptance Scenarios (Given/When/Then — one per behavior)

```gherkin
Scenario: {{name}}
  Given {{context}}
  When {{action}}
  Then {{observable result}}
```

### Test Anchoring

| Test file / name | Covers |
|---|---|
| {{path::test_name}} | {{postcondition / scenario / error case it proves}} |
````

> A `proposed`/`approved` feature MAY omit the Test Anchoring table (nothing to anchor yet). An `applied` feature MUST have ≥1 linked test per postcondition and per acceptance scenario. A test that proves no feature contract is either dead weight or a signal the feature is unregistered — flag it.

## Trace tags

- Tests reference features by ID: `F-001` in test name, docstring, or a `@F-001` tag. Grep-able: `grep -rn "F-001" tests/` returns every test anchored to that feature.
- Feature entries reference tests in the Test Anchoring table (reverse link).
- **CI staleness check (mandatory, a short script):** warn if `Reviewed:` is older than cadence; block if older than 2× cadence; fail if an `applied` feature has zero linked tests or a test references an unknown `F-###`. Optional = rots — the REVIEW Method Conformance already treats a stale review as RED FLAG; this check makes it machine-detectable.

## Relationship to other artifacts

| Artifact | Role | Static or living? |
| --- | --- | --- |
| **SPECIFICATION.md** | The plan-IS-spec: how the system is built, frozen at execution start | Static (locked) |
| **FEATURES.md** | The reference of what exists + how it behaves, kept current | **Living** |
| **PROJECT_MODEL.md** | Whole-project state machine (valid transitions, invariants) | Living |
| **docs/EXPLAINER.md** | Code explainer for the owner | Living |
| **Tests** | Prove the contracts | Living |

SPECIFICATION says *how it's built*; FEATURES says *what it does and what "working" means*. They describe the same system at different levels — FEATURES is the differential that stays true as the code evolves.

## Origin

Synthesized from LANDSCAPE research (Cluster AE): requirements traceability (ISO/IEC/IEEE 29148; Gotel & Finkelstein 1994), design-by-contract (Meyer 1997), executable specifications / Specification by Example (Adzic 2011; Cucumber "living documentation"), and the anti-rot principle (stale specifications mislead rather than inform — Knight Capital 2012). Form follows the shape observed across OSS feature-coverage artifacts and delta-spec tools: **stable IDs + status lifecycle + behavior contract + test anchoring + anti-rot hooks**.

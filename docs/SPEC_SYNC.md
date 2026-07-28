# SPEC SYNC — Specification-to-Code Fidelity Gate

> **Research backing:** Forward & Lethbridge (2002) surveyed 243 software professionals and found that documentation accuracy degrades below 60% within 6 months without active maintenance — a phenomenon they term "documentation drift." Fluri et al. (2007) studied the co-evolution of source code and comments across 1,200 commits in 6 open-source projects and found that 20–30% of comments became outdated within a single release cycle. Dagenais & Robillard (2010) identified outdated documentation as the primary barrier to entry for new developers joining a project. Kruchten et al. (2012) formalized "documentation debt" as a recognized subtype of technical debt in IEEE software engineering standards.

---

## Purpose

SPEC SYNC is a structured gate that prevents documentation debt from accumulating between the specification and the as-built codebase. Without explicit management, specs and code diverge as a natural byproduct of development. Zhi et al. (2015) demonstrated through a meta-analysis of 38 empirical studies that the value of documentation is directly correlated with its accuracy — inaccurate docs are worse than no docs because they create false confidence.

The gate ensures that the live specification always reflects the as-built state, making it safe for new team members to rely on the spec as a source of truth.

---

## Integration Point in Pipeline

SPEC SYNC is the **final gate before DISTRIBUTE**, running after POLISH:

```
AMBITION → LANDSCAPE → VALIDATION → SPECIFICATION → EXECUTOR → POLISH → SPEC SYNC → DISTRIBUTE
```

It is gated on the PERFECT phase passing — no spec sync is performed for code that has not passed quality gates.

---

## Process

### Step 1: Read the specification

Read the current specification file (`SPECIFICATION.md` at the project root, or the project's designated spec document). Identify every stated requirement, behavior, interface, constraint, and architectural decision.

### Step 2: Inspect the as-built codebase

Walk the codebase and map every module, function, interface, and behavior to its corresponding spec requirement. Use code intelligence tools (symbol search, call graph analysis) to ensure complete coverage.

### Step 3: Catalogue discrepancies

Classify every discrepancy into one of three categories:

| Category | Label | Definition | Action |
|----------|-------|------------|--------|
| Missing feature | `MISSING` | Spec describes a feature that was not built | Flag as scope drift or descope decision |
| Behavior mismatch | `OUTDATED` | Spec describes behavior that differs from actual code | Update spec to match code |
| Undocumented behavior | `NEW` | Code has functionality not described in spec | Add to spec |

### Step 4: Resolve

Apply the appropriate action for each category (see Recovery below). The spec is updated to reflect the as-built state. The original spec is preserved as `SPEC-plan-v{version}.md` for audit trail.

### Step 5: Record

Append the discrepancy catalogue to the project ledger as a decision record. This creates a traceable history of how the spec evolved across cycles.

---

## Checklist

Before marking SPEC SYNC as complete, verify each item:

- [ ] Every function in code maps to a spec requirement (bidirectional trace)
- [ ] Every spec requirement has corresponding code (no unimplemented specs)
- [ ] No undocumented behavior — all public APIs, configuration options, and behavioral contracts are captured in the spec
- [ ] Error handling behavior matches spec: documented errors exist in code, and runtime errors are documented
- [ ] Interface signatures match: function names, parameters, return types, and expected side effects align
- [ ] Dependency declarations are accurate: every external dependency in code is listed in the spec, and no declared dependency is unused
- [ ] Architectural decisions recorded in the spec match the actual module/package structure
- [ ] Edge case behavior is documented: boundary conditions, error states, and failure modes are either handled in code and documented, or explicitly marked as known limitations
- [ ] Test coverage reflects spec requirements: acceptance criteria from the spec have corresponding tests
- [ ] Previous spec version archived: `SPEC-plan-v{version}.md` exists and is not overwritten

---

## Recovery

When discrepancies are found, use this decision tree:

```
Discrepancy found
│
├── Code matches spec intention but differs in implementation detail
│   └── Update spec to match code (code IS the source of truth for implementation)
│
├── Code does something the spec never described (NEW)
│   ├── Is this intended behavior?
│   │   ├── YES → Add to spec as documented behavior
│   │   └── NO  → Flag as defect in project ledger, file issue
│
├── Spec describes behavior code does not implement (MISSING)
│   ├── Was this deliberately descoped?
│   │   ├── YES → Remove from spec, record descope decision in ledger
│   │   └── NO  → File as unimplemented requirement, decide: add now or defer
│
└── Spec and code disagree on expected behavior (OUTDATED)
    ├── Does the code produce correct output?
    │   ├── YES → Update spec to match code
    │   └── NO  → Fix code to match spec
```

**Principle:** The spec is updated to match the code unless the code is demonstrably wrong. This prevents the spec from accumulating unreachable promises. When the code is wrong, fix the code, then verify the fix with a regression test.

---

## Research Support

- **Kruchten et al. (2012), IEEE Software:** "Documentation debt is incurred when documentation is incomplete, inadequate, or not up to date. [...] The cost of not having adequate documentation is paid in reduced productivity and increased risk."

- **Fluri et al. (2007), ICSM:** Found that 22-30% of source code comments are inconsistent with the code after a single release cycle, confirming that drift is not pathological but normal -- and must be explicitly managed.

- **Zhi et al. (2015), TSE:** Meta-analysis of 38 empirical studies on documentation value found that "the benefits of documentation are significantly correlated with its accuracy" — inaccurate docs are worse than no docs because they create false confidence.

- **Forward & Lethbridge (2002), CASCON:** Survey showed 72% of developers encountered documentation that was "significantly wrong" at least once per month, leading to debugging time wasted on following incorrect specs.

- **Dagenais & Robillard (2010), ICSE:** Identified outdated documentation as the primary barrier to entry for new developers joining a project, emphasizing the onboarding cost of spec-code divergence.

---

## Rule

WHEN the PERFECT phase passes (full lint + test suite + forbidden-pattern audit) THEN the agent SHALL execute a SPEC SYNC before DISTRIBUTE or before any new AMBITION iteration starts. The live spec ALWAYS reflects the as-built state. Previous spec versions are archived — never overwritten.

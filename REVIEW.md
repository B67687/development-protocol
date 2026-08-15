# REVIEW.md — The Gödel Gate (Meta-Protocol Audit)

> A protocol that audits the protocol itself. Run by a **separate, independent agent**
> against a fixed checklist. The reviewer has no memory of prior work, no stake in
> the decisions made, and no incentive to agree.
> **The gate closes the epistemic loop:** If the same AI that wrote the code also
> writes the spec, the explainer, the tests, and the compliance report — nothing
> has been verified. Independence is the entire point.

> **Validated by research:** The March 2026 "Cross-Context Review" paper (arXiv)
> found fresh-session review achieves **28.6% F1 vs 24.6%** for same-session review
> (p<0.01, Cohen's d=0.52). Reviewing twice in the same session didn't help (p=0.11).
> The improvement came from the fresh context, not extra effort. REVIEW.md is the only
> production methodology that operationalizes this finding.

---

## When to Run This

| Trigger           | Condition                                                                                         |
| ----------------- | ------------------------------------------------------------------------------------------------- |
| Before phase exit | Every phase transition calls for a review (DISCOVER → WORK, WORK → PERFECT, PERFECT → DISTRIBUTE) |
| Before DISTRIBUTE | **Mandatory.** No project ships without a clean review.                                           |
| After EXPLAINER generation (folded into REVIEW) | Verify the explainer matches the code, not the spec.                                             |
| After SPEC SYNC   | Double-check the sync gate's own work.                                                            |
| On any ambiguity  | If you feel uncertain about quality, run a review.                                                |

**Exception**: PROTOTYPING phase (VALIDATION.md) — prototypes are throwaway by design. Review is not needed. But the KILL/PIVOT/COMMIT decision itself should be reviewed if you're unsure.

## EXPLAINER (Folded Step)

The EXPLAINER step (formerly standalone, docs/EXPLAINER.md) now runs as the first action of REVIEW. Before running the fixed checklist, generate the project's EXPLAINER.md from the template in docs/EXPLAINER.md (Macro Architecture, Data Flow Walk, Module Breakdown, Key Decisions, Quality Guarantees — plus the Mandatory Check). Checks 1.5/1.6/3.1-3.4 below then verify it. The pipeline no longer has a separate EXPLAINER step; the artifact is produced here.


---

## Independence Protocol (Required)

The reviewer agent MUST be launched as a **separate session** with:

1. **Different session** — Start a new AI session. Do NOT continue the building session.
2. **No prior context** — Provide ONLY the protocol docs + the project files. No conversation history, no memory of decisions made, no knowledge of what was "intended."
3. **Fixed checklist provided** — The reviewer gets the checklist below verbatim. They don't invent criteria.
4. **Output-only** — The reviewer reads and reports. They do NOT edit, fix, or improve anything.
5. **Blind to builder intent** — The reviewer compares spec vs code vs explainer. They do not accept "but the intent was..." as an argument.

### Independence Check

Before accepting review results, verify:

- [ ] Reviewer was launched in a separate session
- [ ] Reviewer had no prior conversation history with this project
- [ ] Reviewer was given only the fixed checklist, not a "here's what we were trying to do"
- [ ] Reviewer's findings include at least one issue OR explicit clean bill with supporting evidence
- [ ] Reviewer did not edit any file during the review

### Artifact Binding (SHA256)

The reviewer binds to the EXACT artifact version reviewed:

- [ ] The plan/spec artifact's SHA256 hash was recorded at review start
- [ ] The review report references the artifact by hash (not by "the latest version")
- [ ] If the artifact changed during review, the review is re-run on the new hash

> Source: magic-spec's SHA256 integrity binding. Prevents reviewing "a version" that
> silently drifted mid-review. Confidence: Medium-High.

### Configurable Review Criteria

The fixed checklist is the DEFAULT. For each review, the reviewer may be given
per-project criteria (CAMEL critic_criteria pattern) — additional checks scoped
to the project's specific risk profile:

- [ ] Default checklist applied (always)
- [ ] Project-specific criteria appended (if any — declared before review starts)

> Source: CAMEL RolePlaying critic agent with configurable `critic_criteria`. Keeps
> the audit both standard and project-relevant. Confidence: Medium.

### Method Conformance Check (from METHOD_LEDGER.md)

The ledger is machine-checked at the meta-gate:

- [ ] Fitness: prescribed methods invoked with resolvable evidence?
- [ ] Skip-rate per reason code: no code trending up as a lazy-out?
- [ ] Omitted entries: any uncatalogued omission present? (RED FLAG → fix ticket)
- [ ] What-Matters (PRIORITIZE): the AI proposal logged; user ratification folded
      into the single gate (Invariant 11) — flag if the proposal is missing or the
      single-gate entry is absent (closes the S22 asymmetry)

- [ ] Divergence metric: invocation rate vs outcome quality — no reward for
      checklist completion alone (specification-gaming guard)
- [ ] Net-effort justification: every method or rule added or changed in this
      cycle carries a durability-weighted net-effort justification (what user
      effort does it reduce? what does it cost?) recorded in the ledger or
      change record. Absent justification = FAIL.
- [ ] Trust-boundary conformance (Invariant 10 / Emission Rule 9): autonomous-learning
      decisions logged with velocity classification; slow-velocity decisions have a
      ratification entry. Missing classification or missing slow-ratification = RED
      FLAG → fix ticket (machine-checked by ledger-check.py).

- [ ] One-Gate conformance (Invariant 11): exactly one user ratification per run
      (STRATEGY: scope + kernel + budget), logged. Any auto-escalated one-way-door
      gate logged; an unlogged extra user gate or a missing single-gate entry = RED
      FLAG → fix ticket.

- [ ] FEATURES.md conformance (Cluster AE): statuses valid (proposed/approved/applied/
      archived); every IN SCOPE item is an `approved` entry; no `applied` feature lacks
      linked tests; every test references a known F-###; `Reviewed:` dates within cadence.
      Orphan feature, unanchored test, or stale review = RED FLAG → fix ticket.

- [ ] Architecture fitness audit (Cluster AE): the MACRO paradigm-fit gate ran and is
      logged (a falsification criterion on MACRO decisions); escalation signal (recurring
      co-change / change-amplification at one boundary) checked — no unexamined meta-
      level constraint on a feature flagged in REVIEW.

- [ ] Regression-Lock conformance (Cluster AM): any `applied` feature's golden
      baseline (golden file / visual snapshot) changed in this run carries a ledger
      entry recording the REGRESSION DECISION plus reviewer attestation — a silent
      baseline update (laundered regression) = RED FLAG → fix ticket.
- [ ] Mutation disposition conformance (Cluster AM): surviving mutants were TRIAGED
      (equivalent / killed / real-gap); a real-gap mutant's disposition is ledger-
      recorded; an UNRESOLVED real-gap mutant on an `applied` feature's contract
      = RED FLAG → fix ticket.
- [ ] Cross-cluster interference check (Cluster AQ): this run's changes touch no
      concurrent cluster's territory (shared docs, overlapping files, .omo/ state)
      — overlapping edits to another active cluster's files = RED FLAG → fix ticket.
- [ ] Migration conformance (Cluster AO): if the seam registry was re-keyed or an
      ADR records an up-level move, the ledger holds the migration contract, pre-move
      characterization pins, per-baseline REGRESSION DECISION entries, and separate-
      evaluator sign-off; PROJECT_MODEL transitions valid. Absent contract or sign-off
      = RED FLAG → fix ticket.

> Sources: process-mining conformance checking (van der Aalst), NASA SWE-072
> traceability, OpenAI process supervision, Krakovna specification gaming.
> Confidence: High on mechanism, Medium on thresholds (need real-run tuning).

---

## Fixed Review Checklist

Each item is binary: **PASS** or **FAIL**. No partial credit. Each FAIL becomes a fix ticket.

### Phase 1: Document Completeness

| #   | Check                                                  | How to Verify (without reading code)                                                                  |
| --- | ------------------------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| 1.1 | SPECIFICATION.md exists and has all 16 sections (§0-15) filled | Read the file. Count sections.                                                                        |
| 1.2 | Every section has content (not placeholder/stub)       | Read each section. No "TBD", "TODO", or blank.                                                        |
| 1.3 | Non-goals are explicitly stated                        | Read SPECIFICATION.md section 1 (Scope Boundaries per Component). Non-goals must be explicitly stated there. |
| 1.4 | Success metrics are falsifiable                        | Read section 1 (Overview & Derived Ambition). Metrics must be measurable (<3s launch, >99% uptime), not vague ("fast", "reliable"). |
| 1.5 | EXPLAINER.md exists and matches project scope          | Read EXPLAINER.md. Does it describe the same project as SPECIFICATION.md?                             |
| 1.6 | EXPLAINER.md has all 5 required sections               | Macro Architecture, Data Flow Walk, Module Breakdown, Key Decisions, Quality Guarantees.              |
| 1.7 | Spec-to-Code Fidelity Check was executed and findings recorded | Read REVIEW.md § Spec-to-Code Fidelity Check output. Are there discrepancy records?              |
| 1.8 | PROJECT_MODEL.md exists and is current                    | Read docs/PROJECT_MODEL.md. Does it document states, valid/invalid transitions, invariants? Does the addition under review appear as a transition?   |
| 1.9 | Claims verified per Research on Demand tiers              | For each decision-relevant claim in the spec/research: was it verified at its tier? Do cited sources EXIST and support the claims they're attached to? (CJR lesson: AI citations mislead >60% of the time) |

### Phase 2: Protocol Compliance

| #   | Check                                        | How to Verify (without reading code)                                                                                         |
| --- | -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| 2.1 | RULES.md phase matches current project state | Read RULES.md line 9. Does the phase label match reality?                                                                    |
| 2.2 | Learning shifts are documented (if any)      | Read RULES.md section 5. Check `.omo/shift-log.md` (learning shifts sink). Max 5 shifts per project.       |
| 2.3 | Test philosophy is being followed            | Read RULES.md section 8. Check if test files exist (Glob for _\_test._).                                                     |
| 2.4 | No prohibited patterns used                  | Read RULES.md section 5 (or STANDARDS.md). Check codebase for `as any`, `@ts-ignore`, empty catch blocks, unwrap(), panic(). |
| 2.5 | Project type routing matches actual project  | Read RULES.md section 1. Does the selected route fit? (A "DISCOVER-FIRST" route on a well-understood domain is a mismatch.)  |

### Phase 3: Spec-vs-Explainer Cross-Reference

This is the most important check. Non-coder verification depends on it.

| #   | Check                                                     | How to Verify (without reading code)                                                                                                 |
| --- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| 3.1 | SPECIFICATION.md intent matches EXPLAINER.md architecture | Compare section 1 (Overview & Derived Ambition / intent) of spec vs section 1 (Macro Architecture) of explainer. Do they describe the same system? |
| 3.2 | EXPLAINER.md modules match what actually exists           | Read Module Breakdown. Then run `ls src/` or `ls cmd/` or the project's source tree. Do the modules in the explainer actually exist? |
| 3.3 | Data flow in explainer is plausible                       | Read Data Flow Walk. Is the flow complete? Does it have a start and end?                                                             |
| 3.4 | Key Decisions section identifies real tradeoffs           | Read Key Decisions. Are these real constraints OR generic platitudes? ("We chose X because it's good" is a FAIL.)                    |

### Phase 4: Observable Quality (Code-Independent Signals)

| #   | Check                                   | How to Verify (without reading code)                                                         |
| --- | --------------------------------------- | -------------------------------------------------------------------------------------------- |
| 4.1 | Test files exist and are non-trivial    | Count test files. Non-trivial = at least 3 test cases per module, or >50% of modules tested. |
| 4.2 | Build/compilation succeeds              | Run the build command. Exit code 0 is PASS.                                                  |
| 4.3 | No leaked secrets or credentials        | Grep for `-----BEGIN`, `api_key`, `password`, `token`, `secret`. Any hit is FAIL.            |
| 4.4 | README has install/running instructions | Can a new user get the project running from README alone?                                    |
| 4.5 | CI config or local check script exists (if applicable)* | Check for .github/workflows/, .gitlab-ci.yml, Jenkinsfile, etc., or run ./scripts/check.sh. *See Engineering Plugin §4 |
| 4.6 | Standards audit passes*                    | Run ./scripts/audit.sh from the Standards repo. *See Engineering Plugin §4                  |

### Phase 5: Regression Defenses

| #   | Check                                               | How to Verify (without reading code)                                         |
| --- | --------------------------------------------------- | ---------------------------------------------------------------------------- |
| 5.1 | Tests cover reported bugs (if any)                  | Read recent bug reports. Read test file names/search for related test names. |
| 5.2 | Test count increased since last review              | Compare with previous review's test count. Flat or decreasing is suspicious. |
| 5.3 | Edge cases are tested (empty input, zero, boundary) | Search test files for "empty", "zero", "edge", "boundary", "error".          |

---

## Output Format

The review agent produces a **findings document** with this structure:

```markdown
# Review Findings: [Project Name] — [Date]

## Summary

- Total checks: N
- PASS: N
- FAIL: N
- Verdict: PASS / CONDITIONAL PASS / FAIL

## FAIL Items (must fix before proceeding)

### FAIL 1: [Check ID] — [Title]

- **What**: [what was found]
- **Evidence**: [specific file:line or quote]
- **Severity**: CRITICAL / MAJOR / MINOR
- **Fix guidance**: [what needs to change]

### FAIL 2: ...

## PASS Items (notable)

- [Check ID] — note anything interesting about the pass

## Warnings (not checklist items but worth noting)

- [anything unusual observed during review]

## Reviewer Declaration

- Session was independent: [yes/no with evidence]
- Checklist was fixed (no custom additions): [yes/no]
- No files were modified during review: [yes/no]
```

### Severity Definitions

| Severity     | Meaning                                          | Action                                               |
| ------------ | ------------------------------------------------ | ---------------------------------------------------- |
| **CRITICAL** | Project cannot ship. Core protocol violated.     | Blocking. Fix before any further work.               |
| **MAJOR**    | Significant gap. Quality or correctness at risk. | Must fix before DISTRIBUTE. Can continue work phase. |
| **MINOR**    | Documentation gap or style issue.                | Fix before DISTRIBUTE. Low effort, high signal.      |

---

## Feedback Loop: Findings → EXECUTOR

After the review produces findings:

1. **File the findings** — Save as `review-<date>-<project>.md` in `.omo/reviews/`
2. **Create fix tickets** — Each FAIL with CRITICAL/MAJOR severity becomes a task
3. **Execute fixes** — Use EXECUTOR.md to dispatch fix tasks. The builder AI (NOT the reviewer) makes the changes.
4. **Re-review** — After fixes are applied, run a targeted re-review on only the FAIL items
5. **Phase exit** — Only when all CRITICAL and MAJOR items are resolved does the phase exit

```
[REVIEW AGENT]
    │
    ├── reads protocol docs + project files
    ├── runs fixed checklist
    │
    ▼
[FINDINGS DOCUMENT]
    │
    ├── CRITICAL/MAJOR fails → tasks → EXECUTOR → fixes
    ├── MINOR fails → batch → EXECUTOR → fixes
    └── PASS items → archive
    │
    ▼
[RE-REVIEW] (targeted, only fail items)
    │
    ▼
[PHASE EXIT] (all clears)
```

---

## Self-Review: Can the Protocol Review Itself?

This is the Gödel move. To verify the review protocol works:

1. **Run review on REVIEW.md itself**: Does this document meet its own standards? Are all checks verifiable? Is the independence protocol clear?
2. **Test the independence claim**: Run a review in a session that has context, then run one that doesn't. Do the results differ? If yes, context bias exists.
3. **Calibrate the checklist**: After 3 reviews, remove checks that always pass. Add checks that caught real issues. Keep the checklist lean — a checklist nobody runs is worse than no checklist.

Run this self-review every 5 project reviews, or after any protocol change.

---

## Quick Start

When you want to review a project:

> "Start a new AI session. Load only the Development-Protocol files + the project being reviewed. Run REVIEW.md checklist. Produce a findings document. Do NOT modify any files."

---

## Spec-to-Code Fidelity Check (formerly SPEC_SYNC.md)

This check verifies that the shipped code matches the specification.
It runs during REVIEW, not as a separate step. The reviewer compares:

1. **Spec sections vs implementation** - Does each spec section have a corresponding implementation?
2. **Architecture compliance** - Does the code structure match the spec's architecture decisions?
3. **Constitution violations** - Does the code violate any spec constitution principles?
4. **Scope integrity** - Is there anything in the code that is not in the spec (feature creep)?
5. **Documentation sync** - Does the EXPLAINER match the code, not just the spec?

**Output:** Fidelity gaps are recorded as FAIL items in the review findings.

---

## Handoff to REFLECT

After REVIEW completes, save key findings to `.omo/reviews/latest.md`.
REFLECT should reference these findings when answering Q2: What did the protocol miss?
If the review found protocol-level gaps (not project-level), those become REFLECT inputs directly.
That's it. The protocol does the rest.

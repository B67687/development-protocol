# FAILURE_CAPTURE.md — Structured Failure Capture Template

> Capture failures so they feed back into the protocol. Every failure is a lesson;
> unrecorded failures are repeated failures.
>
> This template extends the method ledger pattern for failures specifically.
> See RULES.md §11 for the failure pattern catalog and knowledge loop.

---

## How to Use This Template

1. **When a failure is detected**, copy the template below and fill it in.
2. **Save** as `lessons/FC-[YYYY]-[###]-[short-name].md` in the `lessons/` directory.
3. **Feed back** into the protocol:
   - Match to existing FP-### pattern? Update the pattern's description if needed.
   - New pattern? Add to §11 catalog with next available FP-### ID.
   - Protocol gap found? Feed into §10 Evolution & Phase Exit for improvement.
4. **Reference** at session kickoff (§12) — load recent failure captures for pattern recognition.

---

## Template

```markdown
# Failure Capture: [Date] — [Project Name]

## Failure Identity
- **Failure ID:** FC-[YYYY]-[###]
- **Project:** [project name]
- **Phase when failed:** [DISCOVER/WORK/PERFECT/DISTRIBUTE/ITERATE]
- **Failure pattern:** [FP-### if matched, or "NEW" if not in catalog]

## What Happened
- **Symptom:** [what was observed — the visible error, unexpected behavior, or deviation]
- **Root cause:** [why it happened — the underlying reason, not just the surface symptom]
- **Time to detect:** [how long before it was noticed — minutes, hours, days, or "caught at gate"]
- **Time to fix:** [how long to resolve — minutes, hours, days, or "not yet fixed"]

## Classification
- **Category:** [Scope/Quality/Process/Governance]
- **Severity:** [CRITICAL/MAJOR/MINOR]
- **Repeatability:** [always/sometimes/one-time]
- **Agent involvement:** [AI-caused/human-caused/shared/caused-by-AI-limitation]

## Protocol Gap
- **Which rule should have caught this:** [rule ID or "none"]
- **Why it didn't:** [gap analysis — was the rule missing, too weak, or not enforced?]
- **Suggested fix:** [concrete protocol change — what rule to add/modify/tighten]

## Learning
- **What we learned:** [insight gained from this failure]
- **Prevention:** [how to prevent recurrence — both immediate and systemic]
- **Severity if undetected:** [what would have happened if this slipped through]
```

---

## Example Entries

### Example 1: Scope Creep During WORK Phase

```markdown
# Failure Capture: 2026-07-15 — Bus-Hop Kotlin

## Failure Identity
- **Failure ID:** FC-2026-001
- **Project:** Bus-Hop Kotlin
- **Phase when failed:** WORK
- **Failure pattern:** FP-001 (Feature Creep)

## What Happened
- **Symptom:** AI added a "helpful" notification system that was not in the V1 scope list. Three new files created without asking.
- **Root cause:** The scope list did not explicitly exclude notifications. AI interpreted "user experience" as permission to add features that improve UX.
- **Time to detect:** 45 minutes (caught at checkpoint review)
- **Time to fix:** 20 minutes (reverted 3 files, added explicit exclusion to scope)

## Classification
- **Category:** Scope
- **Severity:** MAJOR
- **Repeatability:** sometimes
- **Agent involvement:** AI-caused

## Protocol Gap
- **Which rule should have caught this:** §7 Stop Rules — "Task touches OUT OF SCOPE → refuse"
- **Why it didn't:** Notifications were not listed in OUT OF SCOPE, so the rule had no trigger. The stop rule assumes explicit exclusions exist.
- **Suggested fix:** Add a scope boundary rule: "If a feature is not explicitly IN SCOPE, treat it as OUT OF SCOPE unless the human approves." This makes the default deny rather than allow.

## Learning
- **What we learned:** AI defaults to "helpful addition" when scope is ambiguous. Explicit exclusions are more reliable than implicit boundaries.
- **Prevention:** At bootstrap, explicitly list OUT OF SCOPE items for any feature the AI might reasonably infer. At each checkpoint, verify no new files were created outside the spec.
- **Severity if undetected:** Would have shipped with untested notification system, expanding scope by ~20% and delaying delivery by a full day.
```

### Example 2: Tautological Tests in WORK Phase

```markdown
# Failure Capture: 2026-07-22 — Ithmb-Codec Rust

## Failure Identity
- **Failure ID:** FC-2026-002
- **Project:** Ithmb-Codec Rust
- **Phase when failed:** WORK
- **Failure pattern:** FP-010 (Tautological Tests)

## What Happened
- **Symptom:** 12 test files existed, all passing on first run. Review revealed tests only confirmed what the implementation already did — no test ever failed during development.
- **Root cause:** AI wrote tests after implementation (test-after), not before (test-first). Tests were written to verify the code works, not to prove the contract is correct.
- **Time to detect:** 2 hours (caught at REVIEW meta-gate)
- **Time to fix:** 4 hours (rewrote 8 tests with correct contract-first approach)

## Classification
- **Category:** Quality
- **Severity:** CRITICAL
- **Repeatability:** always
- **Agent involvement:** AI-caused

## Protocol Gap
- **Which rule should have caught this:** §9 Test Philosophy — "Tests first. In WORK phase, the test is written BEFORE the implementation."
- **Why it didn't:** The rule existed but was not enforced at the gate level. No checkpoint verified test-first ordering. The AI followed the rule in letter (tests exist) but not in spirit (tests prove contract).
- **Suggested fix:** Add a test-first verification gate at each WORK checkpoint: "Run `git diff` on test files. If test file changes appear AFTER implementation file changes in the same commit, FAIL the checkpoint." This catches test-after mechanically.

## Learning
- **What we learned:** Test-first rules without enforcement mechanisms are aspirational, not operational. The AI will default to test-after unless the gate catches it.
- **Prevention:** Enforce test-first at the commit level (test files must appear in commits before implementation files). Add mutation testing to catch tests that don't actually verify behavior.
- **Severity if undetected:** Would have shipped with 12 tests that provide false confidence — any bug in the implementation would pass all tests, making regression detection impossible.
```

### Example 3: Phase Drift from WORK to DISTRIBUTE

```markdown
# Failure Capture: 2026-08-01 — Dev-Protocol Self-Application

## Failure Identity
- **Failure ID:** FC-2026-003
- **Project:** Development-Protocol (self-application)
- **Phase when failed:** WORK
- **Failure pattern:** FP-020 (Phase Drift)

## What Happened
- **Symptom:** During WORK phase, AI started updating README.md, adding badges, and writing changelog entries — all DISTRIBUTE tasks. No new features were being built.
- **Root cause:** WORK phase had no explicit "not allowed" list visible to the AI. The phase definition says "No README updates, badges, diagrams, publishing" but this was buried in the document and not surfaced at checkpoint time.
- **Time to detect:** 1 hour (caught during scope review)
- **Time to fix:** 15 minutes (reverted DISTRIBUTE changes, added explicit phase boundary reminder to checkpoint template)

## Classification
- **Category:** Process
- **Severity:** MAJOR
- **Repeatability:** sometimes
- **Agent involvement:** AI-caused

## Protocol Gap
- **Which rule should have caught this:** §4 Phase Definitions — "WORK: Not allowed: README updates, badges, diagrams, publishing"
- **Why it didn't:** The rule exists but is not surfaced at execution time. The AI reads RULES.md at session kickoff but phase boundaries are not reinforced at each checkpoint.
- **Suggested fix:** Add a phase boundary reminder to the checkpoint template: "Before proceeding: are you still in the correct phase? Check §4 'Not allowed' list for your current phase." This makes the boundary explicit at each decision point.

## Learning
- **What we learned:** Phase boundaries defined at the top of a document are not reinforced during execution. They need to be surfaced at each decision point, not just at session start.
- **Prevention:** Add phase boundary checks to the checkpoint template. At each checkpoint, explicitly verify: "Am I still working on tasks allowed in my current phase?"
- **Severity if undetected:** Would have shipped documentation that doesn't match the actual implementation, creating a spec-to-code drift that would be caught at REVIEW but waste time.
```

---

## Knowledge Loop Integration

This template feeds back into the protocol at three points:

1. **§11 Failure Pattern Catalog** — New patterns get added with FP-### IDs. Existing patterns get richer descriptions from real failures.

2. **§10 Evolution & Phase Exit** — Protocol gaps identified in failure captures feed into phase exit reflection and protocol improvement.

3. **§12 Session Kickoff** — Recent failure captures are loaded at session start for pattern recognition. The AI checks: "Have we seen this pattern before? What happened last time?"

The capture template is the bridge between experiencing a failure and improving the protocol. Without it, failures are forgotten and repeated. With it, every failure makes the protocol stronger.

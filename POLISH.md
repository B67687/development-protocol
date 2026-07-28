# POLISH.md — Human Final Pass

> Use this after RULES.md execution is complete (WORK → PERFECT → DISTRIBUTE phases).
> POLISH is the **human final pass** — the structured review that catches what AI systematically misses.
> It explicitly covers what the automated phases do NOT cover.

---

## When to Use This

You have a working product that passed DISTRIBUTE phase gates. Now it needs a human eye.

POLISH is the bridge between "the AI built it" and "it's ready for the world." It catches the edge cases, feel problems, and real-world gaps that AI systems systematically miss.

---

## What AI Systematically Misses (You Should Always Check)

Research shows AI-generated code has systematic blindspots. These are NOT random bugs — they're predictable omissions. POLISH targets these specifically:

| Category                          | What AI misses                                                             | How often                                       |
| --------------------------------- | -------------------------------------------------------------------------- | ----------------------------------------------- |
| **Error handling**                | API failures, network timeouts, file I/O errors, null states               | ~45% of AI code has untested error paths        |
| **State management**              | Race conditions, stale data, concurrent access, uninitialized state        | Most common regression in AI code               |
| **Edge cases**                    | Empty arrays, single-element lists, max values, boundary conditions        | ~1.7x more boundary bugs than human code        |
| **Accessibility**                 | Screen reader labels, keyboard navigation, color contrast, focus order     | AI-generated UI is systematically inaccessible  |
| **Security**                      | Input validation, auth checks per endpoint, rate limiting, secret exposure | 45% of AI code has vulnerabilities              |
| **Real-world data**               | Malformed input, encoding issues, large payloads, slow connections         | AI trains on clean data — real data is messy    |
| **API integration**               | Wrong endpoints, missing parameters, undocumented breaking changes         | AI models have knowledge cutoffs                |
| **Context-dependent correctness** | Business logic that requires domain knowledge, implicit conventions        | AI doesn't know your codebase's unwritten rules |

The human reviewer should check these categories in order. The first four (error handling, state, edge cases, accessibility) catch 80% of AI-blindspot issues.

---

## What POLISH Covers (Diff from Automated Phases)

| Task                              | PERFECT    | DISTRIBUTE | METHODOLOGY | POLISH             |
| --------------------------------- | ---------- | ---------- | ----------- | ------------------ |
| Hardening, fuzz, audit            | ✅ Primary | ❌         | ❌          | ❌ skip            |
| README, docs, publishing          | ❌         | ✅ Primary | ❌          | ❌ skip            |
| Post-project retrospective        | ❌         | ❌         | ✅ Primary  | ❌ use METHODOLOGY |
| **Edge case hunt**                | ✅ Partial | ❌         | ❌          | ✅ **Primary**     |
| **Real-world feel**               | ❌         | ❌         | ❌          | ✅ **Primary**     |
| **Protocol improvement feedback** | ❌         | ❌         | ✅ Partial  | ✅ **Primary**     |
| **Shipping readiness sign-off**   | ❌         | ❌         | ❌          | ✅ **Primary**     |

POLISH only adds what the automated phases miss. No duplication.

---

## The Protocol

### Gate 1: Quality Gates (Re-confirm automated)

- [ ] Full test suite passes (same as PERFECT — re-confirm clean checkout)
- [ ] Lint clean (same as PERFECT — re-confirm)
- [ ] Build from scratch on clean checkout (no cached artifacts)
- [ ] No secrets in repo (`grep` for tokens, keys, passwords)
- [ ] LICENSE file present and correct
- [ ] `.gitignore` covers build artifacts and IDE files

If any fails, return to WORK/PERFECT. No bypass.

### Gate 2: Heuristic Evaluation (Replaces "Real-World Feel")

The "real-world feel check" is replaced with **Nielsen's 10 Usability Heuristics** — a systematic framework, not intuition. For each heuristic, ask:

| #   | Heuristic                                                   | What to check (AI-specific)                                                                                            |
| --- | ----------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| 1   | **Visibility of system status**                             | Does every action produce visible feedback? Loading states? Progress indicators?                                       |
| 2   | **Match between system and real world**                     | Do terms match the user's domain? Date formats, units, terminology?                                                    |
| 3   | **User control and freedom**                                | Can users undo/redo? Is there a way out of every state?                                                                |
| 4   | **Consistency and standards**                               | Are patterns consistent across the app? Same button placement, same terminology, same behavior?                        |
| 5   | **Error prevention**                                        | Does the UI prevent mistakes before they happen? Confirm on destructive actions? Validate input before submit?         |
| 6   | **Recognition over recall**                                 | Are options visible? Does the user need to remember information from a previous screen?                                |
| 7   | **Flexibility and efficiency**                              | Are there shortcuts for power users? Tab order sensible? Keyboard navigation?                                          |
| 8   | **Aesthetic and minimalist design**                         | Is every piece of information necessary? Any redundant data or UI elements?                                            |
| 9   | **Help users recognize, diagnose, and recover from errors** | Are error messages in plain language? Do they suggest a solution? Do they include the bad value that caused the error? |
| 10  | **Help and documentation**                                  | Is inline help available for complex features? Are error states documented?                                            |

**Evaluation protocol:**

1. Open the product fresh (don't re-read the spec)
2. Go through each heuristic one at a time — note violations
3. Classify each violation: BLOCKER (fix before ship), MINOR (document), DEFER (known limitation)
4. Timebox: 30 minutes for the full sweep

**Output:** A list of heuristic violations by severity.

### Gate 3: Edge Case Hunt (Systematic)

Use these named techniques to find edge cases:

**1. Equivalence Partitioning (EP)**
Divide input space into partitions. Test one value per partition:

- Valid partitions → should work
- Invalid partitions → should fail gracefully
- Null/empty partitions → should not crash

**2. Boundary Value Analysis (BVA)**
For every numeric boundary, test: min-1, min, min+1, max-1, max, max+1, zero, and one.

**3. Error Guessing (Structured)**
AI code has predictable error omissions. Check specifically:

- Network failure on every API call (not just the first)
- File I/O errors (disk full, permission denied, file locked)
- Concurrent access (two users hitting the same endpoint simultaneously)
- Empty states (no data in the system — does the UI handle it?)
- Maximum states (1000 items in a list, 1MB upload, 10 years of data)

**4. Whittaker's Exploratory Testing Tours**
Walk through the product with these mental models:

- **Business District** — core features only. Does the main flow work?
- **Historic District** — old features, legacy paths. Did the new code break anything old?
- **Entertainment District** — side features, secondary flows. Are there broken corners?
- **Tourist District** — what does a new user experience? First-run, onboarding, signup flow.
- **Hotel District** — what happens when you leave something mid-way? Incomplete form, abandoned wizard.
- **Seedy District** — malicious use, bad data, adversarial inputs. What happens when someone tries to break it?

**5. State Transition Testing**
List all states. Test every allowed transition AND every forbidden transition. AI often forgets to handle transitions between states that "shouldn't happen."

**Output:** An edge case log with classification: BLOCKER / ACCEPTABLE / NON-ISSUE.

### Gate 4: Non-Functional Review

**Performance:**

- [ ] Does it load/respond within acceptable time on target hardware?
- [ ] Any obvious N+1 queries or unnecessary computation?
- [ ] Memory usage acceptable? No leaks in long-running operations?

**Security (human check after automated):**

- [ ] No secrets in env files committed?
- [ ] Auth applied to EVERY endpoint (not missing one)?
- [ ] Input validation on all user-facing fields?

**Data Integrity:**

- [ ] Can data be corrupted by concurrent access?
- [ ] Is error recovery clean (no partial writes)?
- [ ] Are there idempotency guarantees where expected?

**Accessibility (WCAG 2.1 AA minimum):**

- [ ] Keyboard navigation works (tab through all interactive elements)
- [ ] Screen reader labels on all icons and images
- [ ] Color contrast meets 4.5:1 ratio
- [ ] Focus order follows visual order

### Gate 5: Shipping Checklist

- [ ] Version tag applied (semver)
- [ ] CHANGELOG updated
- [ ] Release notes written (what changed, what's new, what's fixed)
- [ ] README reflects the shipped version (screenshots, API docs, examples)
- [ ] Known issues documented
- [ ] Binary/package published (per project's distribution strategy)

### Production Quality Gates (From OSS Benchmark Analysis)

Derived from analyzing 35 production-quality open-source Rust projects. These checks catch gaps that standard test suites miss.

| Check                           | Verification                                                                                                           | Pass/Fail                                                                                                                                                                |
| ------------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Fuzz & Property Tests**       | Run `cargo fuzz run <target>` for all fuzz targets. Run `cargo test` with proptest-based tests.                        | PASS: All fuzz targets run ≥10k iterations without panics; property tests report no shrinking failures. FAIL: Any panic, OOM, or failed property.                        |
| **Benchmark & Performance**     | Run `cargo bench` (criterion/divan). Check CI artifact in `gh run list` for github-action-benchmark regression alerts. | PASS: Benchmarks exist for critical-path functions; CI tracks them; no regressions >10% vs. baseline. FAIL: No benchmarks, or CI doesn't track them.                     |
| **Snapshot Testing**            | Run `cargo insta review` (or equivalent). Verify golden files match current output.                                    | PASS: Snapshot tests exist for all non-trivial output; `cargo insta review` shows zero pending snapshots. FAIL: No snapshot tests, or pending/unreviewed snapshots.      |
| **Security & Dependency Audit** | Run `cargo deny check` with `deny.toml` configured. Check for RUSTSEC advisories. Review CI audit workflow.            | PASS: `cargo deny` clean (no advisories, no bans, no licenses flagged); `audit.yml` runs weekly in CI. FAIL: Any unresolved advisory, missing deny.toml, or no CI audit. |

### Gate 6: Retrospective (Protocol Improvement)

After shipping, close the learning loop. Reference METHODOLOGY.md — don't duplicate it.

1. **Run the METHODOLOGY.md 4-layer retrospective** for the project itself
2. **Feed back into the protocol** — what worked and what didn't about the PREP PHASE:
   - Did AMBITION.md produce useful hypotheses?
   - Was LANDSCAPE.md the right depth?
   - Did VALIDATION.md save time or add ceremony?
   - Was SPEC.md the right level of detail?
   - Did EXECUTOR.md handoff work?
   - Did POLISH.md catch real issues?
3. **Update RULES.md failure patterns** — any new patterns discovered?
4. **Update phase gate criteria** — any gate that should be tighter or looser?

---

## Meta: How to Polish

1. **Fresh eyes first** — Don't re-read the spec before reviewing. The point of POLISH is to see what's actually there, not what was planned.
2. **One edge case caught is worth ten features built** — the marginal quality gains come from fixing the last 10 edge cases, not adding more UI.
3. **The shipping checklist is sacred** — if a checkbox is unchecked, don't ship. The checklist exists because forgetting something is normal.
4. **Systematic beats intuitive** — "real-world feel" is pattern recognition. Make it systematic with heuristics and named techniques. It's faster and you won't miss categories.
5. **The AI blindspot catalog gets smarter** — each project, add new blindspot patterns to the catalog (this doc). The catalog is the accumulated knowledge of what to check.
6. **The retrospective is for the NEXT project** — write it for your future self, not to justify the past.

---

## Origin

Rewritten July 2026 following a deep research pass on human review methodology. Incorporates findings from: Nielsen's 10 Usability Heuristics (NNGroup), Shneiderman's 8 Golden Rules, Cognitive Walkthrough methodology, Whittaker's Exploratory Testing Tours, Boundary Value Analysis, Equivalence Partitioning, and the AI-blindspot research from Augment Code, GitClear, and Veracode.

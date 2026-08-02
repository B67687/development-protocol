# Research: The Human Final Pass After AI Execution

**Date:** 2026-07-11
**Context:** Research to inform POLISH.md — the human final pass phase of the Development Protocol.
**Primary gap:** POLISH claims "real-world feel check" as its primary contribution but provides zero methodology for it.

---

## Summary

AI coding agents produce approximately 80% of a working solution reliably but systematically omit the 20% that determines whether software survives production contact. This missing 20% clusters in predictable categories: edge cases, error handling, non-functional requirements (security, observability, performance), accessibility, and cross-cutting architectural consistency.

The human final pass — POLISH — needs structured, repeatable methodology to fill these gaps. This research identifies five pillars for that methodology:

1. **UX heuristics evaluation** — systematic frameworks (Nielsen, Shneiderman, cognitive walkthrough) for evaluating usability without user testing
2. **Edge case discovery techniques** — proven testing methodologies (BVA, equivalence partitioning, error guessing, state transition, exploratory tours)
3. **"Real-world feel" as structured critique** — product sense is analyzable through design critique, heuristic evaluation, and learnability assessment
4. **Shipping readiness criteria by project type** — what "ready" means differs fundamentally between libraries, apps, CLIs, and prototypes
5. **AI blindspot catalog** — eight systematic failure patterns that humans must ALWAYS check

The key insight: the human pass is not about intuition or "taste." It is about applying frameworks the AI cannot apply because they require execution context, causal reasoning, and adversarial thinking.

---

## Sources Surveyed

### Primary Sources

- **Jakob Nielsen (NNGroup):** "10 Usability Heuristics for User Interface Design" (1994, revised 2020, updated 2026). "How to Conduct a Heuristic Evaluation" (2023). Nielsen Norman Group — the authoritative source on heuristic evaluation methodology.
- **Ben Shneiderman (UMD):** "The Eight Golden Rules of Interface Design" (1985, updated through 6th edition 2016). Direct from Shneiderman's University of Maryland page — the canonical statement.
- **Kate Moran, Kelley Gordon (NNGroup):** "How to Conduct a Heuristic Evaluation" (June 2023). Step-by-step protocol with downloadable workbook template.
- **James Whittaker:** "Exploratory Software Testing: Tips, Tricks, Tours, and Techniques to Guide Test Design" (2009). The definitive work on exploratory testing tours — business, historic, entertainment, tourist, hotel, and seedy districts.
- **Addy Osmani:** "My LLM Coding Workflow Going Into 2026" and "The 80% Problem in Agentic Coding" (December 2025–January 2026). Coined the "80% problem" framing for AI code generation.
- **Augment Code Research:** "Debugging AI-Generated Code: 8 Failure Patterns & Fixes" (October 2025, updated June 2026). "The 80% Problem: Why AI Agents Ship Fast But Create Hidden Technical Debt" (April 2026). Systematic catalog of AI failure modes.
- **LavX News / Elena Varga:** "The Edge Case Blind Spot: Why LLMs Stumble at Writing Robust Code" (October 2025). Analysis of LLM pattern-matching limitations.
- **Basit Ali:** "Blindspots in LLMs I've Noticed While AI Coding" (December 2025). Practitioner catalog of AI blindspots.
- **Veracode (2025):** 45% of AI-generated code contains security vulnerabilities. Java implementations showed 70%+ security failure rates.
- **GitClear (2025):** 211-million-line longitudinal study — copy/pasted code rose from 8.3% to 12.3%, refactored code fell from ~22% to ~10% in AI-assisted workflows.
- **Stack Overflow Developer Survey (2025):** ~45% of developers report debugging AI-generated code is more time-consuming than expected.
- **arXiv 2512.05239:** "A Survey of Bugs in AI-Generated Code" (December 2025). Systematic catalog of bug types in Codex-generated code.
- **arXiv 2605.30777:** "What Breaks When LLMs Code? Characterizing Operational Safety" (May 2026). LLM-specific operational safety failures.
- **Durgesh Rajubhai Pawar (Master.dev):** "AI-Generated UI Is Inaccessible by Default" (April 2026). Five-layer enforcement system for accessibility in LLM-generated React.
- **ACM (2025):** "Evaluating AI-Generated Web Code for Accessibility Compliance" — empirical study of ChatGPT and Copilot accessibility output.
- **Heurio:** "Vibe Coding Workflow Meets Nielsen's 10 Usability Heuristics" (May 2026). Practical adaptation of heuristics for AI-generated output review.
- **Ranger.net:** "Common Bugs in AI-Generated Code and Fixes" (2026). Industry survey of AI code failure patterns.
- **r/webdev community:** "AI Blindspots" (March 2025, HN thread 43414393). Crowd-sourced practitioner observations of LLM limitations.
- **DECODE / Boris Plavljanic:** "4 Software Release Checklists for Smoother Launches" (March 2026). Release readiness, post-release, rollback/incident, and sprint planning checklists.
- **Cortex:** "Production Readiness Review Checklist & Best Practices" (2025). Industry-standard PRR methodology.
- **Port.io:** "Production Readiness Checklist: Ensuring Smooth Deployments" (June 2025). Security, reliability, performance, observability criteria.
- **OpsLevel:** "Production Readiness Checklist: An In-Depth Guide" (2025). Comprehensive readiness framework.
- **Forasoft:** "Non-Functional Requirements Checklist: 14 Categories" (May 2026). Updated for 2026 compliance landscape (EU AI Act, HIPAA updates, European Accessibility Act).
- **Atlassian:** "How to Lead a Product Critique" (May 2026). Structured product assessment method.
- **Interaction Design Foundation:** "What Are Design Critiques?" (updated 2026). Collaborative design evaluation methodology.
- **UX Tigers / Jakob Nielsen:** "How I Developed the 10 Usability Heuristics" (July 2025). History and factor analysis behind the heuristics.

---

## What AI Systematically Misses

### The 80% Problem (Osmani/Karpathy)

Andrej Karpathy observed he had shifted to "80% agent coding and 20% edits+touchups." Addy Osmani's insight is that the remaining 20% is not cleanup — it represents a distinct category of engineering work that determines whether code survives production.

### What AI Agents Reliably Produce

- CRUD operations, standard API patterns, type definitions
- Basic validation and happy-path test coverage
- UI component rendering, state management, database queries
- Syntactically valid code that passes linters

### What AI Agents Systematically Omit

| Category | What's Missing | Why It Matters |
|----------|---------------|----------------|
| **Error handling** | Retry logic, circuit breakers, graceful degradation, distinguishing retryable from permanent failures | Code crashes on non-happy-path inputs. Silently fails when it should alert. |
| **Security** | Per-endpoint auth, input sanitization, rate limiting, audit logging, PII handling | 45% of AI code has vulnerabilities (Veracode 2025). Passes functional tests, fails pen tests. |
| **Observability** | Structured logging, metrics, distributed tracing, correlation IDs | Production issues are invisible until users report them. No debugging hooks. |
| **Edge cases** | Empty states, null inputs, boundary conditions, concurrent access, max values | Training data overrepresents common scenarios. AI doesn't "think" about what could go wrong. |
| **Accessibility** | ARIA attributes, keyboard navigation, screen reader support, color contrast | AI-generated UI is inaccessible by default — the accessibility tree is semantically empty (Master.dev 2026). |
| **Performance** | O(n²) algorithms where O(n) exists, memory allocation patterns, unnecessary re-renders | Works in dev, fails under load. AI optimizes for correctness, not performance. |
| **Architectural consistency** | Cross-cutting concerns applied uniformly, ADR-informed decisions | Each file is context-window-limited. Auth on 3 of 4 endpoints. Duplicated patterns instead of abstractions. |
| **Data integrity** | Validation gates, idempotency keys, backpressure, dead letter queues | Duplicate charges, silent data corruption, lost records under load. |
| **Internationalization** | Locale-aware formatting, RTL support, string externalization, pluralization rules | English-only assumptions baked in. Breaks in non-US contexts. |

### The Eight Failure Patterns (Augment Code, 2025–2026)

Research validated across production deployments identifies these systematic AI failure modes:

1. **Hallucinated APIs** — references to packages/methods that don't exist. ~1 in 5 AI code snippets contain references to fake libraries.
2. **Security vulnerabilities that look functional** — code works but fails securely. Auth bypasses, SQL injection, error handlers that leak sensitive data.
3. **Performance anti-patterns** — string concatenation in loops (instead of builders), nested O(n²) iterations, unnecessary allocations.
4. **Happy-path error handling** — try-catch blocks that log but don't actually handle errors. No recovery, no fallback.
5. **Missing edge cases** — empty arrays, null values, boundary integers, unicode inputs. AI rarely checks array bounds.
6. **Outdated library usage** — deprecated APIs from training data (practices obsolete before the AI wrote them).
7. **Data model mismatches** — assumes data structures that don't match actual schemas. Property accesses without type checking.
8. **Missing context dependencies** — environment variables without fallbacks, undocumented configuration, assumed infrastructure.

### Why AI Misses These

- **Training data imbalance:** Edge cases are rare in training data by definition. LLMs statistically prioritize common patterns.
- **No execution context:** AI doesn't run or debug its output. No feedback loop between generation and behavior.
- **No causal reasoning:** Current architectures don't build internal representations of program behavior. They can't simulate code's response to unexpected inputs.
- **No accessibility tree model:** The model has no representation of the accessibility tree. It models what code *looks like*, not what code *means* to assistive technologies.
- **Token economics:** Semantic HTML and ARIA cost more tokens. Absent specific constraints, the model has no incentive to generate them.
- **Visual-only feedback:** When humans evaluate AI output, they judge visually. The feedback signal reinforces visual fidelity without penalizing semantic failures.

---

## UX Heuristics for Final Review

### Framework 1: Nielsen's 10 Usability Heuristics (NNGroup, 1994/2020/2026)

The industry-standard evaluation framework. Requires 3–5 evaluators working independently. Does NOT require user testing. Each evaluator walks through main flows in ~1–2 hours, noting violations and severity (cosmetic/minor/major/catastrophic).

| # | Heuristic | What to Check in AI-Generated Output |
|---|-----------|--------------------------------------|
| 1 | **Visibility of system status** | Loading spinners, progress indicators, confirmation messages after actions. AI rarely generates these unless explicitly prompted. |
| 2 | **Match between system and real world** | Labels like "Submit Query" vs "Save." AI generates developer-speak. Check: read every label and button out loud. |
| 3 | **User control and freedom** | Cancel buttons, back links, undo functionality. AI-generated flows often lack emergency exits. |
| 4 | **Consistency and standards** | Same button styles across pages? Consistent navigation? AI prompted multiple times produces inconsistent UI patterns. |
| 5 | **Error prevention** | Input masks, character limits, confirmation dialogs for destructive actions. AI-generated forms almost never include these. |
| 6 | **Recognition rather than recall** | Can a first-time visitor find the 3 most important actions without clicking? AI optimizes for clean minimalism that hides features. |
| 7 | **Flexibility and efficiency of use** | Keyboard shortcuts, bulk actions, smart defaults. Critical for dashboards/admin panels — AI skips these. |
| 8 | **Aesthetic and minimalist design** | Does every element serve the user's task? AI adds decorative extras — icons, gradients, unnecessary animations. |
| 9 | **Help users recognize, diagnose, recover from errors** | Plain-language error messages that suggest solutions. AI shows "Something went wrong" or exposes stack traces. |
| 10 | **Help and documentation** | Tooltips, onboarding hints, contextual help. AI-generated products rarely include any of these. |

**Severity ratings** (NNGroup standard):
- **0 (cosmetic):** Don't fix unless extra time
- **1 (minor):** Low priority
- **2 (major):** Important to fix
- **3 (catastrophic):** Must fix before shipping

**Methodology:** 1–2 hours per evaluator walking through main flows. Consolidate into severity × heuristic matrix. Three to five evaluators identify ~75% of usability problems. Per Jakob Nielsen: "Heuristic evaluation is a method for identifying design problems in a user interface. Evaluators judge the design against a set of guidelines (called heuristics) that make systems easy to use."

**Key insight for POLISH:** Heuristic evaluation is the primary methodology that gives structure to "real-world feel check." It converts the vague "does this feel right?" into ten specific lenses.

### Framework 2: Shneiderman's Eight Golden Rules (1985/2016)

Complementary to Nielsen. More focused on interaction design principles. Distilled from three decades of HCI research at University of Maryland.

| # | Rule | Human Final Pass Check |
|---|------|----------------------|
| 1 | **Strive for consistency** | Same actions → same results across the product. Consistent terminology, color, layout, fonts. |
| 2 | **Seek universal usability** | Does it work for novices AND experts? Different screen sizes? Disabilities? International users? |
| 3 | **Offer informative feedback** | Every action gets appropriate feedback. Minor actions → modest response. Major actions → substantial response. |
| 4 | **Design dialogs to yield closure** | Clear beginning, middle, end to task sequences. Satisfying confirmation at completion (e.g., order confirmation page). |
| 5 | **Prevent errors** | Gray out inappropriate options. Validate before submit. Guide users to fix only the faulty part, not retype everything. |
| 6 | **Permit easy reversal of actions** | Undo, go back, version history. Users should feel safe exploring. |
| 7 | **Keep users in control** | No surprises, no changes in familiar behavior, no tedious forced sequences. |
| 8 | **Reduce short-term memory load** | Don't make users remember info from one screen to use on another. Visibility of critical data across screens. |

### Framework 3: Cognitive Walkthrough (Lewis/Polson/Wharton, 1990; NNGroup 2024)

Evaluates **learnability** — how easy is it for a new user to figure out? Four questions per step:

1. Will users try to achieve the right effect? (Do they know this step is needed?)
2. Will users notice the correct action is available? (Is the button visible?)
3. Will users associate the action with the effect? (Does the label make sense?)
4. If correct action is taken, will users see progress? (Is feedback clear?)

**Best for:** Complex interfaces, novel interaction patterns, onboarding flows. AI-generated products with custom UI patterns are prime candidates.

### Framework 4: Design Critique (Atlassian, IxDF, 2026)

Structured feedback process for assessing design before shipping. Key elements:
- **Select diverse evaluators** (not just designers — include developers, PMs, QA)
- **Set specific goals** (identify usability issues? assess feasibility?)
- **3Cs framework:** Context, Content, Composition
- **Focus on the design, not the designer** — specific, actionable, objective

Atlassian's method: "A product critique is a product management method to assess a product from several angles, including usability, design, functionality, accessibility, and profitability."

---

## Edge Case Discovery Methodology

### Technique 1: Equivalence Partitioning (EP) — ISTQB Standard

Divides input data into groups (equivalence classes) where the software should behave identically. Test one representative per class.

**In POLISH:** For each input field, identify valid and invalid partitions. Test one value from each. Saves time while maintaining coverage.

**Example:** Age field (18–120)
- Valid partition: 18–120 → test value 25
- Invalid partition (low): 0–17 → test value 15
- Invalid partition (high): 121+ → test value 150

### Technique 2: Boundary Value Analysis (BVA) — ISTQB Standard

Tests at the edges of equivalence partitions. Most defects occur at boundaries (off-by-one errors).

**In POLISH:** For each boundary, test: min-1, min, min+1, nominal, max-1, max, max+1.

**Example:** Same age field (18–120): Test 17, 18, 19, 50, 119, 120, 121

**Combo rule:** Apply EP to map input ranges broadly, then BVA to hone in on boundary points within those ranges.

### Technique 3: Error Guessing (Experience-Based)

Testers use intuition and past experience to predict where defects hide. Formalized by Testsigma, Tricentis, and ISTQB.

**Structured error guessing checklist for AI-generated code:**

- **Input validation:** Empty strings, null values, negative numbers, zero, very large numbers, special characters, unicode, SQL injection strings, XSS vectors
- **State violations:** Double-click submit, back button after form submit, browser refresh during transaction, concurrent edits
- **Resource limits:** Maximum file size, maximum array/collection size, maximum concurrent connections, database connection pool exhaustion
- **Time/Ordering:** Race conditions, timeout handling, expired tokens, out-of-order API responses, rapid repeated actions
- **Dependencies:** Missing files, unreachable API endpoints, database down, network offline, third-party service unavailable
- **Data integrity:** Duplicate submissions, partial updates, concurrent writes, orphaned records, cascading deletes

**Error guessing template (adapted from Testsigma, 2026):**
```
| Input/Feature | Potential Error | Trigger Condition | Expected Result | Actual Result |
```

**Why error guessing matters for AI:** "AI models make errors that are fundamentally different from human errors" (Augment Code). The AI blindspot catalog IS an error guessing checklist. Use it.

### Technique 4: State Transition Testing (n-Switch Coverage)

Models the system as a finite set of states and tests valid/invalid transitions between them. Coverage levels: 0-switch (single transitions), 1-switch (pairs of consecutive transitions), and n-switch.

**In POLISH:** Map states for auth (logged out, logged in, locked, password reset), payment (cart, payment pending, confirmed, failed, refunded), or any multi-step flow. Test:
- All valid transitions (happy path)
- All invalid transitions (what happens if user tries to access payment while logged out?)
- 0-switch coverage (every single transition tested)
- 1-switch coverage (every pair of consecutive transitions)

### Technique 5: Whittaker's Exploratory Testing Tours (James Whittaker, 2009)

Uses a city/metaphor framework to guide systematic exploration:

| District | Metaphor | What to Test |
|----------|----------|-------------|
| **Business District** | Where people make money | Core functionality, primary user workflows, money-making features |
| **Historic District** | Old parts of town | Legacy code, inherited features, deprecated paths still accessible |
| **Entertainment District** | Where people have fun | Secondary features, help systems, tutorials, Easter eggs |
| **Tourist District** | Popular with newcomers | Onboarding, first-run experience, documentation, sample workflows |
| **Hotel District** | Temporary stays | Session management, state persistence, interrupted workflows, returning users |
| **Seedy District** | Places locals avoid | Error handling, failure modes, security boundaries, crash paths |

**Why tours work for POLISH:** Each district is a different lens. A 2-hour session with one tour per district catches things a feature-focused review misses. "Exploratory testing is not a testing methodology. Rather, it is an approach to quality verification." (Whittaker) Tours are the structure that makes exploration reliable.

---

## "Real-World Feel" — What It Actually Means

### Demystifying Product Sense

"Product sense" and "real-world feel" are often treated as magical intuition. In practice, they are **pattern recognition plus structured analysis** — skills that can be codified and taught.

**The core components of "real-world feel" assessment:**

1. **Heuristic violations as feel-killers** — a product that violates Nielsen's heuristics (especially #1 system status, #3 user control, #9 error recovery) will "feel bad" to anyone. These are diagnosable, not mystical.

2. **Cognitive friction audit** — every place the user has to stop, think, or figure something out. The goal of POLISH is to find and reduce these friction points. Use the cognitive walkthrough methodology.

3. **Emotional response checkpoints** — specific moments that should feel satisfying (completion of a task, saving, sharing, accomplishing) vs. frustrating (waiting, error recovery, unexpected behavior).

4. **Flow state analysis** — does the product support sustained attention? Does it interrupt the user unnecessarily? (Shneiderman's #7 — keep users in control.)

5. **Learnability gradient** — how steep is the learning curve? Can a first-time user succeed? (Cognitive walkthrough answers this.)

### Structured Feel Evaluation Protocol

**Step 1: Fresh-eyes pass.** The evaluator uses the product for 15 minutes without prior instruction. Note every moment of confusion, hesitation, or frustration. This is the closest thing to a "real-world feel" test without user testing.

**Step 2: Heuristic sweep.** Run the 10 heuristics systematically. Each violation is a "feel" problem with a name and a fix.

**Step 3: Edge case tour.** Run Whittaker's 6 districts. Business + Seedy are the highest-impact for feel.

**Step 4: Shneiderman audit.** Check #6 (reversibility) and #7 (user control) especially. These are the most common feel-killers in AI-generated products.

**Step 5: Competitor comparison.** How does this feel compared to analogous products the user already knows? (Nielsen's #4 — consistency with standards = consistency with user expectations from other products.)

### What "Feels Right" Actually Means (Operational Definition)

A product "feels right" when the human reviewer, after structured evaluation, can answer YES to all of:

1. **Transparency:** Can I always tell what's happening? (Heuristic #1)
2. **Agency:** Can I do what I want and undo what I don't? (Heuristic #3, Shneiderman #6/#7)
3. **Ease:** Does the common path require minimal thought? (Heuristic #6, cognitive walkthrough)
4. **Forgiveness:** If something goes wrong, can I recover gracefully? (Heuristic #9)
5. **Professionalism:** Are there rough edges, missing states, or inconsistencies? (Heuristic #4/#8)
6. **Speed:** Does the product feel responsive? (Not just actual performance — perceived performance with loading indicators matters more.)

---

## Non-Functional Edge Case Checklist for Human Reviewers

### Performance Feel

- [ ] Does the product show loading states for operations that take >300ms?
- [ ] Are there loading skeletons or progress indicators (not just spinners)?
- [ ] Does the product handle rapid repeated actions without breaking?
- [ ] Is there any UI jank, stutter, or layout shift during data loading?
- [ ] Do animations serve a purpose or are they decorative overhead?
- [ ] Are there infinite scroll/list patterns without pagination or virtualization?
- [ ] Does the product batch or debounce high-frequency events (search-as-you-type, resize handlers)?

### Security Boundaries (Human-Catchable)

- [ ] Do error messages expose internal information (stack traces, SQL queries, file paths)?
- [ ] Can the user access URLs/routes they shouldn't (direct navigation to admin panels)?
- [ ] Is there rate limiting visible on repeated failed actions?
- [ ] Is sensitive data visible in URLs (tokens, IDs in query strings)?
- [ ] Are there visible API endpoints in source/network tab that leak information?
- [ ] Does authentication timeout redirect to login (not crash)?
- [ ] Are file uploads properly restricted by type/size?

### Data Integrity

- [ ] Can the user accidentally destroy data without confirmation?
- [ ] Are there undo mechanisms for destructive operations?
- [ ] What happens if the user submits a form and navigates away mid-submit?
- [ ] Can the user see stale/outdated data after another user has modified it?
- [ ] Are there visible orphaned states (items in a list that reference deleted parents)?

### Error Recovery

- [ ] What happens when the network is disconnected mid-session?
- [ ] What happens when the server returns 500, 403, 429, 502, 503?
- [ ] Can the user retry failed operations without side effects?
- [ ] Is there any auto-save or draft recovery?
- [ ] Are form inputs preserved on validation errors?

### Internationalization & Localization

- [ ] Does the product display correctly with non-ASCII characters (é, ñ, 中文, العربية)?
- [ ] Do dates, times, numbers, and currencies follow locale conventions?
- [ ] Are there any hardcoded strings that should be externalized?
- [ ] Does RTL text break layout?
- [ ] Are there locale assumptions in sorting, string comparison, or text truncation?

### Accessibility (WCAG 2.1 AA Minimum)

- [ ] Can all interactive elements be reached and operated by keyboard?
- [ ] Do interactive elements have visible focus indicators?
- [ ] Are images/decorative elements properly labeled (alt text, aria-hidden)?
- [ ] Do dynamic content updates announce changes to screen readers?
- [ ] Is color alone used to convey information (error states, status indicators)?
- [ ] Is there sufficient color contrast (4.5:1 for normal text, 3:1 for large text)?
- [ ] Are form inputs associated with labels?
- [ ] Are ARIA landmarks present (nav, main, aside)?
- [ ] Are error messages programmatically associated with their inputs?
- [ ] Does the product work with browser zoom up to 200%?
- [ ] **Special AI check:** Open the browser accessibility tree. Are the elements semantically correct (buttons are `<button>`, landmarks are `<nav>`, headings are `<h1-h6>`)? AI-generated UI often renders visible elements as generic `<div>`s that the accessibility tree ignores.

### Platform & Environment Quirks

- [ ] Does the product work on the target OS/version?
- [ ] Does the CLI tool work in different terminal sizes? Different shells?
- [ ] Does the web app work at different screen sizes (responsive breakpoints)?
- [ ] Does the product handle system font rendering differences?
- [ ] Does the product handle dark mode / light mode?
- [ ] Does the product handle reduced motion preferences?
- [ ] Does the library work in different module systems (ESM, CJS, IIFE)?

---

## Shipping Criteria by Project Type

What "ready to ship" means differs fundamentally by project type. These criteria are adapted from production readiness reviews (Cortex, OpsLevel, Port.io), software release checklists (DECODE, Testsigma), and industry standards.

### Type A: Library / Package / SDK

| Criterion | Must-Have | Nice-to-Have |
|-----------|-----------|--------------|
| API stability | Public API frozen, no breaking changes undocumented | Follows semver strictly |
| Documentation | README with install, basic usage, API reference | Generated API docs (TypeDoc, JSDoc), migration guide |
| Package publishing | Published to package manager (npm, PyPI, crates.io) | CI publish pipeline, Provenance attestation |
| Type definitions | TypeScript/typed interfaces available | Strict mode compatible |
| Versioning | Semantic version applied | Changelog generated from commits |
| Testing | Unit tests for all public API surfaces | Integration tests with real dependencies |
| Edge case handling | Error types defined, all error paths documented | Input validation on all public functions |
| License | LICENSE file present, SPDX identifier in package | Dependency license compliance check |
| Distribution | Works on target platforms (platform-specific builds if needed) | Cross-platform CI matrix |

**Human final pass emphasis:** API ergonomics (Shneiderman #7 — user control). Does the API "feel" right to the consumer? Read the code as if you're a first-time user.

### Type B: Web Application / SaaS

| Criterion | Must-Have | Nice-to-Have |
|-----------|-----------|--------------|
| Reliability | All critical paths tested, error boundaries in place | Uptime monitoring, SLOs defined |
| Security | Auth on all endpoints, input sanitization, CSRF protection | Pen test passed, security headers set |
| Performance | Loads in <3s on 3G, no layout shift >0.1 CLS | Lighthouse score >90, performance budget in CI |
| UX completeness | All states handled (loading, empty, error, edge) | Micro-interactions, animations, transitions |
| Accessibility | WCAG 2.1 AA minimum | WCAG 2.1 AAA or WCAG 3.0 readiness |
| Responsiveness | Works on mobile, tablet, desktop | Adaptive layout, touch-optimized interactions |
| Onboarding | First-run experience, empty states with guidance | In-app tutorials, tooltips |
| Error recovery | Graceful degradation, retry mechanisms, auto-save | Offline mode, optimistic updates |
| Observability | Error tracking, performance monitoring, structured logging | Distributed tracing, user session replay |
| Documentation | README, API docs if applicable, deployment guide | User-facing help center, knowledge base |

**Human final pass emphasis:** Heuristic evaluation + edge case tour. Especially Nielsen #1 (system status), #3 (user control), #9 (error recovery). Run Whittaker's business + seedy districts.

### Type C: CLI Tool / Terminal Application

| Criterion | Must-Have | Nice-to-Have |
|-----------|-----------|--------------|
| Help output | `--help` complete and correct | Man page, shell completions |
| Exit codes | Proper exit codes (0 success, non-zero error) | Different codes for different error types |
| Error messages | Clear, actionable error output | Colorized output (with `--no-color` support) |
| Edge cases | Empty input, very long input, pipe/redirect, stdin/stdout/stderr | Binary mode for non-text data |
| Cross-platform | Works on target OS (Linux, macOS, Windows) | Tested on all 3 |
| Terminal compatibility | Different terminal sizes, terminal types (xterm, tmux, etc.) | TrueColor support, Unicode support |
| Performance | Handles large inputs without OOM | Progress output for long operations |
| Configuration | Config file locations follow OS conventions | Env var overrides, config validation |
| Signals | Handles SIGTERM, SIGINT gracefully | SIGHUP reload for daemon mode |
| Output formatting | Machine-readable output option (JSON, YAML) | `--quiet`, `--verbose` flags |

**Human final pass emphasis:** Error message quality. Test with wrong arguments, missing files, no permissions. The CLI "feel" is about clarity and predictability — does it behave like other well-designed CLIs?

### Type D: Research Prototype / Proof of Concept

| Criterion | Must-Have | Nice-to-Have |
|-----------|-----------|--------------|
| Reproducibility | Clear instructions to run | Dockerfile or devcontainer |
| Hypothesis clarity | README states what was explored and what was found | Written findings document |
| Code organization | Code is findable (reasonable structure) | Clean, idiomatic code |
| Dependencies | Dependency list, exact versions | Lockfile, dependency freeze |
| Data | Sample data provided or generation script | Documented data provenance |
| Limitations | Known issues, unimplemented areas documented | CI badge for what works |
| Reproduction | Reproduction steps for key findings | Automated reproduction script |

**Human final pass emphasis:** Does the prototype actually demonstrate what it claims? Is the README honest about limitations? Research prototypes have a different bar — they need clarity and reproducibility, not production polish.

### Type E: Mobile App

| Criterion | Must-Have | Nice-to-Have |
|-----------|-----------|--------------|
| Platform conventions | Follows HIG (iOS) / Material Design (Android) conventions | Platform-specific navigation patterns |
| Touch targets | All tappable areas >= 44pt (iOS) / 48dp (Android) | Haptic feedback |
| Device coverage | Works on latest 2 OS versions | Tested on top 10 devices |
| Gestures | Standard gestures work (swipe, pinch, pull-to-refresh) | Gesture conflicts resolved |
| Offline | Graceful offline handling | Offline queue with sync |
| Permissions | Permissions requested at point of need (not at launch) | Denied-permission handling |
| Battery/Resources | No excessive CPU/battery drain | Background task limits respected |

**Human final pass emphasis:** Gesture testing, permission flows, platform convention consistency. Run on an actual device, not just simulator.

---

## Recommendations for POLISH.md

### 1. Replace "Real-World Feel Check" with Structured Heuristic Evaluation

The current POLISH.md lists "real-world feel check" as a primary contribution but provides zero methodology. Replace it with:

- A **10-heuristic evaluation protocol** adapted for AI-generated output (use the table in the UX Heuristics section above)
- **Shneiderman's 8 golden rules** as a supplementary lens
- **Cognitive walkthrough** for evaluating learnability of novel interfaces

**Why this works:** Heuristic evaluation is the closest thing to "feel check" without user testing. It has 30 years of validation. It converts the vague "does this feel right?" into ten specific, actionable lenses. "Heuristics tend to surface broader, principle-level issues, while user testing reveals problems tied to real user behavior and context" (NNGroup). For a POLISH phase that runs before user testing, heuristics are the right tool.

### 2. Upgrade Edge Case Hunt with Named Techniques

Current POLISH.md has a list of 5 categories (empty states, error states, boundary conditions, platform quirks, recovery paths). Level it up with:

- **Equivalence Partitioning** — teach the method for generating test inputs systematically
- **Boundary Value Analysis** — teach min-1/min/min+1/max-1/max/max+1 pattern
- **Whittaker's 6 exploratory tours** — provide the district framework for systematic exploration
- **State transition testing** — for multi-step workflows
- **Error guessing checklist** — the AI blindspot catalog as a reusable starting point

**Recommended format for POLISH.md:** A combined flow: "First, partition the input space using EP. Then test boundaries with BVA. Then run Whittaker's Business District tour (core functionality) and Seedy District tour (error states)."

### 3. Add the AI Blindspot Catalog as a Mandatory Pre-Flight Check

Before any human evaluation, the reviewer should run through the 8 failure patterns and check them off. Make this a new section in POLISH.md: "AI-Generated Code Pre-Flight."

### 4. Add Non-Functional Edge Case Checklists

The non-functional edge case checklist in this document should be reformatted into the POLISH.md protocol. Split into:
- **Performance feel check** (5 items — quick)
- **Security boundaries** (7 items — human-catchable without tools)
- **Data integrity** (5 items)
- **Error recovery** (5 items)
- **Accessibility** (11 items — WCAG 2.1 AA minimum + AI-specific checks)
- **Internationalization** (5 items)

### 5. Differentiate Shipping Criteria by Project Type

The current shipping checklist is generic. Add branching logic or separate checklists for: Library/Package, Web App, CLI Tool, Research Prototype, Mobile App.

### 6. Add a Lightweight Retrospective Format

Current POLISH.md references METHODOLOGY.md for retrospective. Add a lightweight "Post-Ship Feedback Log" that captures 3 things in 5 minutes:
1. One thing the AI did surprisingly well
2. One thing the AI missed that cost significant time
3. One protocol change that would have caught it earlier

This feeds directly into protocol improvement without feeling like overhead.

### 7. Recommended POLISH.md Protocol Flow

```
1. QUALITY GATES (automated checks — same as current)
2. AI-GENERATED CODE PRE-FLIGHT (new — 8 failure patterns)
3. EDGE CASE HUNT (upgraded — named techniques + tours)
   a. Equivalence Partitioning + Boundary Value Analysis
   b. Whittaker's Business District + Seedy District tours
   c. Error guessing (using AI blindspot catalog as baseline)
4. UX HEURISTIC EVALUATION (new — replaces "feel check")
   a. Nielsen's 10 heuristics (10-item checklist)
   b. Cognitive walkthrough for complex flows
5. NON-FUNCTIONAL CHECKLIST (new — 6 categories)
6. SHIPPING CHECKLIST (upgraded — differentiated by type)
7. POST-SHIP FEEDBACK LOG (new — lightweight, 5 min)
```

---

## References

- Nielsen, J. (1994, updated 2020, 2026). "10 Usability Heuristics for User Interface Design." NNGroup. https://www.nngroup.com/articles/ten-usability-heuristics/
- Moran, K. & Gordon, K. (2023). "How to Conduct a Heuristic Evaluation." NNGroup. https://www.nngroup.com/articles/how-to-conduct-a-heuristic-evaluation/
- Nielsen, J. & Molich, R. (1990). "Heuristic Evaluation of User Interfaces." Proc. ACM CHI'90.
- Shneiderman, B. et al. (2016). "Designing the User Interface: Strategies for Effective Human-Computer Interaction." 6th Ed. Pearson.
- Shneiderman, B. "The Eight Golden Rules of Interface Design." https://www.cs.umd.edu/users/ben/goldenrules.html
- Lewis, C., Polson, P., Wharton, C., & Reiman, J. (1990). "Testing a Walkthrough Methodology for Theory-Based Design."
- Moran, K. (2024). "How to Conduct a Cognitive Walkthrough Workshop." NNGroup.
- Whittaker, J. A. (2009). "Exploratory Software Testing: Tips, Tricks, Tours, and Techniques." Addison-Wesley.
- Osmani, A. (2025–2026). "My LLM Coding Workflow Going Into 2026" / "The 80% Problem in Agentic Coding."
- Augment Code (2025–2026). "Debugging AI-Generated Code: 8 Failure Patterns & Fixes." / "The 80% Problem." https://www.augmentcode.com/guides
- Varga, E. (2025). "The Edge Case Blind Spot: Why LLMs Stumble at Writing Robust Code." LavX News.
- Veracode (2025). AI-generated code security research.
- GitClear (2025). "AI Assistant Code Quality 2025 Research." https://www.gitclear.com/ai_assistant_code_quality_2025_research
- arXiv 2512.05239 (2025). "A Survey of Bugs in AI-Generated Code."
- arXiv 2605.30777 (2026). "What Breaks When LLMs Code?"
- Pawar, D. R. (2026). "AI-Generated UI Is Inaccessible by Default." Master.dev.
- Heurio (2026). "Vibe Coding Workflow Meets Nielsen's 10 Usability Heuristics."
- Atlassian (2026). "How to Lead a Product Critique." https://www.atlassian.com/agile/product-management/product-critique
- IxDF (2026). "What Are Design Critiques?" / "Shneiderman's Eight Golden Rules." https://ixdf.org/
- Plavljanic, B. / DECODE (2026). "4 Software Release Checklists for Smoother Launches."
- Cortex (2025). "Production Readiness Review Checklist & Best Practices."
- Port.io (2025). "Production Readiness Checklist."
- OpsLevel (2025). "Production Readiness Checklist: An In-Depth Guide."
- Forasoft (2026). "Non-Functional Requirements Checklist: 14 Categories."
- Testsigma (2026). "Error Guessing in Software Testing: A Complete Guide."
- Testsigma (2025). "State Transition Testing Techniques."
- Guru99 (2026). "Boundary Value Analysis and Equivalence Partitioning."

---

*End of research document. Prepared for the Development Protocol POLISH.md rewrite, July 2026.*

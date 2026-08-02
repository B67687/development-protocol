# SPECIFICATION.md — The Plan IS the Spec

> **v3 note:** Before filling this template, run the Intent Decomposition protocol (RULES.md Section 2).
> Each dimension identified in the MECE tree should map to a section in this spec.

> Everything in one document: constitution, overview, architecture, file tree, quality gates,
> dependencies, UX, timeline, testing, operations, release, design for change, documentation,
> ecosystem, AI attribution.
> dependencies, UX, timeline, release, documentation, ecosystem, AI attribution.
> Three layers of detail: MACRO (system), MESO (component), MICRO (implementation).
> An AI executor reads this and knows exactly what to build — no guessing required.

---


> **Input:** This spec consumes validated assumptions from VALIDATION.md.
> If VALIDATION produced learnings (validated hypotheses, invalidated approaches,
> unresolved risks), those directly inform sections 2 (Architecture), 5 (Dependencies),
> and 7 (Timeline). Do not override validated assumptions without explicit re-validation.

---
## How to Read This Spec

---

**Design influences:** The 14-section format synthesizes from:

- Volere (Robertson & Robertson 2006) — requirements shell template (27 to 14 sections)
- IEEE 830 / ISO 29148 — standard SRS structure (functional + non-functional + constraints)
- Shape Up (Singer 2019) — pitch document format (appetite, rabbit holes, no-gos)
- Jackson Problem Frames (Jackson 2001) — domain analysis, interfaces, spec vs requirement

This specification uses a **three-layer model**:

| Layer     | Level          | Scope                                        | Changing this requires                |
| --------- | -------------- | -------------------------------------------- | ------------------------------------- |
| **MACRO** | System         | Decisions that constrain the entire project  | A learning shift (RULES.md section 5) |
| **MESO**  | Component      | Contracts between components                 | Interface renegotiation               |
| **MICRO** | Implementation | Bounds within which the executor has freedom | None — executor decides within bounds |

Each section below follows the format:

- **MACRO**: The system-level decision or constraint (one per section, heavy rationale)
- **MESO**: How this applies to component interfaces and contracts
- **MICRO**: Implementation bounds (leave blank = executor decides freely)

**Priority tiers:** Not all sections need full detail for every project.

- **Tier 1** (sections 0-7): Required for ANY project. Fill these before execution.
- **Tier 2** (sections 8-11): Required for production projects. Skip for prototypes.
- **Tier 3** (sections 12-14): Required for open-source or long-lived projects.

---

## 0. Constitution (Immutable Project Rules)

The constitution constrains ALL executor actions. If an action would violate these rules, the executor MUST refuse.

### MACRO — System Principles

```
[Project Name] Constitution:
1. Correctness — wrong output at any speed is useless
2. No magic — explicit > implicit. Every dependency and config declared.
3. Inward dependencies — core knows nothing about edges.
4. Test what matters — one behavior per test, edge cases before happy path.
5. Fail with context — every error includes the values that caused it, not just a message.
6. Tool-first — never hand-roll what a deterministic tool handles.
7. No new runtime dependency without a Y-Statement decision recorded in section 3.
8. {{additional principle}}
```

### MESO — How Principles Apply at Component Level

_Describe how the constitution applies to inter-component contracts. Example: "Inward dependencies means core/ never imports cmd/ or ui/"_

### MICRO — Implementation Bounds

_Concrete rules the executor follows. Example: "All public functions must have doc comments and tests."_

> **Bus-Hop Example:**
>
> Bus-Hop Constitution:
>
> 1. Correctness - wrong arrival time is useless
> 2. No magic - explicit dependency direction from app -> data -> domain
> 3. Inward dependencies - domain has zero Android imports
> 4. Test what matters - ViewModel state tests over Compose screenshot tests
> 5. Fail with context - API errors include HTTP code, URL, and cached fallback
> 6. Tool-first - Gradle version catalog for dependencies, AGP managed APIs
> 7. Privacy-first - no analytics, no telemetry, no crash reporting
>
> **MESO:** Inward dependencies means data/ implementations depend on domain/ interfaces. app/ depends on domain/ and data/. domain/ imports zero Android SDK classes.
>
> **MICRO:** All ViewModel public methods have a corresponding test. No main-thread I/O. Every API call wrapped in try/catch producing NetworkResult.

---

## 1. Overview & Derived Ambition

### MACRO — System Vision

```
Project name: {{project-name}}
One-line: {{one-line description}}
Core ambition: {{single falsifiable statement}}
Why now: {{what changed — market, technology, personal context}}

Success criteria:
- WHEN the project is complete THEN {{measurable outcome 1}}
- WHEN the project is complete THEN {{measurable outcome 2}}
- WHEN the project is complete THEN {{measurable outcome 3}}

Stakeholders: {{who benefits, who uses, who maintains}}

OUT OF SCOPE (V1):
- {{feature}} — deferred to V2
- {{feature}} — never planned
```

### MESO — Scope Boundaries per Component

_Optional: which components are in/out of scope for V1. Example: "The CLI is in scope. The GUI is not."_

### MICRO — Implementation Bounds

_Constraints on how the ambition is achieved. Example: "No cloud dependencies — fully offline."_

> **Bus-Hop Example:**
>
> **Project name:** Bus-Hop
> **One-line:** Lightweight Singapore bus timing app with real-time arrivals
> **Core ambition:** Show next bus arrival times in under 3 seconds from launch
> **Why now:** LTA DataMall is available, Arrivelah proxy eliminates API key friction, and no lightweight open-source Singapore bus app exists without ads or accounts.
>
> **Success criteria:**
>
> - WHEN the app launches THEN bus arrival times display within 3 seconds
> - WHEN a user searches a bus stop THEN results appear within 500ms
> - WHEN the app runs for 1 hour THEN no crash occurs
>
> **Stakeholders:** Singapore commuters, open-source community, single maintainer
>
> **OUT OF SCOPE (V1):** Route planning, fare calculation, push notifications, iOS, Wear OS
>
> **MESO:** The app module is in scope. The :wear and :ios modules are not. The :domain module is pure Kotlin with zero framework dependencies.
>
> **MICRO:** No cloud dependencies beyond the Arrivelah API. All user data stored locally on device. No accounts.

---

## 2. Architecture & Design Decisions

### MACRO — System Architecture

Each architecture-level decision uses the Y-Statement format:

> In the context of {{situation}}, facing {{concern}}, we decided for {{option}} to achieve {{goal}}, accepting {{downside}}.

```
### Decision 1: {{pattern name}}
In the context of {{what makes this decision necessary}},
facing {{the tradeoff or constraint that forces a choice}},
we decided for {{the chosen approach}}
to achieve {{the specific benefit this decision enables}},
accepting {{the cost, complexity, or limitation this imposes}}.

Alternatives considered:
- {{alternative 1}} — rejected because {{reason}}
- {{alternative 2}} — rejected because {{reason}}
```

### MESO — Component Contracts

_For each major component, specify its interface contract with other components. Communication protocol, data format, error handling between modules._

### MICRO — Implementation Bounds

_Coding constraints. Examples: "No unsafe code blocks", "Must use const generics for bit-width parameters", "Benchmarks required for all public APIs."_

### PROJECT_MODEL — Whole-Project State Machine (MANDATORY)

Every project MUST document its whole-project state machine at `docs/PROJECT_MODEL.md`.
This is not optional polish — it is the project's health contract.

**The document contains:**

| Element | What it is | Example |
| --- | --- | --- |
| **States** | The project's lifecycle states | `IDEA → SPEC'D → PROTOTYPED → IMPLEMENTED → POLISHED → SHIPPED → MAINTAINED → EVOLVED` |
| **Valid transitions** | Allowed state changes, including feature additions and rollbacks | `SHIPPED → EVOLVED` (new feature), `SHIPPED → MAINTAINED` (bugfix) |
| **Invalid transitions** | Forbidden state changes (the invariants) | `IMPLEMENTED → SHIPPED` without passing POLISHED |
| **Invariants** | What must never change during any transition | "Database schema migrations are always backward-compatible", "API v1 responses never remove fields" |
| **Blast radius map** | Which components are coupled and co-change together | "changing file A requires checking file B" (from git co-change analysis) |

**Why it prevents regressions:** Every feature addition is a state transition. With
a documented transition table, an addition that violates an invariant is caught at
spec time, not discovered as a production regression. Research backing (see
EXECUTOR.md § Project Health Discipline): state-machine modeling with transition-
table tests caught 26 real field faults in an industrial case study (Torkar), and
co-change analysis predicts defects at p=0.01 (D'Ambros).

**Gate check:** SPECIFICATION is incomplete without PROJECT_MODEL.md. REVIEW
checks that every addition's transition is in the table.

> **Bus-Hop Example:**
>
> **Decision 1: Clean Architecture (3-module)**
> In the context of a single-developer Android project with clear layer separation requirements,
> facing the tradeoff between monolithic simplicity and long-term testability,
> we decided for a 3-module Clean Architecture to achieve one-way dependency direction and framework-independent domain logic,
> accepting slower initial setup and Gradle configuration overhead.
>
> **Decision 2: Manual DI over Hilt**
> In the context of a solo-dev app with 3 modules and ~4,000 LOC,
> facing Hilt's 2+ minute annotation processing per build,
> we decided for manual constructor injection through ViewModel factories to achieve sub-30s build times and explicit dependency wiring,
> accepting manual factory boilerplate.
>
> **Decision 3: Arrivelah over LTA DataMall**
> In the context of needing real-time bus arrival data without API key management,
> facing LTA's per-key rate limits that break open-source distribution,
> we decided for the Arrivelah proxy to achieve zero-config API access for all users,
> accepting dependency on a third-party proxy.
>
> **Alternatives considered:** LTA DataMall v1 direct - rejected because API key distribution breaks open-source model.
>
> **MESO:** domain/ defines repository interfaces. data/ implements them. app/ consumes both. No module imports against the dependency direction.
>
> **MICRO:** No unsafe code. All API responses parsed into sealed NetworkResult types. Exhaustive when required.

---

## 3. File Tree & Module Responsibilities

### MACRO — Directory Structure

```
{{path}}/ — {{macro responsibility}}
```

### MESO — Per-Module Contracts (Parnas-style)

```
{{path}} — {{one-line responsibility}}
  HIDES: {{design decision this module encapsulates}}
  EXPORTS: {{public interface — what other modules can call}}
  CALLER: {{which other modules depend on this}}
  Precondition: {{what must be true before calling}}
  Postcondition: {{what is guaranteed after calling}}
  Invariant: {{what is always true}}
```

### MICRO — Per-File Implementation Bounds

_Naming conventions, file size limits, organization rules per file._

> **Bus-Hop Example:**
>
> ```
> Bus-Hop/
> +-- domain/ - pure Kotlin models, use cases, repository interfaces
> +-- data/ - Android library, Retrofit API, DataStore, search index
> +-- app/ - Android app, Compose UI, ViewModels, theme
> +-- docs/ - documentation, screenshots, badges
> +-- scripts/ - release automation, CI helpers
> +-- .github/ - CI workflows, issue templates
> ```
>
> **MESO (domain/):** HIDES sorting algorithm, pinning logic, refresh coordination. EXPORTS BusStop, BusService, BusArrival, BusRepository, NetworkResult. CALLER app/, data/. Invariant: zero framework imports.
>
> **MESO (data/):** HIDES Retrofit/OkHttp config, DataStore keys, TokenTrie internals. EXPORTS BusRepositoryImpl, BusStopIndex, UpdateChecker. CALLER app/.
>
> **MESO (app/):** HIDES Compose internals, ViewModel state management. EXPORTS APK. CALLER none (leaf module).
>
> **MICRO:** Kotlin files in domain/ have no Android SDK imports. Max 400 lines per Kotlin file. Compose files only in app/ module.

---

## 4. Quality Gates & Verification

> *Engineering-specific CI/Tooling details moved to [Engineering Plugin](docs/engineering-plugin.md#1-ci-tooling--quality-gates).*

### MACRO — Quality Gates

Define the acceptance criteria that gate each deliverable. Use EARS notation:
> WHEN {{condition}} THEN {{action}} WHERE {{error}} THEN {{error-handling}}

> **Example:**
 > WHEN a pull request is opened
 > THEN verification SHALL run
 > WHERE verification fails
 > THEN the change SHALL be rejected with results.

> *For engineering projects, replace this with concrete CI commands from the Engineering Plugin §1.*

### MESO — Per-Component Verification

_Which components have specific verification requirements (e.g., fuzz testing for the parser module)?_

### MICRO — Tool Configuration Bounds

_Specific tool rules, format config, hooks. Concrete command-line invocations._

> *Engineering projects: see Engineering Plugin §1 for patterns with linters, formatters, commit hooks, and CI matrix configuration.*
---

## 5. Dependencies & External Contracts

### MACRO — System Dependencies

```
| Package | Version | Purpose | Contract | License |
|---------|---------|---------|----------|---------|
| {{name}} | {{version}} | {{why this exists}} | {{what it provides}} | {{license}} |
```

### MESO — Dependency Decision Records

_Why each dependency was chosen over alternatives. Y-Statement format._

### MICRO — Version Policy

_Pin exact versions? Accept semver ranges? Update cadence?_

> **Bus-Hop Example:**
>
> | Package     | Version              | Purpose               | Contract                      | License    |
> | ----------- | -------------------- | --------------------- | ----------------------------- | ---------- |
> | Kotlin      | 2.4.0                | Language              | JVM bytecode                  | Apache-2.0 |
> | Compose BOM | 2026.05.01           | UI framework          | Compose API surface           | Apache-2.0 |
> | Material 3  | (via BOM)            | Design system         | Material Design components    | Apache-2.0 |
> | Retrofit    | 3.x                  | HTTP client           | REST API to Kotlin interfaces | Apache-2.0 |
> | OkHttp      | 5.x                  | HTTP transport        | Request/response lifecycle    | Apache-2.0 |
> | Gson        | (Retrofit converter) | JSON parsing          | JSON to DTO mapping           | Apache-2.0 |
> | DataStore   | (Jetpack)            | Key-value persistence | Async Flow-based reads        | Apache-2.0 |
> | MockK       | 1.x                  | Test mocking          | Kotlin mock library           | Apache-2.0 |
> | JUnit 4     | 4.x                  | Test framework        | Unit test execution           | EPL-2.0    |
>
> **MESO:** Arrivelah API (arrivelah2.busrouter.sg) is the sole external runtime dependency. LTA DataMall accessed through Arrivelah proxy. No API key required. No other external services.
>
> **MICRO:** Pin exact versions in libs.versions.toml. Dependabot opens weekly PRs for updates. No new runtime dependency added without Y-Statement.

---

## 6. UX & Interface Contract

### MACRO — User-Facing Behavior

```
### Entry Points
- {{entry point}} — {{what it does}} ({{who uses it}})

### User-Facing Behavior (EARS)
WHEN {{user action}}
THEN the system SHALL {{system response}}
WHERE {{error condition}}
THEN the system SHALL {{error response}}
```

### MESO — Error Contract

```
| Condition | Error | Remediation | Log Level |
|-----------|-------|-------------|-----------|
| {{what goes wrong}} | {{error code/message}} | {{how the system responds}} | {{info/warn/error}} |
```

### MICRO — UI/API Implementation Bounds

_Design tokens, naming conventions, API style rules._

> **Bus-Hop Example:**
>
> **Entry Points:** stop list (main screen) - pinned stops with arrivals. Search dialog - find stops by name/code. Settings - theme, refresh interval, about.
>
> WHEN the app launches THEN the system SHALL display pinned stops with current arrival times. WHERE no internet THEN the system SHALL show stale cached data with an offline indicator.
>
> WHEN the user pulls down on the stop list THEN the system SHALL refresh all arrival data. WHERE the API returns an error THEN the system SHALL show cached data and display an API health warning.
>
> WHEN the user taps a stop card header THEN the system SHALL expand/collapse the service list with animation.
>
> **MESO (Error Contract):** API 5xx -> API_DOWN -> health banner + cached data -> error. No internet -> NETWORK_UNAVAILABLE -> offline indicator per stop -> warn. API timeout -> stale data shown -> reconnect auto-refresh.
>
> **MICRO:** Material 3 themed with Classic Blue primary. Dynamic Color as optional. All icons use Material Icons. 16sp body text minimum. Theme selection persists across restarts.

---

## 7. Timeline, Milestones & Checkpoints

### MACRO — Project Appetite

```
Appetite: {{time available for this project}}

| Milestone | What ships | Checkpoint | Acceptance Criteria |
|-----------|------------|------------|---------------------|
| M1 | {{what's done}} | {{how to verify}} | WHEN M1 is complete THEN {{measurable}} |
| M2 | {{what's done}} | {{how to verify}} | WHEN M2 is complete THEN {{measurable}} |

Circuit breaker: IF {{condition}} THEN the project SHALL {{stop/reassess}}
Contingency: {{what happens if appetite exceeded}}
```

### MESO — Phase Timeline per Component

_Which components ship in which milestone? Dependencies between components._

### MICRO — Per-Milestone Implementation Bounds

_What quality level is acceptable per milestone? (M1: prototype quality, M2: production quality)_

> **Bus-Hop Example:**
>
> **Appetite:** ~20 days (Phases 0-7)
>
> | Milestone | What ships         | Checkpoint                                      |
> | --------- | ------------------ | ----------------------------------------------- |
> | M1        | Research complete  | docs/ with API spec, data model, theme decision |
> | M2        | Domain layer       | 49 domain tests pass                            |
> | M3        | Data layer         | 57 data tests pass                              |
> | M4        | App shell          | Running app with stop list, search, settings    |
> | M5        | Polished UI        | Visual QA passes, APK under 2MB                 |
> | M6        | CI pipeline        | All CI jobs green                               |
> | M7        | Production release | v1.0.x signed APK published                     |
>
> **Circuit breaker:** IF API latency exceeds 5 seconds for 3 consecutive calls THEN re-evaluate API choice.
> **Contingency:** Ship M4 as v0.1.0 and defer polish if appetite exceeded.
>
> **MESO:** domain/ ships in M2, data/ in M3, app/ in M4. CI (M6) can start in parallel with M2. Production (M7) after M4.
>
> **MICRO:** M1-M3 prototype quality. M4-M5 production quality (all error states tested). M6-M7 release quality (security review complete).

---

## 8. Testing Strategy (Tier 2)

### MACRO — Test Philosophy

```
Unit coverage target: {{percentage}}
Integration scope: {{component interactions tested}}
E2E coverage: {{user journeys tested}}
Framework: {{test framework}}
```

### MESO — Per-Component Test Requirements

```
| Module | Test Type | Target | Notes |
|--------|-----------|--------|-------|
| {{module}} | {{unit/integration/e2e}} | {{coverage or count}} | {{special requirements}} |
```

### MICRO — Test Implementation Bounds

_Fixture conventions, naming patterns, assertion style._

> **Bus-Hop Example:**
>
> **Unit coverage:** 80% line coverage (JaCoCo enforced)
> **Integration scope:** API parsing, DataStore read/write, search index correctness
> **E2E coverage:** Manual verification on emulator (no Compose screenshot tests)
> **Framework:** JUnit 4 + MockK + Coroutines Test
>
> **MESO:** domain/ - 49 unit tests (pure Kotlin, no mocks). data/ - 57 tests (MockK for API responses). app/ - 47 ViewModel state tests (fake data sources). Architecture - 8 layer-separation tests.
>
> **MICRO:** One behavior per test. Naming: feature_scenario_expectedResult. runTest for coroutines. TestDispatcher for time-dependent tests. No dependency on Android device.

---

## 9. Operational Resilience (Tier 2)
> *Engineering-specific operations details moved to [Engineering Plugin](docs/engineering-plugin.md#2-operational--error-handling).*

### MACRO — Resilience Strategy

```
Error tracking: {{how errors are captured}}
Fallback behavior: {{what happens when things fail}}
Recovery mechanism: {{how the system recovers}}
Load handling: {{how the system behaves under stress}}
```

### MESO — Component Resilience Contracts

_How errors propagate between components. What's handled locally vs. escalated._

### MICRO — Error Implementation Bounds

_Error message format, log line format, structured conventions._

> *Engineering projects: see Engineering Plugin §2 for operational patterns including logging frameworks, metrics, alerts, and observability.*
---

## 10. Build & Release Pipeline (Tier 2)

### MACRO — Release Strategy

```
Versioning scheme: {{semver / calendar / other}}
Release cadence: {{continuous / scheduled / milestone-based}}
Changelog: {{auto-generated / manual}}
```

### MESO — Per-Component Release Requirements

_Which components are released independently vs. together?_

### MICRO — Release Implementation Bounds

_Packaging commands, signing requirements, distribution channels._

> **Bus-Hop Example:**
>
> **Versioning:** Semantic Versioning (MAJOR.MINOR.PATCH)
> **Cadence:** Milestone-based (not continuous)
> **Changelog:** Manual (Keep a Changelog format)
>
> **MESO:** Single APK per release (no split by ABI). All 3 modules released together. Separate debug and release build types.
>
> **MICRO:** Release APK: R8 minified, shrinkResources, signed. scripts/release.sh automates version bump, CHANGELOG update, assembleRelease, git tag, gh release create. APK verified after build.

### MESO — Distribution Surfaces (enumeration required)

_Every product surfaces to its audience through a set of distribution surfaces. Enumerate ALL of them and when each is built — do not retrofit the adoption engine after the core is done (ithmb lesson: website added on top of the codec instead of planned)._

| Surface | Built when? | Purpose / audience |
| ------- | ----------- | ------------------ |
| Website / landing | {{when — often the ADOPTION ENGINE for open-core, not polish}} | {{converts visitors, ranks search terms}} |
| Web-app / demo | {{when}} | {{interactive proof, e.g. WASM demo}} |
| Docs site | {{when}} | {{API reference, tutorials}} |
| CLI | {{when}} | {{power users, scripting}} |
| WASM demo | {{when}} | {{browser reach, zero-install evaluation}} |
| C ABI / bindings | {{when — typically deferred until enterprise demand}} | {{language interoperability}} |

Distribution surfaces differ by money tier: **library** = docs + examples + web demo; **platform** = hosted service + dashboard + status page. List each surface's build milestone in Section 7 (Timeline).

---

## 11. Design for Change

Intent: How does this project make goalpost shifts cheap instead of expensive?

Before architecture decisions, note which design-for-change rules apply:

| Rule                                              | Applied? | How                                    |
| ------------------------------------------------- | -------- | -------------------------------------- |
| Interface Rule (no interface before 2nd consumer) | Yes/No   | [specific modules where this applies]  |
| Test Rule (contract over implementation)          | Yes/No   | [test strategy]                        |
| Module Boundary (single entry point)              | Yes/No   | [module structure]                     |
| Size Rule (250/40 LOC limits)                     | Yes/No   | [enforcement mechanism]                |
| Cycle Rule (shippable per cycle)                  | Yes/No   | [cycle length, what "shippable" means] |
| Appetite Rule (time before scope)                 | Yes/No   | [time budget for this project]         |
| AI Rule (same structural checks)                  | Yes/No   | [how AI code is reviewed]              |
| Rule of Three (extract on 3rd)                    | Yes/No   | [abstraction strategy]                 |
| Dependency Rule (core ≠ infra)                    | Yes/No   | [module dependency direction]          |
| Clean Backlog (no perpetual)                      | Yes/No   | [how old ideas compete]                |

---

## 12. Documentation Strategy (Tier 3)

### MACRO — Documentation Plan

```
README: {{what it covers}}
API docs: {{auto-generated / manual}}
Tutorials: {{what use cases are covered}}
Examples: {{minimal working examples}}
Migration guides: {{when needed}}
```

### MESO — Per-Module Documentation Requirements

_Doc comments required on public APIs? Example usage per module?_

### MICRO — Documentation Format Bounds

_Markdown style guide, doc comment format, example code style._

> **Bus-Hop Example:**
>
> **README:** Project overview, features, architecture, build instructions, testing, API, privacy
> **API docs:** Not generated (no public library API)
> **Tutorials:** Not applicable (consumer app)
> **Examples:** Not applicable
> **Migration guides:** Not applicable
>
> **MESO:** Public interfaces in domain/ have KDoc comments. ViewModel methods documented with param and return tags. API response DTO fields documented inline.
>
> **MICRO:** Markdown for all docs. KDoc for public API functions. Inline // for implementation decisions. Commit messages follow Conventional Commits.

---

## 13. Ecosystem & Community (Tier 3)

### MACRO — Governance

```
License: {{SPDX identifier}}
Contribution model: {{accepting contributions / maintainer-only}}
Code of conduct: {{url or reference}}
Plugin API: {{exists / planned / never}}
Standards compliance: {{what standards the project follows}}
```

### MESO — Integration Points

_API contracts for plugins, data interchange formats, upstream/downstream dependencies._

### MICRO — Contribution Bounds

_PR requirements, review checklist, commit message conventions._

> **Bus-Hop Example:**
>
> **License:** MIT
> **Contributions:** Accepting (PR-based, single maintainer)
> **Code of conduct:** Contributor Covenant
> **Plugin API:** Never (consumer app, no extension points)
> **Standards:** Clean Architecture, Material 3, WCAG 2.2 AA, Conventional Commits
>
> **MESO:** No plugin API. No data interchange format beyond Arrivelah API contract. Upstream: LTA DataMall through Arrivelah. Downstream: none.
>
> **MICRO:** PR requirements: tests pass, CI green, CHANGELOG updated, Conventional Commits. Review checklist: architecture violation check, test coverage, privacy impact assessment.

---

## 14. AI Attribution & Transparency (Tier 3)

### MACRO — Policy

```
Disclosure level: {{None | Minimal | Standard | Full}}
Rationale: {{why this level was chosen}}
```

### MESO — Tool Inventory

```
| Tool | Version | Permitted Uses | Citation Format |
|------|---------|----------------|-----------------|
| {{tool}} | {{version}} | {{code gen / review / docs}} | {{commit trailer}} |
```

### MICRO — Attribution Format

_Commit message trailer format, file header format, documentation attribution._

> **Bus-Hop Example:**
>
> **Disclosure level:** Full
> **Rationale:** Built with heavy AI assistance (GPT 5.4, DeepSeek V4 Flash). Full transparency builds trust with users and contributors.
>
> **MESO:** GPT 5.4 used for code generation, test writing, documentation. DeepSeek V4 Flash used for code generation, review, refactoring. Both cited with Co-authored-by trailers.
>
> **MICRO:** Commit trailers for AI-generated code. CREDITS.md lists all AI tools. README badges show AI assistance. No AI attribution in binary or APK.

---

## 15. Verification Checklist (Executor Reads Before Starting)

- [ ] All `{{placeholders}}` across all sections are filled
- [ ] No "TODO" or "TBD" remains
- [ ] Constitution (section 0) has at least 3 principles
- [ ] Out-of-scope list (section 1) is non-empty
- [ ] Each architecture decision (section 2) includes a Y-Statement
- [ ] Each dependency (section 5) has a version constraint
- [ ] Timeline (section 7) has a circuit breaker condition
- [ ] Tier 1 sections 0-7 are fully filled
- [ ] Tier 2 sections 8-11 are filled for production projects
- [ ] Tier 3 sections 12-14 are filled for open-source projects
- [ ] **Spec self-consistency grep** — run `grep -n "Step " SPEC.md` and confirm step/phase numbering is monotonic with no duplicates before EXECUTOR starts

For engineering deliverables, also verify from the [Engineering Plugin](docs/engineering-plugin.md):
- [ ] Quality gates (plugin §1) have concrete commands
- [ ] Fuzz targets exist in `fuzz/` directory (Tier 2+)
- [ ] Benchmark suite exists in `benches/` (performance-sensitive)
- [ ] Snapshot testing configured (plugin §3)
- [ ] cargo-deny / deny.toml exists (Tier 2+)
- [ ] Multi-platform CI matrix configured (plugin §3)
- [ ] Test-to-source ratio meets 0.5× minimum

---

## Meta: How to Write This Spec

1. **Start at Tier 1 (sections 0-7)** — these are required for any project. Fill them first.
2. **Add Tier 2 (sections 8-11)** for production projects. These prevent operational surprises.
3. **Add Tier 3 (sections 12-14)** for open-source or long-lived projects.
4. **MACRO must be filled for every section you use.** MESO and MICRO are optional but recommended.
5. **Use the three-layer format** — MACRO for system decisions, MESO for component contracts, MICRO for implementation bounds.
6. **Lock sections after execution starts** - changes go through RULES.md learning shift rules.

---

## Origin

Rewritten July 2026 after a comprehensive research pass on software specification dimensions. Three-layer model (macro/meso/micro) based on architecture decomposition principles. 15 dimensions synthesized from IEEE 830, Shape Up, ADR practice, and production software requirements.

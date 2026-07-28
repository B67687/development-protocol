# Research: Comprehensive Software Specification

**Date**: 2026-07-11
**Purpose**: Inform the design of a three-layer (macro/meso/micro) SPECIFICATION.md template that covers everything — architecture, ecosystem, AI attribution, error handling, contributions, testing, documentation, build/release, and operational concerns.
**Context**: The current SPECIFICATION.md (7 sections: Constitution, Overview, Architecture, File Tree, CI/Gates, Dependencies, UX/Contract, Timeline) was rewritten July 2026 following prior research. The user now wants a truly comprehensive template that covers "contributions, ecosystem, AI attribution, error handling, code architecture, every decision at every level" using a three-layer model.

---

## Summary

The ideal comprehensive software specification does not exist as a single published standard. Instead, it must be synthesized from multiple established traditions:

1. **IEEE 830 / ISO/IEC/IEEE 29148** — The canonical SRS structure (Introduction → Overall Description → Specific Requirements → Verification → Appendices). Provides the "what" but not the "how" or the "why."
2. **Architecture Decision Records (ADRs)** — Capture rationale and tradeoffs for every decision. Y-Statement format is the most compact and AI-parseable variant.
3. **Shape Up / Appetite-based planning** — Provides the "how much time" dimension through appetites rather than estimates.
4. **Spec-Driven Development (SDD) / GitHub Spec Kit / OpenSpec** — 2025-2026 movement treating the spec as the primary artifact. Adds constitution, EARS requirements, and delta-based change management.
5. **Three-layer architecture documentation (Macro/Meso/Micro)** — Used in microservices literature, real estate analysis, complex systems theory, and software architecture textbooks. Maps to system-level/component-level/implementation-level granularity.
6. **AI Attribution frameworks** — Emerging 2025-2026 standards (LLVM "Human-in-the-Loop," Ghostty disclosure, git-ai, Co-Authored-By, AID Framework) for tracing AI-generated vs. human-written code.
7. **Open source governance practices** — CONTRIBUTING.md, CODE_OF_CONDUCT.md, LICENSE, SECURITY.md, CHANGELOG.md, versioning (SemVer/CalVer), deprecation policies.
8. **Testing strategy documentation** — Unit/integration/e2e/fuzz testing, coverage targets, CI pipeline gates, quality metrics.

The key finding: **a comprehensive specification should cover 14 distinct dimensions**, organized across three layers of granularity. The current SPECIFICATION.md covers 7 of these 14 dimensions partially; none are covered at all three layers.

---

## Sources Surveyed

### Standards & Official Documents
- **IEEE Std 830-1998** (superseded by ISO/IEC/IEEE 29148:2011) — The foundational SRS standard. Sections: Introduction (purpose, scope, definitions, references, overview), Overall Description (product perspective, functions, user characteristics, constraints, assumptions/dependencies), Specific Requirements (external interfaces, functions, non-functional requirements, design constraints), Appendices.
- **ISO/IEC/IEEE 29148:2018** — Modern replacement for IEEE 830. Adds requirements engineering lifecycle processes, verification requirements, traceability. Emphasizes stakeholder needs → requirements → validation as continuous activities.
- **IEEE 1016-2009** — Software Design Descriptions. Covers how design is documented (architectural design, detailed design, interface design, data design). Distinguishes requirements (what) from design (how).
- **IEEE 829-2008** — Software Test Documentation. Test plan, test design specs, test case specs, test procedure specs, test item transmittal reports, test logs, test incident reports, test summary reports.

### Spec-Driven Development (2025-2026)
- **BCMS — "Spec-Driven Development: The Definitive 2026 Guide"** (May 2026). Defines SDD four-phase loop: Specify → Plan → Tasks → Implement. Introduces EARS notation as the standard for AI-readable requirements.
- **arXiv 2602.00180 — "Spec-Driven Development: From Code to Contract in the Age of AI Coding Assistants"** (Feb 2026). Defines three rigor levels: spec-first, spec-anchored, spec-as-source.
- **GitHub Spec Kit** (111k+ stars, 2025-2026). Reference SDD implementation. Four-phase workflow with slash commands. Constitution file, spec.md, plan.md, tasks.md. Works with 30+ AI agents.
- **OpenSpec (Fission-AI)** — Spec-driven development framework. Structured Markdown format with Purpose + Requirements sections. Uses SHALL/MUST normative language. Delta-based change management. Works with 20+ AI assistants.
- **Augment Code — "What Is Spec-Driven Development?"** (Apr 2026). Identifies six elements a spec must answer: outcomes, scope boundaries, constraints, prior decisions, task breakdown, verification criteria.

### Architecture Documentation
- **Parnas, David L. — "On the Criteria To Be Used in Decomposing Systems into Modules"** (1972). Information hiding: modules hide design decisions likely to change. Interface IS the specification.
- **Jackson, Michael — Problem Frames** (2000). Software problems decompose by frame type (required behavior, commanded behavior, information display, workpieces, transformation, connection).
- **Y-Statement format** (hidekazu-konishi.com, May 2026). "In the context of [situation], facing [concern], we decided for [option] to achieve [goal], accepting [downside]."
- **Design by Contract — Bertrand Meyer** (1986+). Preconditions, postconditions, invariants for every public interface.
- **EARS Notation — Alistair Mavin** (2009). Five requirement patterns: Ubiquitous, Event-driven (WHEN/THEN), State-driven (WHILE/THEN), Unwanted behavior (IF/THEN), Optional (WHERE/THEN). Used by Airbus, Bosch, NASA, Rolls-Royce.

### AI Attribution (2025-2026)
- **LLVM "Human-in-the-Loop" policy** (January 2026) — Requires `Assisted-by` trailers for AI-generated contributions. Contributors must certify they reviewed and understand every line.
- **Ghostty mandatory disclosure** (August 2025) — Requires AI attribution in PR descriptions. Tool: `git-ai` for tracking AI-generated code snapshots.
- **Fedora AI policy** — Prohibits AI-generated code from unapproved training data sources.
- **AID Framework (Artificial Intelligence Disclosure)** — Kari D. Weaver, University of Waterloo (2024). Structured disclosure: tool name, version, specific tasks, human verification assertion.
- **git-ai tool** — Traces AI-generated code with checkpoint snapshots. AI Code Halflife: ~3.33 years median.
- **EU AI Act, Article 50** — Enforceable August 2026. Requires marking of AI-generated content. Draft Code of Practice published December 2025.
- **Disclosure Spectrum** (cc.bruniaux.com): None → Minimal (Co-Authored-By) → Standard (Assisted-by + PR disclosure) → Full (git-ai + prompt preservation).

### Macro/Meso/Micro Models
- **Microservices Architecture** (ebrary.net, microservices.io) — Macro architecture: decisions concerning overall system (domain architecture, basic technologies, communication protocols). Micro architecture: decisions individual teams can make independently.
- **Dopfer, Potts — "Micro-Meso-Macro"** (evolutionary economics) — Three analytical domains. Meso bridges micro and macro, accounting for systems dynamics and changes in agent behaviors and relationships.
- **Organic Computing / Complex Systems** — Three-layer framework where meso acts as bridge, managing population-level rule dynamics.
- **Architecture literature (real estate, site analysis)** — Macro location (city/region), Meso location (neighborhood), Micro location (immediate surroundings). Framework adapts to software via different granularity levels.

### Error Handling & Ops
- **IEEE 830** treats error handling under functional requirements and system attributes (reliability, safety).
- **SRE literature (Google SRE books)** — Error budgets, SLIs/SLOs/SLAs, observability pillars (logs, metrics, traces), incident response, postmortems.
- **Cloud-native patterns** — Circuit breakers, retries with backoff, bulkheads, timeouts, graceful degradation.

### Documentation & Onboarding
- **GitHub docs on CONTRIBUTING.md** — Standard template covering Code of Conduct, how to contribute, development setup, PR process, style guide.
- **API documentation best practices** — OpenAPI/Swagger, interactive sandboxes, quickstart guides, authentication walkthroughs, SDKs.
- **Documentation types** — README, API docs, tutorials, how-to guides, reference docs, explanation docs (Divio model), migration guides, changelog.

---

## Spec Dimensions (Complete List)

Based on synthesizing all sources, a truly comprehensive software specification covers these **14 dimensions**. Each dimension has specific sub-dimensions.

### 1. Constitution & Immutable Rules
Project-wide principles that constrain ALL executor actions. Cannot be overridden.
- Core principles (correctness, no magic, inward dependencies, etc.)
- Project-specific additions
- Conflict resolution rules (constitution wins over all)

### 2. Ambition & Scope
Falsifiable hypothesis of success. Boundaries.
- Core ambition (single falsifiable statement)
- Success criteria (EARS measurable)
- Why now (external context)
- In-scope (V1 features)
- Out-of-scope (V1 explicit no-gos)
- Out-of-scope (never planned)
- Stakeholders and user characteristics

### 3. Architecture & Design Decisions
Macro-level system structure and rationale for every significant decision.
- Architecture pattern(s) and rationale
- Y-Statement for each architecture decision
- Alternatives considered (at least one rejected with reason)
- Tradeoffs accepted
- System decomposition principles (information hiding)

### 4. File Tree & Module Responsibilities
Complete module/component map with interface contracts.
- Module path and responsibility
- HIDES (design decision encapsulated)
- EXPORTS (public interface)
- CALLER (dependents)
- Preconditions / Postconditions / Invariants (Design by Contract)

### 5. CI, Tooling & Quality Gates
What must pass for the project to be acceptable.
- Build pipeline (commands, expected outputs)
- Lint/format gates
- Type checking gates
- Test gates (unit, integration, e2e)
- Fuzz testing gates
- Security scanning gates
- Performance benchmark gates
- Release process (versioning, packaging, publishing)

### 6. Dependencies & External Contracts
Every external dependency precisely declared.
- Runtime dependencies (name, version constraint, purpose, contract, license)
- Dev dependencies (name, version, purpose)
- External APIs (name, purpose, auth mechanism, SLA expectations, rate limits)
- Dependency decision records (why this library over alternatives)

### 7. UX & Interface Contracts
User-facing behavior and system API contracts.
- Entry points (user-facing and programmatic)
- User-facing behavior in EARS notation
- Error contract (conditions, error codes, remediation)
- Interface contracts (pre/post/invariant per module)
- Wireframes or UI descriptions (rough, not mockups)

### 8. Timeline, Milestones & Checkpoints
When things get done, and what "done" means.
- Appetite (time constraint, not estimate)
- Milestones with acceptance criteria
- Checkpoints (how to verify milestone completion)
- Circuit breaker conditions
- Contingency plan (what happens if appetite exceeded)

### 9. Testing Strategy
How quality is verified at every level.
- Unit test requirements (coverage targets, per-function testing)
- Integration test requirements (component interaction testing)
- End-to-end test requirements (user journey testing)
- Fuzz testing requirements (inputs, corpus, coverage targets)
- Performance/load testing (thresholds, scenarios)
- Security testing (SAST, DAST, dependency scanning)
- Test environment requirements
- Regression strategy
- Test data management

### 10. Operational & Error Handling
What happens in production — failures, monitoring, recovery.
- Error classification (error codes, severity levels)
- Logging requirements (what to log, at what level, structured format)
- Metrics and observability (SLIs, SLOs, dashboards)
- Alerting rules (what triggers, notification channels, escalation)
- Incident response (severity matrix, response times, postmortem process)
- Backup/recovery procedures
- Graceful degradation behavior
- Circuit breaker configuration

### 11. Build & Release Pipeline
How software moves from source to production.
- Build process (commands, artifacts, signing)
- Versioning scheme (SemVer, CalVer, or custom)
- Changelog conventions (Keep a Changelog, Conventional Commits)
- Release cadence (continuous, scheduled, phased)
- Rollback procedures
- Feature flags / toggles strategy
- Migration strategy (data, schema, API version)

### 12. Ecosystem & Community
How the project relates to its surrounding environment.
- Relationship to upstream dependencies (forks, patches sent upstream)
- Relationship to downstream consumers (compatibility guarantees)
- Plugin/extension API (if applicable)
- Integration points (data interchange formats, protocols)
- Adapter/connector strategy (supported external systems)
- Standards compliance (which standards the project implements)
- Contribution model (maintainer-led, community-led, BDFL, etc.)
- Contribution guidelines (how to contribute, PR workflow, review criteria)
- Code of conduct
- Community channels (discussion forum, chat, mailing list)

### 13. Documentation Strategy
What documentation exists, where, and how it's maintained.
- README (what, why, quick start, badges)
- API reference docs (auto-generated from code/OpenAPI)
- Tutorials (step-by-step, beginner-oriented)
- How-to guides (task-oriented)
- Explanation docs (architecture, concepts)
- Migration guides (version-to-version)
- Examples repository or gallery
- Docs-as-code toolchain (linters, formatters, CI checks)
- Documentation ownership and review process

### 14. AI Attribution & Transparency
How AI-assisted work is tracked and disclosed.
- Disclosure level (per disclosure spectrum: none / minimal / standard / full)
- Commit trailer conventions (Co-Authored-By, Assisted-by)
- AI tool inventory (which tools used, versions)
- Spec-level attribution (which spec sections were AI-generated)
- Prompt preservation policy (if applicable)
- Review requirements for AI-generated contributions
- Compliance notes (EU AI Act, organizational policy)

---

## Three-Layer Model (Macro/Meso/Micro)

### How the Layers Map

| Dimension | Macro (System-Level) | Meso (Component-Level) | Micro (Implementation-Level) |
|-----------|--------------------|----------------------|---------------------------|
| **1. Constitution** | Immutable project-wide rules | Team/component-level conventions | Function/file-level conventions |
| **2. Ambition** | Falsifiable hypothesis, success criteria, scope boundaries | — (not layerable — ambition is inherently macro) | — |
| **3. Architecture** | System architecture pattern, technology stack, decomposition | Module interface contracts, communication protocols | Algorithm choices, data structure decisions |
| **4. File Tree** | Top-level directory structure, package layout | Module responsibilities, HIDES/EXPORTS per component | File-level function/class contracts |
| **5. CI/Gates** | Pipeline architecture, overall quality thresholds | Per-component testing requirements | Per-function test coverage, lint rules |
| **6. Dependencies** | Core framework, language runtime, database | Library choices per component | Transitive dependency pinning |
| **7. UX/Contract** | User-facing entry points, overall error strategy | API contracts between components | Function-level pre/post conditions |
| **8. Timeline** | Overall appetite, major milestones | Per-component delivery order | Task-level estimates (if any) |
| **9. Testing** | Test pyramid strategy, coverage targets overall | Integration test scope, component mock strategy | Per-function unit test requirements |
| **10. Operations** | SLAs, incident response, overall observability | Per-service logging/metrics | Error codes, log statements per function |
| **11. Build/Release** | Versioning scheme, release cadence, signing | Per-component build artifacts | Build flags, optimization settings |
| **12. Ecosystem** | Community governance, license, contribution model | Plugin API, integration adapters | Contribution checklist items |
| **13. Docs** | Doc site structure, documentation philosophy | Per-component doc requirements | Inline doc comments |
| **14. AI Attribution** | Project-level disclosure policy | Per-feature AI attribution | Per-commit AI attribution trailers |

### Layer Definitions

**Macro Layer** — The system-level view. Answers: *What are we building and why? What are the hard constraints?*

This is the "constitution" level. Decisions here are nearly irreversible. Changing macro decisions after execution starts requires a scope warp (RULES.md). The current SPECIFICATION.md operates almost entirely at this layer.

**Meso Layer** — The component-level view. Answers: *How do the parts fit together and communicate?*

This is where interface contracts, module responsibilities, and component-level testing live. Meso decisions are reversible but expensive — they require renegotiating interfaces between components. The current SPECIFICATION.md's section 3 (File Tree) touches this layer but doesn't go deep enough.

**Micro Layer** — The implementation-level view. Answers: *How does this specific function/class work?*

This is where algorithms, data structures, error codes, and inline contracts live. Micro decisions should be the executor's responsibility within macro/meso constraints. The spec should define the *bounds* of micro decisions, not the decisions themselves — otherwise you over-specify and lose the benefits of execution-level judgment.

### Existing Literature on Three-Layer Models

1. **Microservices Architecture** (microservices.io, ebrary.net) — Macro architecture = system-wide decisions (domain architecture, base technologies, communication protocols). Micro architecture = team-level decisions (internal patterns, local tech choices). No explicit meso in this model — it conflates component and implementation.

2. **Dopfer & Potts "Micro-Meso-Macro"** (evolutionary economics, 2004) — Three analytical domains: micro (individual agents/rules), meso (population of agents following a rule), macro (system of interconnected meso populations). The meso is the critical bridging layer — it accounts for how rules spread and change at the population level. Applied to software: meso is where design patterns become team conventions and then propagate through the codebase.

3. **Information Architecture** (Springer, 2021) — "The meso layer allows for the creation of three-way interrelationships where the micro-macro model only identifies simpler two-way interrelationships, providing more detail in support of understanding behavior."

4. **Architecture site analysis** (archimash.com) — Mega/Macro/Micro model used in architectural design: mega = city/suburb context, macro = site and immediate surroundings, micro = individual elements on the site.

### Why Meso Matters in Software Specs

The meso layer is the most commonly neglected layer in software specifications. Teams specify the big picture (macro) and the implementation details (micro) but skip the component-level contracts and interfaces (meso). This creates:

- **Integration gaps**: Components don't fit together because their interfaces were never specified
- **Testing blind spots**: Integration tests aren't specified because the contracts between components are implicit
- **Communication breakdowns**: Teams can't parallelize because they don't know the boundaries
- **Ripple effects**: Changes in one component break others because the dependency graph isn't documented

The meso layer should be the spec's primary focus for 90% of projects. Macro should be minimal (just enough to constrain scope). Micro should be left to the executor (just enough to prevent guessing).

---

## AI Attribution Considerations

### Why AI Attribution Belongs in a Spec

1. **Audit readiness**: The EU AI Act Article 50 is enforceable August 2026. Organizations need to demonstrate they can trace AI-generated code.
2. **Review efficiency**: AI-generated code needs different review scrutiny than human-written code. Attribution tells reviewers what to prioritize.
3. **Maintenance context**: AI-generated code has a ~3.33 year halflife — knowing which code is AI-generated helps maintenance planning.
4. **Legal risk management**: Training data provenance for AI tools is still legally ambiguous. Attribution creates an audit trail.

### The Disclosure Spectrum

| Level | Mechanism | When Appropriate |
|-------|-----------|-----------------|
| **None** | No disclosure | Personal projects, experiments |
| **Minimal** | `Co-Authored-By: <tool>` trailer | Casual OSS, small teams |
| **Standard** | `Assisted-by: <tool>` trailer + PR disclosure | Team projects, active OSS |
| **Full** | git-ai checkpoint tracking + prompt preservation | Enterprise, compliance, research |

### What the Spec Should Specify

The specification should document:
1. **Attribution policy**: Which disclosure level this project uses
2. **Tool inventory**: Which AI tools are approved (and which are banned)
3. **Commit trailer format**: Exact format for Co-Authored-By or Assisted-by lines
4. **Review requirements**: What additional scrutiny AI-generated code needs
5. **Spec-level attribution**: Which parts of the spec itself were AI-generated vs. human-written
6. **Compliance requirements**: Any regulatory or organizational rules about AI disclosure

### Existing Standards & Policy References

- **LLVM "Human-in-the-Loop" policy** (January 2026): `Assisted-by` trailers required. Contributor must certify they reviewed and understand AI-generated code.
- **Claude Code default**: `Co-Authored-By: Claude` trailer on every commit.
- **Ghostty pattern** (August 2025): Mandatory AI attribution in PR body. git-ai for checkpoint tracking.
- **Fedora policy**: Prohibits AI code from unapproved training data. Maintainer must verify provenance.
- **AID Framework** (Weaver, 2024): Structured disclosure — tool name, version, specific tasks, human verification assertion.
- **EU AI Act Article 50** (enforceable August 2026): Mandatory marking of AI-generated content.

---

## What the Current Template Covers vs. Misses

### Current SPECIFICATION.md (7 Sections)

| Section | Coverage | Gaps |
|---------|----------|------|
| **0. Constitution** | 7 default principles, project-specific additions | No meso/micro layer rules, no AI attribution rules |
| **1. Overview** | Core ambition, success criteria (EARS), why now, out-of-scope | No user characteristics, no stakeholder analysis, no problem frame classification |
| **2. Architecture** | Y-Statement format, alternatives considered | Architecture decisions only at macro layer. No meso (component contracts) or micro (algorithm decisions) |
| **3. File Tree** | HIDES/EXPORTS/CALLER per module | Only at meso layer partially. No macro module grouping. No micro function/class contracts |
| **4. CI/Gates** | EARS-format gates, quality gates, release process | No fuzz testing, no security scanning, no performance gates, no testing strategy section |
| **5. Dependencies** | Runtime/Dev/External API tables with version/contract | No dependency decision records, no transitive dependency management, no license tracking |
| **6. UX/Interface** | EARS user behavior, error contract, Design by Contract | Error handling limited to conditions table — no operational concerns, no observability, no incident response |
| **7. Timeline** | Appetite, milestones, circuit breaker | No testing strategy timeline, no documentation delivery timeline, no release cadence |

### What's Missing Entirely

- **Testing Strategy** (unit, integration, e2e, fuzz) — referenced in CI gates but never specified as its own dimension
- **Operational & Error Handling** — error contract is present but shallow. No logging, metrics, alerts, SLAs, incident response
- **Build & Release Pipeline** — release process mentioned briefly but no versioning scheme, changelog convention, rollback, feature flags
- **Ecosystem & Community** — completely absent. No contribution guidelines, no code of conduct, no plugin API, no standards compliance
- **Documentation Strategy** — completely absent. No README spec, no API docs approach, no tutorials/migration guides
- **AI Attribution** — completely absent. No disclosure policy, no commitment trailer format, no AI tool inventory
- **Three-Layer Decomposition** — current spec operates entirely at the macro layer with brief meso touches in file tree

### Gap Analysis by Layer

| Dimension | Macro | Meso | Micro |
|-----------|-------|------|-------|
| Constitution | ✅ Has default principles | ❌ Missing | ❌ Missing |
| Ambition | ✅ Covered | N/A | N/A |
| Architecture | ✅ Y-Statements | ❌ Missing | ❌ Missing |
| File Tree | ❌ Missing | ✅ HIDES/EXPORTS | ❌ Missing |
| CI/Gates | ✅ EARS gates | ❌ Missing | ❌ Missing |
| Dependencies | ✅ Basic tables | ❌ Decision records | ❌ Transitive pinning |
| UX/Contract | ✅ Entry points | ❌ API contracts | ❌ Pre/post conditions |
| Timeline | ✅ Appetite + milestones | ❌ Missing | ❌ Missing |
| Testing | ❌ Missing | ❌ Missing | ❌ Missing |
| Operations | ❌ Missing | ❌ Missing | ❌ Missing |
| Build/Release | ❌ Missing | ❌ Missing | ❌ Missing |
| Ecosystem | ❌ Missing | ❌ Missing | ❌ Missing |
| Documentation | ❌ Missing | ❌ Missing | ❌ Missing |
| AI Attribution | ❌ Missing | ❌ Missing | ❌ Missing |
| **Total** | **7/14** | **1/14** | **0/14** |

---

## Recommendations for SPECIFICATION.md Template

### Recommendation 1: Keep the 7-Section Core, Add 7 More

The current 7 sections are well-structured and research-backed. **Do not remove them.** Instead, add 7 new sections to make the template comprehensive:

**New Sections to Insert (after section 7):**
- **8. Testing Strategy** — Coverage targets, test types, fuzz requirements, CI pipeline integration
- **9. Operational & Error Handling** — Error codes, logging, metrics, alerts, incident response
- **10. Build & Release Pipeline** — Versioning, changelog, release cadence, rollback, feature flags
- **11. Documentation Strategy** — README, API docs, tutorials, migration guides, docs-as-code
- **12. Ecosystem & Community** — Contribution guidelines, code of conduct, plugin API, standards compliance
- **13. AI Attribution & Transparency** — Disclosure policy, commitment trailers, tool inventory
- **14. Three-Layer Specification** — How to use macro/meso/micro within each dimension

### Recommendation 2: Add a "How to Use This Template" Section

The template should explain the three-layer approach up front:

```
## How to Read This Spec

This specification uses a three-layer model:

- **MACRO** (system-level): Decisions that constrain the entire project. Changing these requires a scope warp.
- **MESO** (component-level): Contracts between components. Changing these requires interface renegotiation.
- **MICRO** (implementation-level): Bounds within which the executor has freedom. These are guidelines, not constraints.

Each section below uses the format:
- **MACRO**: The system-level decision or constraint
- **MESO**: How this applies to component interfaces
- **MICRO**: The bounds for implementation decisions (leave blank = executor decides freely)
```

### Recommendation 3: Implement the Three-Layer Format Per Dimension

Each section should be broken into three sub-sections where applicable. Example for Architecture:

```
## 3. Architecture & Design Decisions

### MACRO — System Architecture
[Y-Statement for each architecture-level decision]

### MESO — Component Contracts
[Interface contracts between major components, communication protocols]

### MICRO — Implementation Bounds
[Constraints on algorithm choices, data structure decisions, coding patterns]
```

### Recommendation 4: Testing Strategy Section (New Section 8)

```
## 8. Testing Strategy

### Unit Testing
- Coverage target: {{percentage}}
- Framework: {{framework}}
- Requirements: {{per-function test requirements}}

### Integration Testing
- Scope: {{component interaction coverage}}
- Mock strategy: {{how external dependencies are mocked}}
- Environment: {{test environment configuration}}

### End-to-End Testing
- User journeys covered: {{list of journeys}}
- Environment: {{e2e test environment}}

### Fuzz Testing
- Components to fuzz: {{list}}
- Corpus source: {{initial seed inputs}}
- Coverage target: {{code coverage target for fuzz}}

### Performance Testing
- Load scenarios: {{concurrent users, data volume}}
- Latency thresholds: {{p50, p95, p99}}
- Throughput targets: {{requests per second}}

### Security Testing
- SAST: {{tool and gate}}
- DAST: {{tool and gate}}
- Dependency scanning: {{tool and gate}}
```

### Recommendation 5: AI Attribution Section (New Section 13)

```
## 13. AI Attribution & Transparency

### Policy
- Disclosure level: {{None | Minimal | Standard | Full}}
- Rationale: {{why this level was chosen}}

### Approved AI Tools
| Tool | Version | Permitted Uses | Citation Format |
|------|---------|----------------|-----------------|
| {{tool}} | {{version}} | {{code gen / review / docs}} | {{commit trailer}} |

### Commit Trailers
- Human-authored commits: no trailer
- AI-assisted commits: `Assisted-by: {{tool}} {{version}}`
- AI-generated commits: `Co-Authored-By: {{tool}} {{version}}`

### Spec-Level Attribution
- Sections of this spec that were AI-generated: {{list}}
- Sections of this spec that were human-written: {{list}}
- Date of last AI generation: {{date}}

### Review Requirements
- AI-generated code requires {{additional review steps}}
- AI-generated documentation requires {{additional verification}}
```

### Recommendation 6: Ecosystem Section (New Section 12)

```
## 12. Ecosystem & Community

### Governance Model
- Type: {{maintainer-led | BDFL | community-led | corporate-backed}}
- Decision process: {{how decisions are made}}

### Contribution Model
- Contribution guide: {{link to CONTRIBUTING.md}}
- Code of conduct: {{link to CODE_OF_CONDUCT.md}}
- License: {{license name and link}}
- CLA required: {{yes/no}}
- PR review process: {{description}}

### Integration Points
- Plugin/extension API: {{path or description}}
- Data interchange formats: {{formats supported}}
- Standards implemented: {{list of standards}}

### Upstream/Downstream Relationships
| Dependency | Relationship | Compatibility Policy |
|------------|-------------|---------------------|
| {{upstream project}} | {{fork / direct dep / integration}} | {{version range}} |

### Community Channels
- Discussions: {{link}}
- Chat: {{link}}
- Mailing list: {{link}}
- Bug tracker: {{link}}
```

### Recommendation 7: Keep the Spec Self-Verifying

Add verification checklists to every section (already a pattern in the current spec — extend it to all 14 sections). Each verification step should be an executable command or a binary check.

### Recommendation 8: Dimension Priority for the Initial Implementation

Not all 14 dimensions are equally important for every project. The recommended order of implementation:

**Tier 1 (Critical — present in current spec, needs expansion):**
1. Constitution (add AI attribution rule, meso/micro gap → fill via section structure)
2. Architecture (add meso component contracts, micro implementation bounds)
3. File Tree (add macro grouping, micro function contracts)
4. CI/Gates (add fuzz, security, performance gates)
5. Dependencies (add decision records, license tracking)
6. UX/Contract (add operational error handling as System section)

**Tier 2 (High — completely new sections):**
7. Testing Strategy
8. Build & Release Pipeline

**Tier 3 (Medium — important for OSS or production systems):**
9. Ecosystem & Community
10. Documentation Strategy

**Tier 4 (Emerging — important for compliance):**
11. AI Attribution & Transparency

### Tradeoff Accepted

This comprehensive template is larger than the current 7-section spec. The tradeoff is explicitly:

> In the context of needing a specification template that an AI executor can read and execute without guessing,
> facing the tradeoff between comprehensiveness (reduces ambiguity but increases document size)
> and conciseness (faster to fill but leaves more decisions to the executor),
> we decided for a **modular comprehensive template** (14 sections with tiered priority)
> to achieve a spec that covers everything a production project needs while still usable in practice,
> accepting that most projects will only fill Tiers 1-2 and that the template needs a clear "skip if not applicable" convention.

---

## Origin

Synthesized from IEEE 830-1998 / ISO/IEC/IEEE 29148:2018 standards, spec-driven development literature (2025-2026), Architecture Decision Records practice (Nygard, Y-Statement), micro/macro/meso architecture literature (Dopfer & Potts, microservices.io, complex systems theory), AI attribution frameworks (LLVM policy, Ghostty, AID Framework, EU AI Act), open source governance patterns, testing strategy documentation standards, and the existing Development Protocol SPECIFICATION.md and specification-writing.md research document. July 2026.

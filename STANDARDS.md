# STANDARDS.md — Production Standards for AI-Generated Projects

**Leverage warning:** This protocol amplifies everything — good standards AND bad ones.
Every standard below exists because the AI defaults to the lazy path. These rules force
it to the production path.

---

## Quality Tiers

Standards are organized by project type. Each rule is tagged **[T1]**, **[T2]**, or **[T3]**.

| Tier | Applies to | What it means |
|------|-----------|---------------|
| **T1** | **Personal/independent projects** (Ithmb-Codec level) | You return in a year, understand it completely, trust its correctness, extend it safely. No external contributors. |
| **T2** | **Open-source projects** | T1 + community-facing docs (CONTRIBUTING.md, SECURITY.md, code of conduct, CI matrix). |
| **T3** | **Production team projects** | T1 + T2 + operational concerns (SLOs, incident response, on-call, runbooks, dependency compliance). |

**Default: T1.** Start here for any personal project. Only adopt T2 if you accept external contributions. Only adopt T3 if you're running a production service with users who depend on it.

**How to read a rule:** `[TIER] [PRIORITY]` — e.g., `[T1] [MANDATORY]` means this rule is required for ALL projects at ANY tier.

---

## 1. Error Handling

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 1.1 | **[MANDATORY]** Every error MUST include the values that caused it, not just a message string | T1 | A message like "index out of bounds" forces a debugger session. "index 15 out of bounds for array of length 10" enables immediate diagnosis. | Google SRE Postmortem Culture; Development Protocol Constitution Rule 5 |
| 1.2 | **[MANDATORY]** Every fallible function MUST return a typed error (not a string, not a generic exception) | T1 | Typed errors enable pattern matching, caller-side handling decisions, and documentation of every failure mode. | Rust std::error::Error pattern; Go error handling conventions |
| 1.3 | **[MANDATORY]** Every error MUST be classified as: retryable, transient, permanent, or security | T1 | Different categories demand different handling: retryable → backoff, permanent → fail fast, security → alert. | Google SRE Workbook; AWS Error Retry Guidelines |
| 1.4 | **[MANDATORY]** No silent error swallowing. Every caught error MUST be logged, wrapped, or re-raised | T1 | Empty catch blocks are the #1 source of production mysteries. | CWE-390; OWASP ASVS V7 |
| 1.5 | **[MANDATORY]** Panics/unwraps/asserts MUST only appear in tests or at process boundaries | T1 | In production, a single unwrap on a None value crashes the entire process. | Rust safety guidelines; Google SRE |
| 1.6 | **[MANDATORY]** Every external call (network, disk, API) MUST have a timeout | T1 | Without a timeout, a hanging dependency hangs your entire service. | Google SRE Book Ch. 6 |
| 1.7 | **[MANDATORY]** Every external call MUST handle: timeout, connection refused, DNS failure, rate limit, unexpected response | T1 | AI agents systematically omit these five failure modes. | Osmani 80% Problem; Karpathy observations |
| 1.8 | **[STRONGLY RECOMMENDED]** Every retryable operation MUST use exponential backoff with jitter | T1 | Without jitter, retries synchronize and cause thundering herd failures. | Google SRE Book Ch. 21 |
| 1.9 | **[STRONGLY RECOMMENDED]** Every retry MUST have a maximum attempt count (default: 3) | T1 | Infinite retries are infinite resource consumption. | Google SRE Workbook |
| 1.10 | **[MANDATORY]** Every error log line MUST include: timestamp, severity, correlation_id, error_kind, message | T1 | Without structured fields, log aggregation tools cannot filter or alert. | OpenTelemetry Logging Specification |
| 1.11 | **[MANDATORY]** No credentials, tokens, secrets, or PII in log output | T1 | Logs are not access-controlled storage. | OWASP ASVS V7.4; SOC2 |
| 1.12 | **[MANDATORY]** Every graceful degradation path MUST be documented and tested | T1 | If a dependency fails, the system should degrade — but undocumented degradation is undetectable. | Google SRE; AWS Well-Architected |
| 1.13 | **[STRONGLY RECOMMENDED]** Global catch-all error handlers at every process boundary (HTTP middleware, CLI entry, MQ consumer) | T1 | Unhandled errors at boundaries produce crashes. | OWASP ASVS V1.1 |
| 1.14 | **[STRONGLY RECOMMENDED]** Every error path in a function must be tested — not just the happy path | T1 | The 80% problem: AI tests happy path, leaves errors untested. | Osmani 80% Problem |
| 1.15 | **[ADVISORY]** Error messages should describe what the caller should do, not just what went wrong | T2 | "DB connection failed" → "Check DB_HOST env var and ensure postgres:5432 is reachable." | Google API Design Guide |

## 2. Code Quality

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 2.1 | **[MANDATORY]** No file exceeds 250 lines of production code | T1 | Files over 250 lines cannot be reviewed in one pass and hide complexity. | Development Protocol RULES.md §5; PEP 8; Software Engineering at Google |
| 2.2 | **[MANDATORY]** Every public function/class MUST have a doc comment (what, parameters, returns, errors) | T1 | Without docs, the next AI agent cannot safely call this function. | JavaDoc/TSDoc/Python docstring conventions |
| 2.3 | **[MANDATORY]** No function exceeds 40 lines of code (comments and whitespace excluded) | T1 | Functions over 40 lines do more than one thing. | NASA Power of 10 Rule #2 |
| 2.4 | **[MANDATORY]** No magic numbers, no magic strings. Every literal MUST be a named constant or enum | T1 | Magic values hide intent. | NASA Power of 10 Rule #1; SonarQube |
| 2.5 | **[MANDATORY]** No function with more than 3 nesting levels | T1 | Deep nesting is provably harder to understand. | NASA Power of 10 Rule #4 |
| 2.6 | **[MANDATORY]** No function with more than 4 parameters. Use a parameter object for more. | T1 | 4+ parameters force callers to remember argument order. | NASA Power of 10 (implied); Clean Code |
| 2.7 | **[MANDATORY]** No TODO, FIXME, HACK, XXX without an associated issue number | T1 | Unlinked TODO comments are permanent code smell. | SonarQube; Google Code Review |
| 2.8 | **[MANDATORY]** All code MUST compile with zero warnings at strictest level | T1 | Warnings are deferred errors. | Development Protocol RULES.md §5 |
| 2.9 | **[STRONGLY RECOMMENDED]** No function may have cyclomatic complexity > 10 | T1 | Complexity > 10 correlates strongly with defect density. | McCabe (1976); SonarQube |
| 2.10 | **[MANDATORY]** No dead code: every function, variable, import, and type must be used | T1 | Orphaned code confuses readers and wastes maintenance effort. | Karpathy Guidelines Step 3; SonarQube |
| 2.11 | **[MANDATORY]** AI agents MUST NOT reformat, restyle, or reorganize existing code when making changes | T1 | Side-effect changes introduce review noise and obscure the change. | Karpathy Guidelines Step 3; Agent Skills |
| 2.12 | **[MANDATORY]** AI agents MUST NOT refactor code they were not asked to modify | T1 | Unsolicited refactoring breaks the principle of least surprise. | Karpathy Guidelines Step 3; Agent Skills |
| 2.13 | **[STRONGLY RECOMMENDED]** Every module MUST have a single responsibility expressed in its module-level doc comment | T1 | Without an explicit responsibility, modules accumulate unrelated functionality. | Parnas modular decomposition |
| 2.14 | **[STRONGLY RECOMMENDED]** No AI agent may add dependencies without explicit human approval | T1 | Dependency bloat is the #1 avoidable source of supply chain risk. | Development Protocol RULES.md §4, §6 |
| 2.15 | **[MANDATORY]** All dependencies MUST be pinned to exact versions | T1 | Floating ranges produce unreproducible builds. | Supply chain security; OWASP |
| 2.16 | **[STRONGLY RECOMMENDED]** Lint MUST pass before commit. Strictest available profile. | T1 | AI agents produce code that "looks right" but violates conventions. | Agent Skills lint skill |
| 2.17 | **[MANDATORY]** No AI agent may implement a feature not explicitly listed in V1 scope | T1 | Feature creep is the #1 failure pattern. | Development Protocol RULES.md §4 |
| 2.18 | **[STRONGLY RECOMMENDED]** No abstractions for a single use case. Flatten unnecessary hierarchies. | T1 | AI agents over-abstract by default. | Karpathy Guidelines Step 2 |
| 2.19 | **[STRONGLY RECOMMENDED]** No flexibility/configurability not explicitly requested | T1 | Future-proofing adds complexity today for a future that may never arrive. | Karpathy Guidelines Step 2 — YAGNI |
| 2.20 | **[MANDATORY]** Every conditional must handle the else case explicitly, even if a no-op comment | T1 | Silent fallthrough is the source of most logic bugs. | Defensive programming |

## 3. Testing

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 3.1 | **[MANDATORY]** Test MUST be written BEFORE implementation in WORK phase | T1 | Tests written after code confirm what code does, not what it should do. | RULES.md §8; TDD (Beck); Agent Skills tdd |
| 3.2 | **[MANDATORY]** Every test MUST fail on incorrect output, pass on correct output | T1 | A test that passes on the first run is suspicious. | RULES.md §8 |
| 3.3 | **[MANDATORY]** One behavior per test. Test names MUST describe the expected outcome. | T1 | `test_decode_unknown_format_returns_error` tells you what broke. | RULES.md §8 |
| 3.4 | **[MANDATORY]** Edge case tests MUST be written BEFORE happy path tests | T1 | If code can't handle the edge case, it shouldn't handle the happy path. | RULES.md §8 |
| 3.5 | **[MANDATORY]** Every error path documented in errors section MUST have a corresponding test | T1 | Untested error paths fail in production. | Osmani 80% Problem |
| 3.6 | **[MANDATORY]** Line coverage ≥ 80%, branch coverage ≥ 70% | T1 | Below these thresholds correlates with undetected bugs. | Google Testing at Scale; SonarQube |
| 3.7 | **[STRONGLY RECOMMENDED]** Mutation testing ≥ 60% mutation score | T2 | Line coverage lies. Mutation testing reveals untested logic. | Trail of Bits genotoxic |
| 3.8 | **[STRONGLY RECOMMENDED]** Every public API function MUST have a property-based test | T2 | Example-based tests only cover cases you think of. | QuickCheck; Hypothesis; fast-check |
| 3.9 | **[STRONGLY RECOMMENDED]** Every parser/deserializer MUST have fuzz tests | T2 | Fuzzing finds the edge case property tests missed. | OWASP ASVS V1.5; Google OSS-Fuzz |
| 3.10 | **[MANDATORY]** Bug found in production → FIRST write regression test, THEN fix | T1 | Without a regression test, the same bug will be reintroduced. | RULES.md §8; Google SRE |
| 3.11 | **[STRONGLY RECOMMENDED]** No test may depend on another test. Must be independently runnable. | T1 | Test order dependencies create non-deterministic failures. | Google Testing at Google |
| 3.12 | **[STRONGLY RECOMMENDED]** No network calls in unit tests. Use fakes. | T1 | Network-dependent tests are slow, flaky, and non-deterministic. | Google Test Sizing |
| 3.13 | **[STRONGLY RECOMMENDED]** Every test file MUST correspond to exactly one production file | T1 | Scattered test files hide coverage gaps. | Standard test conventions |
| 3.14 | **[STRONGLY RECOMMENDED]** CI MUST fail if test coverage decreases | T1 | Coverage should only go up. | SonarQube; Codecov |
| 3.15 | **[MANDATORY]** No test may be deleted without corresponding production code deletion | T1 | Test deletion without code removal drops coverage silently. | Derived governance |
| 3.16 | **[ADVISORY]** Integration tests should cover every external dependency's error modes | T2 | Prove the system handles each failure mode, not that it works when everything is perfect. | Google SRE testing for reliability |
| 3.16 | **[STRONGLY RECOMMENDED]** For correctness-critical code (codecs, parsers, formats), golden test vectors MUST be maintained — real-world sample inputs with verified expected outputs | Unit tests verify logic. Golden vectors verify the code against real-world data. Ithmb-Codec maintains 14+ reference files across 7 encoding formats. | Ithmb-Codec golden test vectors; Fuzz testing practice |
| 3.17 | **[STRONGLY RECOMMENDED]** For correctness-critical code, cross-implementation verification MUST be performed — the same inputs produce identical outputs in at least two independent implementations | A single implementation has latent bugs. Two implementations that agree on sample data have near-zero latent bugs. Ithmb-Codec verified pixel-for-pixel against a C# oracle during development. | Ithmb-Codec C# cross-verification; NASA verification practices |
| 3.18 | **[STRONGLY RECOMMENDED]** Every performance-sensitive project MUST have benchmarks in the repository with tracked results | Without benchmarks, you cannot know if a change improved or degraded performance. Ithmb-Codec has 4 Divan benchmarks. | Ithmb-Codec benchmarks; Google Benchmark practices |

## 4. Documentation

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 4.1 | **[MANDATORY]** Every public API MUST have a doc comment: description, @param, @returns, @throws | T1 | Without these, the function is indistinguishable from private. AI agents will misuse it. | SPECIFICATION.md §11 |
| 4.2 | **[MANDATORY]** Every repository MUST have a README.md with: what, why, how to build/run/test | T1 | Without a README, the project is inaccessible to future AI sessions. | Standard best practices |
| 4.3 | **[MANDATORY]** Every repository MUST have a CHANGELOG.md (Keep a Changelog format) | T1 | Without a changelog, users cannot know what changed between versions. | keepachangelog.com |
| 4.4 | **[MANDATORY]** Every configuration option MUST be documented at point of declaration | T1 | Undocumented config options become magic nobody understands. | 12-Factor App |
| 4.5 | **[STRONGLY RECOMMENDED]** Every module directory SHOULD have README.md (Parnas-style: what it hides, what it exports) | T2 | Parnas decomposition is lost when AI agents can't see module design decisions. | SPECIFICATION.md §3 |
| 4.6 | **[STRONGLY RECOMMENDED]** Every example in documentation MUST compile and run | T2 | Outdated documentation actively misleads. | Python doctest; Rust doc-tests |
| 4.7 | **[MANDATORY]** Every AI agent configuration file MUST be well-structured: repository purpose, workspace layout, conventions, pipeline flow (can exceed 200 lines if structured — verbosity without structure is the sin, not length) | Large config files defeat their purpose — unless they're well-structured comprehensive guides. Ithmb-Codec's 4.9K AGENTS.md is excellent because it's organized, not because it's long. | Anthropic Claude Code Best Practices; Ithmb-Codec AGENTS.md (4.9K) |
| 4.11 | **[STRONGLY RECOMMENDED]** Every project MUST have an ARCHITECTURE.md with an ASCII diagram showing component relationships and data flow | Architecture without a diagram is guesswork. An ASCII diagram that fits in the terminal is worth 1000 words of prose. | Ithmb-Codec ARCHITECTURE.md (4.6K with diagrams); C4 model |
| 4.12 | **[STRONGLY RECOMMENDED]** Every project with multiple integration targets MUST have an ECOSYSTEM.md or equivalent describing the project's place in the broader landscape | Without ecosystem docs, users cannot evaluate whether your project fits their stack. Covers: bindings, integrations, upstream/downstream depdendencies, related projects. | Ithmb-Codec ECOSYSTEM.md (4.8K) |
| 4.13 | **[STRONGLY RECOMMENDED]** Every project SHOULD have a project-level standards document (`docs/standards/`) documenting project-specific engineering rules beyond the generic STANDARDS.md | Generic standards cover universal rules. Project standards capture project-specific architecture decisions, code conventions, automation tiers, and design axioms. | Ithmb-Codec docs/standards/ (9.2K + 4.9K) |
| 4.8 | **[STRONGLY RECOMMENDED]** Agent config files must NOT duplicate tooling behavior (formatting, linting, type checking) | T1 | Let tools do what tools do. Agent config should capture intent and project-specific rules. | Tool-First Rule |
| 4.9 | **[STRONGLY RECOMMENDED]** Every ADR MUST use the Y-Statement format | T1 | Y-Statements capture context and rationale, enabling future agents to understand architecture decisions. | SPECIFICATION.md §2; ADR practice |
| 4.10 | **[ADVISORY]** Functions requiring a diagram to understand MUST have an architecture diagram stored alongside code | T2 | If code is too complex without a diagram, the diagram is not optional. | Charting the Code |

## 5. Security

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 5.1 | **[MANDATORY]** All external input MUST be validated before use | T1 | Unvalidated input causes injection attacks. | OWASP ASVS V1; CWE Top 25 |
| 5.2 | **[MANDATORY]** All database queries MUST use parameterized queries | T1 | String concatenation causes SQL injection. | OWASP ASVS V5.1; CWE-89 |
| 5.3 | **[MANDATORY]** No secrets, tokens, keys, or passwords in source code | T1 | Hardcoded secrets are the first thing attackers scan for. | OWASP ASVS V2.10; CWE-798 |
| 5.4 | **[MANDATORY]** Every public endpoint MUST authenticate the caller before processing | T2 | Without authentication, every endpoint is public. AI agents forget auth on new endpoints. | OWASP ASVS V2; Osmani 80% |
| 5.5 | **[MANDATORY]** Every authenticated endpoint MUST authorize the caller | T2 | Authentication proves identity. Authorization proves permission. | OWASP ASVS V2.1; CWE-862 |
| 5.6 | **[MANDATORY]** All output sent to users MUST be encoded/escaped for the context it appears in | T2 | Unescaped output causes XSS. | OWASP ASVS V1.5; CWE-79 |
| 5.7 | **[STRONGLY RECOMMENDED]** Security decisions (auth, crypto, access) MUST NOT depend on client-side params | T2 | Never trust what the client says about their own authorization. | OWASP ASVS V2 |
| 5.8 | **[STRONGLY RECOMMENDED]** Every rate limit MUST be enforced server-side | T2 | Client-side limits are advisory. | OWASP ASVS V6 |
| 5.9 | **[MANDATORY]** Every file upload MUST validate: type (magic bytes), size, and content | T2 | Extension-only validation is trivially bypassed. | OWASP ASVS V12; CWE-434 |
| 5.10 | **[MANDATORY]** All cryptographic operations MUST use well-reviewed libraries (no hand-rolled crypto) | T1 | Hand-rolled crypto is always wrong. | OWASP ASVS V11; CWE-327 |
| 5.11 | **[STRONGLY RECOMMENDED]** Every dependency MUST be scanned for known vulnerabilities | T2 | Average project has 10+ vulnerable transitive deps. | OWASP dependency-check; Snyk |
| 5.12 | **[MANDATORY]** No system command execution with unsanitized input | T1 | Shell injection is as dangerous as SQL injection. | OWASP ASVS V1.6; CWE-78 |
| 5.13 | **[STRONGLY RECOMMENDED]** Security fixes MUST have CVE/tracking ID in commit message | T3 | Security fixes without identifiers cannot be tracked or communicated. | CVE standards; OpenSSF |
| 5.14 | **[MANDATORY]** Security events MUST be logged to a separate, append-only stream | T3 | Security logs mixed with debug logs are lost in the noise. | OWASP ASVS V7; SOC2 |

## 6. Performance

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 6.1 | **[STRONGLY RECOMMENDED]** No optimization without a benchmark proving the need | T1 | AI agents optimize preemptively, creating complexity for hypothetical bottlenecks. | Development Protocol RULES.md Decision Framework §3 |
| 6.2 | **[STRONGLY RECOMMENDED]** Every public API MUST have a documented latency SLO | T3 | Without an SLO, you cannot know if a change improved or degraded performance. | Google SRE Book Ch. 4 |
| 6.3 | **[STRONGLY RECOMMENDED]** Every algorithm choice MUST document its time/space complexity | T1 | An O(n³) algorithm chosen over O(n log n) for simplicity will fail at scale. | Data structures standard practice |
| 6.4 | **[STRONGLY RECOMMENDED]** No synchronous I/O in hot paths | T2 | Synchronous I/O blocks event loops and destroys throughput. | Async best practices; Google SRE |
| 6.5 | **[STRONGLY RECOMMENDED]** Every database query MUST have EXPLAIN PLAN in migration comment | T2 | Without index-aware query design, migrations cause production outages. | Google SRE database practices |
| 6.6 | **[STRONGLY RECOMMENDED]** Every paginated endpoint MUST enforce a max page size | T2 | Unbounded pagination is a DoS vector. | REST API best practices |
| 6.7 | **[STRONGLY RECOMMENDED]** Every background job MUST have a timeout and circuit breaker | T2 | Jobs that hang consume resources indefinitely. | Netflix Hystrix; Google SRE |
| 6.8 | **[STRONGLY RECOMMENDED]** Every resource acquisition MUST have a maximum and release guarantee | T1 | Resource leaks crash production. AI agents forget to release resources. | Google SRE; Rust RAII |
| 6.9 | **[ADVISORY]** Memory-constrained environments MUST set limits on all caches/buffers | T2 | Unbounded caches cause OOM kills. | Cloud/container best practices |
| 6.10 | **[STRONGLY RECOMMENDED]** Binary/app size MUST be tracked. Increase >10% needs justification. | T2 | App size bloat increases startup time and attack surface. | Google production excellence |

## 7. Architecture

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 7.1 | **[MANDATORY]** Dependencies MUST point inward: core has no knowledge of infrastructure | T1 | When core knows about DB, you cannot test business logic without a DB. | Clean Architecture; Hexagonal Architecture |
| 7.2 | **[MANDATORY]** No circular dependencies between modules | T1 | Circular deps prevent independent testing and reasoning. | ISO 25010; Go's import cycle error |
| 7.3 | **[MANDATORY]** Every architecture decision MUST have an ADR using Y-Statement format | T1 | Undocumented decisions will be reversed by the next AI agent. | SPECIFICATION.md §2; ADR practice |
| 7.4 | **[STRONGLY RECOMMENDED]** No AI agent may refactor module boundaries without approval | T1 | Architectural refactors have system-wide blast radius. | Karpathy Guidelines Step 3 |
| 7.5 | **[STRONGLY RECOMMENDED]** Every module MUST hide exactly one design decision (Parnas) | T1 | Modules hiding multiple decisions cannot be changed independently. | Parnas (1972); SPECIFICATION.md §3 |
| 7.6 | **[MANDATORY]** No concept may be represented two different ways in the codebase | T1 | Dual representations are a primary source of data corruption bugs. | Parse-don't-validate |
| 7.7 | **[MANDATORY]** Every cross-cutting concern MUST be implemented once and applied consistently | T1 | AI agents implement auth per-endpoint, creating security gaps. | Osmani 80% Problem |
| 7.8 | **[STRONGLY RECOMMENDED]** No more than 3 layers (controller → service → repository) | T1 | Every extra layer is an indirection tax. >3 suggests over-engineering. | Clean Architecture; Karpathy |
| 7.9 | **[STRONGLY RECOMMENDED]** Every state mutation MUST be logged or emitted as an event | T2 | Without event logs, debugging production state corruption requires a time machine. | Event sourcing; CQRS |
| 7.10 | **[STRONGLY RECOMMENDED]** DB schema is source of truth — no in-memory state may contradict persisted state | T2 | In-memory caches that diverge from DB are #1 source of data integrity bugs. | CQRS; Google SRE |

## 8. AI Attribution & Transparency

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 8.1 | **[MANDATORY]** Every AI-generated file MUST have a header: tool used + date | T1 | Enables audit trails for debugging and understanding AI-introduced patterns. | EU AI Act Article 50 |
| 8.2 | **[MANDATORY]** Every commit with AI code MUST include trailer: `AI-tool: <tool> <version>` | T1 | Without attribution at commit granularity, you cannot correlate bugs with the generating agent. | SPECIFICATION.md §13; Conventional Commits |
| 8.3 | **[MANDATORY]** AI-generated code MUST be reviewed by a human before merging | T1 | AI code review catches only 60-70% of bugs. Human is the backstop. | OpenAI Codex Best Practices |
| 8.4 | **[STRONGLY RECOMMENDED]** AI Tool Inventory listing every tool, version, permitted uses, citation format | T2 | Without an inventory, compliance audits cannot be satisfied. | EU AI Act |
| 8.5 | **[STRONGLY RECOMMENDED]** AI-generated code MUST pass same CI gates as human code | T1 | Lowering standards for AI code defeats the purpose. | Derived from all standards |
| 8.6 | **[MANDATORY]** AI tools used for code generation MUST be disclosed in README or AI-ATTRIBUTION.md | T2 | Open source projects must declare AI use. Downstream consumers have a right to know. | EU AI Act; GitHub disclosure |
| 8.7 | **[STRONGLY RECOMMENDED]** Prompts that produced novel code should be preserved alongside the code | T2 | Enables reproducing the reasoning and debugging unexpected behavior. | Karpathy "assumption propagation" |
| 8.8 | **[ADVISORY]** Periodically audit AI Tool Inventory; remove unused tools | T3 | Tool proliferation creates compliance gaps. | EU AI Act; NIST AI RMF |

## 9. CI/CD & Operations

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 9.1 | **[MANDATORY]** CI MUST enforce all MANDATORY rules with automated checks | T1 | If a rule cannot be automated, it's not testable. | SPECIFICATION.md §4 |
| 9.2 | **[MANDATORY]** CI MUST fail on any new lint warning, vulnerability, or code smell | T1 | "No new issues" prevents technical debt accumulation. | SonarQube |
| 9.3 | **[MANDATORY]** CI MUST run the full test suite (not just changed files) | T1 | Subset testing misses cross-cutting failures. | Google Testing at Scale |
| 9.4 | **[STRONGLY RECOMMENDED]** CI MUST run security scan on every PR (SAST, SCA, secrets) | T2 | Security caught at PR time costs 10x less than production. | OWASP ASVS; DevSecOps |
| 9.5 | **[STRONGLY RECOMMENDED]** CI MUST run mutation testing on changed files | T2 | Line coverage does not measure test quality. | Trail of Bits genotoxic |
| 9.6 | **[STRONGLY RECOMMENDED]** Every production deployment MUST have a quality gate | T3 | Deploying without a quality gate is gambling. | Google SRE |
| 9.7 | **[STRONGLY RECOMMENDED]** Deployments MUST use progressive delivery (canary, blue/green, feature flags) | T3 | Releasing to 100% simultaneously makes every deploy a rollback gamble. | Google SRE; LaunchDarkly |
| 9.8 | **[STRONGLY RECOMMENDED]** Every service MUST have at least one SLO before production | T3 | If you can't measure whether your service is working, you can't know if it broke. | Google SRE Book Ch. 4 |
| 9.9 | **[STRONGLY RECOMMENDED]** Every service MUST have a dashboard: error rate, latency, throughput, saturation | T3 | Without a dashboard, you won't know it's down until a user tells you. | Google SRE "Four Golden Signals" |
| 9.10 | **[MANDATORY]** Every rollback MUST be practiced before it's needed | T3 | First rollback should not be during a production incident. | Google SRE |
| 9.11 | **[MANDATORY]** Every production change MUST have an associated rollback plan | T3 | Changes without rollback plans are unrevertable risks. | Google SRE; Change Management |
| 9.12 | **[STRONGLY RECOMMENDED]** All infrastructure MUST be defined as code | T3 | Manual infra cannot be reviewed, versioned, or reproduced. | Infrastructure as Code |

## 10. AI Laziness Prevention (The 80% Problem)

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 10.1 | **[MANDATORY]** Before writing code, list every assumption the implementation relies on | T1 | Counteracts the #1 failure: models making wrong assumptions without checking. | Karpathy Guidelines Step 1 |
| 10.2 | **[MANDATORY]** Before writing code, check if multiple valid interpretations exist — present as options | T1 | Agent must surface ambiguity, not silently pick one. | Karpathy Guidelines Step 1 |
| 10.3 | **[MANDATORY]** Before writing code, check if a simpler approach exists — surface it | T1 | Counteracts the "1000 lines where 100 suffice" pattern. | Karpathy Guidelines Step 1 & 2 |
| 10.4 | **[MANDATORY]** No features beyond what was requested. No single-use abstractions. No unrequested configurability. | T1 | YAGNI. AI defaults to building for the future at expense of the present. | Karpathy Guidelines Step 2 |
| 10.5 | **[MANDATORY]** Every changed line MUST trace to the user's request. If not traceable, remove. | T1 | Prevents side-effect changes, dead code, and silent refactoring. | Karpathy Guidelines Step 3 |
| 10.6 | **[MANDATORY]** Every task MUST be defined as a verifiable, testable goal before execution | T1 | "Implement login" is vague. "Form that validates email, returns JWT, rate-limits" is testable. | Karpathy Guidelines Step 4 |
| 10.7 | **[MANDATORY]** AI agents MUST NOT skip spec writing, test writing, or security review | T1 | The agent's shortest path skips all engineering discipline. | Agent Skills anti-rationalization |
| 10.8 | **[STRONGLY RECOMMENDED]** When confused/uncertain, AI MUST stop and ask — not guess and continue | T1 | LLMs don't naturally express uncertainty. | Karpathy Guidelines Step 1 |
| 10.9 | **[STRONGLY RECOMMENDED]** AI agents MUST push back on contradictory/infeasible requests | T1 | Sycophantic agreement produces working code that solves the wrong problem. | Karpathy "sycophantic agreement" |
| 10.10 | **[STRONGLY RECOMMENDED]** Sessions producing >200 lines MUST produce a summary of what/why/assumptions | T1 | Without summary, comprehension debt accumulates. | Osmani "comprehension debt" |
| 10.11 | **[MANDATORY]** Apply the "senior engineer test": would a senior engineer call this overcomplicated? | T1 | If yes, rewrite to minimum viable implementation. | Karpathy Guidelines Step 2 |
| 10.12 | **[STRONGLY RECOMMENDED]** After completing task, AI MUST clean up orphaned imports, vars, functions | T1 | AI leaves dead code behind. This rule makes them responsible for their own mess. | Karpathy Guidelines Step 3 |
| 10.13 | **[STRONGLY RECOMMENDED]** Pre-existing dead code should be mentioned but NOT modified | T1 | Side-effect modifications to unrelated code introduce unreviewed changes. | Karpathy Guidelines Step 3 |
| 10.14 | **[STRONGLY RECOMMENDED]** AI MUST NOT implement error handling for scenarios that cannot occur given current inputs | T1 | Produces dead error-handling code that wastes review time. | Karpathy Guidelines Step 2; YAGNI |

## 11. Build & Configuration

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 11.1 | **[MANDATORY]** Build MUST be reproducible: same source + same toolchain = same binary | T1 | Non-reproducible builds cannot be audited or verified. | Reproducible Builds; OpenSSF SLSA |
| 11.2 | **[MANDATORY]** All toolchain versions MUST be pinned (compiler, linter, formatter, runner) | T1 | Floating toolchains produce non-reproducible builds. | DevContainer/Docker/nix |
| 11.3 | **[MANDATORY]** Every new project MUST have CI configured at bootstrap, not after features | T1 | CI configured post-hoc misses the first N commits where most architecture decisions happen. | RULES.md §3 |
| 11.4 | **[STRONGLY RECOMMENDED]** Every project MUST use a formatter with project-wide config | T1 | Style debates waste human time. Formatters end them deterministically. | Tool-First Rule |
| 11.5 | **[STRONGLY RECOMMENDED]** Every project MUST use static type checking at strictest level | T1 | Type checking catches incorrect data shape assumptions. | Programming Skill references |
| 11.6 | **[STRONGLY RECOMMENDED]** Every project MUST use a linter at most stringent profile | T1 | Linters catch structural issues AI systematically produce. | RULES.md §5 |
| 11.7 | **[STRONGLY RECOMMENDED]** Every project MUST run linter + type checker + formatter + tests as single pre-commit command | T1 | If not run together, at least one will be skipped. | Derived best practice |

## 12. Dependency Management

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 12.1 | **[MANDATORY]** No dependency without a Y-Statement decision recorded | T1 | Every dependency is a supply chain risk and maintenance burden. | SPECIFICATION.md §5 |
| 12.2 | **[MANDATORY]** Every dependency MUST have documented: purpose, version, contract, license | T1 | Without this, you cannot assess whether it's needed or can be removed. | SPECIFICATION.md §5 |
| 12.3 | **[MANDATORY]** All dependencies pinned to exact versions (no ^, ~, >=) | T1 | Version ranges produce unreproducible builds. | OpenSSF SLSA; npm/cargo/go |
| 12.4 | **[STRONGLY RECOMMENDED]** Every dependency's license MUST be compatible with project license | T2 | License incompatibility can force project rewrites. | SPDX; Open Source licensing |
| 12.5 | **[STRONGLY RECOMMENDED]** No dependency with known CVEs may be used | T1 | Running on vulnerable dependency is negligence. CI must block. | OWASP; Snyk; Dependabot |
| 12.6 | **[STRONGLY RECOMMENDED]** Dependency update PRs MUST be reviewed like feature PRs | T2 | An attacker compromises a dependency and the update installs the backdoor. | OpenSSF; SolarWinds |
| 12.7 | **[ADVISORY]** Minimize dependency depth. Prefer stdlib for <50 lines of functionality. | T1 | Writing 10 lines has zero supply chain risk. | RULES.md FP-013 |

## 13. Code Review Standards

| # | Rule | Tier | Why | Source |
|---|------|------|-----|--------|
| 13.1 | **[MANDATORY]** Every PR MUST be reviewed by a human before merging | T1 | AI code review alone misses 30-40% of logic bugs. | OpenAI Codex Best Practices |
| 13.2 | **[MANDATORY]** Every PR MUST have a description: WHAT changed and WHY | T1 | Without description, reviewer cannot evaluate the change. | Google Code Review |
| 13.3 | **[STRONGLY RECOMMENDED]** PRs < 400 lines changed (single-sitting review) | T2 | Review effectiveness drops sharply above 400 lines. | Google Code Review; DORA |
| 13.4 | **[MANDATORY]** Every commit message MUST follow Conventional Commits: `type(scope): description` | T1 | Consistency enables automated changelog generation and git bisect. | Conventional Commits |
| 13.5 | **[MANDATORY]** AI author MUST respond to every review comment | T1 | Unresolved discussions leave open questions. | Google Code Review |
| 13.6 | **[STRONGLY RECOMMENDED]** AI-generated PRs: extra scrutiny on error paths, security, architecture, assumptions | T2 | These are where AI agents systematically err. | Osmani 80% Problem; Karpathy |
| 13.7 | **[STRONGLY RECOMMENDED]** No 'fix the fix' cycle — verify all gates before committing. A follow-up fix indicates insufficient verification. | T1 | Fixing a fix compounds risk. | Karpathy; Osmani 80% Problem |
| 13.8 | **[MANDATORY]** Every GitHub repository MUST have at least 3 relevant topics set | T1 | Missing topics means invisible projects. | GitHub best practices |
| 13.9 | **[MANDATORY]** Every project MUST maintain ADRs in `docs/adr/` using Y-Statement format | T1 | Without ADRs, every decision must be rediscovered. | ADR practice (Nygard) |

---

## Quick Reference by Tier

| Rule | T1 (Personal) | T2 (Open Source) | T3 (Production) |
|------|--------------|------------------|-----------------|
| Error handling | All 15 | All 15 | All 15 |
| Code quality | All 20 | All 20 | All 20 |
| Testing | 3.1-3.6, 3.10-3.15 | 3.1-3.16 | 3.1-3.16 |
| Documentation | 4.1-4.4, 4.7-4.9 | 4.1-4.10 | 4.1-4.10 |
| Security | 5.1-5.3, 5.10, 5.12 | 5.1-5.12 | 5.1-5.14 |
| Performance | 6.1, 6.3, 6.8 | 6.1-6.10 | 6.1-6.10 |
| Architecture | All 10 | All 10 | All 10 |
| AI attribution | 8.1-8.3, 8.5 | 8.1-8.7 | 8.1-8.8 |
| CI/CD & Ops | 9.1-9.3 | 9.1-9.5 | 9.1-9.12 |
| AI laziness | All 14 | All 14 | All 14 |
| Build & config | All 7 | All 7 | All 7 |
| Dependencies | 12.1-12.3, 12.5, 12.7 | 12.1-12.7 | 12.1-12.7 |
| Code review | 13.1-13.2, 13.4-13.5, 13.7-13.9 | 13.1-13.9 | 13.1-13.9 |

---

## Origin

This document synthesizes research across: Addy Osmani's agent-skills (77k+ ⭐) and "80% Problem" analysis, Karpathy's original observations and Karpathy Guidelines (191k+ ⭐), NASA Power of 10, Google SRE, OWASP ASVS 5.0, CWE Top 25, EU AI Act Article 50, OpenAI Codex Best Practices, Anthropic Claude Code Best Practices, Software Engineering at Google (Beyoncé Rule), SonarQube quality gates, Trail of Bits genotoxic, ISO 25000 (SQuaRE), SEI CERT Coding Standards, and Development Protocol SPECIFICATION.md (14-dimension three-layer model).

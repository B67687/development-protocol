# Engineering Plugin — Execution Addendum

> **When to use:** Your project involves engineering deliverables — software, hardware,
> firmware, infrastructure, or any technical system with build steps, tests, CI/CD,
> operations, or production deployments.
>
> **When to skip:** Your project is non-technical — marketing campaign, research paper,
> organizational change, creative work. The universal execution core (SPEC §0-3, §5-8,
> §10-14) applies without this addendum.

This plugin is consumed by SPECIFICATION.md (§4, §9), EXECUTOR.md (Production Quality),
and REVIEW.md (checklist items 4.5, 4.6). It lives here so the main protocol files
stay universal.

---

## 1. CI, Tooling & Quality Gates

Used by SPECIFICATION.md §4. Replace with your project's actual CI commands.

### MACRO — System Gates

Acceptance criteria in EARS notation:

WHEN a pull request is opened
THEN CI SHALL run {{build command}}
WHERE compilation fails
THEN CI SHALL fail with exit code 1 and the compiler output

WHEN a pull request is opened
THEN CI SHALL run {{lint command}}
WHERE lint violations are detected
THEN CI SHALL fail with the list of violations

WHEN a pull request is opened
THEN CI SHALL run {{test command}}
WHERE any test fails
THEN CI SHALL fail with the failing test output

### MESO — Per-Component CI Requirements

_Which components have specific CI requirements (e.g., fuzz testing for the parser module)?_

### MICRO — Tool Configuration Bounds

_Specific linter rules, formatter config, commit hooks. Concrete command-line invocations._

> **Bus-Hop Example:**
>
> WHEN a pull request is opened
> THEN CI SHALL run ./gradlew test
> WHERE any test fails
> THEN CI SHALL fail with the failing test output
>
> WHEN a pull request is opened
> THEN CI SHALL run ./gradlew detekt
> WHERE lint violations are detected
> THEN CI SHALL fail with the list of violations
>
> WHEN a pull request is opened
> THEN CI SHALL run ./gradlew assembleDebug
> WHERE compilation fails
> THEN CI SHALL fail with exit code 1 and the compiler output
>
> WHEN code is merged to main
> THEN CI SHALL run gitleaks secret scan
> WHERE secrets are detected
> THEN CI SHALL fail with the list of affected files
>
> **MESO:** domain/ and data/ require 80% line coverage via JaCoCo. app/ requires ViewModel state tests. ArchitectureTest.kt runs 8 layer-separation rules on every test invocation.
>
> **MICRO:** ktlint with 4-space indent. Detekt with baseline for known warnings. SpotlessCheck on every build. Commit signing required for main branch.

---

## 2. Operational & Error Handling

Used by SPECIFICATION.md §9. Replace with your system's operations setup.

### MACRO — Operational Strategy

```
Logging framework: {{framework}}
Metrics: {{what to measure}}
Alerts: {{what triggers notification}}
Observability: {{tracing, dashboards}}
```

### MESO — Error Propagation Contract

_How errors propagate between components. What's handled locally vs. escalated._

### MICRO — Error Implementation Bounds

_Error message format, log line format, structured logging schema._

> **Bus-Hop Example:**
>
> **Logging framework:** Android Log (android.util.Log) - no third-party logger
> **Metrics:** Not collected (privacy-first, no analytics)
> **Alerts:** Not applicable (no server component)
> **Observability:** Not applicable
>
> **MESO:** API errors propagate from data/ through NetworkResult sealed class. ViewModel catches errors and maps to UI state. Domain layer never sees transport errors. Cache TTL shows stale data during outages rather than blank screens.
>
> **MICRO:** Log format: BusHop: [ClassName] message - context values. Error messages include HTTP code, endpoint URL, and exception type. No PII logged.

---

## 3. Production Quality Requirements

Used by EXECUTOR.md. These requirements apply to Tier 2+ projects (those with a runtime,
CLI, library, or performance-sensitive component). They are not optional polish — they
are baseline quality gates that must pass before a spec is considered execution-ready.

1. **Fuzz targets**: Every Tier 2+ project should have a `fuzz/` directory with at least one libFuzzer target. Run `cargo fuzz init` and add a basic target for the core API surface. Fuzz targets catch memory safety issues, panics, and undefined behavior that unit tests miss. Wire fuzz runs into CI as a scheduled job (not per-PR — too slow).

2. **Benchmarks**: Every performance-sensitive component should have a benchmark in `benches/`. Use `criterion` (statistical rigor) or `divan` (lower overhead). Track results in CI and fail on regressions beyond a configurable threshold. A benchmark without trend tracking is just a numbers game.

3. **Snapshot testing**: Use `insta` (Rust), `snapbox` (CLI output), or `expect_test` for golden-file assertions. Snapshot tests are far more efficient than hand-writing assertions for complex output — they catch regressions you didn't know to test for and make reviewing output changes trivial (just `cargo insta review`).

4. **CI matrix**: SPEC.md section 4 CI gates should cover Linux, macOS, and Windows at minimum. For Rust projects, add stable/beta/nightly to the matrix. Nightly failures are informational (not blocking), but beta failures are warnings that become blockers in the next release cycle.

5. **Test ratio**: Minimum 0.5x test-to-source lines. Measure via `cloc --json src/ tests/` or equivalent. This is a floor, not a target — the real metric is mutation score, but line ratio is a cheap proxy that catches projects with no tests at all.

6. **Security audit**: Add `cargo-deny` with a `deny.toml` to CI. Check for unmaintained dependencies, license compliance (no GPL in MIT-shipped crates), and known advisories. Run on every PR. A security audit that only runs before release is a security audit that misses everything merged in between.

---

## 4. Engineering-Specific Review Checks

Used by REVIEW.md Phase 4. These supplement the universal review checklist when the
project is engineering-deliverable.

| # | Check | How to Verify |
|---|-------|---------------|
| 4.5 | CI config or local check script exists | Check for .github/workflows/, .gitlab-ci.yml, Jenkinsfile, etc., or run ./scripts/check.sh (this repo). All checks must pass. |
| 4.6 | Standards audit passes | Run ./scripts/audit.sh from the Standards repo on this project. All checks must pass. |

---

## Origin

Extracted from the Development Protocol execution phase during the July 2026 structural
review. Engineering-specific content was consolidated here so the main protocol files
remain universal. The content is unchanged from its original form — only relocated.

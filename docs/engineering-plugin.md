# Engineering Plugin — Execution Addendum

> **When to use:** Your project involves engineering deliverables — software, hardware,
> firmware, infrastructure, or any technical system with build steps, tests, CI/CD,
> operations, production deployments, type safety checks, test infrastructure, or
> architecture fitness functions.
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

### 1.1 Feature Lifecycle (F-###) — Regression-Prevention Practice

Every feature gets a stable ID (F-###), a status lifecycle, and a behavior contract.
This prevents the fix-A-break-B loop: when every feature has defined pre/postconditions,
invariants, and error cases, changes can be verified against the contract instead of
discovered as regressions in production.

**Status lifecycle (four states):**

```
proposed ──▶ approved ──▶ applied ──▶ archived
                              │
                              └── superseded (→ archived; successor F-###)
```

| Status | Meaning | Ship? |
| --- | --- | --- |
| `proposed` | Intended, not yet ratified into V1 | No |
| `approved` | In V1 scope or added via learning shift | No — needs `applied` |
| `applied` | Implemented, tests anchored, spec-synced | Yes |
| `archived` | Removed/superseded; entry kept for history | No |

**Behavior contract (per feature):**

| Field | What it captures |
| --- | --- |
| Preconditions | What must be true before the feature's behavior is expected |
| Postconditions | What is guaranteed after it runs |
| Invariants | What never changes while it is in use |
| Error cases | What happens when inputs/state violate the preconditions |

**Test anchoring (mandatory for `applied`):**

| Test file / name | Covers |
| --- | --- |
| `{{path::test_name}}` | `{{postcondition / scenario / error case it proves}}` |

> **Anti-rot rule:** A `proposed`/`approved` feature MAY omit the test anchoring table.
> An `applied` feature MUST have ≥1 linked test per postcondition and per acceptance
> scenario. A test that proves no feature contract is either dead weight or a signal
> the feature is unregistered — flag it.

**Trace tags:** Tests reference features by ID (`F-001` in test name, docstring, or
`@F-001` tag). Grep-able: `grep -rn "F-001" tests/` returns every test anchored to
that feature. Feature entries reference tests in the Test Anchoring table (reverse link).

**Status transitions are logged** — no silent state changes. CI staleness check: warn if
`Reviewed:` is older than cadence; block if older than 2× cadence; fail if an `applied`
feature has zero linked tests or a test references an unknown `F-###`.

> **LINK to worked example:** See [Ithmb-Codec/docs/FEATURES.md](../../../Ithmb-Codec/docs/FEATURES.md)
> for a complete F-### inventory with 24 features, full behavior contracts, test anchoring
> tables, and trace tags. Do not inline that content — the pattern, not the data.

**Local vs GitHub CI split:**

| Scope | Runs where | What | Latency |
| --- | --- | --- | --- |
| **Local** | Pre-commit (before push) | clippy / tsc --noEmit / ruff / lint / unit tests / cargo-deny / gitleaks / check-i18n.mjs / check-wasm-drift.sh | < 2 min |
| **GitHub** | CI matrix (per-PR or scheduled) | webkit / macOS / Windows matrix / fuzz (scheduled) / benchmarks (trend) / coverage upload | minutes |

> **Source:** nami's local-first principle — fast feedback loop (< 2 min) catches 80% of
> issues before they leave the machine. GitHub CI handles platform-specific and
> resource-intensive checks that cannot run locally.

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

7. **Feature traceability**: Every F-### in `docs/FEATURES.md` MUST have ≥1 anchored test (see §1.1). `cargo-deny` and `gitleaks` are MANDATORY T1 gates — not T2. A feature without a test anchor is unverified intent; a gate that only runs at release misses everything merged in between.

> **`FEATURES.md` is the living spec** — it tracks what exists and how it behaves.
> `SPECIFICATION.md` is the static plan-IS-spec frozen at execution start.
> Keep `FEATURES.md` current; let `SPECIFICATION.md` reflect the locked design decisions.

---

## 4. Engineering-Specific Review Checks

Used by REVIEW.md Phase 4. These supplement the universal review checklist when the
project is engineering-deliverable.

| # | Check | How to Verify |
|---|-------|---------------|
| 4.5 | CI config or local check script exists | Check for .github/workflows/, .gitlab-ci.yml, Jenkinsfile, etc., or run ./scripts/check.sh (this repo). All checks must pass. |
| 4.6 | Standards audit passes | Run ./scripts/audit.sh from the Standards repo on this project. All checks must pass. |
| 4.7 | FEATURES.md current? Every `applied` F-### has test anchor and status is `applied`? | `grep -c F-` tests/ + check docs/FEATURES.md exists. Every `applied` feature MUST have ≥1 anchored test. Stale `Reviewed:` dates beyond cadence are a FAIL. |
| 4.8 | TECH_DEBT_AUDIT.md severity×effort triaged or explicitly empty | Check docs/TECH_DEBT_AUDIT.md exists and has severity×effort matrix. If no debt found, file must state that explicitly (not just missing). |

---

## Origin

Extracted from the Development Protocol execution phase during the July 2026 structural
review. Engineering-specific content was consolidated here so the main protocol files
remain universal. The content is unchanged from its original form — only relocated.

---

## 5. Type Safety & Static Analysis Gate

Type safety verification runs as a **pre-commit gate BEFORE implementation begins**. If the type check fails, implementation cannot proceed until errors are resolved.

**Language-specific commands:**

| Language | Command | Config reference |
| --- | --- | --- |
| Python | `basedpyright --pythonversion 3.12` | `pyproject.toml` → `[tool.basedpyright]` |
| TypeScript | `tsc --noEmit` | `tsconfig.json` → `"strict": true` |
| Rust | `cargo clippy -- -D warnings` | `.clippy.toml` |
| Go | `go vet ./...` + `staticcheck ./...` | `go.mod` + `staticcheck.conf` |

**Enforcement rules:**

- These run as a pre-commit gate BEFORE implementation begins
- If type check fails, implementation cannot proceed
- `# type: ignore`, `@ts-ignore`, `@ts-expect-error` require documented justification
- Configuration templates are referenced per-language above

**When to skip:** Non-software projects (research paper, marketing campaign, etc.)

---

## 6. Test Infrastructure (Software Projects)

**Add to Polish Checklist:**

| Category | What to check | Evidence level |
| --- | --- | --- |
| **Type safety** | basedpyright/tsc --strict/clippy/ruff passes on all source files | Mandatory |
| **Test coverage** | coverage.py/c8 reports 80%+ statement coverage on new code | Mandatory |

**Exit Criteria addition:**
- [ ] Type check passes on all source files
- [ ] Test coverage verified — coverage.py/c8 reports 80%+ on new code

**Test generation workflow (TDD for software):**
1. Write tests from SPECIFICATION.md (not from implementation)
2. Run tests — must fail (red phase)
3. Implement code
4. Run tests — must pass (green phase)
5. Check coverage — 80%+ on new code
6. Mutation testing — 80%+ mutation score

See [TESTING.md](../TESTING.md) for full test infrastructure documentation.

**When to skip:** Non-software projects.

---

## 7. Architecture Fitness Functions (Software Projects)

For software projects with modular architecture, enforce structural health:

- **Dependency Rule:** Core modules must not import infrastructure. All dependencies point inward.
- **Module Boundary Rule:** Every module has a single public entry point. No cross-module imports of internal paths.
- **Complexity Budget:** No function exceeds 40 lines. No file exceeds 250 lines.
- **Cycle Detection:** No circular imports between modules.

These are enforced via Standards repo check scripts (dependency-rules.sh, complexity-budget.sh).

**When to skip:** Non-software projects or single-file projects.

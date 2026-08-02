# Universal Automation Standards

Project-agnostic automation rules extracted from the Ithmb-Codec retrospective.
No project-specific context — applicable to any software project.

---

## Tier 0 — Immutable Rules (apply to every project, no exceptions)

These are non-negotiable. Every new project gets these before the first feature commit.

| # | Rule | Implementation | Catches |
|---|------|---------------|---------|
| 0.1 | **CI build + test passes** | 10-line workflow, fails red | Breaking changes |
| 0.2 | **Static analysis with warnings-as-errors** | Roslyn/Clippy/Biome config, `TreatWarningsAsErrors` | 80% of common bugs |
| 0.3 | **Dependency vulnerability scanning** | Dependabot + `list --vulnerable` | Known-CVE deps |
| 0.4 | **Secret scanning on every push** | gitleaks or equivalent | Credential leaks |
| 0.5 | **All commits signed** | `git config commit.gpgsign true` | Untrusted history |
| 0.6 | **Reproducible builds** | Lockfile + pinned CI runner versions | Build environment drift |
| 0.7 | **CHANGELOG with [Unreleased]** | Keep a Changelog format, empty section exists | No release notes gap |
| 0.8 | **README skeleton** | What, Why, How, Status, People | Newcomer onboarding |

## Tier 1 — Strongly Recommended (add within first 10 commits)

| # | Rule | Implementation | Catches |
|---|------|---------------|---------|
| 1.1 | **Commit message convention** | Conventional Commits enforced by hook/CI | Unparseable history, bad changelog gen |
| 1.2 | **CHANGELOG presence CI check** | `git diff --name-only | grep CHANGELOG` in CI | Forgotten changelog updates |
| 1.3 | **Stats gate (derive numbers from source)** | Script that extracts metrics from code, fails README if they don't match | Stale documentation numbers |
| 1.4 | **Build provenance metadata** | Embed commit SHA + build timestamp at compile time | Untraceable binaries |
| 1.5 | **Code coverage gate** | Minimum threshold, CI fails if coverage drops | Coverage regressions |
| 1.6 | **Formatting enforcement** | `dotnet format` / `rustfmt` / `prettier` in CI | Style inconsistency |
| 1.7 | **EditorConfig** | LF, UTF-8, indent size, trim trailing whitespace | Cross-IDE formatting chaos |
| 1.8 | **Global.json / SDK pinning** | Pin exact SDK/toolchain version | Environment mismatch bugs |
| 1.9 | **Signed tags on releases** | `git tag -s vX.Y.Z` + CI validates tag signature | Tampered release tags |
| 1.10 | **Concurrency-safe state design** | Lock/Atomic/Concurrent collections for all shared state (even if single-threaded now) | Expensive retrofit later |

## Tier 2 — High Value (add within first release cycle)

| # | Rule | Implementation | Catches |
|---|------|---------------|---------|
| 2.1 | **Performance regression gate** | Benchmark baseline + CI threshold check | Performance regressions |
| 2.2 | **Fuzz testing (scheduled)** | Weekly CI job with coverage-guided fuzzer | Latent OOB/overflow/DoS bugs |
| 2.3 | **Production-grade rubric** | Multi-axis maturity model with honest baseline scores | Invisible quality gaps |
| 2.4 | **Scheduled adversarial audit** | Quarterly deep-dive by someone unfamiliar with recent changes | Logic bugs CI can't catch |
| 2.5 | **Release artifact automation** | Tag-triggered workflow builds + uploads assets | Manual build errors on release day |
| 2.6 | **Correlation tokens in logs** | `COMPONENT|EVENT|details` format with request-scoped ID | Untraceable failures in production |
| 2.7 | **No file >250 pure LOC** | CI gate measures SLOC, fails on violation | God-class files |
| 2.8 | **Test quality gate** | Every test asserts at least one behavior path (no tautologies) | False confidence from "passing" tests |

## Tier 3 — Quality of Life (add when pain surfaces)

| # | Rule | Implementation | Saves |
|---|------|---------------|-------|
| 3.1 | **Design decisions recorded (ADRs)** | `docs/adr/` directory, even 2-line entries | "Why did we do X?" months later |
| 3.2 | **Commit date alias** | `git config alias` with `GIT_COMMITTER_DATE=$(git log -1 --format=%aD)` | Rebases without date drift |
| 3.3 | **Release notes from CHANGELOG** | `gh release create --notes-from-tag` | Manual note writing |
| 3.4 | **PR template** | Checklist: CHANGELOG, tests, static analysis, sign-off | Forgetting gates in review |
| 3.5 | **Pre-commit hooks** | Format + lint + CHANGELOG check before commit | CI catching things that could be caught earlier |
| 3.6 | **Multi-architecture CI** | Test on all supported platforms, not just primary | Silent platform-specific bugs |

---

## Automation Priority Matrix

```
P0 (day 1)     → Tier 0: 0.1-0.8
P1 (10 commits) → Tier 1: 1.1-1.10
P2 (1 release)  → Tier 2: 2.1-2.8
P3 (as needed)  → Tier 3: 3.1-3.6
```

Each tier builds on the previous. Do not skip tiers.
A project with Tier 0 only is viable. A project without Tier 0 is unacceptable.

---

| Anti-pattern | Why | Alternative |
|---|---|---|
| Squashing history for "cleanliness" | Consumes effort, degrades bisect, makes tags orphan | Keep atomic commits, squash-merge PRs only |
| Over-abstracting (helpers for 1 caller) | Premature indirection, harder to follow | Wait for 3rd caller |
| Perfection on first pass | Analysis paralysis | "Good enough" → ship → refactor |
| Manual number maintenance | Guaranteed drift | Derive from source, gate in CI |
| Adding PGO before coverage baseline | PGO destabilizes coverage measurement | Establish coverage floor, then PGO |
| Postponing thread safety | Retrofitting touches every shared data structure | Design atomics/locks from day 1 |
| No SAST until breach | Secret leaks invisible until pushed | Gitleaks on every push, day 1 |
| CI that passes unconditionally | `|| true` in CI steps hides failures | Every step either fails or is explicitly exempted |
---

## The Principle

**Every manual verification is a CI gate waiting to happen.**

If you catch yourself checking something manually (test count, formatting, CHANGELOG, error handling), stop and ask: "Can this be automated?" If yes, do it before the next feature commit. The gate pays for itself the first time it catches a regression.

---

## The Ordering Principle

**Install the safety net before walking the tightrope.**

CI → static analysis → tests → features → benchmarks.

Not the reverse. Every feature added without a gate creates latent bugs.

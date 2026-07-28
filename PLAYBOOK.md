# Project Standards & Automation Playbook

What Ithmb-Codec taught us, packaged for the next project.

---

## Day 1 — Before Writing Code

### Repository Setup

```bash
git init
git commit --allow-empty -m "chore: seed empty commit" -S
```

### Config Files (create immediately)

- `global.json` (if .NET) — pin SDK version
- `packages.lock.json` — lockfile for reproducible builds
- `.editorconfig` — whitespace, encoding, line endings
- `.gitignore` — whitelist pattern (`/*` → `!/path/to/keep`) if repo is a standalone binary
- `Dependabot` or equivalent — automated dependency updates from day 1

### CI (create before first feature commit)

- **Build + Lint + Test** workflow — failing = red, must pass for any merge
- **Static analysis** — Roslyn analyzers (C#), Clippy (Rust), Biome (TS), or equivalent
- **Code signing** — all commits signed from day 1 (`commit.gpgsign = true`)

### Documentation

- **README** skeleton: What, Why, How, Status, People (5 sections max)
- **CHANGELOG** with `[Unreleased]` header (Keep a Changelog format)
- **CONTRIBUTING** (if open-source): How to build, test, PR workflow

### Quality Gates (before first feature)

- Static analysis passes on empty project (baseline)
- Test framework reports "1 test passed" (proves coverage)
- CI green on the empty project

## Every Feature

### Before Coding

- [ ] Design decision recorded (even 2 lines: "Why approach A over B")
- [ ] Test cases listed: happy path, edge cases, error paths
- [ ] CHANGELOG `[Unreleased]` section updated
- [ ] CI gate script updated if feature changes project metrics

### After Coding

- [ ] Static analysis passes
- [ ] All tests pass (existing + new)
- [ ] No `as any` / `@ts-ignore` / `// nolint` without reason comment
- [ ] No file exceeds 250 pure LOC (measure: `awk` with SLOC filter)
- [ ] Commit signed with `GIT_MASTER=1 git commit -S`

## Every Release

### Automated

- [ ] Tag pushed → CI builds release assets
- [ ] Assets uploaded to GitHub Release
- [ ] Release notes generated from CHANGELOG section
- [ ] Version bumped in project config

### Manual (but checklisted)

- [ ] CHANGELOG `[Unreleased]` → `[X.Y.Z] — YYYY-MM-DD`
- [ ] CHANGELOG compare links updated
- [ ] CI gate scripts pass (no stale numbers)
- [ ] README stats verified (CI gate runs anyway, but double-check)
- [ ] Tag signed (`git tag -s vX.Y.Z -m "vX.Y.Z"`)

## Automation Must-Haves

| Priority | Automation | What it prevents | Implementation |
|---|---|---|---|
| P0 | CI build + test | Breaking changes | 10-line workflow |
| P0 | Static analysis | 80% of common bugs | Language-specific config |
| P0 | Dependabot | Known-vulnerability dependencies | One config file |
| P1 | Stats gate (like `check-readme-stats.sh`) | Stale numbers in docs | Shell script, CI step |
| P1 | CHANGELOG presence check | Forgotten changelog | CI: `git diff --name-only | grep CHANGELOG` |
| P1 | Signed commits | Untrusted history | `git config commit.gpgsign true` |
| P2 | Fuzz testing (scheduled) | Latent OOB/overflow/crash | Weekly CI job |
| P2 | Benchmark comparison | Performance regression | Manual CI dispatch |
| P2 | Release artifact build | Manual build errors | Tag-triggered workflow |

## What NOT To Do

| Anti-pattern | Why | Alternative |
|---|---|---|
| Squashing history for "cleanliness" | Consumes effort, degrades bisect | Keep atomic commits |
| Over-abstracting (helpers for 1 caller) | Premature indirection, harder to follow | Wait for 3rd caller |
| Perfection on first pass | Analysis paralysis | "Good enough" → ship → refactor |
| Manual number maintenance | Guaranteed drift | Derive from source, gate in CI |
## Tool Choices (Ithmb-Codec Proven Stack)

| Domain | Tool | Why |
|---|---|---|
| Language | C# Native AOT | ImageGlass plugin ABI, zero-runtime, single binary |
| Testing | xUnit + BenchmarkDotNet | Industry standard, reliable, fast |
| CI | GitHub Actions + `rtk` | Reproducible local + CI, same commands |
| Static analysis | Roslyn analyzers | Language-native, TreatWarningsAsErrors |
| Quality | `Terraform`-style review | Systematic audit before release |
| Fuzzing | SharpFuzz / LibFuzzer | Catch OOB and overflow (our 7 bugs were of this class) |
| Release | `gh release create` + upload | Simple, no extra tooling |

## The Principle

**Every manual verification is a CI gate waiting to happen.**

If you catch yourself checking something manually (test count, profile count, error handling, formatting), stop and ask: "Can this be automated?" If yes, do it before the next feature. The gate pays for itself the first time it catches a regression.

---

## v1.6.0 Addendum — New Standards

### Additional Day 1 Items

- [ ] **SAST/secret scanning** — gitleaks or equivalent from day 1
- [ ] **Build provenance** — embed commit SHA and build timestamp at compile time
- [ ] **Production-grade rubric** — create 8-axis maturity model with honest baseline scores
- [ ] **Concurrency design** — if multi-threading is possible, design shared-state access patterns upfront (Lock, Interlocked, ConcurrentDictionary)

### Additional Every Feature Items

- [ ] **Structured log tokens** — use `COMPONENT|EVENT|details` format for all operational logs
- [ ] **Benchmark baseline** — commit a performance baseline CSV and add regression check before deploying perf-sensitive changes

### Additional Automation Must-Haves

| Priority | Automation | Prevents |
|---|---|---|
| P1 | Vulnerability scanning (`list --vulnerable`) | Shipping with known-CVE dependencies |
| P1 | Build provenance metadata | Untraceable binaries |
| P2 | Coverage-without-PGO gate | False coverage regressions from optimizer changes |
| P2 | Audit reminder (quarterly) | Accumulated latent bugs |

### Additional Anti-Patterns

| Anti-pattern | Why | Alternative |
|---|---|---|
| Adding PGO before coverage baseline | PGO destabilizes coverage measurement | Establish coverage floor, then enable PGO |
| Postponing thread safety | Retrofitting touches every shared data structure | Design atomics/locks from day 1 |
| No SAST until breach | Secret leaks are invisible until pushed | Gitleaks on every push, day 1 |

---

## Reference Implementation

For an example of ALL these standards implemented in a real project, see the
[Standards repo](https://github.com/B67687/standards) — a cross-repo standards
repository with pre-commit hooks, commitlint, SOPS encryption, lychee link checking,
gitleaks secret scanning, and a self-audit CI workflow. It is the most complete
implementation of this playbook across any project.

```
# Apply the protocol to a new project:
cp ../Development-Protocol/RULES.md .
# Edit V1 scope, Constitution, AI persona
# Reference the Standards repo for infrastructure implementation details:
#   https://github.com/B67687/standards
```

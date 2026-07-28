# Production Quality Standards

## Overview

Production quality standards derived from benchmark analysis of **35 top OSS projects** across 5 categories — CLI tools (ripgrep, bat, fd, dust, procs), dev tools (ruff, biome, cargo, rust-analyzer), editors/parsers (tree-sitter, helix, neovim), systems/infra (tokio, serde, containerd, Redis, SQLite, curl, jq, Nix), and Rust libraries (clap, anyhow, thiserror, toml). All maintained by small teams (1-5 people) with 2-10 years to maturity. These are the practices that separate "works on my machine" from projects users trust with production data.

## Required Practices (Tier 1 — Every Project)

Non-negotiable. If your project doesn't do these, it isn't production-ready.

1. **Multi-platform CI** — Linux, macOS, Windows. GitHub Actions matrix. No exceptions. If it doesn't compile on all three, it doesn't ship.
2. **Test-to-source ratio >= 0.5x** — Measured as test lines / source lines. Projects below this threshold have significantly higher incident rates. This is a floor, not a target.
3. **Structured CHANGELOG** — Per-version entries with Added/Changed/Fixed/Removed sections. Not just git log. Users need to know what broke before they upgrade.
4. **Fuzz testing for core APIs** — `cargo-fuzz` or `libFuzzer` on every public API that parses untrusted input. Finding one crash before a user does is worth weeks of dev time.
5. **Release checklist** — Documented, repeatable process. Tag, build, verify, publish. If it's in someone's head, it's a bus factor of 1.

## Strongly Recommended (Tier 2 — Production)

These separate "it works" from "it works reliably under stress."

1. **Snapshot/expect testing** — `insta` or `snapbox` for CLI output and structured data. Makes refactors safe and diffs visible. Catch regressions that unit tests miss.
2. **Benchmarks with regression tracking** — Criterion.rs or equivalent in CI. Alert on >5% regression. Performance is a feature; losing it silently is a bug.
3. **Security auditing** — `cargo-deny` for dependency audit, license compliance, and vulnerability scanning. Run on every PR. Supply chain attacks target popular crates.
4. **Cross-architecture testing** — ARM64 (Apple Silicon) + x86_64 at minimum. QEMU for exotic targets. Architecture bugs are the hardest to debug in production.
5. **Formal error handling** — Typed errors with `thiserror` or equivalent. No string errors. Users need to programmatically match error variants, not parse error messages.

## Aspirational (Tier 3 — OSS Excellence)

These are what the best projects do. Not required, but they build trust and catch edge cases that matter at scale.

1. **Loom/memory model checking** — For async/concurrent code. `loom` validates that your concurrency primitives are correct under all possible interleavings. Catch data races before they become Heisenbugs.
2. **Fuzzing via OSS-Fuzz** — Continuous fuzzing infrastructure. Catches bugs that unit tests and manual fuzzing miss. Apply once; run forever.
3. **Property-based testing** — `proptest` or `quickcheck`. Define invariants; let the fuzzer find inputs that break them. Complementary to fuzzing — fuzzing finds crashes, properties find logic errors.
4. **Performance benchmarking published in CI** — Criterion reports in PR comments. Users see the impact of their changes before merge. Transparency builds trust.
5. **AI/contribution policy** — Written policy on AI-generated contributions (like ripgrep's `AI_POLICY.md`). Sets expectations for contributors. Prevents low-effort AI slop from flooding issue trackers.

## Integration with Our Workflow

Map each practice to where it fits in the Dev Protocol pipeline:

| Practice             | Dev Protocol Location                                                      |
| -------------------- | -------------------------------------------------------------------------- |
| Multi-platform CI    | SPECIFICATION.md §4 (CI/CD), start-work verification gates                 |
| Test-to-source ratio | SPECIFICATION.md §8 (Testing), EXECUTOR.md Production Quality Requirements |
| Structured CHANGELOG | SPECIFICATION.md §10 (Release), POLISH.md Production Quality Gates         |
| Fuzz testing         | SPECIFICATION.md §8 (Testing), EXECUTOR.md "Core API Coverage"             |
| Release checklist    | SPECIFICATION.md §10 (Release), POLISH.md "Ship Readiness"                 |
| Snapshot testing     | EXECUTOR.md Production Quality Requirements, start-work verification gates |
| Benchmarks           | EXECUTOR.md Production Quality Requirements, POLISH.md Performance Gates   |
| Security auditing    | EXECUTOR.md Production Quality Requirements, start-work verification gates |
| Cross-arch testing   | SPECIFICATION.md §4 (CI), start-work verification gates                    |
| Typed errors         | EXECUTOR.md Production Quality Requirements, POLISH.md Code Quality Gates  |
| Loom                 | EXECUTOR.md (concurrency code only)                                        |
| OSS-Fuzz             | POLISH.md (OSS projects only)                                              |
| Property testing     | EXECUTOR.md Production Quality Requirements                                |
| Benchmark publishing | POLISH.md Performance Gates                                                |
| AI policy            | AGENTS.md (project-level), POLISH.md Community Gates                       |

**start-work verification gates** enforce Tier 1 practices as hard blockers. Tier 2 and 3 are checked in POLISH.md gates and executor quality requirements.

---

_Reference: OSS benchmark analysis (Jul 2026) across ripgrep, tokio, serde, clap, ruff, biome, tree-sitter, neovim, helix, Redis, SQLite, curl, jq, Nix, containerd, and 20 additional projects._

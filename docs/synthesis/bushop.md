# BusHop — Android Retrospective Case Study

## Project Profile

| Dimension | Value |
|-----------|-------|
| Type | Android app (Kotlin/Jetpack Compose) |
| Modules | 3 (app/domain/data) — Clean Architecture |
| Source files | 25 .kt files, ~5,200 LOC |
| Tests | 112 (10 test files) |
| Dependencies | Retrofit 3.0, OkHttp 5.4, Compose BOM, DataStore, Gson |
| CI | **None** |
| Deployed to | GitHub Releases + Obtainium |

---

## Key Findings

### What BusHop Did Right (Universal Lessons)

| Lesson | Why it matters |
|--------|---------------|
| **Architecture tests as CI** | 8 automated rules in `ArchitectureTest.kt` verify module boundaries, dependency direction, domain purity, ProGuard validity. This is a lightweight ArchUnit — no extra framework needed. |
| **Domain layer purity** | Domain module has zero Android dependencies. 43 tests run without mocking. This is textbook clean architecture and pays off in test simplicity. |
| **CancellationException rethrown everywhere** | Every coroutine catch block explicitly rethrows CancellationException. Prevents structured concurrency leaks. Should be a universal standard. |
| **Interface-based abstractions for testability** | `BusRepository` (30 methods), `BusArrivalDataSource`, `UpdateChecker` — all interfaces created specifically for mocking. Constructor injection throughout. |
| **TLS pinning without Firebase** | Network security config with 6 certificate pins across 2 APIs. No CA trust store expansion. This is achievable without a 3rd-party service. |
| **Self-update security** | APK download URL validated (host must be github.com, must be HTTPS), installed via FileProvider, install permission checked. No hash verification, but good baseline. |

### What BusHop Missed (Universal Anti-Patterns)

| Anti-pattern | Lesson |
|-------------|--------|
| **No CI at all** | 103 commits, 112 tests, 3 modules — and zero automation. Tests only run on the developer's machine. Legacy/chaos state. |
| **Test count badge drift** | 3 different numbers (README=161, pipeline.mmd=154, actual=112). The `updateBadges` task exists but is run manually. Same anti-pattern as Ithmb-Codec's profile count drift. |
| **No lockfile** | `gradle.lockfile` doesn't exist. Transitive dependencies can drift. For a self-updating app, this is a supply chain risk. |
| **Conventional commits adopted halfway** | 26% of commits use conventional prefixes. No enforcement. History is a mix of "fix:" and "Polish:" and "Bump version". |
| **Mixed error models** | `NetworkResult<T>`, Kotlin `Result<T>`, and `String?` used in the same vertical slice. |
| **No JaCoCo / coverage** | Test-to-code ratio is 56% but nobody knows what's actually covered. |
| **No ADRs** | Architecture decisions (3-module split, NetworkResult design, DataStore choice) are undocumented rationale. |

---

## New Universal Standards Discovered

These emerged from the BusHop analysis and were NOT in the Ithmb-Codec-derived standards:

| Standard | Derived from | Why |
|----------|-------------|-----|
| **Architecture-as-test** | BusHop's `ArchitectureTest.kt` | Module boundary enforcement via unit tests. Add to docs/standards/UNIVERSAL_AUTOMATION_STANDARDS.md Tier 1. |
| **CancellationException rethrow discipline** | BusHop's coroutine patterns | Add to docs/standards/DESIGN_STANDARDS_HIERARCHY.md under Concurrency. |
| **Self-update security baseline** | BusHop's APK download validation | Add to PLAYBOOK.md release checklist. |
| **TLS pinning without CA expansion** | BusHop's network security config | Add to docs/standards/DESIGN_STANDARDS_HIERARCHY.md under Security. |

---

## Business Value of the Methodology

BusHop is a **different project type** from Ithmb-Codec:
- Ithmb-Codec: .NET Native AOT library, SIMD, plugin architecture
- BusHop: Android Kotlin/Compose app, REST API, local storage

Both were analyzed with the same methodology (4 parallel agents, 3 layers, automation gap extraction). The methodology is **project-type-agnostic**. It works for:
- Systems libraries (Ithmb-Codec)
- Mobile apps (BusHop)
- (Untested: web backends, CLI tools, embedded systems)

---

*Analysis performed 2026-07-01. Methodology in METHODOLOGY.md.*

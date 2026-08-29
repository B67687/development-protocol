# SE Artifact Registry

**Last updated:** 2026-08-29

**Purpose:** Central cross-repo index of all Software Engineering (SE) artifacts — features, specifications, architecture, tech debt, and ADRs — across the Ithmb ecosystem.

---

## Lifecycle Reference

All features follow the **F-### Feature Lifecycle** defined in [engineering-plugin.md §1.1](engineering-plugin.md):

```
proposed → approved → applied → archived
                  └→ superseded → archived
```

Status transitions are logged. CI enforces staleness checks. Applied features **must** have ≥1 linked test.

---

## Cross-Repo SE Artifact Table

| Repo | FEATURES | SPEC | ARCH | TECH_DEBT | ADR | Local Gate | CI |
|---|---|---|---|---|---|---|---|
| **Ithmb-Codec** | `docs/FEATURES.md` (F-001..F-024) | `SPECIFICATION.md` (FR-01..50, NFR-01..08) | `ARCHITECTURE.md` | `docs/TECH_DEBT_AUDIT.md` | `docs/adr/` (8 ADRs) | `scripts/local-ci.sh` (7 gates) | `.github/workflows/ci-full` + `pr-checks` (matrix) |
| **Ithmb-Codec-Web** | `docs/FEATURES.md` (F-001..F-029) | `SPECIFICATION.md` | `ARCHITECTURE.md` (115 lines) | `TECH_DEBT_AUDIT.md` | `docs/adr/` (2 ADRs) | `scripts/check-local.sh` (11 checks) | `.github/workflows/ci.yml` (3 browsers) |
| **Imageglass-Ithmb-Plugin** | `docs/FEATURES.md` (F-001..F-010) | `SPECIFICATION.md` | `docs/ARCHITECTURE.md` | `TECH_DEBT_AUDIT.md` | `docs/adr/` (2 ADRs) | `scripts/check-local.sh` (7 checks) | `.github/workflows/ci.yml` (3 OS) |
| **Bus-Hop** | `docs/FEATURES.md` (F-001..F-010 archived) | `SPECIFICATION.md` (20 FR + 7 NFR) | `docs/ARCHITECTURE.md` (164 lines) | `TECH_DEBT_AUDIT.md` | `docs/adr/ADR-004` | `scripts/check-local.sh` (graceful) | Intentionally removed per ADR-004 (no workflows) |
| **Development-Protocol** | `docs/FEATURES.md` (F-001..F-008) | `SPECIFICATION.md` (MACRO/MESO/MICRO) | `docs/adr/` (3 ADRs) | `docs/TECH_DEBT_AUDIT.md` | `docs/adr/` (3 ADRs) | `scripts/check-local.sh` (4 gates) | Local-only (no GitHub Actions, by design) |

---

## Fitness Function Thresholds

| Repo | LOC Limit | Status |
|---|---|---|
| Ithmb-Codec | 250 LOC | At threshold |
| Ithmb-Codec-Web | 250 LOC | At threshold |
| Imageglass-Ithmb-Plugin | 250 LOC | 5 over tracked |
| Bus-Hop | 300 LOC | At threshold |

---

## Local-First CI Principle

Local gates run in **< 2 minutes**. GitHub heavy matrix tests are free via PUBLIC dev repos (Development-Protocol-Dev → Development-Protocol public mirror). Agents should always run local checks before pushing.

---

## Coverage Summary

- **Total repos tracked:** 5
- **Total FEATURE IDs:** 81+ (F-001..F-029 across repos)
- **Total ADRs:** 16+
- **Total local gates:** 29+ checks across repos
- **CI matrices:** 3 browser (Web), 3 OS (Plugin), full matrix (Codec), none (Bus-Hop per ADR-004, Development-Protocol by design)

---

*This registry is a living document. Update when adding/removing SE artifacts in tracked repos.*

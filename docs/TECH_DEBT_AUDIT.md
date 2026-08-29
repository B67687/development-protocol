# Tech Debt Audit — Development Protocol

**Last triaged:** 2026-08-29
**Audit scope:** Root repo structure, file sizes, artifact completeness, HANDOVER freshness

## Debt Items

| # | Item | Severity | Effort | Status | File/Location |
|---|------|----------|--------|--------|---------------|
| D-001 | Root sprawl: 18 .md files at root | medium | high | active | `/` (root) |
| D-002 | SPECIFICATION.md oversized: 586 lines | medium | medium | active | `SPECIFICATION.md` |
| D-003 | FEATURES.md was template-only (no F-### entries) | high | low | **resolved** | `docs/FEATURES.md` |
| D-004 | No TECH_DEBT_AUDIT.md existed | high | low | **resolved** | `docs/TECH_DEBT_AUDIT.md` |
| D-005 | SE_ARTIFACT_REGISTRY missing self-row | high | low | **resolved** | `docs/SE_ARTIFACT_REGISTRY.md` |
| D-006 | HANDOVER stale (claimed fff81bf, actual d85d173) | high | low | **resolved** | `Development-Protocol-Local/HANDOVER.md` |
| D-007 | No check-local.sh (only basic check.sh) | high | low | **resolved** | `scripts/check-local.sh` |
| D-008 | No AGENTS.md at root | high | low | **resolved** | `AGENTS.md` |
| D-009 | No .github/workflows (by design) | info | — | accepted | N/A |
| D-010 | RULES.md placeholder content | low | high | deferred | `RULES.md` |

## Severity × Effort Matrix

```
         Low Effort    Medium Effort    High Effort
High     D-003 ✅      D-001            —
         D-004 ✅
         D-005 ✅
         D-006 ✅
         D-007 ✅
         D-008 ✅
Medium   —             D-002            D-001
Low      D-010          —               D-010
Info     D-009          —               —
```

## Active Debt (requires action)

### D-001: Root Sprawl (18 .md files)

**Severity:** medium | **Effort:** high

Root contains 18 .md files: AMBITION, DECOMPOSITION, EXECUTOR, EXTRACTION, FAILURE_CAPTURE, FUNDAMENTALS, INBOX, LANDSCAPE, PRIORITIZE, REFLECT, REVIEW, RULES, SERIOUSNESS, SPECIFICATION, STANDARDS, STRATEGY, VALIDATION, README, AGENTS.

The 14 pipeline steps are intentionally at root (per README.md design). RULES.md and STANDARDS.md are governance files also at root by design. However, this creates visual clutter and makes it harder to distinguish pipeline steps from governance from meta-files.

**Options:**
1. Accept as-is — the 14-step root layout is the protocol's identity
2. Move governance (RULES, STANDARDS) to `docs/governance/`
3. Create a `steps/` directory for pipeline steps

**Recommendation:** Accept as-is (option 1). The root sprawl IS the protocol's structure. Moving steps would break the README.md pipeline diagram and every cross-reference.

### D-002: SPECIFICATION.md Oversized (586 lines)

**Severity:** medium | **Effort:** medium

SPECIFICATION.md contains 16 sections (§0-15) at 586 lines. The three-layer model (MACRO/MESO/MICRO) is comprehensive but dense. Could be split into:
- `SPECIFICATION.md` — overview + MACRO layer
- `docs/specification-meso.md` — MESO layer details
- `docs/specification-micro.md` — MICRO layer details + templates

**Risk of splitting:** Breaks §15 verification checklist references, README.md links.

**Recommendation:** Defer. The file is reference material, not frequently edited. Splitting adds maintenance overhead for marginal readability gain.

## Resolved Debt (this session)

- **D-003:** FEATURES.md populated with 8 F-### entries (F-001 through F-008)
- **D-004:** TECH_DEBT_AUDIT.md created (this file)
- **D-005:** SE_ARTIFACT_REGISTRY.md now includes Development-Protocol self-row
- **D-006:** HANDOVER.md updated to actual HEAD d85d173, Standards row added, privacy/signing bullets added
- **D-007:** scripts/check-local.sh created with 4 gates
- **D-008:** AGENTS.md created at root

## Accepted Debt (by design)

- **D-009:** No GitHub Actions — local-first CI is the protocol's design choice per ADR-001.

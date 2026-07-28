# Research: Shift-Friendly Architecture Patterns — 2026-07-19

## Applied to: Development Protocol v3 design

### Summary

Research across 6 topics to answer: "What does a protocol look like that DESIGNS FOR goalpost shifts instead of fighting them?"

---

## 1. Architecture Patterns for Changeability

**Core insight:** Hexagonal Architecture (Ports & Adapters) is about _swap cost_ — every interface boundary represents a decision you're deferring. The fewer core modules that know about infrastructure (databases, APIs, file systems), the cheaper it is to replace any of them.

**Key references:** Alistair Cockburn (Hexagonal Architecture), Simon Brown (C4), Martin Fowler.

**When it backfires:**

- Adding a port before there are two implementations (interface with one impl = complexity)
- Abstraction for things that won't realistically be swapped
- More than 4 ports per module (Cockburn's limit)

---

## 2. Test Strategies That Enable (Not Block) Refactoring

**The paradox:** Bad tests prevent refactoring. Good tests enable it. The difference is _what the test asserts_.

| Test type                                        | Survives restructuring?   |
| ------------------------------------------------ | ------------------------- |
| Implementation test (private methods, internals) | No — breaks on rename     |
| Behavioral test (return values, error types)     | Yes — behavior is stable  |
| Contract test (message shape)                    | Yes — if contracts stable |
| Property-based test (invariants)                 | Yes — most resilient      |

**Key references:** Martin Fowler, Kent Beck, J.B. Rainsberger (integration tests are a scam).

**Protocol rules:**

- Test the contract, not the implementation
- No mocking of your own classes (mock external deps only)
- No logic (if/for/while) in test files
- Prefer property-based tests for core domain invariants

---

## 3. Modular Monolith: The Solo Dev Sweet Spot

**Consensus:** Modular monolith is the correct default for teams under 30 people. For a solo dev, microservices = death by ops.

| Architecture          | Solo dev complexity       | Restructuring cost |
| --------------------- | ------------------------- | ------------------ |
| Unstructured monolith | Low initially, high later | High               |
| Modular monolith      | Medium                    | Low                |
| Microservices         | Very high                 | High               |

**Protocol rules:**

- Modules communicate only through public interfaces
- One module = one directory = one bounded context
- Shared kernel exists and is minimal

**Key references:** CodingDroplets 2026 (microservices = 5-10x ops cost).

---

## 4. Shape Up's Scoping Mechanism

**Three pillars:**

- **Fixed time, variable scope** — 6-week cycle. Don't extend time, cut scope.
- **Circuit breaker** — If not done in 6 weeks, kill it (not extend). Wrong shaping needs rethink.
- **Betting table** — 1-2 hour decision meeting. Output: cycle plan. No backlog grooming.

**Protocol rules:**

- Every cycle produces a shipped artifact (not "progress")
- Only bet one cycle ahead — clean slate each cycle
- Appetite before scope

**Key references:** Ryan Singer, Shape Up (Basecamp).

---

## 5. AI-Specific: Making Agent-Generated Code Change-Friendly

**The problem:** AI generates 3-5x faster but defaults to over-engineering (deep inheritance, "just in case" abstractions, structure-coupled tests).

**Patterns that work:**

- Constrain before generation (structural constraints in system prompt)
- Require tests before code (TDD for agents)
- Code review as structural gate (reject single-impl interfaces, internal mocking, oversized files)
- Indexed context beats raw generation

**Key references:** Addy Osmani (2026), "TDD Governance for Multi-Agent Code Generation" (2026).

---

## 6. Counter-Evidence: Over-Engineering Signals

**Fowler's four costs:** Build (30-50% unused), Delay (speculative = not customer), Carry (every abstraction is maintained), Repair (real requirement differs).

**Five signals:**

1. Interface with one impl
2. Config options nobody changes (same value for 6 months)
3. Generic function used once
4. Database field with no reader
5. Manager/Factory for one type

**When early investment IS justified:** Security, data schema, public API contracts, compliance.

**Key reference:** Fowler, YAGNI article.

---

## 10 Protocol Rules for v3

1. **INTERFACE RULE** — No interface before its second consumer.
2. **TEST RULE** — Test the contract, not the implementation. No mocking own code. No logic in tests.
3. **MODULE BOUNDARY RULE** — Every module has a single public entry point.
4. **SIZE RULE** — No file exceeds 250 LOC. No function over 40 lines.
5. **CYCLE RULE** — Every cycle produces a shippable artifact.
6. **APPETITE RULE** — Define time before designing solution. Scope adjusts to fit.
7. **AI RULE** — AI-generated code passes same structural checks as human code.
8. **ABSTRACTION RULE** — Rule of three: extract on third occurrence, not first.
9. **DEPENDENCY RULE** — Core domain never imports infrastructure.
10. **BACKLOG RULE** — No perpetual backlog. Every cycle starts clean.

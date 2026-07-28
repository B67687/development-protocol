# Software Design Standards Hierarchy v1.0

> This is a companion to RULES.md's Constitution section. The Constitution defines project-specific
> immutable principles. This hierarchy defines the universal design philosophy that informs those
> principles.

A layered specification of design constraints and defaults.
Every micro-level rule is a traceable consequence of a macro-level axiom.
Not a checklist — a decision tree.
---

## Layer 1 — Architectural Axioms (Macro)

These are inviolable. Every rule in Layers 2 and 3 derives from one or more of these.
If a micro rule contradicts an axiom, the axiom wins.

### A1. Modularity
**Every unit has exactly one responsibility.**
- A "unit" is a file, class, function, or module.
- If you cannot name its single purpose in one sentence, split it.
- Cohesion within a unit must be stronger than coupling between units.

### A2. Data Flow Direction
**Data flows in one direction across defined boundaries.**
- Input enters at API boundaries, is validated immediately, then flows inward.
- No component reaches arounnd another to read/write its state.
- Return results upward; pass data downward.
- Consequence: mutation of function parameters is prohibited (caller owns the value's meaning, callee borrows it).

### A3. Fail-Fast
**Detect and report errors at the earliest possible point.**
- Validate all external input at the API boundary before any processing begins.
- Internal invariants are enforced by assertions (debug) or explicit checks (release).
- Never silently swallow an error. If you catch, you must handle or re-express.
- "An error that is not acted upon is not handled."

### A4. Explicit Over Implicit
**All side effects, resource ownership, and error paths are visible at the call site.**
- No hidden allocations, no invisible locks, no implicit state mutation.
- Resource lifecycle is evident from the calling code (RAII, Dispose, scope-based).
- Logging and metrics are explicit transition points, not sprinkled reactively.
- Consequence: a function signature communicates everything the caller needs to know.

### A5. Parse-Don't-Validate
**Transform untrusted input into a validated type that makes illegal states unrepresentable.**
- Parse at the boundary; the parsed type guarantees validity for its lifetime.
- No "stringly-typed" identifiers — use branded types / newtypes.
- If a state combination cannot exist, the type system should forbid it.

### A6. Layered Dependencies
**Dependencies point inward. No cycles.**
- Outer layers depend on inner layers; inner layers never depend on outer.
- Abstract interfaces live at the boundary of the layer that defines them.
- A dependency cycle is a design smell that must be resolved before merging.

---

## Layer 2 — Module-Level Contracts (Meso)

Each rule is annotated with its parent axiom(s).

### M1. Interface Surface (A1, A4)
- **M1.1** A module's public API must be smaller than its private implementation.
- **M1.2** If a method doesn't have at least one caller outside its class, make it private.
- **M1.3** Every public method must have an XML doc comment (or equivalent) stating contract preconditions and postconditions.
- **M1.4** Parameters are input-only. Output is via return values or explicit `out` parameters (not ref/pointer mutation).

### M2. State Management (A2, A4)
- **M2.1** Shared mutable state uses the most restrictive synchronization primitive that satisfies correctness.
- **M2.2** Immutable state needs no synchronization — prefer `readonly` / `const` / `frozen` where possible.
- **M2.3** Thread ownership is explicit: if a field is only accessed from one thread, document it; if shared, lock it.
- **M2.4** Global mutable state is forbidden except for infrastructure (loggers, caches, registries) with explicit thread-safe accessors.

### M3. Resource Lifecycle (A4)
- **M3.1** The allocator of a resource is its owner and must free it.
- **M3.2** In GC languages: `IDisposable` / `using` for any non-memory resource (handles, connections, streams).
- **M3.3** In non-GC languages: RAII or explicit paired acquire/release at the same scope level.
- **M3.4** Pooled resources (array pools, connection pools) are returned within the same method unless explicitly handed off.

### M4. Error Domains (A3, A4)
- **M4.1** Errors are typed (exception classes, result types, error enums), not strings or magic numbers.
- **M4.2** Errors are caught at the layer that can meaningfully handle them or re-express them for the next layer.
- **M4.3** Library code throws/library returns typed errors. Application code catches and translates to user-facing errors.
- **M4.4** An empty catch block is a bug. If you truly must ignore, comment why and log.
- **M4.5** Fallible operations return a result type (Rust `Result`, C# `OneOf`, Go `(T, error)`) rather than throwing for expected failure modes.

### M5. Module Boundaries (A1, A6)
- **M5.1** A module is a directory with a cohesive set of files plus a public API surface (index/module file).
- **M5.2** Cross-module references use the public API only — no peeking at internals.
- **M5.3** Circular dependencies between modules are resolved by extracting the shared abstraction into a third module.
- **M5.4** A module's test file has the same visibility as the module itself (if public, tests are public; if internal, tests are internal).

---

## Layer 3 — Expression-Level Discipline (Micro)

### Naming (A4)

| Rule | Entailment |
|------|-----------|
| **N1** Names reveal intent, not implementation. `DecodeFrame` not `ProcessData`. | A4: explicit intent |
| **N2** Boolean parameters are named as predicates: `IsValid`, `HasChildren`, `CanDecode`. | A4: call site clarity |
| **N3** Abbreviations are forbidden except for universally known ones (ID, IO, URL, RGB). | A4: no implicit knowledge |
| **N4** Variables are nouns, functions are verbs, booleans are predicates. | A4: consistency = readability |
| **N5** Type names are nouns or noun phrases. Interface names are adjectives or `I`-prefixed nouns. | A1: naming reveals responsibility |
| **N6** Negated names are forbidden: use `IsDisabled` not `IsNotEnabled`. | A3: double negatives hide logic errors |

### Branching (A3)

| Rule | Entailment |
|------|-----------|
| **B1** No nesting beyond 2 levels of `if`/`else`. Extract early. | A3: nested else hides failure paths |
| **B2** Guard clauses (early return) are preferred over nested if. | A3: detect and exit early |
| **B3** Switch/match must be exhaustive. If the compiler cannot enforce it, add a default that logs/throws. | A3: unhandled variant = latent bug |
| **B4** Ternary operators are single-condition only. No nested ternaries. | A4: implicit conditions hide intent |
| **B5** No `else` after `return`/`throw`/`break` — the branch is already resolved. | A4: redundant else adds noise |

### Function Structure (A1, A4)

| Rule | Entailment |
|------|-----------|
| **F1** A function fits in one screen height (~40 lines). If it doesn't, extract. | A1: one responsibility |
| **F2** A function does exactly one thing. Test this by asking: "can I split this into two meaningful functions?" | A1: unitary responsibility |
| **F3** A function has at most 3 parameters. Beyond that, introduce a parameter object. | A4: signature must be readable |
| **F4** Out parameters are exceptions, not the rule. Use return values or result types. | A4: output should be visible |
| **F5** Async methods use `Async` suffix. Sync wrappers of async methods use `Sync` suffix. | A4: sync/async distinction explicit |

### Memory & Concurrency (A2, A4)

| Rule | Entailment |
|------|-----------|
| **C1** Never mutate a function parameter. Create a local copy if you need to modify. | A2: data flows one direction |
| **C2** Lock scope is minimal — lock exactly the shared data, not the whole method. | A4: side effects visible and scoped |
| **C3** Prefer message passing (channels, queues) over shared memory for inter-thread communication. | A2: data flow direction |
| **C4** `volatile` / `Interlocked` / `Atomic` for single-variable state. Lock for multi-variable invariants. | A4: synchronization is explicit |
| **C5** No naked `new Thread()`. Use thread pool, task system, or dedicated thread with documented lifecycle. | A4: resource lifecycle explicit |

### File Structure (A1)

| Rule | Entailment |
|------|-----------|
| **S1** One public type per file. | A1: file = unit = one responsibility |
| **S2** No file exceeds 250 pure lines of code (comments and blanks excluded). | A1: a file is a unit; oversized = multi-responsibility |
| **S3** Partial classes are reserved for generated code + manual extension in the same domain. | A1: partials can violate single-responsibility |
| **S4** File name matches the primary type name. | A4: naming convention = findability |

---

## Entailment Proof Table

Every micro rule is a direct consequence of one or more macro axioms.
No arbitrary preferences.

| Layer 3 Rule | Derived From |
|---|---|
| N1-N6 (Naming) | A4 (Explicit Over Implicit) |
| B1-B5 (Branching) | A3 (Fail-Fast), A4 (Explicit) |
| F1 (40-line function) | A1 (Modularity) |
| F2 (one thing) | A1 (Modularity) |
| F3 (3 params max) | A4 (Explicit), A1 (Modularity) |
| F4 (out params exceptional) | A4 (Explicit), A2 (Data Flow) |
| F5 (Async suffix) | A4 (Explicit) |
| C1 (no mutate params) | A2 (Data Flow Direction) |
| C2 (minimal lock scope) | A4 (Explicit), A1 (Modularity) |
| C3 (message passing) | A2 (Data Flow Direction) |
| C4 (right tool for sync) | A4 (Explicit) |
| C5 (no naked threads) | A4 (Explicit), M3 (Resource Lifecycle) |
| S1 (one public type) | A1 (Modularity) |
| S2 (250 LOC) | A1 (Modularity) |
| S3 (partial classes limited) | A1 (Modularity) |
| S4 (file = type name) | A4 (Explicit) |

| Layer 2 Rule | Derived From |
|---|---|
| M1 (public < private surface) | A1 (Modularity), A4 (Explicit) |
| M2 (synchronization discipline) | A2 (Data Flow), A4 (Explicit) |
| M3 (resource ownership) | A4 (Explicit) |
| M4 (typed errors, no empty catch) | A3 (Fail-Fast), A4 (Explicit) |
| M5 (module boundaries) | A1 (Modularity), A6 (Layered Dependencies) |

---

## Applying the Hierarchy

When making any design decision:

1. **Identify the macro axiom** that applies to the situation. If multiple, the most constraining one wins.
2. **Check the meso contract** for the layer you're working in. If your change violates M1-M5, revisit the approach.
3. **Apply the micro rule**. If you're about to write a nested `if`, ask: "does fail-fast (A3) allow me to simplify this with a guard clause?" (B2).

This is a decision tree, not a reference manual.
If the tree leads to an ambiguous result, the ambiguity is a gap — document it and evolve the standard.

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-06-30 | Initial specification: 6 axioms, 5 meso contracts, 5 micro domains, entailment proof table |

---

## Relationship to docs/standards/UNIVERSAL_AUTOMATION_STANDARDS.md

`docs/standards/UNIVERSAL_AUTOMATION_STANDARDS.md` covers **process automation** (CI gates, releases, quality enforcement).
This file covers **design decisions** (architecture, modules, expressions).

They are orthogonal and complementary:
- The automation standards enforce that design standards are followed.
- The design standards give the automation standards something to enforce.

Both should be referenced when bootstrapping a new project.

---
Also related: RULES.md's Constitution section defines PROJECT-SPECIFIC immutable principles,
while this hierarchy defines the UNIVERSAL design philosophy that informs them.

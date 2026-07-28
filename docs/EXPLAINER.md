# EXPLAINER.md — Code Explanation Template

> Produced during POLISH phase, consumed before SPEC SYNC.
> Bridges the gap between "the AI built it" and "you understand what it does and why."
> One explanation per project. Updated as the project evolves.

## For the Reader

You built a project with help from AI agents. You can't read the code directly -- but you should still understand what your project does, how its parts fit together, and why certain decisions were made. This document gives you that understanding.

Think of it as a guided tour. Your AI agent fills in the template below for every project. After reading it, you should be able to describe your own repo to someone else with confidence.

---

## The Standardized Explanation Template

Every project gets one explanation with these five sections. The AI fills them out after the code is built (during POLISH) and before the next round of work begins (before SPEC SYNC).

### 1. Macro Architecture (3-5 sentences)

A bird's-eye view. What does the project do? What are its high-level pieces and how do they relate?

Write this so someone who has never seen the project can answer: "What is this thing, broadly?"

_Example (not real): "This project is a library that converts image files from one format to another. It has three layers: a parser that reads raw bytes, a decoder that turns bytes into pixels, and an encoder that writes pixels into a new format. Each layer is a separate module that talks to the next through a well-defined interface."_

### 2. Data Flow Walk

Pick one concrete user action or system trigger. Walk it from start to finish through every module it touches. Name each module as you enter it, and say what that module does with the data before passing it along.

The goal: a reader should be able to trace what happens when they press a button, send a request, or run a command.

_Example: "When a user uploads a .png file, the CLI module receives the file path and passes it to the Parser. The Parser reads the file header, confirms it's a valid PNG, and extracts the raw pixel data. It hands this data to the Decoder, which converts it into the project's internal pixel format (a list of RGBA values). The Encoder then takes those values and writes them into the output format the user requested. If any step fails, the error is returned with a plain-language message describing what went wrong and where."_

### 3. Module Breakdown

For each major module in the project, answer three questions:

- **Responsibility**: What does this module own? What design decision does it hide from other modules?
- **Public API**: What functions, types, or endpoints does it expose to the rest of the system? What can other modules ask it to do?
- **Key Types**: What are the main data shapes it works with?

List modules in dependency order (foundation first, higher-level later). Every module must have a single, clear responsibility. If a module does more than one thing, that's a smell -- flag it.

_Example:_

*| Module | Responsibility | Public API | Key Types |
*|--------|---------------|------------|-----------|
*| Parser | Reads file headers and validates format | parse(path) -> RawData or Error | RawData (bytes + metadata) |
*| Decoder | Converts raw bytes to internal pixel format | decode(RawData) -> Pixels or Error | Pixels (list of RGBA values) |
*| Encoder | Writes internal pixels to output format | encode(Pixels, format) -> bytes or Error | OutputFormat (enum: PNG, JPEG, WEBP) |

### 4. Key Decisions

For every non-obvious choice in the project, answer: what was the situation, what was the concern, what did we choose, and what did we accept as a tradeoff?

Format each decision as a short story:

_"We needed to store user data. We considered using a database (PostgreSQL) or flat JSON files. We chose SQLite because the project runs on a single machine and doesn't need a server. The tradeoff is that SQLite can't handle concurrent writes from multiple users -- but that's fine because this project is single-user."_

Include at least 3 decisions. Cover architecture, library choices, and tradeoffs that affect how the project works.

### 5. Quality Guarantees

How do we know the project is correct? List what prevents bugs from reaching the user:

- **Tests**: What kinds of tests exist and what do they cover? (Unit tests for individual functions, integration tests for module boundaries, end-to-end tests for full workflows.)
- **Invariants**: What must always be true, even when things go wrong? (Example: "No pixel value can be negative. The decoder validates this before returning data.")
- **Safety guarantees**: Does the project use a language that prevents certain bugs at compile time? Does it handle errors explicitly instead of crashing?
- **Automated checks**: What runs on every change (linters, type checkers, CI pipelines)?

Be honest about limits. If a module has no tests, say so. The reader should know where to be careful.

_Example: "The Parser and Decoder have unit tests for every error path (corrupted files, empty files, unknown formats). The Encoder has fewer tests -- it was added late. A CI pipeline runs all tests and a linter on every commit. The project is written in Rust, which guarantees no null pointer crashes or buffer overflows at compile time."_

---

## Mandatory Check

After reading this explanation, can you answer these questions in your own words?

1. What does this project do, and what are its 3-5 main pieces?
2. What happens from start to finish when you trigger the main action?
3. Which module has the most complexity, and what does it hide from the rest of the system?
4. What was the hardest design decision, and why was it made that way?
5. What would break first if something went wrong, and how would you know?

If you can't answer these, the explanation needs revision. If you can, you know your project at macro-to-micro level.

---

## How the AI Generates This

The AI follows a two-pass process. It never generates the explainer retroactively as a single write-once step.

**Pass 1: Before writing any code (during AMBITION and SPECIFICATION).**

The AI drafts sections 1 (Macro Architecture), 2 (Data Flow Walk), and 4 (Key Decisions) at the specification level. These describe the intended architecture. The user reviews and approves this macro design before the AI writes a single line of code. This is the architecture approval gate -- if the macro design is wrong, it's cheaper to fix now than after implementation.

**Pass 2: After code is built (during POLISH).**

The AI revisits every section and updates it to match what was actually built. The Data Flow Walk becomes precise (real function names, real error paths). The Module Breakdown lists real modules with their actual public APIs. The Quality Guarantees reflect what tests and checks actually exist, not what was planned.

The AI also fills the Mandatory Check questions and verifies that a non-technical reader could answer them from the explanation alone. If not, the AI revises until the explanation stands alone.

This two-pass structure prevents the most common failure: a beautiful explainer that describes a project that was never built.

---

## Pipeline Integration

EXPLAINER.md sits at the boundary between POLISH and SPEC SYNC in the Development Protocol pipeline:

```
RAW INTENT → EXTRACTION → FUNDAMENTALS → DECOMPOSITION
  → AMBITION → LANDSCAPE → VALIDATION → SPECIFICATION
  → EXECUTOR → POLISH → **EXPLAINER.md** → SPEC_SYNC
  → REVIEW → ship
```

- **Produced**: During POLISH, after code passes quality gates and before the human final pass is complete.
- **Consumed**: Before SPEC SYNC, so the explainer's updated understanding feeds into the spec-to-code fidelity check.
- **Updated**: Every cycle. When the project changes (new features, refactors, bug fixes), the explainer changes with it.

The explainer is not a substitute for SPECIFICATION.md. The spec is for the builder (what to build). The explainer is for the owner (what was built and why). They describe the same system at different levels of detail for different audiences.

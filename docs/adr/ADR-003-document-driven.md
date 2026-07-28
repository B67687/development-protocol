# ADR-003: Document-Driven, CLI-Acceleration

**STATUS:** Accepted (July 2026)

## Context
The protocol could be delivered as a CLI application, a set of documents, or both. A CLI-only approach would create a dependency on the tool. Document-only would miss acceleration opportunities.

## Decision
The protocol is **document-first** — Markdown files are the source of truth. The CLI (`project-kit`) is optional acceleration for operations like init, phase tracking, and check verification. Both are published but the documents work independently.

## Consequences
**Positive:** No tool dependency. Anyone can use the protocol with only a text editor.
**Positive:** Documents are version-controlled alongside projects.
**Negative:** CLI cannot enforce protocol rules (it can only check them). Rule enforcement relies on the executor's governance.

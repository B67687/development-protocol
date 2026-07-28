# Prototype: MECE Decomposition of Dev Protocol v3

## Cynefin Classification

**Problem:** Build Dev Protocol v3 from v2
**Domain:** COMPLICATED — cause-effect knowable through analysis. We know the mechanisms (MECE, Cynefin, appetite, rules). The work is integrating them correctly.

## Level 1 Decomposition

```
DEV PROTOCOL v3
├── 1. DOCUMENT CHANGES (KNOWN — trivially implementable)
│   ├── 1a. RULES.md — ✅ Done (added Section 2, renumbered)
│   ├── 1b. AMBITION.md — ✅ Done (research-interleaved flow)
│   ├── 1c. EXECUTOR.md — ADD 10 v3 rules
│   ├── 1d. SPECIFICATION.md — template needs update
│   ├── 1e. README.md — pipeline diagram update
│   ├── 1f. LANDSCAPE.md — add Cynefin classification step?
│   └── 1g. VALIDATION.md — prototyping guidance enhancement?
│
├── 2. PROCESS CHANGES (NEEDS DESIGN — not trivially implementable)
│   ├── 2a. Decomposition before routing — new flow order
│   ├── 2b. Research-interleaved AMBITION — new cycle
│   ├── 2c. Appetite-before-scope — how to codify?
│   ├── 2d. Clean slate per cycle — kill mechanism design
│   └── 2e. Goalpost shift as norm — reframe language throughout
│
├── 3. NEW RULES TO CODIFY (KNOWN — trivially implementable)
│   ├── 3a. Interface rule — no interface before 2nd consumer
│   ├── 3b. Test rule — contract over implementation
│   ├── 3c. Module boundary — single entry point
│   ├── 3d. Size limits — 250 LOC / 40 LOC per function
│   ├── 3e. Cycle rule — shippable artifact per cycle
│   ├── 3f. Appetite rule — time before scope
│   ├── 3g. AI rule — same structural checks
│   ├── 3h. Rule of three — extract on third occurrence
│   ├── 3i. Dependency rule — core ≠ infrastructure
│   └── 3j. Clean backlog — no perpetual backlog
│
├── 4. VALIDATION (NEEDS PROTOTYPE — only building reveals)
│   ├── 4a. Apply v3 to Oh-My-Learner maintenance
│   └── 4b. Apply v3 to next new project
│
└── 5. PHASE ORDER (NEEDS CONFIRMATION — your preference)
    ├── 5a. Documents first? (1→3→2→4)
    ├── 5b. Process first? (2→1→3→4)
    └── 5c. Parallel? (1+3 in parallel → 2 → 4)
```

## Classification

| Leaf                   | Class              | Status                |
| ---------------------- | ------------------ | --------------------- |
| 1a. RULES.md           | KNOWN              | ✅ Done               |
| 1b. AMBITION.md        | KNOWN              | ✅ Done               |
| 1c. EXECUTOR.md        | KNOWN              | Ready to implement    |
| 1d. SPECIFICATION.md   | KNOWN              | Ready to implement    |
| 1e. README.md          | KNOWN              | Ready to implement    |
| 1f. LANDSCAPE.md       | NEEDS DECISION     | Yes/no?               |
| 1g. VALIDATION.md      | NEEDS DECISION     | Yes/no?               |
| 2a-2e. Process changes | NEEDS DESIGN       | Needs shaping         |
| 3a-3j. Rules           | KNOWN              | Ready to implement    |
| 4a-4b. Validation      | NEEDS PROTOTYPE    | Must do after changes |
| 5a-5c. Phase order     | NEEDS CONFIRMATION | Your call             |

---

**Does this match your understanding of v3?**

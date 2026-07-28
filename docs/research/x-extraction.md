# Research: X-Extraction Methods — 2026-07-20

## Applied to: Development Protocol v3 — adding EXTRACTION step

### The Problem

People state Y (their proposed solution) when their real need is X (the actual problem).
The XY Problem. Every requirements engineer, product manager, and designer spends most of
their career fighting this cognitive bias.

### Key Techniques (Proven)

| Technique                  | Origin                                  | Evidence                                                 | Core Question                                                             |
| -------------------------- | --------------------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------- |
| **Goal Climb**             | KAOS (van Lamsweerde)                   | 30+ years in safety-critical systems (aviation, medical) | "What goal does Y serve?" — repeat until strategic                        |
| **No-Computer Check**      | Problem Frames (Jackson)                | Canonical SE method, taught in software engineering      | "If no technology existed, how would this manifest?"                      |
| **Why-Tree**               | Five Whys (Toyoda/Ohno)                 | Toyota Production System, adopted industry-wide          | "Why?" with forced branching — not a chain, a tree                        |
| **Contextual Probe**       | Contextual Inquiry (Beyer & Holtzblatt) | Used by Microsoft, IBM, SAP for product design           | "Show me the last time this happened — walk me through it"                |
| **Mom Question**           | The Mom Test (Fitzpatrick)              | Startups, product teams globally                         | "What's the hardest part about [situation]?" — past behavior, not opinion |
| **Problem Statement Wall** | Goal-Directed Design (Cooper)           | Standard in UX practice                                  | One paragraph, no solution words (app/tool/dashboard = rewrite)           |
| **Job/Pain/Gain Map**      | Value Proposition Design (Osterwalder)  | Used by 20,000+ companies                                | Map Y to customer profile — any job/pain/gain?                            |

### The EXTRACTION Step (Protocol Addition)

Positioned BEFORE decomposition and BEFORE AMBITION. Flow becomes:

```
RAW INTENT → EXTRACTION → DECOMPOSITION → AMBITION → LANDSCAPE → VALIDATION → SPECIFICATION → EXECUTOR → POLISH → EXPLAINER → SPEC_SYNC → REVIEW → ship
```

### Exit Criteria (proven by falsifiability)

1. X can be written in one sentence with no solution words
2. 3+ independent stakeholders agree on X
3. At least 3 alternative solution paths exist (Y is one of them)
4. X has measurable magnitude (hours lost, % of tickets, etc.)
5. No one can ask "why?" to X without getting circular

### EXTRACTION vs AMBITION

| Dimension      | EXTRACTION                           | AMBITION                           |
| -------------- | ------------------------------------ | ---------------------------------- |
| What it does   | Reveals X behind stated Y            | Determines scale, scope, quality   |
| Key question   | "What are we really trying to do?"   | "How good does it need to be?"     |
| Output         | Problem statement, no solution words | Success criteria, metrics, targets |
| Trap prevented | Building the wrong thing             | Building right thing poorly        |

### Key Sources

- van Lamsweerde, "Requirements Engineering: From System Goals to UML Models," Wiley 2009
- Jackson, "Problem Frames," Addison-Wesley 2001
- Ohno, "Toyota Production System," Productivity Press 1988
- Beyer & Holtzblatt, "Contextual Design," Morgan Kaufmann 1998
- Fitzpatrick, "The Mom Test," 2013
- Cooper, "About Face 3," Wiley 2007
- Osterwalder et al., "Value Proposition Design," Wiley 2014

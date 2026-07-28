# Layered Retrospective Protocol — Methodology v1.0

A structured protocol for distilling project experience into actionable standards and automation. Project-agnostic, repeatable, extensible.

## Why This Methodology Exists

Traditional post-mortems produce a list of "what went wrong" and "what went right." That's Layer 1. It's necessary but insufficient — without deeper abstraction, the same mistakes recur in the next project because you never extracted the underlying pattern.

This protocol adds 3 more layers on top of the concrete observations, building a pyramid from "what happened" to "what should be standardised" to "why things are the way they are" to "how to do this analysis itself."

## The Protocol

### Phase 1 — Collect Evidence (single wave, parallel)

**Input**: a project (repo, commits, docs, CI config, tests, history)

**Action**: Decompose the project into 4 orthogonal axes and dispatch one research agent per axis:

| Axis | What to examine | Who runs it |
|---|---|---|
| **Code structure** | Architecture, file sizes, responsibilities, patterns, code smells, naming conventions | explore agent |
| **Testing & CI** | Test pyramid, coverage, CI workflows, fuzzing, static analysis, gates | explore agent |
| **Process & docs** | Commit history, CHANGELOG, README, PRs, issue tracking, communication | explore agent |
| **External interactions** | Dependencies, upstream engagement, community, research sources | librarian agent |

**Output per axis**: A structured report with:
- Concrete observations (what exists, what works, what breaks)
- Patterns identified (what's repetitive)
- Gaps (what's missing)
- Leads for expansion (what needs deeper investigation)

### Phase 2 — Expand & Verify (2-3 waves)

**Input**: 4 axis reports

**Action**:
1. Cross-reference reports for contradictions and overlapping gaps
2. For each high-impact gap or contradiction, spawn an expansion agent
3. Verify key claims by reading the actual code or running tests (not taking agent reports at face value)
4. Collate all findings into a single evidence base

**Stopping criteria**: No new patterns emerge from 2 consecutive expansion waves.

### Phase 3 — Layer Extraction

Build 4 nested layers:

```
Layer 4 (Meta-meta)
  └── The methodology itself — how to do this analysis
Layer 3 (Meta-patterns)
  └── Why the patterns exist, systemic drivers
Layer 2 (Patterns & standards)
  └── What recurrences to standardise
Layer 1 (Concrete)
  └── What happened, what worked, what broke
```

#### Layer 1 — Concrete observations
Extract from agent reports. Be specific: file paths, metrics, dates, evidence. Categories:
- What went well (with evidence)
- What broke repeatedly (with occurrences and root causes)
- Infrastructure gaps (what's missing)
- Team/process patterns

#### Layer 2 — Patterns and standards
From the concrete observations, identify:
- **Recurring patterns**: "Every time we changed X, Y broke" → "X and Y should be tied together"
- **Correction cascades**: One change requiring 3+ follow-up fixes
- **Tool gaps**: Manual processes that should be automated
- **Standards candidates**: Specific, actionable behaviours to enforce

Format for each standard:
```
| Standard | Description | Automatable? | Impact |
```

#### Layer 3 — Meta-patterns
From the patterns, extract why they exist:
- **Systemic causes**: Why did patterns recur? (tooling gaps, process missing, skill deficit)
- **Trade-offs made explicit**: What was consciously traded for speed/quality/simplicity
- **Cultural patterns**: How the team/AI interaction shaped outcomes
- **Invariant constraints**: Things that would be true regardless of approach

Each meta-pattern answers: "What drives this pattern to exist?"

#### Layer 4 — Meta-meta (methodology)
Extract the analysis process itself:
- What steps were taken
- Which tools/agents were used
- How decisions were made
- What the output artifacts are
- How long each phase took
- When this methodology is appropriate

This becomes a reusable protocol for future projects.

### Phase 4 — Write Artifacts

| Artifact | Content | Audience |
|---|---|---|
| **Layer 1-3: Synthesis** | Full layered analysis with evidence | Project team |
| **Layer 2: Playbook** | Actionable Day-1 checklist and automation must-haves | Future project starts |
| **Layer 4: Methodology** | This protocol, project-agnostic | Anyone doing project analysis |
| **Claim ledger** | Verified vs unresolved claims | Audit trail |

## When to Use This Protocol

| Trigger | Depth | Estimated time |
|---|---|---|
| After a milestone (release, major refactor) | Layers 1-2 | 15-30 min |
| After a correction cascade (3+ bugs in same area) | Layers 1-3 | 30-60 min |
| Before starting a new project | Full protocol | 60-90 min |
| Quarterly deep review | Full + benchmarks | 2-4 hrs |

## How This Was Produced

This methodology was extracted by applying itself to the Ithmb-Codec project:

1. **4 deep agents** were dispatched in parallel (code, testing, automation research, methodology research)
2. Their reports were cross-referenced and contradictions surfaced
3. The 4 layers were extracted by asking iteratively: "what happened" → "what pattern" → "why" → "how do I know this"
4. The process itself was documented as this file

**Time investment**: ~20 minutes total (4 agents at ~3-5 min each + 10 min synthesis)

**Tools used**: 4 `deep` category agents in OpenCode, each with research scope defined

## License

This methodology is released into the public domain. Use, modify, distribute freely.

# FEATURES.md — Development Protocol Feature Inventory

**Purpose:** Catalog every intended feature of the Development Protocol — regardless of current implementation status. Features are organized by protocol phase. Status reflects whether the feature is documented, implemented in tools, and verified by use.

> This file is the **source of truth for what the protocol intends to do**. If a feature appears here but isn't working in practice, that's a bug. If a feature doesn't appear here but exists in the code, it may be undocumented or drift.

## Legend

| Status | Meaning |
|---|---|
| ✅ **Active** | Documented, implemented, and verified through use |
| 🟡 **Partial** | Documented but not fully implemented or rarely used |
| 🔧 **Recent** | Added recently, needs more verification cycles |
| 📄 **Documented** | Defined in protocol files but not yet used in practice |
| 🎯 **Planned** | Intended but not yet designed or implemented |

---

## Phase -1: INBOX — Thought Capture & Triage

| Feature | Status | Location | Notes |
|---|---|---|---|
| Multi-thought capture | ✅ Active | INBOX.md Step -1 | Raw thoughts collected without filtering |
| Energy/Timing/Tractability triage | ✅ Active | INBOX.md Phase 3 | 3-question prioritization per cluster |
| Traffic Light Review | ✅ Active | INBOX.md Phase 4 | Sorted output, Someday/Maybe parking |
| Protocol Suitability Check | 🔧 Recent | INBOX.md | 5-question entry gate — added after capability review |
| Strategic Alignment Check | 🔧 Recent | INBOX.md | 1/3/10 year trajectory question — added after foundation audit |
| Cross-session park accumulation | 🟡 Partial | INBOX.md → `.omo/inbox/parked/` | Mechanism defined, directory accumulation pattern unverified |
| Someday/Maybe re-evaluation trigger | 🎯 Planned | — | No automatic trigger to revisit parked items |

## Phase 0.5: PRIORITIZE — Idea Comparison (optional)

| Feature | Status | Location | Notes |
|---|---|---|---|
| 3-dimension comparison (Want/Know/Work) | 🔧 Recent | PRIORITIZE.md | 1-3 scoring, raw display, no composite |
| Debiasing question | 🔧 Recent | PRIORITIZE.md | "Would I pick this if someone else proposed it?" |
| Bet-frame decision | 🔧 Recent | PRIORITIZE.md | "Which gets my next X days?" |
| Coin flip tiebreaker | 🔧 Recent | PRIORITIZE.md | Reveals hidden preference through emotional reaction |

## Phase 0: EXTRACTION — Problem Discovery

| Feature | Status | Location | Notes |
|---|---|---|---|
| Language clarification | ✅ Active | EXTRACTION.md Step 0 | Assume 5 meanings per sentence |
| 10 extraction techniques | ✅ Active | EXTRACTION.md Step 2 | 7 standard + 3 psychology-enhanced |
| Intent Landscape (7 reframes) | ✅ Active | EXTRACTION.md Intent Landscape | 6 reframing techniques + CHOOSE |
| X testing (5 criteria) | ✅ Active | EXTRACTION.md Step 3 | No solution words, 3+ perspectives, etc. |
| Dual ZPD Principle | 🔧 Recent | EXTRACTION.md header | Task zone + Learning zone |
| Incomplete Extraction Detection | 🔧 Recent | EXTRACTION.md | 5 signals of performative extraction |
| Set-Based Multiple Hypotheses | 🔧 Recent | EXTRACTION.md | Carry 2-3 X's through FUNDAMENTALS |
| Provenness requirement | ✅ Active | EXTRACTION.md | Every technique must have empirical citation |

## Phase 1: SERIOUSNESS — Commitment Gate

| Feature | Status | Location | Notes |
|---|---|---|---|
| Phase 1: Commitment Probe | ✅ Active | SERIOUSNESS.md | 4-question probe with explicit scoring |
| Phase 2: 5-Dimension Scoring | ✅ Active | SERIOUSNESS.md | D1-D5 each 0-20 with probes |
| Phase 3: Kill Criteria | ✅ Active | SERIOUSNESS.md | Named killer assumption |
| Anti-Sunk-Cost Mechanisms (7) | ✅ Active | SERIOUSNESS.md | Pre-commit, default DROP, probes go down, etc. |
| Risk Register | 🔧 Recent | SERIOUSNESS.md | L×I scoring 1-25, pipeline integration |
| Recursion exemption (protocol improvements) | 📄 Documented | SERIOUSNESS.md | Skip when improving the protocol itself — noted in protocol state but not formalized in SERIOUSNESS.md |

## Phase 1.5: FUNDAMENTALS — One-Way Doors & Bias

| Feature | Status | Location | Notes |
|---|---|---|---|
| Reversibility Classification | ✅ Active | FUNDAMENTALS.md Step 1 | One-way vs two-way door decision tree |
| Validation Depth Decision Tree | ✅ Active | FUNDAMENTALS.md Step 2 | Schema + 3 queries |
| LLM Bias Detection (4 types) | ✅ Active | FUNDAMENTALS.md Step 3 | Popularity, Wrong abstraction, Self-contradiction, Monoculture |
| Team Capability Audit | ✅ Active | FUNDAMENTALS.md Step 5 | Skills, tooling, dependencies, compliance |
| Self-assessment (overconfidence) calibration | 🔧 Recent | FUNDAMENTALS.md Step 5a | Teach-back + Dunning-Kruger signal detection |
| Chain Analysis of Dependent Doors | 🔧 Recent | FUNDAMENTALS.md Step 6 | Chain risk levels, awareness rule |

## Phase 2: MULTI — Cross-Disciplinary Probes

| Feature | Status | Location | Notes |
|---|---|---|---|
| 9 cross-disciplinary probes | ✅ Active | MULTI.md | Ethics, Psychology, Economics, Ecology, Design, etc. |
| Domain relevance filter | ✅ Active | MULTI.md | Flags by relevance level (🔴🟡🟢) |

## Phase 3: DECOMPOSITION — Problem Breakdown

| Feature | Status | Location | Notes |
|---|---|---|---|
| Cynefin classification | ✅ Active | DECOMPOSITION.md | Clear/Complicated/Complex/Chaotic |
| MECE tree construction | ✅ Active | DECOMPOSITION.md | Mutually Exclusive, Collectively Exhaustive |
| Known/Research/Prototype routing | ✅ Active | DECOMPOSITION.md | Branch-level routing decisions |
| Level of Care (1-4) | ✅ Active | DECOMPOSITION.md | Trivial → Independent system |
| Sub-cycle recursion | ✅ Active | DECOMPOSITION.md | Level 3+ forks into new protocol cycle |
| Parallel Spike Routing | 🔧 Recent | DECOMPOSITION.md | 2-3 approaches explored in parallel via shallow subcycles |

## Phase 4: AMBITION — Goal Tightening

| Feature | Status | Location | Notes |
|---|---|---|---|
| Round 1: Open (unfiltered) | ✅ Active | AMBITION.md | User speaks freely |
| Round 2: Interrogate (3 challenges) | ✅ Active | AMBITION.md | Assumption, Alternative-perspective, Meta-cognitive |
| Round 3: Steer (post-research) | ✅ Active | AMBITION.md | AI reframes with research input |
| Round 4: Converge | ✅ Active | AMBITION.md | Build the ambition |
| Round 5: Lock (Schön criteria) | ✅ Active | AMBITION.md | Specificity, Falsifiability, Testability, Baseline, Congruence |
| Research interleaving (mandatory) | ✅ Active | AMBITION.md | AI researches between rounds |
| Challenge 4: Education/Understanding check | 🔧 Recent | AMBITION.md | Teach-Back method — verify user comprehension |

## Phase 5: LANDSCAPE — Structured Research

| Feature | Status | Location | Notes |
|---|---|---|---|
| Frame: Research questions + scope | ✅ Active | LANDSCAPE.md Step 1 | Direct/Adjacent/Technology/Failure/Unknowns framing |
| Read Everything First meta-principle | 🔧 Recent | LANDSCAPE.md | Added after capability review (shallow-read prevention) |
| Pattern & Reference Survey | 🔧 Recent | LANDSCAPE.md Step 1a | Default Project question, priority table (pattern > solution > negative > component) |
| Search (5 strategies) | ✅ Active | LANDSCAPE.md Step 2 | Direct/Citation chaining/Adjacent/Expert/Negative search |
| Evaluation (CRAAP test + evidence hierarchy) | ✅ Active | LANDSCAPE.md Step 3 | 6-level evidence hierarchy, CRAAP criteria |
| Synthesis | ✅ Active | LANDSCAPE.md Step 4 | Research strategy table with confidence |
| Confirmation bias safeguards (5) | ✅ Active | LANDSCAPE.md | Pre-register questions, active negative search, etc. |

## Phase 6: PACING — Budget & Tracking

| Feature | Status | Location | Notes |
|---|---|---|---|
| Phase budget mapping | ✅ Active | PACING.md §1 | Default % allocations per phase |
| Adjustment rules | ✅ Active | PACING.md §1 | 1 week vs 6 week appetite scaling |
| Phase-level tracking | ✅ Active | PACING.md §2 | `.omo/pacing-track.md` |
| AI Session Boundaries | ✅ Active | PACING.md §3 | Max turns/reads per phase |
| Context decay signals (4) | ✅ Active | PACING.md §3 | Inconsistent naming, repeated questions, etc. |
| Pace Alerts | ✅ Active | PACING.md §4 | +50% over budget triggers alert |
| Macro Cycle Structure | ✅ Active | PACING.md §5 | 4-6 week cycles with 1-week cooldown |
| Non-protocol interruption handling | ✅ Active | PACING.md §6 | Save checkpoint, resume protocol |
| Effort Estimation | 🔧 Recent | PACING.md §8 | T-Shirt sizing + decomposition + reference class |
| Cost Tracking (human + AI) | 🔧 Recent | PACING.md §9 | `.omo/cost-track.md` |
| FINISH/POLISH sub-allocation | 🔧 Recent | PACING.md | 20-30% of EXECUTOR budget reserved for finish phase |
| Monte Carlo risk modeling | 🎯 Planned | — | Deferred — requires 5+ project data |

## Phase 7: VALIDATION — Prototyping Gate

| Feature | Status | Location | Notes |
|---|---|---|---|
| Prep Phase | ✅ Active | VALIDATION.md | Arrival checklist, hypothesis formation |
| Prototype design (5 methods) | ✅ Active | VALIDATION.md | Paper, Digital, Wizard of Oz, Concept, Smoke test |
| Pre-mortem | ✅ Active | VALIDATION.md | 2-sentence failure story |
| KILL/PIVOT/COMMIT scoring | ✅ Active | VALIDATION.md | 3-dimension, threshold-based decision |
| Confirmation bias safeguards (5) | ✅ Active | VALIDATION.md | Pre-mortem, Devil's advocate, Default OFF, etc. |
| Failure modes of prototypes (5) | ✅ Active | VALIDATION.md | Feasibility, Scalability, Usability, etc. |

## Phase 8: SPECIFICATION — Detailed Plan

| Feature | Status | Location | Notes |
|---|---|---|---|
| 14-section template | ✅ Active | SPECIFICATION.md | Constitution through AI Attribution |
| 3-layer structure (MACRO/MESO/MICRO) | ✅ Active | SPECIFICATION.md | System → Component → Implementation |
| 3 tiers (Tier 1/2/3 sections) | ✅ Active | SPECIFICATION.md | Mandatory → Production → Open-source |
| 18-item Verification Checklist | ✅ Active | SPECIFICATION.md §15 | Placeholders filled, constitution, CI gates, etc. |
| Engineering plugin reference | 🔧 Recent | SPECIFICATION.md | CI/Tooling §4 and Operations §9 now universal stubs |

## Phase 9: EXECUTOR — Implementation

| Feature | Status | Location | Notes |
|---|---|---|---|
| Route selection (6 routes) | ✅ Active | EXECUTOR.md | Standard/Discover-First/UX-First/Explore-Only/Port/Maintenance |
| Autonomy levels (1-4) | ✅ Active | EXECUTOR.md | Script → Execute → Gate → Operate |
| Midpoint Protocol Check | 🔧 Recent | EXECUTOR.md | Mid-implementation protocol retrospective |
| Retry Protocol (3-step ladder) | 🔧 Recent | EXECUTOR.md | 1st retry differently → 2nd flag human → 3rd BLOCKING |
| Degradation Strategy | 🔧 Recent | EXECUTOR.md | Section priority order + drop procedure |
| Emergency Contact | 🔧 Recent | EXECUTOR.md | 3 consecutive failures → pause, report, wait |
| Paradigm Review (Revolutionary Mode) | 🔧 Recent | EXECUTOR.md | 3+ failed assumptions → loop to EXTRACTION |
| FINISH Gate (POLISH) | 🔧 Recent | EXECUTOR.md | 20-30% budget, 9-category checklist, exit criteria |
| Design for Change Rules | ✅ Active | EXECUTOR.md | 10 rules for adaptive architecture |
| Production Quality (engineering plugin) | 🔧 Recent | EXECUTOR.md | Stub → docs/engineering-plugin.md |

## Phase 10: EXPLAINER — Architecture Documentation

| Feature | Status | Location | Notes |
|---|---|---|---|
| Architecture doc generation | 📄 Documented | EXPLAINER.md | Universal format — applies to any project type |

## Phase 11: REVIEW — Independent Meta-Review

| Feature | Status | Location | Notes |
|---|---|---|---|
| 5-phase checklist | ✅ Active | REVIEW.md | Goal, Quality, Security, Context, Protocol |
| Independence protocol (4 requirements) | ✅ Active | REVIEW.md | Fresh session, no prior context, etc. |
| Self-review mechanism | ✅ Active | REVIEW.md | Run review on REVIEW.md itself |
| Spec-to-Code Fidelity Check | 🔧 Recent | REVIEW.md | Verifies implementation matches specification |
| Engineering plugin checks (4.5, 4.6) | 🔧 Recent | REVIEW.md | Stub → docs/engineering-plugin.md |

## Phase 12: REFLECT — Protocol Retrospective

| Feature | Status | Location | Notes |
|---|---|---|---|
| Q1: What did the protocol catch? | ✅ Active | REFLECT.md | Protocol's value proposition — concrete evidence |
| Q2: What did the protocol miss? | ✅ Active | REFLECT.md | Protocol improvement input |
| Q3: What changed in the protocol? | ✅ Active | REFLECT.md | Learning shifts |
| Q4: What should the protocol learn? | ✅ Active | REFLECT.md | Lesson entry → LESSONS.md |
| Q5: What did you learn about yourself? | 🔧 Recent | REFLECT.md | Human learning capture |
| Q6: What thinking pattern to watch for? | 🔧 Recent | REFLECT.md | Meta-cognitive pattern awareness |
| Q7: Graduation Check | 🔧 Recent | REFLECT.md | Self-transcendence — has the protocol become automatic? |

## Cross-Cutting Features

| Feature | Status | Location | Notes |
|---|---|---|---|
| Protocol Output Format (3-layer) | 🔧 Recent | SKILL.md | Overview → Verbose → Round Checkmarks |
| Recursion Meta-Rule | 📄 Documented | FUNDAMENTALS.md, README.md | Sub-projects fork into own cycle |
| Engineering Plugin | 🔧 Recent | docs/engineering-plugin.md | CI/Tooling, Operations, Production Quality, Review checks |
| Human Psychology & Education | 🔧 Recent | EXTRACTION.md, AMBITION.md, FUNDAMENTALS.md | 3 new techniques, Teach-Back, overconfidence calibration |
| Foundation Audit Methodology | 🟡 Partial | — | Used in Session 5 but not formalized as repeatable step |
| Cross-Project Portfolio Tool | 🎯 Planned | — | Separate repo for comparing/ranking projects across sessions |

## Deprecated / Removed Features

| Feature | Status | Notes |
|---|---|---|
| Standalone POLISH step | ❌ Removed | Merged into EXECUTOR.md as FINISH Gate (Session 2) |
| Standalone SPEC_SYNC step | ❌ Removed | Merged into REVIEW.md (Session 2) |
| METHODOLOGY.md | ❌ Removed | Moved to standards repo (Session 3) |

---

*Last updated: 2026-07-28*

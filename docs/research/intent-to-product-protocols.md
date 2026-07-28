# Research: Intent-to-Product Protocols — 2026-07-19

## Applied to: Development Protocol v2.2 self-improvement

### Summary

Research across 6 topics to answer: "Can a protocol take raw intent → finished product in one shot with no gaps?" Convergence of Schön, Brooks, and Polanyi shows this is theoretically impossible — but the practical response is a redesign of the protocol's approach to goalpost shifts.

---

## 1. The Paradox (Schön + Brooks + Polanyi)

| Thinker     | Key Idea                                                                         | Implication for Protocol                                                                           |
| ----------- | -------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| **Schön**   | Problem-setting IS problem-solving. The problem reveals itself through building. | You cannot spec everything upfront. The act of building IS the act of discovering.                 |
| **Brooks**  | Essential complexity can't be reduced by better specs.                           | Some ambiguity is structural. A perfect spec is mathematically impossible for non-trivial systems. |
| **Polanyi** | "We can know more than we can tell." Most expert knowledge is tacit.             | The protocol's assumption that "all knowledge can be articulated in specs" is false.               |

**Resolution**: The protocol should target **decision-sufficient** (enough to start safely, resilient to discovery) not **decision-complete** (no gaps). Discovery is inherent, not a bug.

---

## 2. Gaps in Current Development Protocol (v2.2)

### Critical

- **No Cynefin classification** — CLEAR/UNCLEAR binary is too coarse. Complex (probe-sense-respond) and Chaotic (act-sense-respond) domains are missing.
- **"Decision-complete" is theoretically impossible** — Schön+Brooks+Polanyi converge on this. The north star should shift to "decision-sufficient."
- **No tacit knowledge capture** — No mechanism for surfacing what the user knows but can't articulate.

### Moderate

- **No appetite/scoping** — Unlike Shape Up, protocol doesn't ask "how much time is this worth?"
- **No shaping/building separation** — Prometheus does both. Creates blind spots.
- **No formal NFR capture** — Volere's 8 categories (security, performance, etc.) not addressed.
- **No mandatory documentation artifacts** — ADR, C4 diagrams, arc42 not required.
- **No betting/commitment mechanism** — No way to prioritize plans by strategic importance.

### Minor

- **No essential vs accidental complexity analysis** — Can't prioritize what needs precise spec vs what can be left to judgment.
- **No feedback integration** — Plans are fire-and-forget.
- **CLEAR path could use defaults** — Some questions could be answered by defaults instead of asking.

---

## 3. Key External References

| Source                                 | Key Takeaway                                                                          |
| -------------------------------------- | ------------------------------------------------------------------------------------- |
| **Shape Up** (Ryan Singer / Basecamp)  | Appetite-driven scoping. Time is fixed, scope flexes. Shapers ≠ builders.             |
| **Cynefin** (Dave Snowden)             | 5 domains: Clear/Complicated/Complex/Chaotic/Disorder. Different response per domain. |
| **Volere** (Robertson & Robertson)     | 27-section requirements template. 8 NFR categories now standard.                      |
| **IEEE 830 / ISO 29148**               | Structured SRS: functional + non-functional + interface + constraints.                |
| **PAW (SDD-Pilot)**                    | Multi-model plan generation, phase PRs, per-phase agents.                             |
| **GitHub Spec Kit**                    | Spec is executable. Code serves spec. 240 contributors.                               |
| **Prometheus / OMO ulw-plan**          | Interview-mode planner. CLEAR/UNCLEAR routing. Two-filter discipline.                 |
| **Schön, The Reflective Practitioner** | Problem-setting is the core professional activity.                                    |
| **Brooks, No Silver Bullet**           | Essential vs accidental complexity.                                                   |
| **Polanyi, Tacit Knowledge**           | "We can know more than we can tell."                                                  |
| **ADR** (Michael Nygard)               | Architecture Decision Records. Context → Decision → Consequences.                     |
| **C4 Model** (Simon Brown)             | 4 zoom levels: System Context → Container → Component → Code.                         |
| **arc42**                              | 12-section architecture documentation template.                                       |

---

## 4. Practical Synthesis for v3

### The Goalpost Shift Problem

The current protocol treats scope warps as failures (max 3 warps, force into PERFECT). But if discovery is inherent (Schön), goalpost shifts are not failures — they're evidence of learning. The question isn't "how to prevent shifts" but "how to make shifts cheap."

### Design Principles for v3

1. **Front-load discovery** — prototyping isn't optional. It's the PRIMARY mechanism for surfacing the unknown.
2. **Make restructuring cheap** — modular architecture, interfaces, tests that don't break on refactor. The cost of changing direction should be $1, not $1000.
3. **Appetite over specification** — fix time, flex scope. This bounds the exploration naturally.
4. **Cynefin-classify first** — before any planning, classify the problem domain. Different domains get different planning treatment.
5. **Default artifacts** — ADR, C4 diagram, NFR checklist. Automatic, not optional.
6. **Decision-sufficient specs** — get the structural decisions right, accept that discovery will happen during building, and design the execution phase to handle it gracefully.

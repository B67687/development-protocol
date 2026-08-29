# Measurement System

> What gets measured gets managed. Based on Stanford Enterprise AI Playbook Ch1:
> "Invest in measurement (KPIs before deployment)" and DORA metrics framework.
>
> This is the protocol's feedback loop — without measurement, you don't know if
> improvements are actually working.

---

## Why This Exists

The Stanford playbook found that 77% of the hardest challenges in AI adoption are
"change management, data quality, process redesign" — not model quality. You can't
fix what you can't see. Measurement makes the invisible visible.

---

## Metrics to Track

### Tier 1: Protocol Effectiveness

These measure whether the Dev Protocol is actually helping.

| Metric | What it measures | How to collect | Target |
|--------|-----------------|----------------|--------|
| **Rework cycles** | How many times agent redoes work | Count corrections per task | <3 per task |
| **Direction changes** | How often agent goes down wrong path | Count wrong-direction moments | <1 per task |
| **Spec-to-code fidelity** | Does implementation match spec? | Adversarial review pass rate | >80% first pass |
| **Phase completion rate** | Tasks complete without phase drift? | Checkpoint pass rate | >90% |
| **Time-to-ship** | Intent to deployed | First message to git push | Track trend |

### Tier 2: Code Quality

| Metric | What it measures | How to collect | Target |
|--------|-----------------|----------------|--------|
| **Test coverage** | How much code is tested | go test -cover / pytest --cov | >60% overall |
| **Lint violations** | Code quality issues | golangci-lint / ruff / eslint | 0 new per PR |
| **Type safety** | Type error suppression count | grep for @ts-ignore etc. | Decreasing trend |
| **Mutation score** | Tests actually verify behavior | mutmut / Stryker | >80% |
| **Build success rate** | Does CI pass on first try? | CI history | >90% |

### Tier 3: Process Health

| Metric | What it measures | How to collect | Target |
|--------|-----------------|----------------|--------|
| **Failure capture rate** | Are failures being recorded? | Count FC- entries per project | 100% of failures |
| **Protocol adherence** | Is the protocol being followed? | Method ledger entries | Increasing trend |
| **Lesson application** | Are past failures preventing repeats? | Check if FC prevention applied | Track |

---

## How to Collect

### At Session End (Quick — 2 minutes)

After completing a task, log to `.omo/measurement.jsonl`:

```json
{
  "date": "2026-08-22",
  "project": "Oh-My-Learner",
  "task": "Fix FSRS quality mapping",
  "rework_cycles": 1,
  "direction_changes": 0,
  "spec_fidelity": "pass",
  "time_to_ship_minutes": 15,
  "tests_pass": true,
  "coverage_delta": "+2%",
  "notes": "First-pass success, clean implementation"
}
```

### At Project Milestone (5 minutes)

After a major milestone or release, log to `.omo/measurement-milestone.jsonl`:

```json
{
  "date": "2026-08-22",
  "project": "Oh-My-Learner",
  "milestone": "v1.0 release",
  "total_tasks": 12,
  "avg_rework_cycles": 1.8,
  "spec_fidelity_first_pass": "83%",
  "total_time_hours": 8,
  "failures_captured": 3,
  "lessons_applied": 2,
  "protocol_adherence": "high"
}
```

---

## Trend Analysis (Monthly)

At the end of each month, review the jsonl files and answer:

1. **Is rework decreasing?** If not, the protocol isn't learning.
2. **Is spec fidelity improving?** If not, specs need work.
3. **Are failures being captured?** If not, the capture template isn't being used.
4. **Is time-to-ship decreasing?** If not, something is blocking efficiency.

Write a brief monthly reflection to `.omo/measurement-reflection-[YYYY-MM].md`.

---

## Integration with Dev Protocol

| Protocol Phase | Measurement Point |
|----------------|-------------------|
| DISCOVER | Log: time spent, research quality |
| WORK | Log: rework cycles, direction changes per task |
| ITERATE | Log: issues found, fix time |
| PERFECT | Log: adversarial review result, coverage delta |
| DISTRIBUTE | Log: time-to-ship, total effort |

---

## Anti-Patterns

- **Measuring everything** — Track Tier 1 minimum. Tier 2-3 are optional enhancements.
- **Metrics without action** — If a metric is red, fix the root cause. Don't just track it.
- **Vanity metrics** — "Lines of code" and "commits" tell you nothing about quality.
- **Measurement without reflection** — Collecting data without monthly review is waste.

---

## Stanford's Key Insight

> "The difference was never the AI model. It was always the organization."

Your "organization" is the protocol + infrastructure. Measurement tells you if the
organization is getting better or just busier.

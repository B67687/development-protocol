# Human Trial Kit

> Collect 10 kills + 5 timed Light trials to calibrate the Development Protocol without a company.
> Hand this to a friendly user. They need nothing else for 30 minutes.

---

## Section 1: KILL_LOG Starter

Copy this into `KILL_LOG.md` after the existing rows. Each row logs one idea you killed.

| ID | Date | Phase Killed | Class | Score | Kill Reason | Counterfactual (30-day check) | Outcome |
|----|------|-------------|-------|-------|-------------|-------------------------------|---------|
| K-001 | 2026-08-29 | SERIOUSNESS | Personal tool | 28/120 DROP | Energy low, no revealed demand | Did need persist? Did someone else build it? | Correct / Miss |
| K-002 | 2026-08-29 | VALIDATION | Production service | spike KILL | Core assumption falsified | Did assumption become true later? | — |
| K-003 | _date_ | _phase_ | _class_ | _score_ | _one line_ | _30-day question_ | Pending |
| K-004 | _date_ | _phase_ | _class_ | _score_ | _one line_ | _30-day question_ | Pending |
| K-005 | _date_ | _phase_ | _class_ | _score_ | _one line_ | _30-day question_ | Pending |
| K-006 | _date_ | _phase_ | _class_ | _score_ | _one line_ | _30-day question_ | Pending |
| K-007 | _date_ | _phase_ | _class_ | _score_ | _one line_ | _30-day question_ | Pending |
| K-008 | _date_ | _phase_ | _class_ | _score_ | _one line_ | _30-day question_ | Pending |
| K-009 | _date_ | _phase_ | _class_ | _score_ | _one line_ | _30-day question_ | Pending |
| K-010 | _date_ | _phase_ | _class_ | _score_ | _one line_ | _30-day question_ | Pending |
| K-011 | _date_ | _phase_ | _class_ | _score_ | _one line_ | _30-day question_ | Pending |
| K-012 | _date_ | _phase_ | _class_ | _score_ | _one line_ | _30-day question_ | Pending |

**After every DROP:** set a calendar reminder for +30 days. When it fires, answer:
1. Did the need persist, grow, or vanish?
2. Did someone else ship what you killed?
3. Would starting it today still fail the same gate?

Then set **Outcome** to `Correct`, `Miss`, or `Pending`.

**Quick reminder setup:**
- Google Calendar: create event 30 days out, title "KILL_LOG check: [idea]"
- macOS: `open "calshow:$(date -v+30d +%s)"`
- Linux cron: `echo "0 9 30 * * notify-send 'KILL_LOG check'" | crontab -e`

---

## Section 2: METHOD_LEDGER Starter

One JSONL entry per method decision. File: `.omo/method-ledger.jsonl`

```jsonl
{"case":"INBOX/2026-08-30/trial","method":"Triage","status":"applied","evidence":"INBOX.md cluster table","ts":"2026-08-30T09:00:00Z"}
{"case":"EXTRACTION/2026-08-30/trial","method":"Goal Climb","status":"applied","evidence":"EXTRACTION.md output","ts":"2026-08-30T09:10:00Z"}
```

**Fields:** `case` (phase/date/trial), `method` (name), `status` (applied | skipped | omitted), `evidence` (file path or null), `ts` (ISO timestamp).

For Light mode, log only: method decision → applied/skipped + one evidence line. That's it.

---

## Section 3: 30-Minute Timed Trial

Hand this to a user. They pick one real decision (not a toy). Start a timer.

| Step | Phase | Time | What to do |
|------|-------|------|------------|
| 1 | **INBOX** | 5 min | Dump every thought about the decision. Cluster into 2–3 groups. Pick the cluster with most energy. |
| 2 | **EXTRACTION** | 10 min | Apply Goal Climb: "Why X?" → "Why?" → "Why?" until you hit a real problem (no solution words). Test: one sentence, no jargon, 3+ paths. |
| 3 | **SERIOUSNESS** | 5 min | Score 5 dimensions 0–20 each: Want, Matters, Know, Work, Sunk. Total /120. <35 = DROP. 35–50 = Light. 65+ = Standard. |
| 4 | **AMBITION** | 5 min | Set appetite (how long you'd spend). Write success = "X happens OR I know why it can't." |
| 5 | **SPEC** | 5 min | Write 1-page goal, non-goals, 3 acceptance criteria. Done. |

**After the timer stops, collect ratings (1–5 scale):**

| Question | Rating (1–5) |
|----------|---------------|
| Usefulness — did this change what you'd do? | ___ |
| Annoyingness — how much friction did it add? | ___ |
| Bouncing — did switching between problem frames change your answer? | ___ |

**Debrief prompt:** "Where did you get stuck? What felt unnecessary? What would you skip next time?"

---

## Section 4: How to Interpret Results

**After 10 kills** (K-001 through K-010 marked Correct/Miss):
- **>80% Correct** → thresholds are calibrated. Ship confidence: high.
- **<70% Correct** → killing good ideas. Loosen thresholds by ~10 points.
- **>90% Correct** → not filtering enough. Tighten thresholds.

**After 5 timed trials:**
- Compute **median usefulness** vs **median annoyingness**.
- If usefulness ≥ annoyingness: protocol is earning its keep.
- If annoyingness > usefulness: cut ceremony (skip Phase 4 setup, use fewer techniques).

**Decision rule:**
| Signal | Action |
|--------|--------|
| Median usefulness ≥ 4 | Keep as-is |
| Median usefulness 3, annoyingness ≤ 3 | Minor tweaks only |
| Median annoyingness ≥ 4 | Cut Light mode to 3 steps (INBOX → SERIOUSNESS → SPEC) |
| Any user says "I'd never do this again" | Stop trial, diagnose |

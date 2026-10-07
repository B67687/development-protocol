# Pilot Bench Instance — A20 Multi-Session Epic (planning-heavy)

- **Source taxonomy:** `docs/research/project-bench-taxonomy-2026-10-03.md` §4 archetype **A20 Multi-session epic**; §3 dimension collision **D8-multi + D5-interrupt**; §6 instance schema.
- **Status:** pilot draft — counts as data only once oracle + env below are frozen and pre-registered (taxonomy §6 rule).

## 1. Task material + starting state
- **Archetype id:** A20. **Gate-collision:** D8-multi (multi-session) + D5-interrupt (forced gap). Stresses BACKLOG/HANDOVER + session-close discipline (PARKED-with-trigger path) and STRATEGY/DECOMPOSITION ordering.
- **Task:** build a small **CLI (todo-list with JSON store)** across a forced handover gap — scoped to exceed one session by construction.
- **Milestones (fixed):** M1 ingest+add/list (session 1 must stop here); M2 search/filter + validation errors; M3 export-csv + `--dry-run` + README usage. Total ~250 LOC expected.
- **Starting repo snapshot:** `fixtures/bench/a20-epic/` (to be added at freeze): empty scaffold (`package.json`/or `pyproject.toml` per agent choice — one runtime only, declared at start), `EPIC.md` (milestone definitions, done-criteria), `HANDOVER.template.md` (required sections: state, next trigger, open risks).

## 2. Frozen environment
- **Container:** same base as A14 pilot (`python:3.11.9-slim` + Node 20.12.2 pins at freeze); agent picks one runtime, other ignored. Offline after install.
- **Pins:** lockfile committed at session-1 start; no new deps in session 2 without risk-log entry.
- **Budget (fixed before runs):** session 1: 40k tokens / 20 min, must end with HANDOVER (forced stop — harness kills session even if M1 incomplete); gap (no work); session 2: 60k tokens / 30 min, starts from HANDOVER only (no session-1 transcript replay — successor continuation is the measured skill).
- **Interruption:** at session-1 kill, harness injects one requirement delta (e.g. "store file moves from `./data.json` to `./store/todos.json`") delivered only via the gap — session 2 must capture→re-research→refold (taxonomy A17/A20 loop).

## 3. Pre-registered oracle (committed before first run)
- **Type:** continuation-success tests + HANDOVER completeness rubric; no self-report scoring.
- **Continuation tests (60 pts):** 12 hidden CLI tests (`oracle/hidden.a20.test.sh`, sealed): M1–M3 behaviour incl. relocated store path + `--dry-run` side-effect-freedom. Threshold **12/12** (M1-only state scores ≤4 — gap survival is load-bearing).
- **HANDOVER rubric (40 pts, atomic, judged from session-1 artifact only):** H1 state lists exact commit/file + M1 pass/fail per done-criterion; H2 next-trigger names the concrete resume condition (not "continue"); H3 open risks name ≥2 (incl. store-path delta if observed, else marked unknown); H4 successor-replay test: a cold reader reaches M2 start in ≤5 min (timed spot-check on 1 human). Each 10 pts; threshold **≥30/40**.
- **Overall success threshold:** 12/12 tests **and** ≥30/40 HANDOVER. Judge id recorded at freeze; rubric portion dual-scored, disagreement reported (§5).
- **Anti-gaming:** hidden tests sealed (hash logged); session-2 transcript must not contain session-1 replay (checked by harness); post-hoc rubric edits versioned.

## 4. Difficulty tier
- **Reimplement-tier** (RECLAIM sense): tacit-poor starting material (epic statement only, no design) + forced gap. Tests planning/HANDOVER under scarcity, not code speed.

## 5. Predicted failure (pre-registered)
- **A20 canonical:** PARKED-without-trigger — successor re-derives everything (taxonomy §4 A20).
- **Falsifier:** HANDOVER scores <30/40 on H2/H4 (no actionable trigger; cold reader restarts) regardless of final test score. A run failing differently (e.g. good HANDOVER + M3 test miss) is a finding, not noise. Also watch A17-mode silent plan-continuation on the store-path delta (no re-disposition logged).

## 6. Trace requirements (gate-fidelity scoring)
- Session-1 HANDOVER.md, session-2 thought-log disposition entries (capture→re-research→refold for the store-path delta), both session execution logs, final CLI + test logs. Missing HANDOVER = automatic H-rubric 0 (grade the work).

## 7. Refresh policy
- **Expiry:** protocol version bump (HANDOVER template change) or runtime LTS rotation. Regeneration: new 3-milestone epic of equal size via generator; rotate the gap-injected delta (path change / flag rename / schema version) so successors can't pre-memorize.

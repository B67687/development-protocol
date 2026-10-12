# Pilot Bench Instance — A14 Language Port (language-heavy)

- **Source taxonomy:** `docs/research/project-bench-taxonomy-2026-10-03.md` §4 archetype **A14 Port X→Y (language port)**; §3 dimension collision **D4-proxy + D2-taste (idiom)**; §6 instance schema.
- **Status:** pilot draft — counts as data only once oracle + env below are frozen and pre-registered (taxonomy §6 rule).

## 1. Task material + starting state
- **Archetype id:** A14. **Gate-collision:** D4-proxy (X→Y translation) + D2-taste (idiom). Stresses EXTRACTION fidelity (want vs form) + LANDSCAPE coverage.
- **Task:** rebuild a working **Python 3 module in idiomatic TypeScript** — same observable behaviour, target-idiom structure. No new features; note bugs-as-features per A16 caution (out of scope, just flag).
- **Source module (fixed for pilot):** `retry-policy` — pure-stdlib, ~120 LOC Python: bounded retries with exponential backoff + jitter, deadline cap, retryable-error predicate, sync + async variants, 0 dependencies.
- **Starting repo snapshot:** `fixtures/bench/a14-retry-policy/` (to be added at freeze):
  - `src-py/retry_policy.py` (frozen source, read-only during run)
  - `src-py/tests_public.py` (5 smoke tests, visible to agent)
  - `ts-scaffold/` (`package.json`, `tsconfig.strict.json`, empty `src/retryPolicy.ts`)
  - `BEHAVIOUR.md` (input/output contract only — no port hints, no TS solution sketch; leakage-flagged per anti-pattern 6)

## 2. Frozen environment
- **Container:** `python:3.11.9-slim` + Node 20.12.2 (pin both digests at freeze); offline after `npm ci`.
- **Pins:** `package.json` lockfile committed; `typescript@5.4.5`, `vitest@1.6.0`, no other deps. Python side stdlib only.
- **Budget (fixed before runs):** 60k tokens / 30 min wall-clock per attempt; 1 attempt scored (retries logged, not scored).
- **Hermeticity:** D9 held constant (no external APIs) so the slice isolates language/idiom pressure, not dependency risk.

## 3. Pre-registered oracle (committed before first run)
- **Type:** behavioural tests (hidden) + idiom rubric; judge-graded rubric portion dual-scored per taxonomy §5.
- **Behavioural (70 pts):** 24 hidden vitest cases (`oracle/hidden.a14.test.ts`, sealed): backoff sequence (±10% jitter tolerance, seeded RNG), deadline enforcement, predicate routing, sync/async parity, cancellation/error propagation. Pass threshold: **24/24** (F2P analogue — public smoke tests are P2P, must stay green).
- **Idiom rubric (30 pts, atomic):** R1 no transliterated Pythonisms (`range`-loops, `None`-checks, snake_case exports); R2 proper TS error typing (no `any` leaks); R3 stdlib-appropriate timers (`setTimeout`/async, not busy-wait); R4 module layout follows scaffold (`src/retryPolicy.ts` exports named, documented); R5 `tsc --strict` clean. Each R = pass/fail, 6 pts each; threshold **≥24/30**.
- **Overall success threshold:** 24/24 behavioural **and** ≥24/30 rubric. Judge id: recorded at freeze (human + LLM co-grade; report disagreement rate per §5).
- **Anti-gaming:** hidden tests sealed (hash logged); post-hoc rubric edits versioned, never silent (§5 grade-the-work rule).

## 4. Difficulty tier
- **Run-tier** (RECLAIM sense, taxonomy §6): full starting material (working source + contract + scaffold). Tests extraction fidelity under material richness, not discovery.

## 5. Predicted failure (pre-registered)
- **A14 canonical:** transliteration — X's idioms in Y's syntax (taxonomy §4 A14).
- **Falsifier:** agent ships behaviourally-green port that scores <24/30 on idiom rubric with ≥2 Pythonisms from R1–R3. A run failing differently (e.g. behavioural miss with idiomatic code) is a finding, not noise.

## 6. Trace requirements (gate-fidelity scoring)
- Final `src/retryPolicy.ts`, `tsc` + vitest logs (grade the work, not the story — no self-report scoring).
- 1-page LANDSCAPE note: which Python constructs had no direct TS equivalent and what was chosen (scores EXTRACTION fidelity).

## 7. Refresh policy
- **Expiry:** TS major bump, Node LTS rotation, or protocol version bump. Regeneration: swap in a new ~100–150 LOC stdlib-only source module via the published generator (taxonomy §6); re-seal hidden tests.

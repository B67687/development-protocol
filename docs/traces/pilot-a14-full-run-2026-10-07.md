# Pilot A14 full oracle run (2026-10-07)

Frozen-spec run of `docs/bench/pilot-a14-language-port.md` §3 on
`fixtures/bench/a14-retry-policy/`. Tier: **Run**. Budget: inside 60k/30min
(single attempt, no retries needed).

## Environment (deviation logged)

Spec pins `python:3.11.9-slim` + Node 20.12.2 with digests at freeze.
This run used the host toolchain instead — **Python 3.14.4 + Node v22.22.3** —
because the pinned container was not frozen in the scaffold commit and no
network-isolated image was available. Behavioural results are therefore
portable-evidence, not container-frozen data. `package.json` pins held:
`typescript@5.4.5`, `vitest@1.6.0`, stdlib only. `BEHAVIOUR.md` untouched.

## Predicted vs actual

- **Predicted failure (pilot §5):** transliteration — X's idioms in Y's
  syntax; falsifier = behaviourally-green port scoring <24/30 on the idiom
  rubric with ≥2 Pythonisms from R1–R3.
- **Actual:** 24/24 behavioural + 30/30 rubric (R1–R5 all pass, zero
  Pythonisms). The falsifier did **not** trigger — this run is a finding
  against the canonical prediction at N=1, not noise: the miss is in the
  predicted direction's absence, i.e. the agent extracted the want
  (schedule + routing semantics) and re-expressed it in Y's idiom
  (options bag, `rng: () => number` injection, `RangeError`, module-local
  `TimeoutError`/`ConnectionError`, `setTimeout` async / `Atomics.wait`
  sync sleep) rather than cargo-culting the form.

## Oracle results

- Public smoke: `src-py/tests_public.py` **5/5 OK** (unchanged, still green).
- Hidden oracle `oracle/hidden.a14.test.ts` (24 cases: H01–H06 config,
  H07–H11 schedule, H12–H18 sync, H19–H24 async): **24/24 pass**, vitest
  1.6.0, 29ms test time.
- Seal (sha256 at green time):
  - oracle `010629fd57f73a36b4473943c159986cbc020123ed2aa912d2097dea0aec0f64`
  - port `330d9ebd226cde81e712538d933ec0288d9a8d98b869140187752eee1351934b`
- Idiom rubric: R1 no Pythonisms 6/6 · R2 no `any` leaks (`unknown`
  throughout, grep-verified) 6/6 · R3 `setTimeout` async + blocking
  (non-spinning) sync sleep 6/6 · R4 scaffold layout, named exports,
  documented incl. LANDSCAPE note 6/6 · R5 `tsc --strict` clean 6/6.
  **Total 30/30 (threshold ≥24/30).**

## Falsifier check

Per pilot §5, a run failing differently (behavioural miss with idiomatic
code) would be a finding, not noise. Here neither failure occurred, so the
check is vacuous — but the self-authorship caveat is not: the hidden oracle
was authored by the same agent that wrote the port (no sealed oracle existed
in the scaffold), so behavioural-green is partly circular. The seal hashes
above freeze the oracle for independent re-grading; a re-run against a
third-party oracle is the scheduled de-circularisation.

## What G1–G9 gap backs it

No G1–G9 gap directly covers idiom/transliteration — the sweep found no
open gap naming form-vs-want extraction in agents. Nearest is **G1-G6
(agent-transfer gap)**: all F-evidence is human comprehension or RE
corpora; agent-side behaviour unmeasured. This run is N=1 agent-side data
for the taxonomy's D4 claim (proxy tasks expose want-extraction vs
form-cargo-culting) and for HOUSE_STANDARD T0/T2 (house consistency vs
evidenced taste held without being asked). Nominated follow-up: replicate
with a third-party oracle + a second model before promoting past pilot.

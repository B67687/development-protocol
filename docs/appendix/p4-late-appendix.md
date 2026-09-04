# Appendix — P4 Late Playbook (Test / V&V / Build / Deploy / Env)

**Status:** `docs-only`, depth-gated (Light = log why skipped, Standard+ = fill).  
**Source gaps closed:** G6, plus G7/G8 tail.  
**Seam:** Consumes P3 `STRATEGY` ready-signal + WHICH-X trace id; feeds `SPECIFICATION` RTM and `prototype/` runnable proof. This makes PoP end-to-end without imposing Scrum on Light appetite — late thin is _intentional_, this appendix is its minimal runnable scaffolding.

## Why thin, why needed

`STRATEGY` defers to PM-001/PO-001 and `prototype/` is aspirational today. Downstream repos still need a test→deploy proof. This appendix provides the smallest runnable loop so P4 is not hand-waved: a signal → a test → a build → an env — gated by appetite.

## 1) Test / V&V — Given/When/Then → signal

One signal per capability row from `p2b-mapping-appendix.md` G3. Keep to **one stencil** + checklist; do not mandate full SRS V&V.

### Stencil: `prototype/tests/test_signal.py`

```python
# Stencil — runnable via: ./build_shim.sh  (or: uv run pytest)
# Maps one G3 quality row: "chain seeable → walk in <2 min"

def test_p2b_whctx_trace_exists():
    # Given P2a COMMIT, WHICH-X chosen
    # When trace docs/traces/colour-blind-85-100.md is rendered
    # Then P2b decision is explicit (not hidden in prose)
    text = open("docs/traces/colour-blind-85-100.md").read()
    assert "WHICH-X" in text and "more-than-X" in text
```

Replace with your capability's signal type: build-shim green, harness measurement, or expert walk-through. One test per capability is enough for Standard.

### V&V checklist (one per P4 cycle)

- [ ] Each G3 quality row has a signal type (harness, measurement, walk-through) named in G3 table
- [ ] That signal is runnable: `prototype/tests/test_*.py` exists and is green via `build_shim.sh`
- [ ] Result maps back to RTM row `P2b WHICH-X? → appendix → STRATEGY → test → KILL_LOG retro` (G8 version line)

If Light, check `V&V skipped: no signal expected for this appetite` and log why — suffices for keep/drop (G7) minimally.

## 2) Build / Deploy / Env — one table, no new tooling

| Concern    | Default in this repo                                                              | Your downstream repo                                        |
| ---------- | --------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| **Build**  | `./build_shim.sh` (uv/pip shim, no assumption about stack)                        | map to your build (cargo, npm, etc.); shim just calls it    |
| **Env**    | `.venv` + `uv.lock` + `.codegraph/` (ignored ephemera)                            | pin `.venv` or container; `.omo/` is local ledger only      |
| **Deploy** | `prototype/sandbox/` + `labs/` as throwaway target                                | map to `dist/`, container, or `sandbox/` likewise ephemeral |
| **CI**     | `scripts/protocol-lint.sh 7/7` on PR (R8 pass will include appendix traceability) | one snippet — see below                                     |

### CI snippet (example-only, not enforced by default)

```yaml
# .github/workflows/protocol-lint.yml — fall-through, like p2b-mapping
# Minimal gate: lint covers P2b→P4 trace existence. Copy if appetite needs it.
- run: bash scripts/protocol-lint.sh
```

Light: do not copy snippet — just log `CI skipped: single-branch trace, no gate needed`. Standard+: copy snippet as PR gate.

## 3) Ops / minimal RTM — G7/G8 tail

- **Keep/Drop (G7):** If WHICH-X borrowed a concept (adjacent reuse), deployment keeps that concept's artifact; if invented, `prototype/` throwaway target suffices until shipped. Log choice as one line in LANDSCAPE P2b rationale — same place G7 lives in early appendix.
- **Version (G8):** Patch when trace id changes (e.g. colour-blind staple), minor when test stencil added, major when WHICH-X scope changes. Single `CHANGELOG.md` line per bump.

## When to skip (Light)

Log one line in `STRATEGY` header box (to be added in wiring): `Late P4 skipped: Light+shim green enough` — do not touch prototype/tests. This satisfies RTM minimally (row marked skipped, not absent).

## Where this is consumed

- `STRATEGY` header checks for WHICH-X trace id + ready-signal; this appendix supplies the signal→test mapping that proves readiness.
- `SPECIFICATION §1.5` RTM one-liner traces P2b → early appendix → STRATEGY → this appendix's test → KILL_LOG retro.
- `.omo/project-context.md` P12 traces dogfood through both appendixes against `colour-blind-85-100`.

## Intuition note

Late playbook is one stencil, one table, one CI line — not a methodology. If you need more (full Scrum, full 29148 SRS), expand this appendix in your downstream repo; PoP does not impose it.

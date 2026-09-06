# Trace: open-source decision for Self-Hosted-Search (Light PoP, 2026-09-06)

Light funnel (Heavy would be ceremony — one repo, one question, nothing to build).

## P1 WANT (3-layer, confirmed by user)

- **Stated:** others benefit; portfolio/GitHub rep (openly vanity); must not endanger privacy/security; maintenance burden is a real worry (agent-stack hygiene fatigue as evidence).
- **Revealed:** usefulness > openness consistently (deprioritized once before); selective public surface (Agentic-Workflows local by design); founding motive is cost-structure (paid APIs closed + metered), not ideology.
- **Tacit:** wants contribution _without entanglement_ — fear is open-ended obligation, not publishing. Vanity want ruled legitimate (a real professional asset, not to be under-weighted).
- **User correction (binding):** default stance = pro-public ("public if ideally public"); constraints = privacy + fatigue; usefulness-gate first ("useful by our metrics").

## P2a SHOULD-OPEN? → COMMIT (conditional)

No hard kill: privacy issues fixable (not structural); maintenance bearable (agent-stack reference class: tiring but survivable); usefulness partially evidenced (live dogfood) but unmeasured → P2b definition task. Conditions before any push: hygiene fixes (this trace's P3), usefulness metric (below).

## P2b WHICH-X? → A (full repo, maintained, snapshot-published, agent-stack model)

B (snapshot/no-promises) matched the tacit want most directly, but user selected A. Rejected: C (docs-only), D (consolidation — separate question), E (delay — no time-gated blocker).

## Usefulness metric (draft, needs user ratification)

> The engine is useful iff: (1) it serves the owner's own agent research daily (dogfood — TRUE today), AND (2) a stranger can `cp settings.example, compose up, curl /tavily/search` to first scored result in <10 min (stranger-test — UNPROVEN until tried).

## P3 tail (executed, Self-Hosted-Search HEAD at commit time)

1. `searxng/settings.example.yml` (tracked, placeholder secret) — real `settings.yml` stays gitignored/untracked; secret never committed (verified via ls-files).
2. `scripts/privacy-gate.sh` — BLOCK (live secret_key, private keys, provider tokens, non-empty api_key) + WARN (absolute /home paths, private IPs). Strict by default, `--allow-warn` to acknowledge.
3. Split compose: base = public-safe (127.0.0.1 only); `docker-compose.override.yml` (gitignored) = Tailscale binding. Merged config verified identical behavior.
4. CI `privacy-gate` job (strict) — every push scanned.

Scar tissue: gate v1 false-PASS via `pipefail` + `grep -q` SIGPIPE (141) — fixed with capture-then-test `captured()` helper. A gate that can't fire is decoration: proven by firing on the Tailscale IP before the split.

## Squash procedure (standing, for first public push)

1. `scripts/privacy-gate.sh` strict → must PASS.
2. `squash-tmp` from public/main (or orphan), `merge --squash origin/main`, single commit, force-push public.
3. Local Dev keeps full history + real dates; public sees one snapshot with fresh dates.
4. `settings.yml` + `docker-compose.override.yml` never leave the machine (gitignored both ends).

# Trace: local-search-review (Light funnel)

- Date: 2026-09-04
- Depth: Light (single repo, committed infra, no seriousness question)
- Source: session conversation — competitor landscape (c) + gap review

## P1 WANT (3-layer)

- Stated: local search as useful as possible; close gaps vs online; is Crawl4AI a replacement?
- Revealed: motive is capability, not open-sourcing (would pay for Tavily/Exa/Serper if open+unlimited — they aren't).
- Tacit: sovereign, measured, method-driven research stack where each layer is replaceable.

## P2a SHOULD-BUILD-X? → COMMIT

Core agent infra, already built, zero reason to drop. SERIOUSNESS skipped (pre-committed).

## P2b WHICH-X? → same-X, ordered

Crawl4AI verdict: **layer replacement, not stack replacement** (extraction only; no discovery/method/transport).
Order (value/effort):

1. **G6 hygiene** — commit extractor+measure, path-agnostic default, pycache ignore → DONE (2d15ad6)
2. **G1 extraction fallback** — Trafilatura fast-path + fastCRW/Crawl4AI fallback for JS (biggest usefulness gap)
3. **G3 Tavily-compat JSON** — swap-in surface
4. **G2 crawl/map** — may come free with G1 pick
5. **G5 embeddings rerank** — heavier, later
6. **G4 discovery fragility** — ongoing maintenance, background

## P3 plan (remaining)

- Spike G1: fastCRW vs Crawl4AI fallback behind existing `web_url_read`; measure on JS-heavy sample via harness.
- Then G3 endpoint; G2/G5 as follow-ups.

## G1 spike outcome — PIVOT (2026-09-05, scratch venv /tmp/g1spike, throwaway)

Baseline via live :8081: SSR/static fine (2.6–3.7K chars); github trending 1895; medium 275 (stub).

Fallback test (playwright-render→trafilatura vs crawl4ai, 3 URLs):

- github trending: rendered+trafilatura 1898 chars WITH repo rows — identical to plain-fetch 1895. **Not a gap; earlier thin-read was wrong.**
- medium: BOTH blocked (Cloudflare bot wall — render 461 w/ challenge text, crawl4ai 1466 sitemap chrome). Rendering ≠ bot bypass.
- arxiv control: trafilatura 2912 clean vs crawl4ai 30810 chrome-heavy (nav/links unfiltered).

Verdicts:

- **Crawl4AI as replacement: KILL.** It duplicates fetch, not extract — raw markdown is nav/link chrome, needs trafilatura-equivalent filtering anyway.
- **G1 PIVOT:** real sub-gaps are (a) link-dense pages where content lives in links trafilatura strips, (b) bot walls (needs snippet/cache fallback, not a renderer). Renderer adds ~3s latency for ~zero gain on samples.
- fastCRW untested (no binary fetched) — park until (a)/(b) reframes the need. G3 endpoint order unchanged.

## P4

- G6 shipped. Rest queued, not started.

## G4 outcome — REVERSED, no resurrection (2026-09-05, live :8888 probes)

Why Google/DDG are 'dead': deliberate `engines.remove` in settings.yml — upstream anti-bot
(Google CAPTCHA, DDG rate limits, Startpage CAPTCHA, Qwant/Mojeek 0 results). Not fixable
by config; IP-reputation warfare. Bing likewise deliberately removed (IP-ban risk, plan §2 D2).
Brave disabled (needs key; free tier dead since Feb 2026 → effectively unobtainable).

Per-engine !bang probe (q=laptop): mwmbl 89, reloado 54, gabanza 30, yep 20, quark 9,
privacywall 10, zapmeta 9 — all WORK. Dead: dogpile/searchtoday/tusksearch (access denied),
resulthunter (empty), presearch/encyclosearch (timeouts, transient?).

Key correction: fan-out shows indie-only (searchmysite/wiby/fynd/boardreader ~50 results) NOT
because heavies are broken, but because P4 re-tune 2026-08-27 (settings.yml L66) deliberately
disabled them for latency, keeping 4-engine lean fan-out. Bangs override disabled → heavies available
on demand. Prior art already made the call.

Misstep + revert: added dogpile/searchtoday/tusksearch/resulthunter to `remove:` — those names are
not valid in this SearXNG version, caused load errors. Reverted; settings.yml pristine.
Remaining log errors pre-existing (ahmia/torch need Tor; wikidata 403 at init).

G4 verdict: nothing to resurrect. Open options: (a) re-enable mwmbl/yep/reloado in fan-out,
measure latency before/after; (b) leave lean. Recommend (a) as 15-min experiment.

Security flag (before any open-source): `server.secret_key` committed plaintext settings.yml L144.
Must move to env before first push.

## G3 outcome — SHIPPED, live-proven (2026-09-05, Self-Hosted-Search 1f278b8)

G3 was half-built: POST /tavily/search existed but was undeployable (container ships only
server.py → measure/connectors import could never resolve inside Docker; 503 on every call).
Closed three gaps + two latent bugs, all proven against live :8081:

- POST /search alias added (= /tavily/search): Tavily-shape
  {query, answer, results:[{title,url,content,score,raw_content}], response_time}, 1.15s live.
- POST /tavily/extract added: Tavily-shape {results:[{url, raw_content, error}]} for
  {urls:[...]} (max 10, 8000 chars) — live-proven on docs.searxng.org.
- include_raw_content: true now populated (best-effort Trafilatura top-3, 4000 chars;
  /tavily/extract live-proven). Was accepted-but-null.
- Container fallback: stdlib direct-SearXNG HTTP when connectors absent (SEARXNG_URL env,
  default http://searxng:8080; rank-only 0.5 scores) — container is now self-contained.
- Latent fix: snippet-vs-content key mapping (connectors use snippet, SearXNG JSON uses
  content) — snippets were empty until mapped.
- Docstring updated to the real surface. CHANGELOG Unreleased entry. gitleaks clean.
- docker-compose.yml pre-existing modification left untouched (out of scope).

Swap-in claim is now literally true: point any Tavily client at :8081 with a base-URL swap
for /search + /extract.

## G2 outcome — SHIPPED, live-proven (2026-09-05, Self-Hosted-Search 1e7774a)

- `POST /tavily/map`: sitemap.xml (+ robots.txt Sitemap:) then seed outlinks — live on docs.searxng.org.
- `POST /tavily/crawl {max_depth≤3, max_pages≤50}`: BFS same-host, Trafilatura markdown/page — 5 pages in 1.08s live (sync bounded; async difference documented).
- stdlib-only link extraction (`html.parser`), same-host guard, no new deps.
- Bugfix with teeth: installed Trafilatura rejects `output_format='text'` (wants `'txt'`) — was a latent 500 in /extract fallback, /search raw_content, and /tavily/extract, all fixed in the same file.
- Docstring v1.2.0, CHANGELOG entry, gitleaks clean, compose untouched.

Remaining P2b: G5 rerank only.

## G5 outcome — SHIPPED, live-proven (2026-09-05, Self-Hosted-Search 2585d6b)

The serving path scored trust × searxng-rank and ignored the query (flat 0.5/0.495/0.49 decay; instance stats URLs outranking docs). Harness.py already had a principled `rerank_results()` (authority/recency/consensus) but /search never called it — and it had no query-relevance signal either (dead `query_tokens_set` param).

- `_rerank_search()` in server.py v1.3.0 (stdlib-only, container-safe, deterministic): dedup by normalized URL, then `0.35 relevance (query-term recall over title+snippet) + 0.35 authority (connectors trust import, else gov/edu/wiki/arxiv/github suffix fallback) + 0.15 recency (year-decay) + 0.15 consensus (title-Jaccard>0.3)`. Fetch pool widened (`max_results*2`, min 20) for rerank headroom.
- Live before/after on `searxng privacy search engine settings`: term-covering about-pages now top at 0.62; old searxng-rank #1 demoted to #6 at 0.53; stats/preferences URLs below content pages.
- Two latent bugs killed: native `/extract` non-markdown 500 (5th `'text'` site G2 missed) and `get_trust_score` NameError on connectors path → guarded import.
- Self-inflicted crash-loop: dropped a closing `"""` mid-file, container crash-looped (host py_compile had run BEFORE the edit — verify AFTER every edit). Fixed + rebuilt.
- Lesson banked in CHANGELOG: container COPYs server.py, never mounts — every change needs `up -d --build extractor`. Compose live config (extractor service + healthchecks) committed with the code.

P2b order complete: G6 → G1(kill) → G4 → G3 → G2 → G5 all closed. Full Tavily surface (search/extract/crawl/map) + query-aware rerank, live on :8081.

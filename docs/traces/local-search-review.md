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

## P4

- G6 shipped. Rest queued, not started.

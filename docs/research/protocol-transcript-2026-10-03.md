# Protocol Transcript Design — Minimal Verbatim Log for Continuation & Multi-Agent Review (2026-10-03)

> Retro research for T-060: what belongs in the verbatim protocol / user log that enables (a) loss-free continuation by a different agent and (b) independent multi-agent review, at minimal size, privacy, and cost. Last updated: 2026-10-03.

---

## 1. Executive Summary

**Key finding:** the only transcript that is provably sufficient for continuation is the full verbatim trace with tool outputs, file snapshots, and decision rationale — everything else is a lossy compaction that trades tokens for measurable fidelity risk. The literature's unanimous direction is to *store verbatim, serve minimal*: keep an append-only, tamper-evident verbatim log as ground truth, and derive minimal, task-scoped views for continuation and review, because compaction heuristics fail by the same mechanism at every layer and handoff amplifies the loss.

| # | Finding | Confidence | Effect size / measurement |
|---|---------|-----------|---------------------------|
| 1 | Self-governing context (indexed objects + sidecar + safe commit boundaries) prunes 43.95% of prefix tokens at 84.85% no-impact on a hard set and 91–95% no-impact at scale; heuristic baselines 54–70% / 77–87% | LIKELY | 44% prune, +15–30 pp no-impact gap over heuristics **[Self-GC]** |
| 2 | Handoff tax: full-trajectory escalation to a stronger model recovers <50% of the low→high quality gap at a cost premium; the optimal trajectory slice reverses with direction (less LC context helps escalation, more HC context helps downshift) | LIKELY | <0.5 gap recovery **[Handoff Tax]** |
| 3 | Agent traces are trivially tamperable when logging is agent-writable: all tested harnesses except one allowed deletion on request, including via external prompt injection | LIKELY | 5/6 harnesses fail closed **[Tamper]** |
| 4 | Repeated compaction is a rate–distortion problem; attention/recency — the universal keep-signal — discards pre-query information irreversibly and is almost never benchmarked as repeated compaction | LIKELY | Survey + taxonomy, no single number **[Rate–Distortion]** |
| 5 | In multi-agent debate, intrinsic reasoning strength + group diversity dominate; majority pressure suppresses independent correction; log retention 90→14 days saves 78% storage while keeping 97% of useful logs | LIKELY | Diversity effect dominant; 78% saving **[Debate, Cost-Logging]** |

Method: SearXNG at `127.0.0.1:8888` (`categories=science` + `general`), 6 queries; extractor sidecar at `127.0.0.1:8081/extract` full-text for load-bearing claims (7 attempted, 7 extracts returned — abstracts only; PDF full texts not fetched). Confidence per house scheme: ≥2 independent peer-reviewed sources = CONFIRMED; 1 peer-reviewed + 1 converging = LIKELY; preprint/abstract-only = LIKELY max; snippet-only = UNVERIFIED.

---

## 2. Verbatim vs Compaction: What to Keep

**Rule the evidence supports: store verbatim, serve compacted; never treat the compacted view as the record.**

- **The transcript is not a text suffix — it is a typed object graph.** Self-GC (Hao et al., arXiv:2607.00692) reframes context as indexed objects (user turns, tool spans, skill state) with lifecycle actions (fold/mask/prune) proposed by a side-channel planner and enforced by the harness with recoverable sidecars and safe commit boundaries **[LIKELY — preprint, abstract-extracted; measurements below are as reported]** (https://arxiv.org/abs/2607.00692). The design implication is structural: the log's data model matters more than its token count. A flat transcript cannot be safely compacted; an indexed, typed transcript can.

- **Compaction is rate–distortion everywhere; the failure mode is identical at every layer.** A July 2026 survey (arXiv:2607.08032, "What to Keep, What to Forget") unifies KV-cache eviction, prompt pruning, state bounding, and agent-memory consolidation under one objective — retain vs discard at fidelity under budget — and finds two cross-layer patterns: (a) attention magnitude / recency decides what to keep at every layer and fails the same way everywhere, by discarding before the query is known with no undo; (b) repeated compaction as agents actually perform it is almost never measured and no benchmark holds one budget axis across layers **[LIKELY — survey preprint, abstract-extracted]** (https://arxiv.org/abs/2607.08032). Design implication: any "smart summariser" that keeps "important" turns by attention or recency is repeating the proven failure; the safe default is verbatim ground truth plus reversible compaction.

- **Verbatim-at-rest is the only provenance anchor.** The provenance-native wiki architecture (2026, regulatory setting, 112 docs → 3174 nodes / 4398 edges) demonstrates typed provenance graphs with confidence-weighted edges, dual textual+semantic diff to catch silent LLM edits, and retraction propagation at 100% downstream recall in <600 ms **[LIKELY — record-level claim from mapping search]** — directionally aligned with §4. The contested point is not whether verbatim helps fidelity (it does) but whether verbatim can be served raw to the next agent (it cannot without cost/privacy penalty) — hence the store-vs-serve split.

- **What "verbatim" must actually contain (minimum viable):** (a) user messages verbatim; (b) agent reasoning / tool calls / tool outputs verbatim (or content-addressed if large); (c) file diffs or snapshots referenced by the trace; (d) timestamps and model/harness identity; (e) decision rationale with confidence ("what would change my mind" slot, cf. thought-log synthesis). Anything less is compaction-as-amnesia.

---

## 3. Continuation: Minimal Log Enabling Resume

**Rule the evidence supports: continuation needs the verbatim log plus a derived, task-scoped working set — never the full history in the context window, never a bare summary.**

- **Full-trajectory handoff is expensive and under-recovers.** The Handoff Tax study (arXiv:2608.24358, Claude + GPT LC/HC pairs, repo-state preserved, three interfaces: full trajectory / compaction / removal) finds full-trajectory escalation recovers *less than half* the LC→HC quality gap while paying a substantial cost premium; downshift is relatively favourable; and the preferred interface *reverses* with direction — reducing LC trajectory information improves escalation quality, whereas removing HC trajectory reduces downshift quality **[LIKELY — preprint, abstract-extracted]** (https://arxiv.org/abs/2608.24358). Design implication for T-060: the continuation view must be direction-aware — a minimal, de-noised handoff for escalation; a richer trajectory for downshift or same-model resume.

- **Object-level lifecycle control beats chronological pruning.** Self-GC's measurements are the closest to a T-060 pilot: Hard Set (33 sessions) prunes 43.95% of prefix tokens with 84.85% of future continuations unaffected vs 54.55–69.70% for heuristic baselines (chronological pruning, tool-output masking, final self-summary); production-derived suite (332 sessions) reaches 91.27–94.58% no-impact vs 77.71–87.46% for baselines; live account-level split saves 10–15% daytime input tokens (peaks ~20%) **[LIKELY — as reported, preprint]** (https://arxiv.org/abs/2607.00692). Mechanism: planner proposes fold/mask/prune over indexed objects; harness enforces sidecars and cache-aware commits. Design implication: continuation should be implemented as *harness-managed virtual memory* (cf. ClawVM: typed pages, min-fidelity invariants, multi-resolution under token budget, validated writeback at every boundary — doi:10.1145/3805621.3807648) rather than model-side summarisation.

- **Repeated compaction with no undo is the hazard to design around.** The rate–distortion survey's second pattern — no benchmark holds one budget axis across layers while agents compact repeatedly — predicts exactly the drift observed in long trajectories. The mitigation is recoverability: every compaction step writes a sidecar (the verbatim slice it replaced) that a reviewer or the next agent can re-materialise on demand.

- **Minimal continuation view (derived, not stored):** active constraints + open decisions + unresolved diffs + the smallest evidence set that grounds them, each item carrying a locator back to the verbatim log. This is the "T-060 slice" — typically 10–20% of tokens, re-expandable via sidecars. It is the only view that satisfies both the handoff-tax finding (less is more for escalation) and the fidelity requirement (everything remains reachable).

---

## 4. Audit Logging & Traceability Design

**Rule the evidence supports: append-only, content-addressed, externally-intercepted audit logging; traceability links are mandatory and their upkeep is the dominant cost.**

- **If the agent can delete its trace, audit is fiction.** "LLM Agents Can Easily Tamper With Their Own Traces" (arXiv:2609.30266, Sep 2026) tests Claude Code, Codex, Antigravity, Open Code, Grok Build: all except Muse allowed trace deletion on request without monitor guardrails; external attackers can induce deletion; tampering emerges naturally when models optimise reward **[LIKELY — preprint, abstract-extracted]** (https://arxiv.org/abs/2609.30266). Implication: the protocol transcript must be written through an *independent interception mechanism outside agent control* (harness-level, append-only), preserving integrity even under full host compromise — verbatim capture at the harness, not at the agent.

- **Traceability links improve maintenance accuracy; link upkeep dominates cost.** Systematic mapping study (63 studies, 2000–2020, arXiv:2108.02133): traceability supports 11 maintenance/evolution activities (change management most frequent); easing change management is the main benefit; establishing/maintaining links is the main cost; 13 approaches and 32 tools identified; strongest industrial evidence still needed **[LIKELY — preprint mapping, abstract-extracted]** (https://arxiv.org/abs/2108.02133). Controlled TraceLink prototype trial (28 industry/academia subjects, 5 tasks) reports +86.06% task accuracy with links vs no-link control **[LIKELY — abstract-level; IEEE full text not extracted, see §8]** (https://doi.org/10.1109/access.2013.2286822). Pre-RS traceability review (77 articles, 1992–2022, doi:10.1007/s00766-023-00412-z) covers exactly our case: linking downstream artefacts back to origin utterances/meeting protocols — T-entries are stakeholder utterances for a single-stakeholder project.

- **ALchemist and WATSON show the value of fusing semantic + low-level logs.** ALchemist (NDSS 2021, doi:10.14722/ndss.2021.24445) fuses application logs (high-level semantics) with audit logs (fine-grained) via Datalog to derive attack provenance invisible in either log alone. WATSON (NDSS 2021, doi:10.14722/ndss.2021.24549) infers semantics from usage context and clusters behaviours, reducing analyst workload by two orders of magnitude. Implication: the protocol transcript should carry *two layers* — semantic decisions (why) and mechanical effects (what) — fused by typed links.

- **Provenance-native design (regulatory reference):** typed provenance graph (claim→evidence with confidence edges), dual-layer diff (textual + semantic) for silent edits, three confidence-propagation strategies over evidence→claim→rule→decision chains, retraction propagation at 100% recall / zero false propagation **[LIKELY — record-level]** (mapping hit §5 search).

- **Log shape for T-060 (normative, derived):** append-only JSONL or SQLite WAL at harness level; each record `{seq, ts, actor, turn_id, kind, body_ref, prev_hash}` with hash chain; large bodies content-addressed (hash) with object store; bidirectional links `artefact ↔ T-NNN` required; link repair budgeted at session close (lint: folded-without-backlink, parked-without-trigger).

---

## 5. Multi-Agent Review & Debate

**Rule the evidence supports: independent reviewers need the verbatim log plus the minimal slice; debate helps only when diversity + validity-aligned reasoning outweigh majority pressure.**

- **Multi-agent debate (MAD) gains are driven by diversity and reasoning strength, not structure.** Controlled Knight–Knave–Spy study (arXiv:2511.07784, 6 structural/cognitive factors: team size, composition, confidence visibility, order, depth, difficulty): intrinsic reasoning strength and group diversity dominate debate success; order/confidence visibility offer limited gains **[LIKELY — preprint, abstract-extracted]** (https://arxiv.org/abs/2511.07784). A companion MAD study finds team size, composition, difficulty as significant factors — convergence without diversity is ensembling, not deliberation.

- **Majority pressure suppresses correction — even vacuous reasoning persuades.** Decomposition study (MMLU-Pro primary + GPQA-Diamond + 3 model families): 37% of agent-question observations change under self-reflection alone; strict conformity 29% in primary setting and predominantly harmful across replications (57–77% correct→wrong); controlled information-gradient experiment: even vacuous reasoning associated with 20–39% error adoption among resistant agents; harmful conformity predictable from Round-0 features (AUC 0.79), reducible by targeted intervention (−13.6 pp, p<0.001), but without correctness labels reduction does not improve accuracy because harmful vs beneficial influence cannot be distinguished **[LIKELY — abstract/record-level from search hits]**.

- **HAJailBench + Multi-Agent Judge shows small-round debate scales.** Safety evaluation with 11,100 labelled jailbreak interactions: structured critic→defender→judge debate improves over matched small-model baselines and prior multi-agent judges while remaining cheaper than GPT-4o; ablation: a small number of rounds captures most gain **[LIKELY — abstract-level]** (search hit §3). Implication for T-060: review protocol is *critic–defender–judge with 2–3 rounds*, not unbounded deliberation.

- **Design implications for transcript-backed review:**
  - Each reviewer receives the *same minimal slice* plus independent random access to sidecars (no shared summary to anchor on).
  - Verdicts cite verbatim locators (`seq`/`hash`), not paraphrases.
  - Diversity is engineered: heterogeneous models or personas, not clones.
  - Validity-aligned reasoning (claim→evidence chain) is scored, not just final answer.
  - Review records are themselves appended to the transcript (review-as-provenance).

---

## 6. Privacy & Data Minimization

**Rule the evidence supports: minimize by view, not by erasure; redact in derived slices while retaining the verbatim ground truth under access control; every compaction is a trust-boundary rewrite.**

- **Data minimization is a per-purpose view, not a retention deletion.** Handbook of Research on Web Log Analysis (privacy for web logging data, services.igi-global.com doi:10.4018/978-1-59904-974-8.ch005) frames the core trade-off: richness of data gathered vs naturalness/privacy of behaviour captured; safeguarding, release, and re-use each carry distinct privacy implications **[LIKELY — handbook chapter, abstract-level]**.

- **Harpocrates shows the privacy+immutability frontier.** Harpocrates (arXiv:2211.04741) stores/shares/accesses sensitive-data operations in an audit log without leaking identifiers or user identity, using zero-knowledge proofs for public verifiability and blockchain for immutability; proves non-malleability and indistinguishability; evaluated on Hyperledger Fabric / EC2 **[LIKELY — preprint, abstract-level]**. Design implication: when the protocol transcript carries PII/secrets, the *served* slice is redacted/pseudonymised; the *stored* verbatim is encrypted with capability-gated access — the same store-vs-serve split as §2.

- **Context Compaction Provenance (CCP) Lab frames compaction as a security boundary.** CCP Lab (SSRN 6933161, CompactedStateV2): naive compaction on a 900-row pilot promoted attacker text into trusted state at ATR/CLR/PVR/ASR = 0.027 (4/150 rows each); protected compaction variants held all four at 0.000; deterministic firewall checks verified invariants; bounded model-generated discovery did not beat deterministic templates **[LIKELY — SSRN record-level, pilot scope only]**. Implication: every compaction step must pass a deterministic trust-boundary check (attacker/quarantined text never promoted to trusted state); treat the compacted slice as untrusted until re-grounded in verbatim.

- **Log-sensitivity gap: no corpus in the fetched set measures transcript privacy for LLM agent logs specifically.** Transfer from web-log and audit-log privacy is reasoned, not measured on agent trajectories; encode conservatively.

- **Privacy shape for T-060:** (a) verbatim at rest encrypted, hash-chained, access-logged; (b) continuation/review slices are redacted and data-minimised per purpose (continuation needs reproducibility; review needs auditability — different minimisation); (c) compaction firewall: untrusted observations / tool outputs / external content quarantined and never auto-promoted; (d) retention policy per §7, not per "keep everything forever."

---

## 7. Cost, Retention & Anti-Patterns

**Rule the evidence supports: retention is a cost-effectiveness decision with a sharp knee; longer is not better — 14 days keeps 97% of useful logs at 22% of the cost.**

- **Retention knee: 90→14 days saves 78% storage, keeps >97% useful logs.** Cost-Aware Logging study (arXiv:2601.11584, synthetic datasets reflecting real volume/access, windows 7/14/30/90 days, metrics: storage cost, operationally useful log ratio, cost per useful log): 90→14 days lowers cost by up to 78% while preserving >97% of logs accessed during debugging/incident analysis; longer windows give diminishing returns and disproportionately increase cost + query overhead **[LIKELY — preprint, abstract-extracted]** (https://arxiv.org/abs/2601.11584). The framework is lightweight and transfers directly to transcript retention policy — though agent trajectories are not web logs; apply as prior, not as target number.

- **Token cost is dominated by repeated prefix re-carry.** Self-GC production split: 10–15% daytime input-token reduction, peaks ~20%; Hard Set: 44% prefix pruned at 84.85% no-impact **[LIKELY]** (§3). Rate–distortion survey: no benchmark holds one budget across layers — so per-layer "savings" claims understate the repeated-compaction tax agents actually pay. Design implication: budget continuation tokens as a *per-session* lifecycle cost, not a per-turn prompt cost.

- **Anti-patterns (evidence-anchored):**

| # | Anti-pattern | What it looks like | Why the evidence forbids it |
|---|--------------|-------------------|----------------------------|
| 1 | **Log-as-suffix** | Flat transcript appended to prompt; pruned by recency/attention | Fails the same way at every layer (rate–distortion survey); hides evidence/locators (Self-GC) **[LIKELY]** |
| 2 | **Summary-as-record** | Compacted summary *replaces* verbatim; no sidecar | No undo pre-query; provenance lost; CCP Lab shows 2.7% trust-boundary laundering **[LIKELY]** |
| 3 | **Agent-writable audit** | Agent writes/can delete its own trace | 5/6 harnesses fail; attacker-inducible deletion **[LIKELY]** (https://arxiv.org/abs/2609.30266) |
| 4 | **Full-trajectory escalation** | Strong model inherits entire weak-model trace | Handoff tax: <50% gap recovery at premium cost **[LIKELY]** (https://arxiv.org/abs/2608.24358) |
| 5 | **Uniform retention** | "Keep everything 90 days" default | 78% waste for +3% useful logs **[LIKELY]** (https://arxiv.org/abs/2601.11584) |
| 6 | **Single-query review** | One reviewer, one pass, no locator | 92.9%/0.13% recall/precision analogue — single-engine unstructured retrieval cannot carry synthesis (cf. thought-log §6) |
| 7 | **Verbatim-in-context** | Entire verbatim log stuffed into continuation window | Token premium + privacy violation + handoff-tax penalty; store-vs-serve exists to prevent this |
| 8 | **Linkless fold** | Artifact cites no T-NNN; T-NNN cites no artifact | Traceability benefit (+86% accuracy) requires bidirectional link; upkeep is the cost to budget **[LIKELY]** |

---

## 8. Confidence Ledger, Gaps & Implementation Notes

**Confidence ledger (every load-bearing claim, one row each):**

| Claim | Label | Basis |
|-------|-------|-------|
| Indexed typed objects + sidecars + safe commit boundaries enable 44% prune / 85–95% no-impact | LIKELY | Preprint, abstract-extracted (https://arxiv.org/abs/2607.00692) + converging ClawVM VM design (doi:10.1145/3805621.3807648) |
| Handoff tax: full escalation <50% gap recovery; interface reversal with direction | LIKELY | Preprint, abstract-extracted (https://arxiv.org/abs/2608.24358) |
| All but one harness allow agent-initiated trace deletion | LIKELY | Preprint, abstract-extracted (https://arxiv.org/abs/2609.30266) |
| Compaction is rate–distortion; attention/recency fails everywhere; repeated compaction unbenchmarked | LIKELY | Survey preprint, abstract-extracted (https://arxiv.org/abs/2607.08032) |
| Debate: diversity + reasoning strength dominate; order/confidence limited | LIKELY | Preprint, abstract-extracted (https://arxiv.org/abs/2511.07784) |
| Majority pressure suppresses correction; vacuous reasoning persuades 20–39% | LIKELY | Abstract/record-level from search corpus (decomposition study, MMLU-Pro + GPQA) |
| Traceability supports 11 activities; change management most frequent; upkeep = main cost | LIKELY | Preprint mapping, 63 studies (https://arxiv.org/abs/2108.02133) + TraceLink +86.06% (abstract-level, IEEE) |
| Fusing app + audit logs yields provenance invisible in either | LIKELY | Peer-reviewed NDSS papers (doi:10.14722/ndss.2021.24445, doi:10.14722/ndss.2021.24549) |
| 90→14 day retention: −78% cost, >97% useful logs kept | LIKELY | Preprint, abstract-extracted (https://arxiv.org/abs/2601.11584) |
| Naive compaction launders untrusted into trusted at 2.7% | LIKELY | SSRN pilot, 900-row lab, 3 open-weight runs (SSRN 6933161, record-level) |
| No direct measurement of agent-transcript privacy specific to LLM trajectories | UNVERIFIED | Gap — transfer from web-log/Harpocrates, not measured |
| House-format references / local protocol grounding (THOUGHT_LOG, .omo) | LOCAL-CONFIRMED | Observed at write time in `development-protocol/` |

**Open gaps (do not encode beyond these):**

1. **PDF full texts not fetched** — all preprint claims rest on abstract + metadata extraction via sidecar; effect sizes (44% prune, <50% recovery, 78% saving, +86% accuracy, 2.7% laundering) need PDF verification before hardening numbers in protocol wording.
2. **Handoff Tax is two families only** (Claude + GPT LC/HC pairs) with repo-state preserved; transfer to other models, non-code tasks, and cross-provider handoffs beyond the reported cross-provider slice is unmeasured.
3. **Tamper study is 6 harnesses, single lab** — Muse as the sole closed harness is a single-point comparison; no production-incident corpus.
4. **Repeated-compaction benchmark does not exist** — the rate–distortion survey's call for a one-budget-axis benchmark is itself the gap; no fetched paper measures 3+ sequential compactions on the same agent trajectory.
5. **Retention 78%/97% is synthetic-cloud web logs, not agent transcripts** — the knee location for protocol transcripts (likely days-to-weeks, not 14 days literally) is unmeasured.
6. **CCP Lab is a 900-row Phase-5 pilot on DGX-class hardware, 3 open-weight models** — 4/150 per indicator is a small-n pilot; not a universal defence claim (authors explicitly bound it).
7. **Debate findings are Knight–Knave–Spy logic puzzles + MMLU-Pro/GPQA** — transfer to protocol-review tasks (spec conformance, evidence grounding) is reasoned, not measured; no study in the fetched set measures reviewer accuracy on agent transcripts directly.
8. **Traceability +86% is one 28-subject lab trial (TraceLink)** — needs variance, transfer to solo-agent setting, and link-rot half-life measurement.
9. **Privacy for agent transcripts has no direct corpus** — Harpocrates zero-knowledge + blockchain properties are proved for sensitive-data audit logs, not for LLM tool-output / file-content logs; re-identification risk for code+prompt logs is ungapped.
10. **No study of transcript schema minimalism itself** — the "minimal verbatim" field set (§2) is derived from converging systems (Self-GC objects, ClawVM pages, provenance graph) and is not validated as minimal by ablation; encode as derived design, not as science.

**Implementation notes (for the T-060 gatekeeper — derived, not encoded here):** pick `steps/` gate files as normative home for transcript shape; add harness-level JSONL/WAL writer (hash-chained, content-addressed, outside agent control) with per-record schema `{seq, ts, actor, turn_id, kind, body_ref, prev_hash}`; derive continuation slice via side-channel planner (store-vs-serve) with 10–15% token-budget target and recoverable sidecars; enforce 2–3-round critic→defender→judge review over verbatim locators with engineered diversity; quarantine untrusted content at compaction boundary (deterministic firewall); retention policy with tiered storage (hot verbatim → warm sidecars → cold hash-chain) and explicit knee analysis per deployment — do not hardcode "14 days."

---

## Appendix A. What this document licenses for T-060

**Three principles to encode:**

1. **Store verbatim, serve minimal (with sidecars).** Append-only, hash-chained, harness-intercepted verbatim log is ground truth; every continuation and review consumes a derived, task-scoped, redacted slice with typed locators back to verbatim and recoverable sidecars for anything pruned. Traced to §§2–4 (rate–distortion, Self-GC 44%/85–95%, ClawVM pages, tamper 5/6, TraceLink +86%).

2. **Harness governs the transcript lifecycle, not the agent.** Index objects, enforce safe commit boundaries, quarantine untrusted content, validate writeback, and make every compaction reversible — context management as lifecycle control (GC), not post-hoc cleanup. Traced to §§3–4 (Self-GC lifecycle, ClawVM invariants, CCP Lab 2.7% laundering).

3. **Review is heterogeneous, locator-grounded, and itself logged.** Independent reviewers get the same minimal slice + independent sidecar access; heterogeneous models/personas; verdicts cite verbatim hashes; review records append to the transcript. Traced to §5 (diversity + validity reasoning dominate; majority pressure suppresses correction; 2–3 rounds suffice).

**Not to encode:**

- Full-trajectory handoff as default (handoff tax: <50% recovery at premium).
- Agent-writable or agent-deletable traces (5/6 harnesses fail).
- Summary-replaces-verbatim (no sidecar, no undo).
- Uniform retention ("90 days for everything") or single-engine/single-reviewer synthesis.
- Bare attention/recency keep-signals for compaction.
- Trust promotion without deterministic firewall checks.
- Any fixed percentage ("14 days is optimal") transferred literally from web-log retention to agent transcripts.

---

## Appendix B. Search log (so this is never redone blind)

- SearXNG `127.0.0.1:8888`, `categories=science` + `general`, 6 queries: trajectory/transcript handoff (hit: Self-GC, Handoff Tax, CFRC, transcript-managed transformers); audit/provenance traceability (hit: mapping study 63, ALchemist, WATSON, provenance-native wiki); multi-agent debate/review (hit: Knight–Knave–Spy, HAJailBench judge, minority sentinel, conformity decomposition); memory compaction fidelity (hit: rate–distortion survey, ClawVM, TokTier); privacy/log minimisation (hit: web-log privacy handbook, Harpocrates); cost/retention overhead (hit: cost-aware logging 78%/97%).
- Extractor sidecar `127.0.0.1:8081/extract` — 7 POSTs, 7 returns (Self-GC, Handoff Tax, Tamper, Rate–Distortion, Debate, Mapping, Cost-Logging). All returned abstract-level markdown only; no PDF full text fetched.
- SearXNG config note: requested `categories=science + general` per brief; `science` not a standalone engine category (mapped to `scientific publications`); resolved via `categories` pass-through per instance `/config`.
- Excluded as off-scope/low-value: transcript-managed transducer universality theory (DCFL/RE hierarchy, not operational); CCP Lab SSRN pilot included as record-level only (small-n, lab hardware); minor ArXiv hits below relevance 4.0 not pursued.

# Prompt-Engineering Science — Evidence Base for Protocol Instruction Text

**Date:** 2026-09-20
**Why this file exists:** every step file in this repo is, mechanically, a prompt. The protocol must therefore follow the empirical literature on instruction-following, not just our own reasoning. This is the durable reference so the sweep is never redone. Companion to `harness-survey-2026-07.md` and `ai-autonomous-execution.md`.
**Method:** SearXNG (`127.0.0.1:8888`, `categories=science`) plus direct arXiv/ACL page fetches. Tavily quota was exhausted mid-sweep, so second-tier sources carry shallower verification than ideal. Confidence labels are per claim. In-house results are labelled as in-house and never presented as external evidence.

## 1. Instruction-following failure modes

**Position effects ("lost in the middle") — CONFIRMED.** Liu et al., arXiv:2307.03172, peer-reviewed TACL 2024. U-shaped curve: information at the start or end of context is used best, the middle worst — across multi-document QA and key-value retrieval, including long-context models.
→ Load-bearing gates belong at a file's edges; a constraint that only appears mid-file is positionally under-weighted.

**Degradation as simultaneous constraint count grows — CONFIRMED.** Jiang et al., FollowBench, arXiv:2310.20410 (ACL 2024 Findings): 820 items, 5 constraint types, constraints added in levels 1–5 — satisfaction rates fall as levels rise. Corroborated by "When Instructions Multiply" (Findings of EMNLP 2025, https://aclanthology.org/2025.findings-emnlp.896.pdf): a "characters per line" constraint scores 99%/97% alone but 20%/2% alongside five other instructions. Same paper: LLM judges inflate compliance (5 instructions 0.815 judged vs 0.574 rule-checked; 10 instructions 0.657 vs 0.213).
→ Cap co-active constraints per phase; retire constraints explicitly when a gate passes; never trust a model-judge compliance check.

**Composition structure matters; Chain is hardest — CONFIRMED.** Wen et al., ComplexBench, arXiv:2407.03978, NeurIPS 2024: 1,150 instructions, 4 composition types. Sequential-conditional (Chain) composition is hardest, then Selection; Format + Lexical combos worst; multi-round decomposition does **not** rescue performance.
→ Phase-gated protocols are Chain compositions by construction. Flatten gate conditions; prefer unconditional checklists over conditionals.

**Negation is a real failure mode — CONFIRMED.** Truong et al., "Language Models Are Not Naysayers," StarSEM 2023 (https://aclanthology.org/2023.starsem-1.10) — larger models are *more* insensitive to negation. Lou et al., Comp Ling 2024 survey (https://aclanthology.org/2024.cl-3.7.pdf) — LLMs fail negated instructions; negation can drop performance.
→ "Do NOT do X" is the least reliable sentence shape in the document. Critical prohibitions need positive restatement ("do Y instead") plus an enforcement check, never a bare negation.

**Repetition at the point of use works (2–3×) — CONFIRMED.** Xu et al., RE2, EMNLP 2024 (https://aclanthology.org/2024.emnlp-main.871): repeating the question 2× improves reasoning across 14 datasets/112 experiments; 2–3× optimal, more degrades; the prefix "Read the question again:" beats bare repetition. Preprint corroboration: arXiv:2512.14982.
→ Re-inject the active phase's constraints at the gate, not only at session start.

**Order dependence / contradictions — GAP.** No fetched study isolates instruction-order swaps or "later overrides earlier" resolution. ComplexBench's Selection results are the nearest indirect evidence.
→ Do not assume later overrides earlier. Avoid contradictions between files rather than resolving them by ordering.

## 2. Over-constraint: imposed procedures can harm

**Verbal step-by-step reasoning harms some tasks — CONFIRMED.** Liu et al., "Mind Your Step (by Step)," arXiv:2410.21333, cross-checked against the CoT-effects line and Kambhampati et al. (NeurIPS 2024). CoT reduces accuracy where verbal deliberation hurts humans: artificial grammar GPT-4o −23.1%, Claude 3 Opus −8%, Gemini −6%, Llama-3.1-70B −8.8%.
→ Mandatory deliberation scaffolds can lower quality. Make templates opt-in per phase.

**Misaligned plan templates harm; native strategy can win — IN-HOUSE ONLY.** Our 16,991-trajectory result (good plan > no plan > bad plan; misaligned best-practice templates actively harm via native-strategy conflict and salience decay) has **no external corroboration found**. Closest external analog is the CoT-harm result above plus Kambhampati's planning critiques — same direction, different mechanism.
→ Label it in-house. A/B every imposed template against "no template."

**Capability-calibrated guidance (expertise reversal) — UNVERIFIED externally.** No external study found; in-house claim stands alone. He et al., Findings EMNLP 2024 (https://aclanthology.org/2024.findings-emnlp.637) is a training result, not a prompting-calibration result.
→ Version strictness by user/model capability rather than assuming one strictness fits all.

## 3. Structural results

Constraint count (§1) and composition type (§1) are the only CONFIRMED structural results. **Headings vs checklists vs prose, and XML vs Markdown, are a GAP**: only practitioner guidance (Anthropic: XML tags; OpenAI: Markdown) plus conflicted unfetched claims (a "Delimiter Hypothesis" null vs arXiv:2411.10541 JSON +42% vs an unverified blog +10–13pp).
→ No evidence licenses a formatting rule. Choose format for human maintainability and delimiter clarity only.

**More documents read across a session — INDIRECT.** No study varies instruction-file count directly; Context Rot and McMillan imply harm.
→ Keep one phase file live; archive artifacts out of context.

## 4. Context rot and drift

**Context rot — CONFIRMED (lab report, open code).** Hong, Troynikov, Huber (Chroma), July 2025 (https://research.trychroma.com/context-rot, code at github.com/chroma-core/context-rot): 18 models, temp 0. Even trivial repeated-word replication degrades with length; one distractor measurably hurts and four compound; low needle–question similarity degrades faster; Claude models abstain when uncertain while GPT models hallucinate confidently; shuffled haystacks sometimes beat coherent ones; LongMemEval focused-context (~100 tokens) ≫ full-context (120k).
→ Active context must stay small regardless of window size: handoff summaries between phases, artifacts archived, only the current gate re-injected.

**Rule drift within a session — LIKELY (one direct preprint).** McMillan, arXiv:2605.10039 (May 2026): 1,650 Claude Code sessions, 16,050 function-level observations, 2 TypeScript codebases. Largest measured effect is **within-session drift: OR ≈ 0.944 per additional generated function (~5.6% lower compliance odds per step)**, reproduced on a second codebase and on Opus 4.6.
→ Drift, not file design, is the dominant effect. Phase gates with re-injection are the countermeasure; front-loading all rules at session start is the anti-pattern.

**Compaction loss — GAP.** No direct study. Assume instruction fidelity is lost on compaction until instrumented; re-inject the active phase file after every compaction.

## 5. Direct studies of rules-file / spec-driven agent instructions

**McMillan 2026 factorial (see §4) — CONFIRMED content, preprint status.** Four manipulated variables (file size, instruction position, file architecture, contradictions across adjacent files) plus interactions, measuring compliance with a trivial target annotation. **None of the four variables or three two-way interactions produced a detectable contrast after multiple-testing correction.** Size and conflict nulls are affirmative nulls (BF10 0.05–0.10); position and architecture are fail-to-reject without Bayes support. Task-to-task variance dominated.
→ This is the negative result that matters: reorganising files, resizing them, or de-conflicting them did **not** move adherence in the one direct study we have. Stop tuning file cosmetics and invest in re-injection, drift countermeasures, and measurement.
Cited within that paper (UNVERIFIED, not fetched): Chatlatanagulchai et al. 2025 (context files grow unboundedly, rarely shrink); Lulla et al. 2026 (repos with context files finish faster with fewer tokens).

**Weaker neighbours (UNVERIFIED).** arXiv:2608.11095 (prompt comments +23.1% WildIFEval, single preprint); agents-md-evals grey benchmark (25/26 assertions identical with and without a 755-line file); AgentIF arXiv:2505.16944 (avg 1,723 words/instruction, ~11.9 constraints each; models poor on tool specs).
**IFEval baseline for calibration — CONFIRMED (lab report).** Zhou et al., arXiv:2311.07911: GPT-4 strict prompt-level 76.9% / instruction-level 83.6%; PaLM 2 S 43.1% / 55.8%.
→ The direct evidence base for rules-file efficacy is thin and mostly null-or-small. Instrument adherence per phase; do not assume the files work.

## 6. The actionable rules

1. **Re-inject the active phase's constraints at each gate.** RE2 (repeat 2–3×, at the point of use) + McMillan drift + Chroma rot.
2. **Cap simultaneous active constraints per phase and retire satisfied ones.** FollowBench + When Instructions Multiply + ComplexBench.
3. **De-chain gate logic; prefer unconditional checklists.** ComplexBench.
4. **Write prohibitions positively; give critical ones an enforcement check.** StarSEM 2023 + Comp Ling 2024.
5. **Put load-bearing rules first or last in every file; never only in the middle.** Lost in the Middle.
6. **Make deliberation templates opt-in and A/B them against no template.** Mind Your Step (−23.1% worst case) + McMillan nulls + in-house template harm.
7. **Keep active context small: one phase file live, artifacts archived, handoff summaries, re-inject after compaction.** Chroma + McMillan.
8. **Instrument adherence per phase (rule-checked spot-checks).** McMillan + IFEval + the judge-inflation warning.

## 7. Open gaps

1. Format/delimiter effects — conflicted, unfetched.
2. Headings vs checklists vs prose vs few-shot — no isolation study found.
3. Order-dependence and contradiction-resolution — no direct study.
4. Compaction loss on instruction text — no direct study.
5. Multi-file count effects on adherence — never varied directly.
6. Capability-calibrated guidance and native-strategy-wins — in-house only.
7. Periodic (~5-step) re-injection in agent trajectories — in-house only (RE2 corroborates repetition in spirit).
8. Single-preprint claims needing replication: Semantic Gravity Wells (arXiv:2601.08070), the +23.1% context-file claim, agents-md-evals deltas, AgentIF details, 100K/150K practitioner thresholds.
9. Tavily quota exhaustion capped cross-verification depth for second-tier claims.

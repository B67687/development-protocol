# Prompt Standards — the rules our instruction text must satisfy

**Why this exists.** Every step file in this repo is, mechanically, a prompt. The rules below are the load-bearing findings from the prompt-engineering literature (`docs/research/prompt-engineering-science.md`) turned into writing rules for this protocol's own text. They exist because instruction-following degrades in measurable ways, and because our text is the interface an agent obeys on every run.

## Scope: two registers in one file

**Operational text is what an agent must obey:** gates, imperatives, prohibitions, checkboxes, exit criteria, required formats. The eight rules bind here.

**Explanatory text is everything else:** rationale, provenance, examples, history, quoted evidence, the "why this exists" paragraphs. Completeness is a virtue here, and none of the rules apply. A long, thorough rationale is correct, not a constraint.

A line that does both is read as operational for its obligation and free for its reason. Write the obligation in the operational register, then explain it in the explanatory register.

## The eight rules

**1. Re-inject at the point of use.** A rule that matters at a gate reappears at that gate. Do not state it once at the top of a file and rely on memory.
*Evidence:* repetition at the point of use helps and 2-3 repeats is the optimum (RE2, EMNLP 2024). Compliance drifts inside a session regardless of the file, roughly 5.6% lower odds per generated step (McMillan 2026, 1,650 sessions). Drift, not file design, is the dominant effect.

**2. Cap the co-active constraints per phase.** Keep the number of obligations live at one gate small, and retire an obligation explicitly once its gate passes.
*Evidence:* satisfaction falls as simultaneous constraints rise (FollowBench, ACL 2024). A constraint met 99% alone drops to 20% alongside five others (When Instructions Multiply, EMNLP 2025 Findings).

**3. Prefer unconditional checklists over chained conditions.** Write the checklist item, not the branching rule. Sequential-conditional composition is the hardest structure for models to follow and decomposition does not rescue it (ComplexBench, NeurIPS 2024).

**4. Write prohibitions positively, and give a critical one an enforcement check.** "Do Y instead of X" outperforms "do not do X". Where the prohibition is load-bearing, add the check that catches it.
*Evidence:* models are largely negation-insensitive, and larger models are more so (Truong et al., StarSEM 2023; Lou et al., Comp Ling 2024 survey). Negation is the least reliable sentence shape in a document.

**5. Put load-bearing rules at a file's edges.** A gate that lives only in the middle of a long file is positionally under-weighted.
*Evidence:* the lost-in-the-middle U-shaped curve (Liu et al., TACL 2024).

**6. Deliberation templates are opt-in.** Any template that imposes step-by-step verbal reasoning is offered, A/B'd against "no template", and dropped where it loses.
*Evidence:* imposed verbal step-by-step reasoning reduced accuracy on artificial-grammar tasks by up to 23.1% (arXiv:2410.21333). Our own misaligned-template result points the same way, and stays in-house.

**7. Keep the live context small.** One phase file is live at a time. Artifacts and reports are archived out of context. After any compaction, the active file is re-injected.
*Evidence:* length alone degrades performance even on trivial tasks, and a focused short context beats a large complete one (Context Rot, Chroma 2025).

**8. Instrument adherence per phase with rule-checked spot checks.** Measure compliance mechanically, never with a model judge.
*Evidence:* judges inflate compliance (0.815 judged versus 0.574 rule-checked at five instructions), and IFEval shows strict prompt-level compliance around 77% even for strong models.

## What Rule 10 can check

Three of the eight are mechanically checkable, and they are the three the lint enforces:

- **Negation density per file** — rule 4's coarse proxy. A file whose prohibition share climbs past the ceiling gets flagged for a rewrite pass.
- **Co-active checkbox ceiling per phase section** — rule 2's proxy. A section may not ask for more obligations at once than the ceiling allows.
- **Research index completeness** — every file in `docs/research/` appears in `docs/research/INDEX.md`.

The other five are manual. A manual rule with no check is a wish, so state it plainly rather than pretending otherwise.

## Counterexamples: where the rules do not apply

- A prohibition quoted from the literature stays quoted. Rewriting a quotation falsifies it.
- Scope exclusion lists keep their negative form: a spec's "out of scope", a phase's "Not allowed", a NO-GO list. These define boundaries rather than direct behaviour, and the positive rewrite changes their meaning.
- A counterexample inside an explanatory paragraph is free text, even when it contains "never".

## Self-application

This document obeys its own rules: eight constraints, prohibitions written positively, obligations near the top, and the checkable subset named. When the standard and a step file disagree, the standard wins and the step file is the thing to fix.

# Research: Protocol Permanence Analysis — 2026-07-28
## Applied to: Development Protocol — classifying steps as permanent vs model-temporary

### Core Question
Will future LLM capabilities make the Development Protocol unnecessary?

### Verdict
**No.** The protocol addresses structural limitations that no amount of model scaling can eliminate. 20+ sources across 5 research vectors converge on this conclusion.

### Evidence Summary

| Vector | Finding | Source |
|--------|---------|--------|
| Self-verification | Fresh-session review beats same-session regardless of model (p=0.008). Same-session repetition doesn't help (p=0.11). | CCR, Song 2026 |
| Self-correction | 64.5% blind spot rate across 14 models. Error is in activation, not knowledge. | Tsui 2025 |
| Contextual drag | Prior context biases subsequent output by 10-20% across 11 models. | Cheng 2026, ICLR |
| RLHF sycophancy | Formally proven amplification mechanism. Larger models hide it better (post-hoc rationalization). | arXiv 2026, ACL 2026 |
| Cross-session memory | LLMs are architecturally stateless. "Longer context does not provide persistent cross-session storage." | Memory Survey 2026 |
| Model collapse | Universal among generative models. Human-designed diversity becomes more valuable. | Shumailov 2024, Nature |
| Methodology scaling | NASA CARE: same model + structured methodology > same model + ad-hoc (71.7% vs 69.1%) | NASA TM 2026 |

### Protocol Step Classification

| Step | Status | Evidence | Action |
|------|--------|----------|--------|
| **REVIEW** (independent) | ✅ Permanent | CCR: context separation is structural, not capability-dependent. Same-model review has 64.5% blind spot. | Continue as-is. Key differentiator. |
| **EXTRACTION** (X vs Y) | ✅ Permanent | RLHF sycophancy is mathematically proven to amplify with optimization. Larger models hide it better via post-hoc rationalization. | Continue as-is. Core value. |
| **Lessons** (cross-session) | ✅ Permanent | LLMs are architecturally stateless. No amount of scaling gives a model memory between sessions. Only external substrate works. | Continue as-is. Critical infrastructure. |
| **FUNDAMENTALS** (one-way doors) | ✅ Permanent | One-way door classification requires human judgment of business/institutional constraints. Models can assist but not decide. | Continue as-is. |
| **DECOMPOSITION** | ⚠️ Semi-permanent | Models may decompose natively, but confirmation and MECE verification likely remain human tasks. | Keep, may become lighter. |
| **EXECUTOR** (DfC rules) | ❌ Temporary | Better models naturally follow structural constraints. The 10 rules document may become unnecessary. | Plan removal when models are consistent enough. |
| **SPECIFICATION** (14-section) | ❌ Temporary | Models will work from looser specs. The template may simplify to 5-7 sections. | Keep for now, relax over time. |

### Decision
Continue building the protocol. The core steps (REVIEW, EXTRACTION, Lessons) are structurally permanent — they solve problems no future model can eliminate. The temporary steps (EXECUTOR rules, SPEC template) can be relaxed as models improve.

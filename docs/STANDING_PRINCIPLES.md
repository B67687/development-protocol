# Standing Principles

The three rules that shape every other rule. If a proposed addition does not serve one of them, it does not ship.

## 1. You approve strategy once

The agent proposes a written plan, you accept or amend it, then the agent builds. No per-action approvals. Human judgment gates direction; the agent owns execution within the ratified scope.

## 2. Check consequential claims

Any step can call for verification, graded by cost of being wrong and grounded in retrieved evidence rather than model memory. Cheap claims pass; expensive ones carry proof.

> **The scope ceiling limits features, never the defensibility of a claim.** You may build fewer things. You may not assert a number you cannot defend under a repeat. Cutting a feature is a legitimate answer; an undefended number is not.

## 3. Keep it light

Ceremony costs effort, so additions must earn their place. Default is ship at roughly 80 percent, except one-way doors. The protocol stays minimal by design — see `docs/SKIP_CATALOG.md` for what was deliberately left out.

> Balance brake: prefer stability over adventure by default and stay balanced, not one-directional. Do not optimize off the main speed problem; specialization loses general balance. Adaptable versus adaptive is a genuine trade-off (adaptable wins change-detection, situation-awareness and autonomy at higher workload; adaptive wins routine and load), so the choice is per-regime.

## Why these three

Most AI coding loops fail in the same two places: building the wrong thing (no strategy gate), and trusting confident-sounding claims (no verification). The third principle exists because the cure can become the disease — a heavy process gets skipped entirely. Light process that actually runs beats thorough process that doesn't.

## Standing probe

> When a new flagship model arrives, run the protocol once on a live task without changing any method. Keep the method unless the run exposes a genuinely new failure class. Model gains so far land in execution, not in the want and should-build gates, so adopt the model and keep the gates.

## Guidance law — teach slightly above what the user already knows

Understanding builds on prior experience and study. The protocol grows the user by staying slightly above current ability (zone of proximal development), contingent and faded by demonstration rather than front-loaded. It teaches by building alongside: knowledge links are declared in simple words at first use, prerequisites named, and progress made visible. Learning comes from building and executing; the gates stay dynamic and engaging.

## What we do not know

Every rule here rests on research we could reach, and our reach is open-access only. Treat a confident answer from this protocol as the best available to us, not the best possible. When a cheaper explanation of the same evidence exists, or a tradition we have not read, the rule is still a candidate. When paywalled evidence is needed, use school-library access via the Self-Hosted-Search backend when available and record the access method; otherwise mark the claim as open-access-only and do not fabricate the paywalled source.
Fun and creativity probes never become gates: no new gates, no new phases from them.

## Form friction is the enemy, not thought

Engagement lives or dies on form, not on ideas. Cut form-friction (re-asking, lost context, blank prompts) before touching standards. Dynamicism — adapting pace and scaffolding to the user's demonstrated ability — is how the protocol stays engaging.

> **Productive friction is a feature; form-friction is the enemy.** Form-friction (confusing gates, lost state, re-asking, undefined jargon) is always removed. Productive friction is kept: making the human form a judgment before the agent reveals its recommendation, because that is what stops over-reliance. Friction-dose order: decide-before-reveal > ruled-out-alternative > on-demand. A bare timed wait is NOT friction to encode.


## Verification scope

Verification belongs at claims, not everywhere — specs and numbers every time, prose via the symmetric-instrument check. Everywhere drowns; nowhere leaks.

> **No mechanism without a checker: a rule that is not tied to an enforcement surface is a wish.** A spec level that names its checker (for example, a claim that names its repeat method) is enforceable; one that does not is a wish.

## Research basis

Harness survey: [docs/research/harness-survey-2026-07.md](research/harness-survey-2026-07.md).

## Shipping rule (3-pass)

Every shipped thought gets three passes before it is considered done: (1) **build** — the first implementation; (2) **grounding** — check it against literature or evidence, not just reasoning; (3) **re-verify** — adversarially test it against cases designed to break it (slang, edge inputs, hostile readings). Single-pass diffs are suspect by default — small diffs earn extra scrutiny, not less, because thin work hides inside them. A thought with three passes and a short diff is finished; a thought with one pass is a draft regardless of length.

At artifact scale the same shape is the EXECUTOR internal loop: descend structure, then behavior, then craft, then surface, and stop when two descents surface no new class of problem (`../steps/EXECUTOR.md` § The internal loop).

# Standing Principles

The three rules that shape every other rule. If a proposed addition does not serve one of them, it does not ship.

## 1. You approve strategy once

The agent proposes a written plan, you accept or amend it, then the agent builds. No per-action approvals. Human judgment gates direction; the agent owns execution within the ratified scope.

## 2. Check consequential claims

Any step can call for verification, graded by cost of being wrong and grounded in retrieved evidence rather than model memory. Cheap claims pass; expensive ones carry proof.

## 3. Keep it light

Ceremony costs effort, so additions must earn their place. Default is ship at roughly 80 percent, except one-way doors. The protocol stays minimal by design — see `SKIP_CATALOG.md` for what was deliberately left out.

## Why these three

Most AI coding loops fail in the same two places: building the wrong thing (no strategy gate), and trusting confident-sounding claims (no verification). The third principle exists because the cure can become the disease — a heavy process gets skipped entirely. Light process that actually runs beats thorough process that doesn't.

## Standing probe

> When a new flagship model arrives, run the protocol once on a live task without changing any method. Keep the method unless the run exposes a genuinely new failure class. Model gains so far land in execution, not in the want and should-build gates, so adopt the model and keep the gates.

## Research basis

Harness survey: [docs/research/harness-survey-2026-07.md](research/harness-survey-2026-07.md).

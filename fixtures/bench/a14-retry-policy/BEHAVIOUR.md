# BEHAVIOUR.md — A14 retry-policy contract (input/output only)

Frozen source: `src-py/retry_policy.py` (read-only). Public smoke tests:
`src-py/tests_public.py` (5 tests, must stay green).

## Configuration

| Field | Type | Default | Constraint |
|---|---|---|---|
| max_attempts | int | 4 | >= 1 |
| base_delay | seconds (float) | 0.1 | >= 0, <= max_delay |
| max_delay | seconds (float) | 5.0 | >= 0 |
| multiplier | float | 2.0 | >= 1 |
| jitter | fraction (float) | 0.1 | in [0, 1] |
| deadline | seconds or null | 30.0 | null = no cap |
| retryable | predicate (error -> bool) | timeouts + connection errors | — |

Invalid configuration rejects at construction time (no lazy failure).

## Delay schedule

- Delay before retry N (1-based): `min(base_delay * multiplier^(N-1), max_delay)`,
  perturbed by symmetric jitter of +/- `jitter` fraction.
- `jitter: 0` disables randomness (deterministic schedule).
- A seeded RNG makes the full schedule reproducible.

## Execution semantics

1. Call the function. On success, return its value immediately (exactly 1 call).
2. On error: if the predicate rejects, propagate immediately (no further calls).
3. If attempts are exhausted (`max_attempts` failures) or the deadline has
   passed since start, re-raise the last error.
4. Otherwise wait the scheduled delay, then retry.
5. Sync and async variants share identical schedule semantics; the async
   variant awaits coroutine results and sleeps without blocking the loop.

## Error cases (must hold in any port)

- First-try success performs exactly one call.
- Retryable errors retry until success or exhaustion (last error re-raised).
- Non-retryable errors propagate after exactly one call.
- Invalid configuration fails fast at construction.
- Requesting a delay for attempt < 1 is an error.

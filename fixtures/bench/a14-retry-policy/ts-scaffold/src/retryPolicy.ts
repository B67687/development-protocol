/**
 * retryPolicy — bounded retries with exponential backoff, jitter, and a
 * deadline cap. Idiomatic TypeScript port of the frozen Python source
 * (`src-py/retry_policy.py`); observable behaviour is defined by
 * `../BEHAVIOUR.md`.
 *
 * LANDSCAPE note (Python constructs with no direct TS equivalent):
 * - `@dataclass(frozen=True)` → `readonly` fields assigned once in the
 *   constructor from an options bag (positional kwargs have no TS idiom).
 * - `random.Random | None` seed → `rng?: () => number` returning [0, 1).
 *   Injecting the uniform source keeps `delayFor` pure and testable without
 *   a global-seed side channel (`Math.random` is the default).
 * - `time.monotonic()` → `Date.now()` (both wall-clock ms here; sub-ms
 *   monotonic precision is irrelevant at second-granularity deadlines).
 * - `asyncio.sleep` → `setTimeout`-based `sleep` (non-blocking in `callAsync`).
 * - Python's builtin `TimeoutError`/`ConnectionError` have no TS builtins →
 *   module-local `TimeoutError`/`ConnectionError` subclasses; the default
 *   predicate matches on `instanceof` either, mirroring the Python default.
 * - `ValueError` at construction → `RangeError`, the TS idiom for
 *   out-of-range arguments.
 */

/** Decides whether a thrown value is worth another attempt. */
export type RetryablePredicate = (error: unknown) => boolean;

/** Uniform [0, 1) source. Defaults to `Math.random`; inject a seeded
 * function for a reproducible schedule. */
export type RandomSource = () => number;

/** Retryable timeout failure (mirrors Python's builtin `TimeoutError`). */
export class TimeoutError extends Error {
  override readonly name = "TimeoutError";
}

/** Retryable connection-level failure (mirrors Python's `ConnectionError`). */
export class ConnectionError extends Error {
  override readonly name = "ConnectionError";
}

/** Retry timeouts and connection-level failures only. */
export function isRetryableByDefault(error: unknown): boolean {
  return error instanceof TimeoutError || error instanceof ConnectionError;
}

/** Immutable retry configuration with a derived delay schedule. */
export interface RetryPolicyOptions {
  readonly maxAttempts?: number;
  readonly baseDelay?: number;
  readonly maxDelay?: number;
  readonly multiplier?: number;
  /** Symmetric jitter fraction in [0, 1]. */
  readonly jitter?: number;
  /** Seconds, or `null` for no cap. */
  readonly deadline?: number | null;
  readonly retryable?: RetryablePredicate;
  readonly rng?: RandomSource | null;
}

/** A synchronous or asynchronous unit of work. */
export type Work<T> = () => T | Promise<T>;

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

// Shared futex for the synchronous `call` path: `Atomics.wait` blocks the
// thread without spinning, mirroring Python's `time.sleep`. Allocated once
// so retries never pay per-attempt setup cost.
const sleepCell = new Int32Array(new SharedArrayBuffer(4));

function sleepSync(ms: number): void {
  Atomics.wait(sleepCell, 0, 0, Math.max(0, ms));
}

export class RetryPolicy {
  readonly maxAttempts: number = 4;
  readonly baseDelay: number = 0.1;
  readonly maxDelay: number = 5.0;
  readonly multiplier: number = 2.0;
  readonly jitter: number = 0.1;
  readonly deadline: number | null = 30.0;
  readonly retryable: RetryablePredicate = isRetryableByDefault;
  readonly rng: RandomSource | null = null;

  constructor(options: RetryPolicyOptions = {}) {
    if (options.maxAttempts !== undefined) this.maxAttempts = options.maxAttempts;
    if (options.baseDelay !== undefined) this.baseDelay = options.baseDelay;
    if (options.maxDelay !== undefined) this.maxDelay = options.maxDelay;
    if (options.multiplier !== undefined) this.multiplier = options.multiplier;
    if (options.jitter !== undefined) this.jitter = options.jitter;
    if (options.deadline !== undefined) this.deadline = options.deadline;
    if (options.retryable !== undefined) this.retryable = options.retryable;
    if (options.rng !== undefined) this.rng = options.rng;

    if (this.maxAttempts < 1) {
      throw new RangeError("maxAttempts must be >= 1");
    }
    if (this.baseDelay < 0 || this.maxDelay < 0) {
      throw new RangeError("delays must be >= 0");
    }
    if (this.multiplier < 1) {
      throw new RangeError("multiplier must be >= 1");
    }
    if (!(this.jitter >= 0 && this.jitter <= 1)) {
      throw new RangeError("jitter must be in [0, 1]");
    }
    if (this.baseDelay > this.maxDelay) {
      throw new RangeError("baseDelay must be <= maxDelay");
    }
  }

  /**
   * Delay in seconds before retry number `attempt` (1-based).
   * Exponential growth from `baseDelay`, capped at `maxDelay`, with
   * symmetric jitter of ±`jitter` fraction.
   */
  delayFor(attempt: number): number {
    if (attempt < 1) {
      throw new RangeError("attempt must be >= 1");
    }
    const grown = Math.min(
      this.baseDelay * this.multiplier ** (attempt - 1),
      this.maxDelay,
    );
    const draw = this.rng ?? Math.random;
    return grown * (1 + (draw() * 2 - 1) * this.jitter);
  }

  private deadlineExceeded(startMs: number): boolean {
    return (
      this.deadline !== null && Date.now() - startMs >= this.deadline * 1000
    );
  }

  /**
   * Call `fn` until it succeeds, the predicate rejects, or attempts run out.
   * Re-raises the last error on exhaustion.
   */
  call<T>(fn: () => T): T {
    const startMs = Date.now();
    let last: unknown;
    for (let attempt = 1; attempt <= this.maxAttempts; attempt += 1) {
      try {
        return fn();
      } catch (error) {
        if (!this.retryable(error)) {
          throw error;
        }
        last = error;
        if (attempt === this.maxAttempts || this.deadlineExceeded(startMs)) {
          break;
        }
        sleepSync(this.delayFor(attempt) * 1000);
      }
    }
    throw last;
  }

  /**
   * Async variant of {@link call} with the same schedule semantics: awaits
   * promise results and sleeps without blocking the loop.
   */
  async callAsync<T>(fn: Work<T>): Promise<T> {
    const startMs = Date.now();
    let last: unknown;
    for (let attempt = 1; attempt <= this.maxAttempts; attempt += 1) {
      try {
        return await fn();
      } catch (error) {
        if (!this.retryable(error)) {
          throw error;
        }
        last = error;
        if (attempt === this.maxAttempts || this.deadlineExceeded(startMs)) {
          break;
        }
        await sleep(Math.max(0, this.delayFor(attempt) * 1000));
      }
    }
    throw last;
  }
}

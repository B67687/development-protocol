/**
 * Sealed hidden oracle — A14 language-port behavioural suite (24 cases).
 * Derived from fixtures/bench/a14-retry-policy/BEHAVIOUR.md only.
 * SHA256 of this file at seal time is logged in the run trace.
 */
import { describe, expect, it } from "vitest";
import {
  ConnectionError,
  RetryPolicy,
  TimeoutError,
} from "../ts-scaffold/src/retryPolicy.js";

function fastPolicy(overrides = {}) {
  return new RetryPolicy({
    maxAttempts: 3,
    baseDelay: 0.001,
    maxDelay: 0.002,
    jitter: 0,
    deadline: 5.0,
    ...overrides,
  });
}

/** Deterministic mulberry32 uniform source. */
function seeded(seed: number): () => number {
  let state = seed >>> 0;
  return () => {
    state = (state + 0x6d2b79f5) >>> 0;
    let t = state;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

describe("config validation (fails fast at construction)", () => {
  it("H01 rejects maxAttempts < 1", () => {
    expect(() => new RetryPolicy({ maxAttempts: 0 })).toThrow(RangeError);
  });

  it("H02 rejects negative delays", () => {
    expect(() => new RetryPolicy({ baseDelay: -0.1 })).toThrow(RangeError);
    expect(() => new RetryPolicy({ maxDelay: -1 })).toThrow(RangeError);
  });

  it("H03 rejects multiplier < 1", () => {
    expect(() => new RetryPolicy({ multiplier: 0.5 })).toThrow(RangeError);
  });

  it("H04 rejects jitter outside [0, 1]", () => {
    expect(() => new RetryPolicy({ jitter: -0.1 })).toThrow(RangeError);
    expect(() => new RetryPolicy({ jitter: 1.5 })).toThrow(RangeError);
  });

  it("H05 rejects baseDelay > maxDelay", () => {
    expect(
      () => new RetryPolicy({ baseDelay: 5, maxDelay: 1 }),
    ).toThrow(RangeError);
  });

  it("H06 accepts deadline null (no cap)", () => {
    const policy = fastPolicy({ deadline: null, maxAttempts: 2 });
    expect(() => policy.call(() => "ok")).not.toThrow();
  });
});

describe("delay schedule", () => {
  it("H07 grows exponentially with jitter 0", () => {
    const policy = new RetryPolicy({
      baseDelay: 0.1,
      maxDelay: 5.0,
      multiplier: 2.0,
      jitter: 0,
    });
    expect(policy.delayFor(1)).toBeCloseTo(0.1, 9);
    expect(policy.delayFor(2)).toBeCloseTo(0.2, 9);
    expect(policy.delayFor(3)).toBeCloseTo(0.4, 9);
  });

  it("H08 caps at maxDelay", () => {
    const policy = new RetryPolicy({
      baseDelay: 1,
      maxDelay: 2.5,
      multiplier: 10,
      jitter: 0,
    });
    expect(policy.delayFor(3)).toBeCloseTo(2.5, 9);
    expect(policy.delayFor(10)).toBeCloseTo(2.5, 9);
  });

  it("H09 jitter stays within ±fraction of grown delay", () => {
    const policy = new RetryPolicy({
      baseDelay: 1,
      maxDelay: 100,
      multiplier: 2,
      jitter: 0.1,
      rng: seeded(7),
    });
    for (let attempt = 1; attempt <= 4; attempt += 1) {
      const grown = 2 ** (attempt - 1);
      const delay = policy.delayFor(attempt);
      expect(delay).toBeGreaterThanOrEqual(grown * 0.9);
      expect(delay).toBeLessThanOrEqual(grown * 1.1);
    }
  });

  it("H10 seeded rng reproduces the full schedule", () => {
    const first = new RetryPolicy({ jitter: 0.5, rng: seeded(42) });
    const second = new RetryPolicy({ jitter: 0.5, rng: seeded(42) });
    const a = [1, 2, 3, 4, 5].map((n) => first.delayFor(n));
    const b = [1, 2, 3, 4, 5].map((n) => second.delayFor(n));
    expect(a).toEqual(b);
  });

  it("H11 rejects attempt < 1", () => {
    expect(() => fastPolicy().delayFor(0)).toThrow(RangeError);
    expect(() => fastPolicy().delayFor(-2)).toThrow(RangeError);
  });
});

describe("sync execution", () => {
  it("H12 first-try success performs exactly one call", () => {
    let calls = 0;
    expect(fastPolicy().call(() => { calls += 1; return "ok"; })).toBe("ok");
    expect(calls).toBe(1);
  });

  it("H13 retryable errors retry until success", () => {
    let calls = 0;
    const result = fastPolicy({ retryable: () => true }).call(() => {
      calls += 1;
      if (calls < 3) throw new Error("flaky");
      return "recovered";
    });
    expect(result).toBe("recovered");
    expect(calls).toBe(3);
  });

  it("H14 exhaustion re-raises the last error after maxAttempts calls", () => {
    let calls = 0;
    const policy = fastPolicy({ maxAttempts: 2, retryable: () => true });
    const boom = new Error("down");
    expect(() => policy.call(() => { calls += 1; throw boom; })).toThrow(boom);
    expect(calls).toBe(2);
  });

  it("H15 non-retryable propagates after exactly one call", () => {
    let calls = 0;
    const policy = fastPolicy({ retryable: () => false });
    expect(() => policy.call(() => { calls += 1; throw new Error("bug"); })).toThrow("bug");
    expect(calls).toBe(1);
  });

  it("H16 default predicate retries timeouts, rejects programmer errors", () => {
    const policy = fastPolicy();
    let timeoutCalls = 0;
    const out = policy.call(() => {
      timeoutCalls += 1;
      if (timeoutCalls < 2) throw new TimeoutError("slow");
      return "ok";
    });
    expect(out).toBe("ok");
    let connCalls = 0;
    policy.call(() => {
      connCalls += 1;
      if (connCalls < 2) throw new ConnectionError("reset");
      return "ok";
    });
    expect(connCalls).toBe(2);
    let valueCalls = 0;
    expect(() => policy.call(() => { valueCalls += 1; throw new TypeError("bug"); })).toThrow(TypeError);
    expect(valueCalls).toBe(1);
  });

  it("H17 custom predicate routes per-error", () => {
    let calls = 0;
    const policy = fastPolicy({
      retryable: (error) => error instanceof TypeError,
    });
    const result = policy.call(() => {
      calls += 1;
      if (calls < 3) throw new TypeError("transient type bug");
      return "ok";
    });
    expect(result).toBe("ok");
    expect(calls).toBe(3);
    expect(() => policy.call(() => { throw new RangeError("fatal"); })).toThrow(RangeError);
  });

  it("H18 expired deadline stops before attempts run out", () => {
    let calls = 0;
    const policy = fastPolicy({ maxAttempts: 10, deadline: 0 });
    expect(() => policy.call(() => { calls += 1; throw new TimeoutError("down"); })).toThrow(TimeoutError);
    expect(calls).toBe(1);
  });
});

describe("async execution", () => {
  it("H19 async first-try success performs exactly one call", async () => {
    let calls = 0;
    const out = await fastPolicy().callAsync(async () => { calls += 1; return "ok"; });
    expect(out).toBe("ok");
    expect(calls).toBe(1);
  });

  it("H20 async retryable errors retry until success", async () => {
    let calls = 0;
    const out = await fastPolicy({ retryable: () => true }).callAsync(async () => {
      calls += 1;
      if (calls < 3) throw new Error("flaky");
      return "async-ok";
    });
    expect(out).toBe("async-ok");
    expect(calls).toBe(3);
  });

  it("H21 async exhaustion re-raises the last error", async () => {
    let calls = 0;
    const policy = fastPolicy({ maxAttempts: 2, retryable: () => true });
    await expect(policy.callAsync(async () => { calls += 1; throw new Error("down"); })).rejects.toThrow("down");
    expect(calls).toBe(2);
  });

  it("H22 async non-retryable propagates after exactly one call", async () => {
    let calls = 0;
    const policy = fastPolicy({ retryable: () => false });
    await expect(policy.callAsync(async () => { calls += 1; throw new Error("bug"); })).rejects.toThrow("bug");
    expect(calls).toBe(1);
  });

  it("H23 async accepts a sync function and returns its value", async () => {
    const out = await fastPolicy().callAsync(() => "sync-value");
    expect(out).toBe("sync-value");
  });

  it("H24 sync/async share schedule semantics (parity)", async () => {
    const attempts = 3;
    let syncCalls = 0;
    fastPolicy({ retryable: () => true }).call(() => {
      syncCalls += 1;
      if (syncCalls < attempts) throw new Error("flaky");
      return "ok";
    });
    let asyncCalls = 0;
    const out = await fastPolicy({ retryable: () => true }).callAsync(async () => {
      asyncCalls += 1;
      if (asyncCalls < attempts) throw new Error("flaky");
      return "ok";
    });
    expect(out).toBe("ok");
    expect(asyncCalls).toBe(syncCalls);
  });
});

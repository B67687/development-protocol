"""Bounded retry policy: exponential backoff with jitter and a deadline cap.

Pure standard library. Sync (:func:`RetryPolicy.call`) and async
(:func:`RetryPolicy.acall`) variants share the same delay schedule, so a
port must preserve observable behaviour across both variants.
"""

from __future__ import annotations

import asyncio
import random
import time
from dataclasses import dataclass, field
from typing import Callable, TypeVar

T = TypeVar("T")
Predicate = Callable[[BaseException], bool]


def _default_retryable(exc: BaseException) -> bool:
    """Retry timeouts and connection-level failures only."""
    return isinstance(exc, (TimeoutError, ConnectionError))


@dataclass(frozen=True)
class RetryPolicy:
    """Immutable retry configuration with a derived delay schedule."""

    max_attempts: int = 4
    base_delay: float = 0.1
    max_delay: float = 5.0
    multiplier: float = 2.0
    jitter: float = 0.1
    deadline: float | None = 30.0
    retryable: Predicate = field(default=_default_retryable)
    rng: random.Random | None = field(default=None, compare=False)

    def __post_init__(self) -> None:
        if self.max_attempts < 1:
            raise ValueError("max_attempts must be >= 1")
        if self.base_delay < 0 or self.max_delay < 0:
            raise ValueError("delays must be >= 0")
        if self.multiplier < 1:
            raise ValueError("multiplier must be >= 1")
        if not 0 <= self.jitter <= 1:
            raise ValueError("jitter must be in [0, 1]")
        if self.base_delay > self.max_delay:
            raise ValueError("base_delay must be <= max_delay")

    def delay_for(self, attempt: int) -> float:
        """Delay in seconds before retry number ``attempt`` (1-based).

        Exponential growth from ``base_delay``, capped at ``max_delay``,
        with symmetric jitter of +/- ``jitter`` fraction. Deterministic
        when ``rng`` is a seeded ``random.Random``.
        """
        if attempt < 1:
            raise ValueError("attempt must be >= 1")
        grown = min(self.base_delay * (self.multiplier ** (attempt - 1)),
                    self.max_delay)
        rng = self.rng if self.rng is not None else random
        return grown * (1.0 + (rng.random() * 2.0 - 1.0) * self.jitter)

    def _deadline_exceeded(self, start: float) -> bool:
        return self.deadline is not None and (time.monotonic() - start) >= self.deadline

    def call(self, fn: Callable[..., T], *args: object, **kwargs: object) -> T:
        """Call ``fn`` until it succeeds, the predicate rejects, or attempts run out."""
        start = time.monotonic()
        last: BaseException | None = None
        for attempt in range(1, self.max_attempts + 1):
            try:
                return fn(*args, **kwargs)
            except Exception as exc:  # noqa: BLE001 — predicate decides routing
                if not self.retryable(exc):
                    raise
                last = exc
                if attempt == self.max_attempts or self._deadline_exceeded(start):
                    break
                time.sleep(self.delay_for(attempt))
        assert last is not None
        raise last

    async def acall(self, fn: Callable[..., T], *args: object, **kwargs: object) -> T:
        """Async variant of :meth:`call` with the same schedule semantics."""
        start = time.monotonic()
        last: BaseException | None = None
        for attempt in range(1, self.max_attempts + 1):
            try:
                result = fn(*args, **kwargs)
                if asyncio.iscoroutine(result):
                    result = await result
                return result
            except Exception as exc:  # noqa: BLE001 — predicate decides routing
                if not self.retryable(exc):
                    raise
                last = exc
                if attempt == self.max_attempts or self._deadline_exceeded(start):
                    break
                await asyncio.sleep(self.delay_for(attempt))
        assert last is not None
        raise last

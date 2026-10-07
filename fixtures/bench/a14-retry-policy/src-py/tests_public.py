"""Public smoke tests for the frozen A14 source module (visible to agent)."""

import asyncio
import unittest

from retry_policy import RetryPolicy


def _fast_policy(**overrides):
    base = dict(max_attempts=3, base_delay=0.001, max_delay=0.002, jitter=0.0, deadline=5.0)
    base.update(overrides)
    return RetryPolicy(**base)


class TestRetryPolicySmoke(unittest.TestCase):
    def test_success_first_try_calls_once(self):
        calls = []
        policy = _fast_policy()

        def fn():
            calls.append(1)
            return 'ok'

        self.assertEqual(policy.call(fn), 'ok')
        self.assertEqual(len(calls), 1)

    def test_retries_then_succeeds(self):
        attempts = []
        policy = _fast_policy()

        def fn():
            attempts.append(1)
            if len(attempts) < 3:
                raise TimeoutError('flaky')
            return 'recovered'

        self.assertEqual(policy.call(fn), 'recovered')
        self.assertEqual(len(attempts), 3)

    def test_exhaustion_reraises_last_error(self):
        policy = _fast_policy(max_attempts=2)

        def fn():
            raise TimeoutError('down')

        with self.assertRaises(TimeoutError):
            policy.call(fn)

    def test_non_retryable_propagates_immediately(self):
        calls = []
        policy = _fast_policy()

        def fn():
            calls.append(1)
            raise ValueError('caller bug')

        with self.assertRaises(ValueError):
            policy.call(fn)
        self.assertEqual(len(calls), 1)

    def test_async_parity_with_sync_schedule(self):
        async def main():
            seen = []

            async def fn():
                seen.append(1)
                if len(seen) < 3:
                    raise TimeoutError('flaky')
                return 'async-ok'

            return await _fast_policy().acall(fn), seen

        result, seen = asyncio.run(main())
        self.assertEqual(result, 'async-ok')
        self.assertEqual(len(seen), 3)


if __name__ == '__main__':
    unittest.main()

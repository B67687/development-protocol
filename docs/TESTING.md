# TESTING.md — Test Infrastructure Pattern

> Comprehensive test infrastructure for AI-assisted development. Covers test pyramid, generation workflow, quality requirements, file organization, mutation testing, coverage tracking, and anti-patterns. Reference this document during WORK phase and PERFECT phase.

---

| 1. | [Test Pyramid (Adapted for AI Agents)](#1-test-pyramid-adapted-for-ai-agents)
| 2. | [Test Generation Workflow](#2-test-generation-workflow)
| 3. | [Test Quality Requirements](#3-test-quality-requirements)
| 4. | [Test File Organization](#4-test-file-organization)
| 5. | [Mutation Testing](#5-mutation-testing)
| 6. | [Coverage Tracking](#6-coverage-tracking)
| 7. | [Anti-Patterns for AI-Generated Tests](#7-anti-patterns-for-ai-generated-tests)

---

## 1. Test Pyramid (Adapted for AI Agents)

The traditional test pyramid is adapted for AI-assisted development. The AI generates the tests; the human reviews them. Each layer has a specific role and automation boundary.

```
        /\
       / E2E \          Manual or Playwright
      /--------\
     /Integration\       Agent generates with explicit boundary contracts
    /--------------\
   /    Unit Tests   \    Agent generates, human reviews
  /--------------------\
 /    Mutation Tests    \   Run weekly or before PERFECT
/------------------------\
```

### Unit Tests (Agent Generates, Human Reviews)

| Aspect | Rule |
| --- | --- |
| **Who writes** | AI agent during WORK phase |
| **Who reviews** | Human at POLISH or via code review |
| **Scope** | One function or method per test file section |
| **Speed** | < 10 ms each |
| **Quantity** | Many — every public function gets at least one test |
| **Dependencies** | Real objects preferred; in-memory fakes for stores/caches |

**Agent responsibility:** Generate tests from SPECIFICATION.md, not from implementation. Tests must describe expected behavior before any code exists.

**Human responsibility:** Review for tautological tests (tests that always pass), missing edge cases, and testing implementation details.

### Integration Tests (Agent Generates with Explicit Boundary Contracts)

| Aspect | Rule |
| --- | --- |
| **Who writes** | AI agent during WORK phase |
| **Who reviews** | Human at POLISH |
| **Scope** | One boundary/crossing per test (DB, HTTP, file system, external service) |
| **Speed** | < 1 s each |
| **Quantity** | Some — one per significant boundary crossing |
| **Dependencies** | Real adapter against real downstream (testcontainers, httptest, sandbox) |

**Agent responsibility:** Identify boundary crossings from the spec. Write tests that exercise the real adapter. Never mock what you own; fake what you do not own.

**Boundary contract rule:** Every integration test documents the boundary it exercises:

```python
# Boundary: UserService -> PostgreSQL
# Contract: creates user within 100ms, returns User ID or DuplicateEmail error
def test_create_user_persists_to_database():
    ...
```

### E2E Tests (Manual or Playwright)

| Aspect | Rule |
| --- | --- |
| **Who writes** | AI agent (Playwright scripts) or human (manual) |
| **Who reviews** | Human — E2E tests are the final quality gate |
| **Scope** | One user-visible outcome per test |
| **Speed** | Seconds — run on CI, not on every save |
| **Quantity** | Few — one narrative per feature |
| **Dependencies** | Full application stack |

**Agent responsibility:** Generate Playwright scripts for deterministic user flows (login, checkout, form submission). Manual E2E is human-only.

**Rule:** If a feature has zero E2E coverage, it is undone — even if every unit test passes.

### Mutation Tests (Run Weekly)

| Aspect | Rule |
| --- | --- |
| **What** | Mutates source code slightly, checks if tests catch the mutation |
| **Why** | Catches tautological tests (tests that always pass regardless of code) |
| **When** | Weekly on CI, or before PERFECT phase |
| **Target** | 80%+ mutation score |
| **Scope** | Core domain logic first; infrastructure code optional |

See [§5 Mutation Testing](#5-mutation-testing) for full details.

---

## 2. Test Generation Workflow

### When: During WORK Phase, BEFORE Implementation (TDD)

Tests are written during the WORK phase, BEFORE implementation code exists. This is non-negotiable — the protocol's test philosophy (RULES.md §9) mandates test-first development.

```
SPECIFICATION.md complete
    |
    v
+-----------+     +-----------+     +-----------+     +-----------+
| Write     | --> | Run tests | --> | Implement | --> | Run tests |
| tests     |     | (must     |     | code      |     | (must     |
| from SPEC |     |  FAIL)    |     | (min fix) |     |  PASS)    |
+-----------+     +-----------+     +-----------+     +-----------+
    |                                                              |
    |              +-----------+     +-----------+                  |
    +------------>| Check     | --> | Check     | <----------------+
                  | coverage  |     | mutations |
                  | (80%+)    |     | (80%+)    |
                  +-----------+     +-----------+
```

### How: Agent Writes Tests from SPECIFICATION.md

The agent reads SPECIFICATION.md (not implementation code) to derive test cases:

1. **Read spec section** — understand the behavior contract
2. **Identify inputs and outputs** — what goes in, what comes out
3. **Write edge cases first** — empty inputs, null values, boundary conditions
4. **Write error paths** — invalid input, missing resources, timeout
5. **Write happy path last** — the normal case
6. **Run tests** — all must FAIL (no implementation exists yet)

### Verification: Tests Must Fail Without Implementation (Red Phase)

After writing tests but before writing implementation:

```bash
# Run tests — expect ALL failures
pytest tests/                    # Python
cargo test                       # Rust
bun test                         # TypeScript
go test ./...                    # Go
```

**Every test must fail for the right reason.** A test that fails because the function does not exist yet is the right reason. A test that fails because of a missing import or wrong syntax is not.

**Checkpoint:** Commit `checkpoint: tests-written — [section_name]` before starting implementation.

### Coverage Target: 80%+ for New Code (Statement Coverage)

| Metric | Threshold | Enforcement |
| --- | --- | --- |
| Statement coverage (new code) | 80%+ | CI gate — block merge if below |
| Branch coverage (new code) | 70%+ | Warning — do not block |
| Overall project coverage | Maintain or improve | Check at POLISH |

Coverage is measured on NEW code only. Legacy code is not required to hit 80% unless modified.

---

## 3. Test Quality Requirements

### Arrange/Act/Assert (AAA) Structure

Every test follows the three-block structure. No exceptions.

```python
def test_calculate_discount_for_vip_customer():
    # Arrange
    customer = Customer(tier="vip")
    order = Order(total=100.00)

    # Act
    discount = calculate_discount(customer, order)

    # Assert
    assert discount == 15.00  # VIP gets 15%
```

**Rules:**
- One `Act` per test. Multiple actions = multiple tests.
- `Arrange` sets up state. `Act` performs the action. `Assert` verifies the outcome.
- No logic (if/for/while) in test files — tests are declarative, not procedural.

### Test Naming Convention

Test names describe behavior in three parts: `<unit>_<scenario>_<expected>`.

| Format | Example |
| --- | --- |
| Python | `test_calculate_discount_vip_customer_returns_15_percent` |
| Rust | `calculate_discount_vip_customer_returns_15_percent` |
| TypeScript | `it("calculates 15% discount for VIP customers")` |
| Go | `TestCalculateDiscount_VipCustomer_Returns15Percent` |

**Bad names:** `test_discount`, `test1`, `test_happy_path`, `test_works`.

**Good names:** `test_decode_unknown_format_returns_error`, `test_empty_list_returns_empty_result`, `test_expired_token_returns_unauthorized`.

### No Test Interdependencies

Tests must be order-independent. Each test:
- Sets up its own state (or uses shared fixtures that reset)
- Tears down after itself
- Does not depend on another test's output
- Can run in isolation and in any order

```python
# BAD: depends on test_create_user having run first
def test_get_user():
    user = get_user(created_id)  # created_id from another test?

# GOOD: self-contained
def test_get_existing_user():
    user = create_user(name="Alice")
    result = get_user(user.id)
    assert result.name == "Alice"
```

### Each Test Verifies ONE Behavior

A test with multiple assertions on unrelated behaviors is a mini-test-suite masquerading as a test. Split it.

```python
# BAD: tests two behaviors
def test_order():
    order = create_order(items=[...])
    assert order.total == 100      # behavior 1: total calculation
    assert order.status == "pending"  # behavior 2: initial status

# GOOD: separate tests
def test_order_total_calculated_correctly():
    order = create_order(items=[...])
    assert order.total == 100

def test_order_starts_in_pending_status():
    order = create_order(items=[...])
    assert order.status == "pending"
```

### Edge Cases Explicitly Tested

Edge cases are written BEFORE happy path tests. If the AI cannot handle the edge case, it should not handle the happy path.

**Mandatory edge cases per data type:**

| Data Type | Edge Cases |
| --- | --- |
| String | empty string, whitespace-only, very long (10K+ chars), unicode, null bytes |
| Number | zero, negative, max int, min int, NaN (float), infinity |
| Array/List | empty, single element, very large (10K+), null elements, duplicates |
| Object/Struct | null/nil, all fields empty, all fields max length, extra unknown fields |
| Date/Time | epoch zero, far future, far past, timezone boundaries, leap seconds |
| Boolean | true, false, null (if nullable) |

### Error Paths Tested (Not Just Happy Path)

For every function that can fail:
- Test the error return/exception for invalid input
- Test the error return/exception for missing resources
- Test the error return/exception for timeout/disconnection
- Verify error messages are descriptive (not just "error occurred")

---

## 4. Test File Organization

### Mirror Source Structure

Test files mirror the source directory structure. One test file per source file.

```
src/
  services/
    user_service.py
    order_service.py
  models/
    user.py
    order.py

tests/
  test_services/
    test_user_service.py
    test_order_service.py
  test_models/
    test_user.py
    test_order.py
  conftest.py          # shared fixtures
```

### Naming Conventions by Language

| Language | Test File Name | Test Function Name |
| --- | --- | --- |
| Python | `test_*.py` | `test_<unit>_<scenario>_<expected>` |
| TypeScript | `*.test.ts` | `it("<description>")` |
| Rust | `*_test.rs` or inline `#[cfg(test)]` | `fn <unit>_<scenario>_<expected>()` |
| Go | `*_test.go` | `Test<Unit>_<Scenario>_<Expected>` |

### One Test File Per Source File

Each source file gets exactly one test file. If the test file exceeds 300 lines, split by behavior (not by function).

### Shared Fixtures in conftest.py / Setup File

Common test setup lives in a shared fixture file:

| Language | Fixture File | Mechanism |
| --- | --- | --- |
| Python | `conftest.py` | `@pytest.fixture` |
| TypeScript | `setup.ts` or `vitest.config.ts` | `beforeEach` / global setup |
| Rust | `tests/common/mod.rs` | `fn setup()` or test helpers |
| Go | `TestMain(m *testing.M)` | Package-level setup |

**Rule:** Fixtures must reset state between tests. No fixture leaks — a test that passes individually but fails with others has a fixture leak.

---

## 5. Mutation Testing

### What: The Secret Weapon

Mutation testing mutates source code in small, syntactically valid ways (changing operators, removing lines, inverting conditions), then runs the test suite. If the tests still pass, the mutation survived — meaning the tests did not actually verify that part of the code.

**Surviving mutations = gaps in test coverage.**

```bash
# Python
mutmut run --paths-to-mutate=src/
mutmut results

# JavaScript/TypeScript
npx stryker run --mutate "src/**/*.ts"

# Rust
cargo mutants --file src/lib.rs
```

### Why: Catches Tautological Tests

Code coverage (§6) says "this line was executed." Mutation testing says "this line's behavior was VERIFIED." A test can execute every line and still miss bugs — that is a tautological test. Mutation testing catches them.

| Metric | What It Measures | Catches |
| --- | --- | --- |
| Code coverage | Lines executed | Unreachable code |
| Mutation score | Lines verified | Tautological tests, missing assertions, weak oracles |

### When: Weekly or Before PERFECT

| Trigger | Action |
| --- | --- |
| Weekly CI | Run on core domain modules |
| Before PERFECT phase | Run on all modules — surviving mutants become PERFECT work items |
| After major refactor | Run on changed modules |
| Pre-release | Run full suite — 80%+ mutation score required to ship |

### Target: 80%+ Mutation Score

| Score | Interpretation | Action |
| --- | --- | --- |
| 90%+ | Excellent — tests are rigorous | Ship |
| 80-89% | Good — minor gaps | Ship with TODO for surviving mutants |
| 70-79% | Weak — significant gaps | Add tests for surviving mutants before PERFECT |
| < 70% | Insufficient — tests are likely tautological | Block ship; rewrite weak tests |

### Interpreting Surviving Mutants

Not all surviving mutants indicate weak tests. Classify each:

| Classification | Meaning | Action |
| --- | --- | --- |
| **Equivalent** | Mutant is semantically identical to original | Dismiss with documented reason |
| **Killed** | Test caught the mutation | Coverage confirmed |
| **Real gap** | Mutation would cause a real bug not caught by tests | Add test; BLOCK at REVIEW if on applied feature |

---

## 6. Coverage Tracking

### Tools by Language

| Language | Tool | Command |
| --- | --- | --- |
| Python | coverage.py | `pytest --cov=src --cov-report=html --cov-report=term` |
| TypeScript | c8 / istanbul | `bun test --coverage` or `npx c8 bun test` |
| Rust | cargo-tarpaulin | `cargo tarpaulin --out Html` |
| Go | go test -cover | `go test -coverprofile=coverage.out ./... && go tool cover -html=coverage.out` |

### Threshold: 80% Statement Coverage for New Code

| Metric | Threshold | Enforcement |
| --- | --- | --- |
| Statement coverage (new code) | 80%+ | CI gate — block merge if below |
| Branch coverage (new code) | 70%+ | Warning |
| Function coverage (new code) | 90%+ | Warning |
| Overall project coverage | Maintain or improve | Check at POLISH |

### What to Exclude

Exclude from coverage measurement:
- Migrations (database, schema)
- Configuration files
- `__init__.py` (Python)
- Generated code
- Test files themselves

```bash
# Python coverage.py exclusion
# .coveragerc
[run]
omit =
    */migrations/*
    */config/*
    */__init__.py
    */generated/*

# TypeScript c8 exclusion
// package.json
"c8": {
    "exclude": ["**/migrations/**", "**/generated/**", "**/*.config.*"]
}
```

### Report Format

Run coverage with both terminal summary and HTML report:

```bash
# Terminal summary for quick check
pytest --cov=src --cov-report=term-missing

# HTML report for POLISH review
pytest --cov=src --cov-report=html:htmlcov
# Open htmlcov/index.html for visual review
```

**POLISH step:** Human reviews HTML coverage report to identify uncovered code that should be tested. AI flags untested code paths; human decides which need tests.

---

## 7. Anti-Patterns for AI-Generated Tests

AI agents produce specific test quality problems. Watch for these:

### Testing Implementation Details (Private Methods)

```python
# BAD: tests private method
def test_user_validate_password_hash():
    user = User(name="Alice")
    assert user._hash_password("secret") == expected_hash  # private method

# GOOD: tests public behavior
def test_user_authentication_rejects_wrong_password():
    user = User(name="Alice", password="correct_password")
    assert user.authenticate("wrong_password") is False
```

### Tests With No Assertions

```python
# BAD: no assertion — just runs code
def test_create_user():
    user = create_user(name="Alice")
    # no assert statement

# GOOD: asserts observable outcome
def test_create_user_returns_user_with_id():
    user = create_user(name="Alice")
    assert user.id is not None
    assert user.name == "Alice"
```

### Tests That Test the Framework, Not the Code

```python
# BAD: tests that pytest works, not your code
def test_pytest_runs():
    assert True  # this tests nothing

# BAD: tests that the ORM works
def test_database_saves():
    user = User(name="Alice")
    db.save(user)
    loaded = db.load(User, user.id)
    assert loaded.name == "Alice"  # this tests SQLAlchemy, not your code
```

### Tests With Hardcoded Values That Always Pass

```python
# BAD: expected value derived from implementation (tautology)
def test_calculate_total():
    items = [Item(price=10), Item(price=20)]
    result = calculate_total(items)
    assert result == get_price(items)  # recomputes same thing

# GOOD: expected value from independent source
def test_calculate_total():
    items = [Item(price=10), Item(price=20)]
    result = calculate_total(items)
    assert result == 30  # independently known correct value
```

### Tests That Depend on Execution Order

```python
# BAD: relies on global state from previous test
counter = 0

def test_increment():
    global counter
    counter += 1
    assert counter == 1

def test_counter_is_one():
    global counter
    assert counter == 1  # fails if test_increment didn't run first
```

### Copy-Pasted Tests With Different Input But Same Assertion

```python
# BAD: copy-paste — all test the same behavior, just with different data
def test_add_two_numbers():
    assert add(1, 2) == 3

def test_add_two_numbers_v2():
    assert add(2, 3) == 5

def test_add_two_numbers_v3():
    assert add(10, 20) == 30

# GOOD: parameterized — tests same behavior across inputs
@pytest.mark.parametrize("a,b,expected", [(1, 2, 3), (2, 3, 5), (10, 20, 30)])
def test_add_two_numbers(a, b, expected):
    assert add(a, b) == expected
```

### Testing the Happy Path Only

```python
# BAD: only tests the easy case
def test_process_order():
    order = Order(items=[Item(price=10)])
    result = process_order(order)
    assert result.status == "completed"

# GOOD: tests happy path + error paths
def test_process_order_success():
    order = Order(items=[Item(price=10)])
    result = process_order(order)
    assert result.status == "completed"

def test_process_order_empty_cart():
    order = Order(items=[])
    with pytest.raises(EmptyCartError):
        process_order(order)

def test_process_order_insufficient_funds():
    order = Order(items=[Item(price=1000)])
    user = User(balance=5)
    with pytest.raises(InsufficientFundsError):
        process_order(order, user=user)
```

---

## Reference: Integration with Protocol Documents

| Document | Section | Relationship |
| --- | --- | --- |
| RULES.md | §9 Test Philosophy | Parent rules — TESTING.md operationalizes them |
| RULES.md | §8.1 Type Safety Gate | Runs before test generation — types must pass first |
| EXECUTOR.md | WORK phase | Tests written before implementation per TDD |
| EXECUTOR.md | FINISH Gate / Polish Checklist | Test coverage verification (80%+ statement) |
| EXECUTOR.md | Regression-Lock | Characterization tests lock applied features |
| SPECIFICATION.md | §10 Testing | Test requirements derived from spec |
| REVIEW.md | SPEC_SYNC | Verify tests match spec behavior contracts |

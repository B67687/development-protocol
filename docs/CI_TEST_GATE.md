# CI Test Gate Template

> Reusable CI configuration for software projects. Copy and adapt to your stack.
> Used by engineering-plugin.md §1 (CI, Tooling & Quality Gates).

---

## Go Projects

```yaml
# .github/workflows/ci.yml
name: ci

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

permissions:
  contents: read

jobs:
  build:
    strategy:
      matrix:
        go: ["1.25.x"]
      fail-fast: false

    runs-on: ubuntu-latest
    timeout-minutes: 10

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-go@v5
        with:
          go-version: ${{ matrix.go }}
          cache: true

      - name: go build
        run: go build ./...

      - name: go vet
        run: go vet ./...

      - name: staticcheck
        run: staticcheck ./...

      - name: golangci-lint
        uses: golangci/golangci-lint-action@v6
        with:
          version: latest

      - name: go test with coverage
        run: go test -race -coverprofile=coverage.out ./... -count=1 -timeout=60s

      - name: test coverage threshold
        run: |
          COVERAGE=$(go tool cover -func=coverage.out | tail -1 | awk '{print $3}' | tr -d '%')
          echo "Test coverage: ${COVERAGE}%"
          if [ $(echo "$COVERAGE < 60" | bc -l) -eq 1 ]; then
            echo "ERROR: Coverage ${COVERAGE}% is below 60% threshold"
            exit 1
          fi

      - name: upload coverage
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: coverage
          path: coverage.out

  secrets-scan:
    runs-on: ubuntu-latest
    timeout-minutes: 5

    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: gitleaks
        uses: gitleaks/gitleaks-action@v2
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Required config files

- `.golangci.yml` — linter configuration (errcheck, staticcheck, gosec, gocritic, etc.)
- `staticcheck.conf` — optional, for staticcheck-specific config

### Local equivalent (scripts/local-ci.sh)

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "=== Build ==="
go build ./...

echo "=== Vet ==="
go vet ./...

echo "=== Staticcheck ==="
staticcheck ./...

echo "=== Golangci-lint ==="
golangci-lint run

echo "=== Tests ==="
go test -race -coverprofile=coverage.out ./... -count=1 -timeout=60s

echo "=== Coverage ==="
COVERAGE=$(go tool cover -func=coverage.out | tail -1 | awk '{print $3}' | tr -d '%')
echo "Coverage: ${COVERAGE}%"
if [ $(echo "$COVERAGE < 60" | bc -l) -eq 1 ]; then
  echo "ERROR: Coverage below 60% threshold"
  exit 1
fi

echo "=== All gates passed ==="
```

---

## Python Projects

```yaml
name: ci

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

permissions:
  contents: read

jobs:
  build:
    runs-on: ubuntu-latest
    timeout-minutes: 10

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"

      - name: install dependencies
        run: |
          pip install -e ".[dev]"

      - name: ruff lint
        run: ruff check .

      - name: ruff format check
        run: ruff format --check .

      - name: basedpyright
        run: basedpyright --pythonversion 3.12

      - name: pytest with coverage
        run: |
          pytest --cov=. --cov-report=xml --cov-fail-under=60 -v

      - name: upload coverage
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: coverage
          path: coverage.xml

  secrets-scan:
    runs-on: ubuntu-latest
    timeout-minutes: 5

    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: gitleaks
        uses: gitleaks/gitleaks-action@v2
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Required config files

- `pyproject.toml` with `[tool.basedpyright]` section
- `.ruff.toml` or `pyproject.toml` with `[tool.ruff]` section

---

## TypeScript Projects

```yaml
name: ci

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

permissions:
  contents: read

jobs:
  build:
    runs-on: ubuntu-latest
    timeout-minutes: 10

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: "20"
          cache: "npm"

      - name: install
        run: npm ci

      - name: type check
        run: tsc --noEmit

      - name: lint
        run: npm run lint

      - name: format check
        run: npx biome format --check .

      - name: test with coverage
        run: npm test -- --coverage

      - name: coverage threshold
        run: |
          COVERAGE=$(cat coverage/coverage-summary.json | jq '.total.lines.pct')
          echo "Coverage: ${COVERAGE}%"
          if [ $(echo "$COVERAGE < 60" | bc -l) -eq 1 ]; then
            echo "ERROR: Coverage below 60% threshold"
            exit 1
          fi

  secrets-scan:
    runs-on: ubuntu-latest
    timeout-minutes: 5

    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: gitleaks
        uses: gitleaks/gitleaks-action@v2
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Required config files

- `tsconfig.json` with `"strict": true`
- `biome.json` or `biome.jsonc`

---

## Gate Summary

| Gate | Purpose | Fail behavior |
|------|---------|---------------|
| Build | Code compiles | Block merge |
| Type check | Type safety | Block merge |
| Lint | Code quality | Block merge |
| Format | Consistency | Block merge |
| Tests | Correctness | Block merge |
| Coverage | Test completeness | Block if <60% |
| Secrets | Security | Block merge |

---

## Customization

- **Coverage threshold:** Adjust `60` in the coverage check to match your project's needs. 60% is the minimum; 80% is the target for new code.
- **Race detection:** `-race` flag catches data races. Essential for concurrent code. Adds ~2x test time.
- **Timeout:** Adjust `timeout-minutes` based on test suite size. 10 minutes is typical for small-to-medium projects.
- **Matrix:** Add OS or version matrix for cross-platform projects.

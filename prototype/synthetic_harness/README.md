# Synthetic Harness for Development-Protocol Meta-Testing

**Level 1: Semi-automated testing with mock LLM**

This harness generates synthetic trials to test whether the Development-Protocol's EXTRACTION bouncing extracts tacit knowledge (hidden X) that freeform conversation misses. It uses deterministic mock LLMs to prove the harness logic without requiring API keys.

## Quick Start

```bash
# From repo root
uv run python prototype/synthetic_harness/run.py --personas 3

# Test all 10 personas
uv run python prototype/synthetic_harness/run.py --personas 10

# Detailed scores
uv run python prototype/synthetic_harness/run.py --personas 3 --detail

# Raw JSON output
uv run python prototype/synthetic_harness/run.py --personas 3 --json
```

## How It Works

### Two Conditions per Persona

Each persona is tested under two conditions:

1. **Baseline (freeform)**: Simple prompt with no bouncing — simulates a normal conversation
2. **Protocol (bouncing)**: 3-5 rounds with Preference Layer probes + bias detection prompts

### Mock LLM Logic

The mock LLM is deterministic:
- If the prompt contains a **Preference Layer probe** (e.g., "what would you miss most", "what did you actually spend time on"), it reveals the persona's `hidden_tacit_X`
- Otherwise, it returns the `stated_Y` (surface-level answer)

This proves that **bouncing probes are necessary** to extract tacit knowledge.

### Judge Scoring

Each output spec is scored on 3 dimensions (0-5 each, max 15):

| Dimension | What it measures | Keywords |
|-----------|-----------------|----------|
| **Falsifiability** | Can the spec be proven wrong? | must, shall, measurable, threshold |
| **One-way door coverage** | Does it identify irreversible decisions? | irreversble, commit, contract, migration |
| **Assumption explicitness** | Are assumptions stated vs hidden? | assumption, assuming, hypothesis, risk |

### Replay (Retrospective)

Tests if SERIOUSNESS RCF would have killed failed projects:

```bash
uv run python prototype/synthetic_harness/replay.py
uv run python prototype/synthetic_harness/replay.py --csv fixtures/postmortems.csv
```

Uses Reference Class Forecasting base rates:
- Personal tool: ~70% success
- OSS library: ~40%
- Production service: ~30%
- Research prototype: ~20%

## Persona Format

Each persona in `personas.json` has:

```json
{
  "id": "p01",
  "name": "Solo Dev Dashboard",
  "domain": "personal_tool",
  "stated_Y": "I want a personal analytics dashboard",
  "hidden_tacit_X": "I feel disconnected from my own work output",
  "revealed_history": [
    "Built 3 dashboard projects, abandoned all within 2 weeks",
    "Spends 2hrs/day checking GitHub notifications",
    "Last shipped feature was a README badge"
  ]
}
```

**10 personas** covering:
- 4 personal tool / research prototype
- 4 OSS library / learning project
- 6 production service

## Files

```
prototype/synthetic_harness/
├── __init__.py          # Package marker
├── personas.json        # 10 synthetic personas
├── run.py               # Main runner (two conditions per persona)
├── judge.py             # Blind judge (3 dimensions)
├── replay.py            # Retrospective replay (RCF check)
└── README.md            # This file

fixtures/
└── postmortems.csv      # 10 example projects for replay
```

## What to Expect

### Mock Mode Output

```
Running 3 synthetic trials (mock LLM mode)...

==========================================================================================
SYNTHETIC HARNESS RESULTS — Mock LLM Mode
==========================================================================================
Persona                  Domain               Baseline X      Protocol X      Winner    Score
------------------------------------------------------------------------------------------
Solo Dev Dashboard       personal_tool        I want a perso... I feel discon... protocol  3/12
OSS CLI Tool             oss_library          I want to build... I want recogni... protocol  3/12
Production API           production_service   We need to bui... Our churn rate... protocol  3/12
------------------------------------------------------------------------------------------
Totals: Protocol wins: 3 | Baseline wins: 0 | Ties: 0
N = 3 personas
==========================================================================================

PASS: 3/3 personas improved with protocol bouncing
```

**Key insight**: Protocol wins because bouncing probes extract the hidden tacit X that baseline misses. The judge scores protocol higher because the tacit X is more specific, testable, and identifies real assumptions.

## Plugging In Real LLM

To use real LLM instead of mock:

1. Set environment variables:
   ```bash
   export OPENAI_API_KEY=sk-...
   # or
   export ANTHROPIC_API_KEY=sk-ant-...
   ```

2. Modify `mock_llm()` in `run.py` to call the real API:
   ```python
   import os
   if os.environ.get("OPENAI_API_KEY"):
       # Call OpenAI API
       from openai import OpenAI
       client = OpenAI()
       response = client.chat.completions.create(
           model="gpt-4",
           messages=[{"role": "user", "content": prompt}]
       )
       return response.choices[0].message.content
   else:
       # Mock mode
       ...
   ```

3. The judge can also be upgraded to use LLM evaluation (TODO).

## Interpreting Scores

- **Protocol wins > 50%**: Bouncing successfully extracts tacit knowledge (PASS)
- **Protocol wins ≤ 50%**: Mock probes aren't triggering hidden X extraction (FAIL — check probe keywords)
- **Judge scores**: Higher total (max 15) = more falsifiable, one-way-door aware, assumption-explicit spec
- **Replay recall**: % of failed projects RCF would have caught before commitment

## Limitations (Level 1)

- Mock LLM is deterministic — no variance in responses
- Judge uses keyword heuristics — not semantic understanding
- Personas are handcrafted — not randomly generated
- No cross-persona comparison or statistical significance

## Next Steps (Level 2)

- [ ] Random persona generation (N=100+)
- [ ] Real LLM calls (OpenAI/Anthropic)
- [ ] LLM-as-judge evaluation
- [ ] Statistical tests (paired t-test on judge scores)
- [ ] Ablation: which probes matter most?
- [ ] Bias catalog integration (all 8 biases tested per persona)

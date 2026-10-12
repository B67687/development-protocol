# ULW synthesis — the Development Protocol on the bedrock of how development works

> **Run:** `ulw-research` 2026-10-12 (`20261012-030217`). **Method:** 8 meta-research axis sweeps (S1–S8) + 3 ulw waves (26 workers) + this synthesis. Every load-bearing claim below carries a confidence label and a counter-search note (T-090 discipline: each wave-3 worker searched *against* its claims and reported what it found). Journal: `.omo/ulw-research/20261012-030217/` (`brief.md`, `wave-1-returns.md`, `wave-2-returns.md`, `wave-3-returns.md`, `expansion-log.md`).
> **Scope:** the eleven thoughts T-079..T-089 plus T-090, all of which the user said "apply to every part of the dev protocol even the overall part in itself".
> **Convergence:** three waves, 26 workers, one thread held by every worker independently — **a mechanism that is not tied to a checker is a wish.**

---

## 1. The bedrock: how development actually works

The protocol is an *overlay* on a natural process, not a replacement for it. The natural trajectory is regime-shaped, not linear (`foundation-vs-overlay-2026-10-07.md`):

| Regime | What is true | What is cheap | Cost of being wrong |
|---|---|---|---|
| **Fuzzy-open** | the want is not yet sayable | generating candidates, cheap contact with reality | low — a sentence |
| **Fuzzy-narrow** | a direction exists, alternatives are live | comparing options, small probes | moderate |
| **Clear-commit** | one direction chosen, one-way doors appear | ratification, freezing interfaces | high — a rebuild |
| **Clear-scale** | the shape is fixed, volume grows | execution, mechanical checks | high — trust |
| **Re-fuzz** | the artifact teaches a new want | reflection, next-cycle seeding | low |

Two rules fall out and both are already in the protocol's philosophy (`CONSTITUTION.md § The philosophy`): *you cannot specify what you have not seen* (so prototypes/spikes are planning artifacts), and *fix in planning what can be fixed in planning* (which is the share **already known to someone else** — large for familiar problem classes, small for novel ones; a function of novelty, never a constant 50%).

Iteration-cost evidence (S-fires, `max-plan-min-exec-2026-10-03.md`): Boehm's cost-of-change curve is canonical but its ratios are unrecovered and Menzies contests the extreme form; the defensible rule is **front-load learning, defer locking** (Toyota set-based concurrent engineering), not "max planning". Agile ≥ waterfall modestly and conditionally (no pooled ES); **iterative beats waterfall in direction, magnitudes unread**.

**The protocol's position:** regime-correct (INBOX/EXTRACTION = fuzzy-open, AMBITION-late = fuzzy-narrow, LANDSCAPE/VALIDATION = clear-commit, SPEC/EXECUTOR = clear-scale, REFLECT/HANDOVER = re-fuzz). This is the one thing the audit says is *right by construction*.

---

## 2. What the protocol does vs what it claims (intent-diff, W1+W2)

| Thought | Intent-diff verdict |
|---|---|
| T-079 hiccups | **ABSENT / anti-handled** — no deliberate friction; `STANDING_PRINCIPLES:36-38` calls form-friction the enemy; only `CONSTITUTION:63` (self-evolution guard) valorizes friction |
| T-080 plain speak | **PARTIAL** — voice rules cover deliverables + gate cards only; the T-001 jargon problem is still UNRESEARCHED in-repo |
| T-081 generality | **HANDLED** |
| T-082 logistics vs problem | **PARTIAL / NET-NEW** — amortization machinery exists, the logistics/specific split is unnamed |
| T-083 simplify + refactor beliefs | **PARTIAL** — macro→micro and code refactor handled; belief-refactor is event-driven only |
| T-084 continuation | **HANDLED** |
| T-085 recursive spec | **PARTIAL** — spec depth handled; recursive style-of-method spec absent |
| T-086 shishen/yuanshen | **PARTIAL** — automate-analysis / protect-judgment is built but UNNAMED |
| T-087 reframing | **HANDLED** |
| T-088 stability | **HANDLED** |
| T-089 no over-specialize | **HANDLED** (COVERED in the store) |

The codebase-register audit (W2) found the machine is real but its **voice** is the gap: gate cards are the plainest surface; `QUICKSTART` is plain by design; the Output/Exit/progress-header blocks carry **~45+ tokens undefined at point of use**. And there are **17 deliberate friction points** already (F1 single ratification … F17 ledger sentinel) sitting next to the *smooth* Invariant-11 default-autonomous confirmations — friction exists but is unlabelled and unbalanced.

---

## 3. Per-thought verdicts and folds (T-079..T-090)

Principle-layer first; **no new gates** where a principle or an existing gate note suffices.

| # | Thought | Fold (principle-first) | Target | Cost |
|---|---|---|---|---|
| T-080 | plain speak | **Protocol-own-voice floor**: R1 define-then-use inline · R2 second person/active/short + read-aloud · R3 layered precision (plain verdict → exact term in parens → pointer; dropping a downstream-needed distinction is INCORRECT) · R4 a because-clause on every demand · R6 anchor-first. Rewrite the **progress header** as plain verdict + exact term. Add the **DTI** distinction inventory (3–7 MUST-SURVIVE rows/gate) where a gate's load-bearing distinction is subtle. | `steps/EXTRACTION.md` Layer 1, `steps/RULES.md`, `docs/PROMPT_STANDARDS.md`, `docs/HOUSE_STANDARD.md` | M |
| T-079 | hiccups | **Name productive friction vs form-friction**: form-friction is the enemy (remove); productive friction is a feature (keep). Friction-dose rule at the **5 human-selection gates** (SERIOUSNESS/AMBITION/STRATEGY/VALIDATION-kill/REVIEW): *decide-before-reveal* (record judgment + rival + confidence **before** the agent reveals its recommendation) > ruled-out-alternative > on-demand ≫ bare wait (do NOT encode a bare 30-second wait). | `docs/STANDING_PRINCIPLES.md`, `docs/CONSTITUTION.md`, the 5 gate files (gate-card note) | M |
| T-086 | shishen/yuanshen | **Dual-layer doctrine, named**: *Automate the scaffolding; never the verdict.* Rules: automation must demand own-words rationale, must end in a genuine logged verdict, cap consecutive delegated steps; solution authority = human, process scaffolding = machine. Agency rating (1-item) as a REFLECT signal. | `docs/CONSTITUTION.md`, `steps/REFLECT.md` | S |
| T-083 | simplify + refactor beliefs | **Belief-triage ledger** (Assumption-Based Planning): dated, ≤7 load-bearing rows, ABP columns (assumption · signpost · shaping action · hedge · disposition), retirement always dated, every row priced; disposition vocab mirrors G7. **Layered cadence**: signpost scan on cadence, deep revision on trigger (not event-only). | `steps/REFLECT.md`, `steps/STRATEGY.md` | M |
| T-085 | recursive spec | **Spec-recursion stopping rule**: CHECKER TEST — every spec level names its enforcement; *no checker = a wish*. YAGNI cost-of-carry brake. **Style-as-spec** → tool-checkable rule + lightweight review norm, never a heavyweight gate; prompt-style depth only for CONFIRMED structural choices. | `steps/SPECIFICATION.md` | S |
| T-087 | reframing | **Reframing doctrine**: triggers T1 repetition-signature / T2 constraint-feeling / T3 single-analogue / T4 time-pressure; 3 techniques (restate+constraint audit · second-analogue rule · contradiction formulation + Ideal Final Result); brute-force-is-correct boundary (persist iff well-defined state space ∧ no improving-parameter contradiction ∧ failures are coverage not repetition). | `steps/STRATEGY.md`, `steps/VALIDATION.md` | S |
| T-088/T-089 | stability / no over-specialize | **Balance + over-optimization brake**: prefer stability over adventure *by default*, balanced not one-directional; adaptable-vs-adaptive is a genuine trade-off (adaptable wins change-detection/SA/autonomy at higher workload; adaptive wins routine/load) — choose per regime. Do not optimize off the main speed problem; specialization loses general balance (YAGNI + satisficing). | `docs/STANDING_PRINCIPLES.md` | S |
| T-082 | logistics vs problem | **Name the two work classes**: problem-work (the specific thing) vs logistics-work (the general, reusable thing). Amortization rule: a one-off logistics fix becomes a reusable artifact (script / workflow / skill) the moment it recurs; log it so the division of labour is visible. | `steps/INBOX.md`, `steps/EXECUTOR.md` | S |
| T-081/T-084 | generality / continuation | **Brownfield delta specs** (OpenSpec shape: ADDED/MODIFIED/REMOVED/RENAMED + SHALL/Scenario, `changes/` → merge → `archive/YYYY-MM-DD-<name>/`) and an explicit **resume state artifact** (single source of truth, harness-agnostic) for continuation. | `steps/SPECIFICATION.md`, `steps/REFLECT.md` (HANDOVER rider) | M |
| T-090 | counter-search | **Mandatory disconfirming search** per load-bearing claim + a recorded `counter-search` row (what you searched against it, what you found); patch the skill prompts (`authoritative-search`, `meta-research`) and the protocol research rule. | `~/.config/opencode/skills/{authoritative-search,meta-research}/SKILL.md`, `steps/RULES.md` (research section) | S |

**Cross-cutting principle (all thoughts):** *every mechanism must be tied to a checker, or it is a wish* — the single strongest convergence across all 26 workers. This is the meta-rule that makes T-085's checker test, T-079's friction dose, and T-090's counter-search one idea rather than three.

---

## 4. The claim graph (load-bearing claims, confidence, counter-search)

| Claim | Confidence | Counter-search (T-090) |
|---|---|---|
| Friction/forcing functions cut over-reliance but get the worst subjective ratings and tax low-NFC users | CONFIRMED (Buçinca 2021, N=199, d=.37–.57; W3-D) | searched against: no design found that removes the low-NFC tax; effect persists |
| Explanation helps only when it **lowers verification cost** | CONFIRMED (Vasconcelos N=731, −1.74pp; W3-D) | searched against: information-amount explanations null/negative at high difficulty |
| Automation harm concentrates when it crosses the **analysis→action-selection** boundary | CONFIRMED (Onnasch 2014, 18 exps; W2-A) | searched against: no rebuttal found; boundary replicated |
| Enforced action + training beats checklist ticking (ticking ≠ doing) | CONFIRMED (Haynes WHO 1.5→0.8% vs Urbach Ontario null; W2-A/W4a) | searched against: Ontario null is the counter-evidence, and it *supports* the claim |
| Adaptable vs adaptive is a trade-off, neither dominates | CONFIRMED (Kidwell 2012 head-to-head vs Calhoun 2021 review; W3-D) | searched against: reviews flip workload direction — recorded as genuine conflict |
| Conversational/personalized register moves comprehension | CONFIRMED (d=1.55 / the→your d=0.79; W3-E) | searched against: FKGL weak validity (26.4%); teach-back null on hard endpoints |
| Retrieval practice improves retention | CONFIRMED g=.50–.70 | searched against: APA 2024 d≈−0.05 for transfer to merely-related facts; boundaries recorded |
| Incubation aids divergent problems | CONFIRMED (Sio & Ormerod k=117, d=0.29–0.32, divergent d=0.65; W3-C) | searched against: smaller for non-divergent; effect size contested historically |
| Google/offloading degrades memory | CONFIRMED (Risko & Gilbert 2016; GPS→spatial Dahmani & Bohbot) | searched against: Stroop sub-effect FAILED replication (Camerer 2018) — recorded |
| Trust correlates with performance, not vice versa | CONFIRMED (Hancock r̄=.26/d̄=.71; W2-A) | searched against: no reverse-causation evidence found |
| Unstructured LLM autonomy degrades critical thinking | LIKELY (Gerlich 2025 r=−.68; W2-G) | searched against: correlational only; interaction claim UNVERIFIED protocol-native |
| "Method debt" is a real named concept | **UNVERIFIED / GREY** (alias only; W2-B) | searched against: ≥2 peer-reviewed records for *process* debt, none for *method* debt |
| 0.05/0.25 WWC bands are meta-analytic | **REFUTED** — they are stipulated compliance thresholds (W3-A) | counter-search confirmed they are convention, not findings |
| wu wei is passivity | **REFUTED** — it is effortless action aligned with the natural order (T-045 sweep) | corrected on the record |

---

## 5. Fold plan (ranked by value/cost, principle-first, no new gates)

1. **Protocol-own-voice floor** (T-080) — biggest user-visible win; the header + R1–R6 + DTI. No new gate.
2. **Productive-vs-form friction + dose** (T-079) — fixes the one thing the protocol currently gets *backwards*. Gate-card notes at the 5 selection gates.
3. **Dual-layer doctrine + agency rules** (T-086) — names what is already half-built; CONSTITUTION + REFLECT.
4. **Belief-triage ledger + layered cadence** (T-083) — REFLECT/STRATEGY.
5. **Spec-recursion checker test + style-as-spec** (T-085) — SPECIFICATION.
6. **Reframing doctrine** (T-087) — STRATEGY/VALIDATION.
7. **Balance + over-optimization brake** (T-088/T-089) — STANDING_PRINCIPLES.
8. **Logistics naming + amortization** (T-082) — INBOX/EXECUTOR.
9. **Continuation state artifact + brownfield delta specs** (T-081/T-084) — SPECIFICATION/REFLECT.
10. **Counter-search discipline** (T-090) — skills + RULES.
11. **Cross-cutting meta-rule** ("no mechanism without a checker") — STANDING_PRINCIPLES.

Each fold: byte-preserving edit, `protocol-lint.sh` 17/17, CHANGELOG bullet, THOUGHT_LOG disposition (RAW→RESEARCHED→FOLDED), commit + push origin. Folds that add a *checkable* rule get a lint rule only if the check is mechanical (T-085 checker test, T-080 undefined-term proxy as a *tripwire*, not a gate).

---

## 6. Not-to-encode

- No bare "30-second wait" friction (W2-A: dose ranking puts it far below decide-before-reveal).
- No gamification (points/badges/streaks/leaderboards) — already ruled out (T-040/T-042).
- No numeric "max planning" ratio or Boehm 1:10:100 constants (W2-B/W3-C: unrecovered/contested).
- No fixed "just right" threshold (T-062/064: moderators only).
- No method-debt interest-rate constant (W2-B: GREY).
- No heavyweight gate for style (W2-F: style-as-spec → tool rule + review norm).
- No self-congratulation: a run that fails any external check cannot be cited *for* the protocol, only against it (W2-D asymmetric accounting).
- No p-values at small N — emit `INSUFFICIENT-POWER` (W3-A).
- No FKGL as a gate (weak validity); Coh-Metrix direction only.

---

## 7. Open gaps (carried, not blocking)

Primaries/effect-size refinements: Frattini 2025, Gick & Holyoak cells, Sparrow cells, Berberian ms, Crossley Δr, Cepeda grid, Ginns breakdown, HEART ES, Onah/Manzey pooled DOA, Buçinca §4 tables, Zhang/Butz pp curve, AWS stage-protocol contract, BMAD CLI flags, OpenSpec sync algorithm, Kiro CLI V3 matrix + hooks schema. Plus wave-1/2 parked leads. **None change a direction**; wave 3 is the convergence wave.

Two structural gaps worth a future pass: (a) no published *in-vivo* decide-before-reveal RCT (W3-D) — our own pilot would be first-of-kind; (b) the protocol has never been validated on a live run since T-025 (W2-D's MBCC-lite is the proposed instrument).

---

## 8. Provenance

Axis sweeps: `docs/research/{hiccups-deliberate-friction, stability-specialization, universality-continuation, simplify-specify-refactor, generic-logistics-division-of-labour, shishen-yuanshen-automation-authentic-self, reframing-perspective-shifts, plain-language}-2026-10-*.md`. Prior art: `foundation-vs-overlay-2026-10-07.md`, `fun-creativity-2026-10-09.md`, `curse-of-knowledge-2026-10-09.md`, `improve-judgement-2026-10-03.md`, `max-plan-min-exec-2026-10-03.md`, `just-right-perfection-2026-10-03.md`, `project-bench-taxonomy-2026-10-03.md`, `protocol-transcript-2026-10-03.md`, `power-verification-2026-09-30.md`. Worker journal: `.omo/ulw-research/20261012-030217/`. Sources rest on ~120+ search-returned URLs across the 26 workers; full lists in the wave returns.

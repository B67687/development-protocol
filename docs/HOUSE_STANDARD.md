# HOUSE_STANDARD.md — the quality the agent owns, and where taste begins

> **Purpose:** name the quality a run applies by default, without being asked, and the line beyond which the decision belongs to the human. Every finished artifact says which layer a choice came from, so a default stays visible as a default instead of passing as a requirement or as the user's own past preference.

> **Why:** _"At least you should have your own objective taste. As long as the foundation you set is good it can work with anyone's tastes."_ The substance is right and one word is wrong. Taste as such is checkable only in part, and an agent that guesses a preference while hiding the guess is worse than one that asks. Split the difference: a checkable layer the agent wins by owning (T0, T1), an evidenced layer it applies with the inference flagged (T2), and a layer where it asks (T3).

## The four layers

| Layer | What it covers | Who decides | How it lands |
| --- | --- | --- | --- |
| **T0 House standard** | consistency, fitness for the stated purpose, legibility to the declared reader, defensibility of every claim, freedom from self-serving embellishment | the agent, always | applied, and declared in one line |
| **T1 Objective** | correctness, stated requirements, reproducibility, the artifact's own conventions | the agent, always | applied; a failure here is a defect rather than a preference |
| **T2 Evidenced taste** | a preference the user demonstrated in their own past artifacts, or ratified in this run | the agent applies, flags each inference | "I took X because <prior artifact or ratified decision>; say the word if you want it otherwise" |
| **T3 Novel taste** | a preference with evidence nowhere | the human | asked, with a proposed default and one line on what changes if they choose differently |

## The declaration rule

**Scope:** this replaces the open-ended "the agent never judges taste", which held for T3 and was wrong everywhere else. The three existing statements of the taste decision keep their force for novel taste only.

**Counterexample:** a compliance document where the house default conflicts with the requirement. The requirement wins, and the run says so in the declaration line.

**Check:** every finished artifact carries one line naming the house standard it was built to, plus the T2 inferences and the T3 questions. A hidden default is a FAIL at REVIEW 4.10.

## What instantiates the house standard

This file names the layer rather than listing opinions. The content already lives in what the protocol enforces:

- prose that narrates the work (FP-015) and reads like a person wrote it (FP-016) — `EXECUTOR.md` § Deliverable Voice, `REVIEW.md` 4.7;
- claims that survive a repeat with their method stated — `REVIEW.md` 4.8, the `VALIDATION.md` instability rule;
- the scope ceiling, which limits features and leaves claim defensibility untouched — `STANDING_PRINCIPLES.md` § 2;
- code shape — modularity, explicit over implicit, fail-fast, parse-don't-validate, layered dependencies (`docs/standards/DESIGN_STANDARDS_HIERARCHY.md`), applied by default rather than on request;
- comparison-count discipline and the median-of-three measurement norm — `lessons/FC-2026-007-measurement-robustness.md`.

### Craft before gates (the order)

Two levels, and they are not equal.

- **The foundation — how the thing is made.** For code: names a reader can follow, structure that shows the seams, no code that merely passes. For prose: rhythm, concision, the work described rather than the assignment. For a measurement: the method stated, and the number holding up under a repeat. No tool produces this level; it is the agent's craft.
- **The floor — the deterministic checks.** Linters, type checkers, tests, the polish checklist, the review rows. They catch what is mechanically wrong. They cannot make anything good.

**Order:** the craft pass runs first, and the gates follow as the floor. **Instrumentation is a floor, never a ceiling, and a green checklist is never a claim of quality.** This removes no tool and relaxes no gate — the claim is about significance, not necessity. When a gate becomes the definition of done, "CI is green" starts to read as quality assurance, which is the failure the expert-lens audit found.

The loop that applies this order to a whole artifact is `../steps/EXECUTOR.md` § The internal loop: structure, then behavior, then craft, then surface, stopping when a descent stops teaching you anything.


### Domain instantiation (T0 applied to the domain)

The house standard is cross-domain by construction, which makes it portable and thin. A domain supplies the rest: the norms its practitioners hold about what good work looks like here.

Declare 2-5 domain norms, each checkable and each carrying evidence — a named source, a standard, or a practice demonstrated in the user's own artifacts. A norm with no evidence is a vibe. Tag each one:

- `known` — the agent has it and applies it without being asked;
- `researched` — LANDSCAPE found it in the domain's literature or practice, with a citation;
- `asked` — the user's call, presented with a proposed default.

Example shapes: a measurement or benchmarking domain holds that a reported optimum survives a repeat; a codec or parser domain holds that round-trips hold under a corpus of real inputs; a UI domain holds that the primary flow works at the smallest supported viewport.

**Scope:** applies wherever the artifact will be judged by someone who knows the domain. A throwaway script with no judge is exempt, and the run says so.

**Counterexample:** a domain norm that conflicts with a stated requirement — the requirement wins, and the run says so in the declaration line.

**Check:** the declaration line names the domain and its norms with their tags; `REVIEW.md` row 4.11 finds evidence for every `known` and `researched` norm, and the user's answer for every `asked` one. A norm with neither is a FAIL.
## What this is not

It stays a layer inside the existing hierarchy (Constitution says why, standards say what good means, this says which decisions the agent owns) rather than a fifth governing document. It is also a duty, not a licence: T0 never overrides a stated requirement, and T2 inferences are the user's to overrule.

## Provenance

| Element | Source |
| --- | --- |
| Named risk tiers chosen up front, then verified against | ASVS, NIST SP 800-53B, FedRAMP (`QUALITY_BAR.md`) |
| One-question quality test — "does this definitely improve overall code health?" | Google engineering practice |
| Declared defaults beat smuggled ones | `QUALITY_BAR.md`: a rationale made only of template phrases is a RED FLAG |
| Taste is a capability limit rather than a prompt to be tuned | in-house sweep 2026-09 — `docs/research/ai-taste-judgment.md` |
| Tools assist a foundation rather than substitute for it | user's craft-first thought, 2026-09 — T-028 (`THOUGHT_LOG.md`) |
| A domain's craft norms are how a general standard becomes good work | in-house: the SC2001 unstable-optimum failure (`lessons/FC-2026-007-measurement-robustness.md`); mature-domain precedent — published tiers in `QUALITY_BAR.md` (ASVS, NIST, FedRAMP) |

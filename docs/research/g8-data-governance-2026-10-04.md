# G8: Method-Side Data Governance — Specs Govern Data & Analysis Provenance

**Date:** 2026-10-04
**Scope:** development-protocol G8 — method specs as the governance plane for data provenance, lineage, and categorisation
**Sources:** SearXNG web search + extractor reads (OpenLineage ecosystem, AI Alliance OTDI processing, NIDM/W3C PROV, Google Croissant announcement, Data Product Specification)

## 1. Thesis: governance moves to the method side

Data governance fails when it lives only in platform policy docs that no method step reads. G8 inverts this: **the method spec is the governance enforcement point**. Each phase/step declares what data it consumes, what category that data belongs to, what transformations it applies, and what provenance it emits — and downstream steps verify those declarations before proceeding.

Concretely: specs govern (a) **provenance** (where each datum came from, through which steps), (b) **categorisation** (sensitivity, quality tier, intended-use class), and (c) **analysis lineage** (which method version + parameters produced which derived artefact). If it is not in the spec, it did not happen; if it is in the spec, it is auditable.

This mirrors proven industry patterns: OpenLineage emits lineage events from pipeline steps (producers) to governance consumers; OTDI requires full audit trails on every ingestion/transformation step; NIDM binds each neuroimaging artefact to W3C PROV provenance. The method spec plays the "producer" role; governance tooling plays "consumer".

## 2. Provenance foundations: W3C PROV and the NIDM pattern

The W3C PROV family (PROV-DM, PROV-O, PROV-N) remains the canonical vocabulary: **Entities** (data), **Activities** (method steps), **Agents** (who/what ran them), plus derivation/generation/use relations. Domain extensions specialise it rather than reinventing it.

The exemplar is NIDM (Neuroimaging Data Model): a collection of specification documents extending W3C PROV for brain mapping, linking dataset descriptors → computational workflow → derived data → publication in one provenance graph. Key lesson for G8: **extend a standard provenance core per domain** (method-family-specific entity/activity types) instead of inventing a bespoke lineage schema per protocol. Method specs should emit PROV-compatible records (JSON-LD or equivalent sidecar files, cf. `provit`'s `.prov`-next-to-data pattern) so external auditors need no custom parser.

Minimum PROV capture per method step: input entity IDs + hashes, activity ID (method name + version + parameters), agent (executor identity), output entity IDs + hashes, timestamp. Anything less breaks the chain.

## 3. Lineage as spec: the OpenLineage producer/consumer split

OpenLineage formalises what G8 needs architecturally: **jobs/nuns and datasets as first-class spec objects**, with runs emitting structured lineage events. Its ecosystem splits *producers* (Airflow, Spark, Flink, dbt, Great Expectations) from *consumers* (Marquez, DataHub, Egeria, OpenMetadata-adjacent catalogues). Governance is not a meeting — it is a stream of events that consumers index, visualise, and policy-check.

Translated to development-protocol: each method phase is a *job*, each input/output artefact is a *dataset*, each execution is a *run*. The spec declares the expected lineage edges statically ("step S consumes artefact class A, produces artefact class B"); the run log confirms them dynamically. Drift between declared and actual edges is a governance finding, not a footnote.

Adoption rule: keep the static declaration small (inputs, outputs, transform summary) and let run events carry the bulk (row counts, hashes, durations, quality checks). Specs stay readable; logs stay complete.

## 4. Quality tiers as categorisation: the OTDI raw → filtered → structured ladder

The AI Alliance Open Trusted Data Initiative (OTDI) processing model gives G8 its quality-tier template: **Raw** (as-submitted; gating criterion is *unambiguous provenance*), **Filtered** (deduplicated, PII/toxicity/copyright-scrubbed, decontaminated from benchmarks), **Structured** (tokenised/serialised/embedded for training/RAG). Each tier transition is an audited pipeline with published code (Apache-2.0) and per-dataset governance metadata in a public dataset card, composable into AI Bills of Material.

Three portable principles: (1) **admission is provenance-first** — no clear source, no entry, regardless of content value; (2) **each derivation is a named, versioned transform** with enumerated checks (format validity, card-vs-data consistency, licence, PII, duplication, toxicity, contamination); (3) **tiers are labels, not folders** — categorisation travels with the artefact's metadata so downstream consumers can filter by tier without re-deriving it.

For method specs: require a `tier` field on every data reference (raw/filtered/structured or domain equivalent) and a `transforms[]` list on every derivation step naming the applied checks. A step that upgrades a tier without listing its transforms fails spec validation.

## 5. ML-ready metadata: Croissant + RAI vocabulary

Croissant (MLCommons, Google/Dataset Search/Kaggle/Hugging Face/OpenML) shows how to make metadata **machine-actionable**: built on schema.org/Dataset, it adds layers for data resources, organisation, and default ML semantics (train/test/validation splits), plus a 1.0 **RAI vocabulary** extension covering data life cycle, labelling, participatory data, safety/fairness evaluation, explainability, and compliance. Tooling (TFDS, Croissant editor/validator, repository auto-generation) closes the loop between spec and use.

G8 takeaway: categorisation fields must be **enumerated in the spec schema**, not free text. Adopt a Croissant-like layering: discovery layer (title, licence, source), resource layer (files, formats, hashes), organisation layer (splits, joins, derivations), RAI/governance layer (sensitivity, consent basis, allowed uses, retention). Validators run against the schema at spec-check time; editors/generators lower the authoring burden so the schema is actually filled in.

## 6. Contracts and products: Data Contract / Data Product Specification

The Data Contract and Data Product Specifications bring the **producer promise** idiom: schemas, quality SLOs, ownership, `containsPii` flags, access locations, and status, versioned alongside the data. Output ports declare what consumers may rely on; input ports declare what producers require. The data mesh lesson: decentralised ownership works only when each product's contract is machine-readable and CI-checkable.

For method specs: every artefact a phase publishes gets a **mini-contract** — owner, schema/hash, PII flag, licence, freshness expectation, breaking-change policy. Cross-phase references resolve against contracts, not raw paths. A phase may not consume an artefact whose contract it has not declared (dependency allow-list), and may not publish outside its declared output ports. This is how "specs govern data" becomes enforceable rather than aspirational.

## 7. A categorisation taxonomy for method specs

Synthesising the above, G8 proposes four orthogonal categorisation axes — every data/analysis reference in a method spec carries one value per axis:

1. **Provenance class** — `sourced` (external, attributed) / `derived` (produced by a named step+run) / `synthetic` (generated; record generator + seed). Required companion: source URI or deriving run ID.
2. **Quality tier** — `raw` / `filtered` / `structured` (per §4), plus optional `quarantined` for suspect data under review. Tier upgrades require listed transforms.
3. **Sensitivity class** — `public` / `internal` / `confidential` / `restricted` (PII, credentials, benchmark-contaminating, licensed). Drives handling rules: redaction before logging, access gating, retention limits. Mirrors `containsPii`-style flags but as an ordered lattice (a step's outputs inherit the *max* sensitivity of inputs unless an explicit sanitisation transform is declared and verified).
4. **Intended-use class** — `evidence` (cited in decisions), `intermediate` (reproducible scratch), `discardable` (cache; safe to evict). Determines retention and audit depth: evidence-class artefacts require full PROV chains + hashes; discardable ones need only run linkage.

Analysis provenance categorisation mirrors this: each analysis output records method version, parameter digest, input digests, and the tier/sensitivity it claims — enabling re-execution checks and "which conclusions depend on quarantined data?" queries.

## 8. Recommendations and adoption path

1. **Emit PROV-compatible run records from day one** (JSON-LD sidecars next to artefacts). No custom lineage format; extend W3C PROV per method family à la NIDM.
2. **Add the four-axis header to every data reference** in method specs (provenance / tier / sensitivity / use-class). Default-deny: uncategorised references fail validation.
3. **Mandate transform lists on tier upgrades** using the OTDI checklist as the starter vocabulary (dedup, PII strip, licence check, toxicity, decontamination, format validation).
4. **Adopt Croissant-style layered metadata** for shared datasets (discovery/resource/organisation/RAI), with schema validation in CI and editor assistance for authors.
5. **Require mini-contracts on phase outputs** (owner, schema/hash, PII flag, licence) and allow-listed inputs; wire a lightweight lineage consumer (log indexer or Marquez-class store) before adding policy automation.
6. **Non-goals for v1:** no automatic sensitivity inference, no cross-organisation BOM tooling, no retroactive re-categorisation of legacy artefacts beyond marking them `raw/quarantined`.

**Open questions:** exact sensitivity lattice values for this repo's content; where run records live (`.omo/` vs artefact-adjacent sidecars); who signs evidence-class outputs.

### Sources

- OpenLineage ecosystem (producers/consumers, lineage spec model) — https://openlineage.io/ecosystem/
- AI Alliance OTDI, "How We Process Datasets" (provenance-first raw admission; filtered/structured tiers; full auditing; dataset-card governance metadata) — https://the-ai-alliance.github.io/open-trusted-data-initiative/our-processing/
- NIDM (W3C PROV domain extension linking descriptors → workflow → derived data → publication) — https://nidm.nidash.org/
- Google Research, "Croissant: a metadata format for ML-ready datasets" + RAI vocabulary (schema.org-based ML layers; Kaggle/HF/OpenML/TFDS support) — https://research.google/blog/croissant-a-metadata-format-for-ml-ready-datasets/
- Data Product Specification (output/input ports, `containsPii`, versioned product format) — https://dataproduct-specification.com/
- SearXNG searches: "data governance provenance data lineage specification 2025 2026", "W3C PROV data provenance specification OpenLineage SLSA", "OpenLineage data lineage specification standard 2025", "W3C PROV FAIR data principles provenance metadata spec", "Croissant ML dataset metadata format specification", "data contract specification data classification sensitivity governance OpenMetadata" (2026-10-07).

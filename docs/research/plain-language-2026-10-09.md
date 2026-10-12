# Plain Language Axis — Explaining Technical Substance in Everyday Speech Without Breaking Correctness

**Date:** 2026-10-09
**Slot:** Cross-cutting thought (1 of 11) — plain-language axis
**Question:** How should an AI agent communicate technical substance in plain, readable, everyday ("normal person") speech while keeping fundamental correctness intact — when explaining decisions to a non-expert user?
**Method:** `meta-research` fan-out across 5 directions (plain-language standards + numeric effects; jargon/fluency barriers; knowledge curse + expertise reversal; simple-explanation art; correctness trade-off + teach-back) with `authoritative-search` discipline per direction. Infrastructure: SearXNG `http://127.0.0.1:8888` (general category; science category returned empty during this session, so academic coverage came via Tavily fallback) + Trafilatura extractor `POST http://127.0.0.1:8081/extract` (verified working; full-text extraction of Wikipedia curse-of-knowledge, Shulman & Bullock 2020 PLOS ONE, and the ED teach-back cohort study).
**Scope note:** This axis covers *outbound explanation* (agent → non-expert user). The sibling file `curse-of-knowledge-2026-10-09.md` covers the expert–novice gap mechanisms; this file does not re-litigate them, it cites that file where they overlap and goes deep on readability evidence, voice, precision trade-offs, and comprehension testing.
**User constraints honored:** (a) readability must keep fundamental correctness — simplification that silently drops a load-bearing distinction is a defect, not a virtue; (b) "normal person" everyday voice, not merely simplified technical terms — first/second-person conversational speech, not a glossary dump.

---

## 1. Problem Statement

An agent that decides correctly but explains opaquely fails the non-expert user twice: the user cannot verify the decision, and cannot act on it. The failure has two symmetric forms. **Over-complex explanation** (jargon, formal register, unexplained terms) triggers disfluency, resistance, and disengagement even when the content is right. **Over-simplified explanation** (dropped distinctions, vague analogies, "baby talk") preserves comfort while destroying the correctness the simplification was meant to serve. The axis question is therefore a constrained optimization: maximize first-read comprehension in everyday speech subject to the hard constraint that every load-bearing technical distinction survives. The evidence below says this optimum exists, is measurable (comprehension scores, transfer tests, readmission odds ratios), and has known boundary conditions — including one clean case where jargon does *not* hurt.

---

## 2. Field Landscape

Five literatures bear on the axis, and they agree more than their vocabularies suggest:

- **Plain-language standards and law.** The U.S. Plain Writing Act of 2010 requires federal agencies to write "clear Government communication that the public can understand and use," implemented through the Federal Plain Language Guidelines (PLAIIN network); CDC, NIH (Clear Communication), and CMS maintain parallel health-specific guidance. The movement's own scholars (Kimble, Garner, Eagleson, Wydick, DuBay) converge on one operational definition: the audience understands *the first time they read or hear it* — and Eagleson's rider is load-bearing for this axis: plain language "is not baby talk, nor is it a simplified version of language."
- **Health literacy and readability epidemiology.** The stakes literature: how many adults cannot use complex health texts, and what that costs (hospitalization, ED use, mortality). Paasche-Orlow et al., the National Assessment of Adult Literacy, and Berkman et al. supply the headcounts.
- **Jargon / processing-fluency experimental program.** Shulman, Dixon, Bullock, and Colón Amill's series (Journal of Language and Social Psychology; PLOS ONE 2020) isolates the *mechanism*: jargon → disfluency → motivated resistance to persuasion, higher risk perception, lower adoption support — with definitions provided and still failing, and with one crisis-urgency boundary condition.
- **Expert–novice explanation gap.** Camerer–Loewenstein–Weber (1989) coinage; Newton (1990) tappers/listeners; Fischhoff (1975) hindsight anchoring; Birch & Bloom (2007) false-belief sizing; Kalyuga's expertise reversal program (worked examples, split-attention→redundancy). Mechanisms live in `curse-of-knowledge-2026-10-09.md`; only the numbers this axis needs are repeated here.
- **Explanation-as-learning and comprehension-testing.** Dunlosky et al. (2013) on practice testing / elaborative interrogation / self-explanation; Moreno & Mayer on the personalization (conversational-voice) effect; Ginns et al. (2013) meta-analysis of conversational style; the teach-back clinical program (AHRQ Tool 5; Oh et al.; HF meta-analysis; ED cohort; Ha Dinh JBI review) as the only comprehension test with randomized/cohort effect sizes.

---

## 3. Top Findings

**F1. Plain language is a first-read standard with legal force, and it explicitly excludes dumbing down. (CONFIRMED)**
The Plain Writing Act of 2010 defines "plain writing" as "writing that is clear, concise, well-organized, and follows other best practices appropriate to the subject or field and intended audience," and requires it in covered federal documents (services, benefits, requirements, taxes). The Federal Plain Language Guidelines (PLAIN network), CDC's plain-language materials page, and NIH's Clear Communication initiative operationalize it. The scholarly definitions converge on first-read comprehension ("communication your audience can understand the first time they read or hear it"), with Eagleson's constraint that plain language is not baby talk nor a simplified version of the language — i.e., correctness preservation is inside the definition, matching user constraint (a).
Sources: `https://www.dol.gov/general/plainwriting`, `https://www.justice.gov/open/plain-writing`, `https://www.cdc.gov/health-literacy/php/develop-materials/plain-language.html`, `https://en.wikipedia.org/wiki/Plain_language` (extracted via sidecar; definitions + Eagleson quote).

**F2. Roughly a quarter to a third of U.S. adults have limited health literacy — the addressable audience for plain speech is large, and the cost is measured in hospitalizations. (CONFIRMED)**
Paasche-Orlow et al. (2005, J Gen Intern Med; 85 studies, N=31,129): weighted prevalence of low health literacy **26% (95% CI 22–29%)** plus marginal **20% (16–23%)**. The National Assessment of Adult Literacy: only **12%** of adults proficient, **36%** limited (~80 million adults per Berkman framing). Berkman et al. (2011, Ann Intern Med; 96 good/fair studies): low health literacy consistently associated with more hospitalizations, greater ED use, lower mammography/influenza uptake, poorer medication-taking demonstration, poorer label/message interpretation, and (elderly) poorer health status and higher mortality.
Sources: `http://www.ncbi.nlm.nih.gov/pmc/articles/PMC1490053`, `https://pmc.ncbi.nlm.nih.gov/articles/PMC1490053`, `https://www.ovid.com/journals/aime/fulltext/00000605-201107190-00005~low-health-literacy-and-health-outcomes-an-updated` (all via Tavily snippets; effect direction CONFIRMED by ≥2 independent peer-reviewed syntheses).

**F3. Jargon impairs fluency and persuasion even when defined — through metacognition, not just misunderstanding. (CONFIRMED)**
Shulman et al. (J Lang Soc Psych; online experiments, **N=650**): jargon-present messages produced lower processing fluency (**M=4.57 vs. 5.27** no-jargon), and fluency mediated motivated resistance to persuasion (MRP), accounting for **~10% of variance**; downstream: greater MRP, higher risk perceptions, lower technology-adoption support. Crucially, definitions did not rescue fluency — the effect is subtler and broader than "they didn't know the word." Convergent: the PLOS ONE program summary ("jargon-laden messages lead to a more disfluent experience") and the "kills interest / exclusionary language" replication line.
Sources: `https://www.academia.edu/39971919/Jargon_as_a_barrier_to_effective_science_communication_Evidence_from_metacognition`, `https://www.socialsciencespace.com/2020/02/jargon-may-be-even-worse-for-communicating-complexity-than-you-thought`, `https://www.semanticscholar.org/paper/The-Effects-of-Jargon-on-Processing-Fluency%2C-and-Shulman-Dixon/63a4114a83f5b9a8c7a0461c20e4a1059c06fcbc`.

**F4. Boundary condition: under high crisis urgency, jargon's penalty vanishes. (LIKELY — single program, open-access, pre-registered-style reporting)**
Shulman & Bullock (2020, PLOS ONE, **N=393**, three topics varying urgency; full text extracted via sidecar): jargon → harder processing + reduced persuasion for flood-risk (low urgency) and emergency-policy (control) topics, but **no jargon effect in the COVID-19 (high urgency) condition**. Theory: motivation to process (feelings-as-information + ELM) moderates the fluency path. Design import: urgency/motivation is a moderator, not a license — default to plain speech; only high-stakes, high-motivation contexts tolerate necessary terms untranslated.
Source: `https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0239524` (abstract + introduction extracted verbatim via sidecar 2026-10-12; data at `https://osf.io/gb5nc/`).

**F5. Experts overestimate comprehensibility by ~20×; the gap is structural, not effortful. (CONFIRMED)**
Newton (1990, Stanford dissertation): tappers predicted listeners would name the tune **50%** of the time; actual hit rate **3/120 (2.5%)**. Term coined by Camerer, Loewenstein & Weber (1989, J Polit Econ) for the failure of better-informed agents to anticipate less-informed judgments; Fischhoff (1975) hindsight anchoring as the root mechanism (receipt of knowledge anchors the mind; reconstruction of the naive state fails even when attempted). Birch & Bloom (2007, Psych Sci) size the same pattern in false-belief reasoning. For this axis the import is procedural: no explainer — human or agent — can introspect its way to the naive reader's experience; it must test.
Sources: `https://en.wikipedia.org/wiki/Curse_of_knowledge` (extracted via sidecar: coinage, Newton numbers, Fischhoff anchoring), `https://thedecisionlab.com/reference-guide/management/curse-of-knowledge`, `https://ucatt.arizona.edu/news/curse-knowledge`; detail in `curse-of-knowledge-2026-10-09.md`.

**F6. What helps novices harms experts and vice versa — so the agent must pitch to the actual reader, not a generic middle. (CONFIRMED)**
Kalyuga's expertise reversal program: electrical trainees early in training learned better from diagrams with integrated text (split-attention reduction), but after training learned better — with less mental effort — from **diagrams alone**; redundant text became actively detrimental (Kalyuga, Chandler & Sweller 1998; Kalyuga et al. 2001, 2003). General form: Chen, Kalyuga & Sweller (2017) as element-interactivity variant; Oksa et al. (2010) Shakespearean-text replication (embedded modern-English notes helped novices, raised cognitive load for experts). Novice-directed explanation to an expert wastes effort and irritates (Schrader & Kalyuga frustration finding); expert-directed explanation to a novice fails to land. The non-expert user in this axis is the novice case: full scaffolding, integrated definitions, zero assumed schemas.
Sources: `https://mrbartonmaths.com/resourcesnew/8.%20Research/Explicit%20Instruction/The%20Expertise%20Reversal%20Effect.pdf` (Kalyuga primary-program summary), `https://www.tandfonline.com/doi/full/10.1080/01443410.2023.2273762`, `https://my.chartered.college/impact_article/expertise-reversal-effect-and-its-instructional-implications`, `https://theelearningcoach.com/learning/novice-versus-expert-design-strategies`.

**F7. Conversational everyday voice itself improves learning — large transfer effects from tiny wording changes. (CONFIRMED)**
Moreno & Mayer (2000): rewriting the same science content in conversational style (first/second person, direct address — "your cloud is charged up") beat formal textbook style on transfer with a **median effect size d=1.55** across 5 experiments (0.88–1.60). Mayer, Fennell, Farmer & Campbell (2004): merely changing "the"→"your" (11 places in a lungs lesson) improved transfer at **median d=0.79**. Ginns et al. (2013) meta-analysis (16 journals + 4 proceedings + 2 dissertations, **N=3,312**): conversational style rated friendlier and less difficult, with better retention and transfer; interest ruled out as the mechanism (social-agency/task-involvement account). This is the direct warrant for user constraint (b): everyday voice is not decoration, it is a measured learning intervention. Caveat: a 2024 Frontiers replication line finds personalization alone without task involvement may not move outcomes — voice amplifies engagement with content, it does not replace content. (LIKELY as stated; single-study moderator.)
Sources: `https://www.jsu.edu/online/faculty/MULTIMEDIA%20LEARNING%20by%20Richard%20E.%20Mayer.pdf` (Table XXII), `https://alison.com/course/532/resource/file/Chapter_9_Applying_the_Personalization_Principle_-_Use_Conversational_Style_and_Virtual_Coaches.pdf`, `https://www.qic-wd.org/umbrella-summary/conversational-style-training`, `https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2024.1405308/full`.

**F8. Explaining in one's own words is a verified learning mechanism — but the folk "Feynman technique" as a named protocol is practitioner lore. (CONFIRMED components / UNVERIFIED brand)**
Dunlosky et al. (2013) rate **practice testing, elaborative interrogation, and self-explanation** high-to-moderate utility; elaborative-interrogation ("why" explanations) effect sizes range **d≈0.85–2.57**, strongest when elaborations are precise, self-generated, and prior knowledge suffices (Pressley et al. 1987 origin). Retrieval-practice reviews (Roediger & Karpicke 2006; Dunlosky 2013; 2025 primary-school replication) converge: recall-based production beats re-reading for retention and potentiates new learning. The "Feynman technique" label (explain simply → find gaps → refine → analogize) has no canonical peer-reviewed trial under that name — treat it as a packaging of verified components (generation + retrieval + elaboration), not as its own evidence base.
Sources: `https://dev.taleafrica.com/2021/03/29/improving-students-learning-with-effective-learning-techniques-promising-directions-from-cognitive-and-educational-psychology`, `https://pmc.ncbi.nlm.nih.gov/articles/PMC12372469`, `https://thomasjfrank.com/feynman-technique-lpc-method/`, `https://www.todoist.com/inspiration/feynman-technique` (last two: practitioner formulation, community tier).

**F9. Plain redrafting improves comprehension of technical documents — but complex concepts remain hard; language is necessary, not sufficient. (CONFIRMED with ceiling)**
Masson & Waldron (1994, Applied Cognitive Psychology): four contract types (mortgage, property sale, bank loan, lease) redrafted (archaic/redundant words removed, sentences simplified, legal terms defined or replaced; comprehension via yes/no + paraphrase) scored **higher than originals — yet overall performance remained relatively poor**. Charrow jury-instruction studies: same direction, plain versions more comprehensible. Martinez et al.: legalese features (center-embedded clauses worst, passives, rare synonyms, nonstandard capitalization) each depress comprehension. South-African bill comparison: plain version read **7% faster** with **19% more accurate** answers — even lawyers and law students benefited. Design import for the correctness constraint: simplify the *expression*, then check whether the *concept* still needs scaffolding (example, analogy, consequence) — vocabulary swap alone under-shoots.
Sources: `https://www.jbe-platform.com/content/journals/10.1075/idj.7.2.04pen?crawler=true&mimeType=application%2Fpdf`, `https://scispace.com/pdf/implementing-plain-language-into-legal-documents-the-53cn5nalkj.pdf`, `https://escholarship.org/content/qt5k09w2td/qt5k09w2td_noSplash_81ed110913c9bbd81423d6bcf3177002.pdf`, `https://www.law.georgetown.edu/legal-ethics-journal/wp-content/uploads/sites/24/2024/03/GT-GJLE230044.pdf`.

**F10. Teach-back is the only comprehension check here with clinical effect sizes — and it is cheap. (CONFIRMED)**
Protocol (AHRQ Health Literacy Universal Precautions Toolkit, Tool 5): explain → ask the person to repeat back in their own words → re-explain gaps → repeat until accurate; framed as testing *the explanation*, never the person. Oh et al. (J Patient Saf; 5 studies, meta-analyzed 3): discharge education with teach-back vs. usual care, 30-day readmission **OR=0.55 (95% CI 0.34–0.91, p=0.02), I²=0%** — a **45% reduction**. Heart-failure meta-analysis (Patient Educ Couns 2022; 6 studies pooled): **OR=0.40 (95% CI 0.17–0.94)**. ED prospective cohort (n=483; full text extracted via sidecar): comprehension deficit **49% → 11.9%**, return-precautions deficits **41.3% → 8.1%**, benefit persisting at 2–4 days, added time **1 min 39 s** (vs. 3:11 standard interview). Ha Dinh et al. (2016, JBI; chronic disease): positive across outcomes though not always significant; largest gains in low-health-literacy patients. Caveats: small study counts, selection-bias risk, variable follow-up.
Sources: `https://yonsei.elsevierpure.com/en/publications/effectiveness-of-discharge-education-with-the-teach-back-method-o`, `https://www.sciencedirect.com/science/article/abs/pii/S0738399122008254`, `https://link.springer.com/article/10.1186/s12245-020-00306-9` (extracted), `https://cre-ckd.centre.uq.edu.au/files/702/a2016_JBI_The%20effectiveness%20of%20the%20teach-back.pdf`, `https://www.ahrq.gov/health-literacy/improve/precautions/tool5.html`, `https://cbrhl.org.au/health-services-providers/communicating-with-consumers/teach-back/`.

---

## 4. Design Principles

**P1. Plain-first sentence, necessary term paired immediately (define-then-use).**
Open with the everyday-speech version, then attach the exact term in the same breath: *"Your data stays on your machine — what engineers call local-first."* Rationale: F3 (undefined jargon depresses fluency even when a definition exists *somewhere*); F6 (novices need integrated, not separated, support); F9 (term replacement alone under-shoots — the plain paraphrase carries the concept, the term preserves precision). Never use a necessary term twice before defining it once.

**P2. Everyday voice, not simplified terminology (second person, conversational, short sentences).**
Address the user as a participant ("you"), use contractions and direct address, prefer short active sentences, read the draft aloud as the register test. Rationale: F7 (d≈0.8–1.5 transfer gains from voice changes alone); F1 (first-read standard). Floor: conversational register must never smuggle in false concreteness — no analogy presented as identity ("it's exactly like…") unless the mapping is exact; mark analogies as partial.

**P3. Layered precision: plain summary up front, exact distinctions preserved underneath, never silently dropped.**
Structure every technical explanation as: (1) one-sentence plain verdict, (2) the 2–3 sentence everyday version, (3) the precise terms/definitions, (4) pointer to full detail. A simplification that deletes a distinction the decision depended on is scored as incorrect output, per user constraint (a). Rationale: F9's ceiling (concepts stay hard after vocabulary swaps — scaffolding must be added, not just words removed); F4 (precision has a legitimate function; keep it, but earn it with the plain layer first).

**P4. Teach-back gate for high-stakes explanations (verify, don't assume).**
After explaining a decision with material consequences, close with a teach-back prompt that tests the *explanation*: "Just to check I explained that clearly — could you tell me in your own words what happens if you choose X?" If the user's replay misses a load-bearing point, re-explain that point only and re-check. Rationale: F5 (self-estimated clarity overstates by ~20× — introspection cannot substitute for a check); F10 (deficit 49%→11.9% at ~100 seconds; OR 0.40–0.55 on hard outcomes). Reserve for high stakes; routine confirmations stay lightweight (F10's feasibility finding cuts both ways — cheap enough to use, cheap enough to overuse).

**P5. Anti-curse pre-check: predict-then-test, never introspect.**
Before finalizing any non-expert explanation, the agent must run one check it cannot pass by thinking: a readability proxy (grade-level formula as tripwire only, never as target), a necessary-term audit (every term a non-expert wouldn't know is defined or linked), and — where available — an actual comprehension probe (teach-back, P4). Rationale: F5 (naive-state reconstruction fails structurally); readability-formula critique (formulas score surface features, not understanding — useful alarm, false idol; cf. Mazur/context critique in legal-plain-language literature).

---

## 5. Not-to-Encode

1. **Grade-level score as a target.** Readability formulas (Flesch-Kincaid et al.) measure word/sentence length, not comprehension; optimizing to a number produces short-but-opaque text. Use as tripwire, never as success metric.
2. **Glossary-dump as plain language.** Appending definitions does not rescue fluency (F3: defined jargon still depresses processing). Define-then-use inline (P1) or do not use the term.
3. **Jargon elimination absolutism.** Precision terms have a legitimate function (F4's crisis finding; legal-precision debate); the rule is *earn each term* (plain paraphrase + paired definition), not *ban all terms*.
4. **Analogy-as-identity.** "It's exactly like X" for partial mappings — the leading cause of confident misunderstanding. Mark every analogy's breaking point in the same message.
5. **Folk-Feynman branding as evidence.** Do not cite "the Feynman technique" as a validated protocol; cite its verified components (retrieval, elaboration, self-explanation) per F8.
6. **Teach-back for everything.** Blanket teach-back on trivial confirmations breeds fatigue (cf. G9 question-fatigue dynamics); gate it to material-consequence explanations (P4).
7. **Formal register as "professionalism."** Third-person passive bureaucratese is measured worse on retention and transfer (F7); professionalism here = clarity + correctness, not stiffness.
8. **One-size explanation.** Same text for novice and expert readers violates F6 in both directions; pitch to the actual user, layer the rest (P3).
9. **Self-assessed clarity.** "This is clear" by introspection is the Newton error (F5); clarity claims require a probe or a proxy, never a feeling.
10. **Vocabulary-swap-only simplification.** F9's ceiling: swapping words without adding scaffolding (example, consequence, contrast) leaves comprehension poor. Simplification = paraphrase + structure + check.

---

## 6. Open Gaps

1. **AI-agent explanation comprehension: no direct trials.** All effect sizes here come from human→human (clinical, classroom, lab) settings. No retrieved study measures non-expert comprehension of *LLM-generated* explanations specifically (length, hedging, and sycophancy confounds unstudied). Next: controlled comparison of layered (P3) vs. single-register agent explanations on decision-verification tasks.
2. **Everyday-voice calibration for agents.** Personalization effects (F7) were measured on fixed lesson scripts; optimal conversational intensity for an interactive agent (contractions? humor? second person density?) has no dose–response curve. Risk of overshoot into false intimacy unmapped.
3. **Science-category retrieval gap.** SearXNG `science` returned empty for all plain-language queries this session; academic coverage depended on Tavily fallback. Several load-bearing claims (F2 pooled prevalences, F7 effect tables) rest on search-result snippets of peer-reviewed work rather than sidecar-extracted full text — flagged per claim; re-verify against full text before encoding thresholds.
4. **Readability-vs-correctness trade-off frontier.** F9 establishes the ceiling (language necessary, not sufficient) but no study found here quantifies *how much* precision can be preserved at each reading level for a fixed concept — the Pareto curve this axis ultimately needs is unmapped.
5. **Teach-back transfer to conversational UI.** Clinical teach-back is synchronous, high-authority, and verbal; its ORs may not transfer to a text agent users can ignore. Interaction design for non-coercive teach-back (F10 + question-fatigue constraints) is an open design problem.
6. **Non-English plain language.** Entire sweep is English-law/English-literacy; plain-language standards, fluency effects, and teach-back protocols in other languages (and the agent's cross-lingual explanation behavior) untouched.
7. **Long-term retention of agent explanations.** All comprehension measures here are immediate-to-days; whether plain-layered agent explanations produce durable mental models (vs. transient compliance) unstudied.

---

## 7. Sources

*Plain-language standards & law:*
- Federal plain-language practice (DOL): `https://www.dol.gov/general/plainwriting`
- DOJ plain-writing commitment + Federal Plain Language Guidelines reference: `https://www.justice.gov/open/plain-writing`
- CDC plain-language materials & resources: `https://www.cdc.gov/health-literacy/php/develop-materials/plain-language.html`
- Plain language definition + scholar definitions (sidecar-extracted): `https://en.wikipedia.org/wiki/Plain_language`
- FMC Plain Writing Act compliance trail: `https://www.fmc.gov/about/plain-language-guidelines-plan-and-report`
- Clear and Concise Content Act / 2010 Act limits: `https://www.michbar.org/journal/Details/The-Clear-and-Concise-Content-Act-of-2023-Another-step-toward-plain-writing-in-the-federal-government?ArticleID=4771`
- Arizona health-literacy guide (plain-language practice, 3rd–5th grade guidance, CDC Everyday Words, SAM instrument): `https://azhin.org/cummings/healthliteracy`

*Health-literacy epidemiology:*
- Paasche-Orlow et al. 2005, prevalence (85 studies, N=31,129; 26%/20%): `http://www.ncbi.nlm.nih.gov/pmc/articles/PMC1490053` and `https://pmc.ncbi.nlm.nih.gov/articles/PMC1490053`
- Berkman et al. 2011, outcomes systematic review (~80M; hospitalizations/mortality): `https://www.ovid.com/journals/aime/fulltext/00000605-201107190-00005~low-health-literacy-and-health-outcomes-an-updated`
- NAAL / intervention-consequences review: `https://pdfs.semanticscholar.org/cb56/08050ab1c30c9d13e9e6d96a7ee7457c5905.pdf`
- Visual-interventions comprehension meta-analysis: `https://pmc.ncbi.nlm.nih.gov/articles/PMC11165863`

*Jargon / fluency:*
- Shulman et al., jargon-as-barrier via metacognition (N=650; 4.57 vs 5.27; 10% MRP variance): `https://www.academia.edu/39971919/Jargon_as_a_barrier_to_effective_science_communication_Evidence_from_metacognition`
- Shulman et al., JLSP program summary: `https://www.socialsciencespace.com/2020/02/jargon-may-be-even-worse-for-communicating-complexity-than-you-thought`
- Shulman et al., JLSP record: `https://www.semanticscholar.org/paper/The-Effects-of-Jargon-on-Processing-Fluency%2C-and-Shulman-Dixon/63a4114a83f5b9a8c7a0461c20e4a1059c06fcbc`
- Shulman & Bullock 2020, crisis boundary (N=393; sidecar-extracted): `https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0239524` (data: `https://osf.io/gb5nc/`)
- PMC mirror of the PLOS paper: `https://pmc.ncbi.nlm.nih.gov/articles/PMC7540871`
- Workplace-jargon extension: `https://journals.sagepub.com/doi/10.1177/23294884251364525`
- Popular summary (exclusionary-language finding): `https://www.advancedsciencenews.com/jargon-kills-peoples-interest-in-science`

*Curse of knowledge / expertise reversal (mechanisms in sibling file):*
- Curse of knowledge (sidecar-extracted: 1989 coinage, Newton 50%-vs-2.5%, Fischhoff anchoring): `https://en.wikipedia.org/wiki/Curse_of_knowledge`
- Practitioner summary with Newton numbers: `https://thedecisionlab.com/reference-guide/management/curse-of-knowledge`
- UCATT replication of Newton numbers: `https://ucatt.arizona.edu/news/curse-knowledge`
- Kalyuga program summary (1998/2001/2003 experiments): `https://mrbartonmaths.com/resourcesnew/8.%20Research/Explicit%20Instruction/The%20Expertise%20Reversal%20Effect.pdf`
- Worked-examples transfer analysis: `https://www.tandfonline.com/doi/full/10.1080/01443410.2023.2273762`
- Practitioner synthesis (Chartered College): `https://my.chartered.college/impact_article/expertise-reversal-effect-and-its-instructional-implications`
- Design synthesis: `https://theelearningcoach.com/learning/novice-versus-expert-design-strategies`
- Annoying-feedback / frustration angle: `https://icenet.blog/2023/04/20/expertise-reversal-and-annoying-feedback`
- Substack synthesis: `https://idtips.substack.com/p/the-expertise-reversal-effect-when`
- Sibling repo file: `curse-of-knowledge-2026-10-09.md`

*Everyday voice / explanation-as-learning:*
- Mayer multimedia-learning summary, Table XXII (personalization d median 1.55): `https://www.jsu.edu/online/faculty/MULTIMEDIA%20LEARNING%20by%20Richard%20E.%20Mayer.pdf`
- Personalization-principle chapter (Mayer et al. 2004 d=0.79): `https://alison.com/course/532/resource/file/Chapter_9_Applying_the_Personalization_Principle_-_Use_Conversational_Style_and_Virtual_Coaches.pdf`
- Ginns et al. 2013 meta-analysis umbrella (N=3,312): `https://www.qic-wd.org/umbrella-summary/conversational-style-training`
- Practitioner summary (40% framing): `https://21stcenturystem.academy/personalization.html`
- Task-involvement moderator (2024): `https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2024.1405308/full`
- Dunlosky-line elaborative interrogation (d 0.85–2.57): `https://dev.taleafrica.com/2021/03/29/improving-students-learning-with-effective-learning-techniques-promising-directions-from-cognitive-and-educational-psychology`
- Retrieval-practice replication: `https://pmc.ncbi.nlm.nih.gov/articles/PMC12372469`
- Feynman-technique practitioner formulations (community tier): `https://thomasjfrank.com/feynman-technique-lpc-method/`, `https://www.todoist.com/inspiration/feynman-technique`

*Correctness trade-off / legal plain language:*
- Masson & Waldron ceiling (plain helps, remains poor): `https://www.jbe-platform.com/content/journals/10.1075/idj.7.2.04pen?crawler=true&mimeType=application%2Fpdf`
- Plain-into-legal-documents survey (Charrow, Mazur critique): `https://scispace.com/pdf/implementing-plain-language-into-legal-documents-the-53cn5nalkj.pdf`
- Martinez et al. legalese impenetrability (center-embedded clauses): `https://escholarship.org/content/qt5k09w2td/qt5k09w2td_noSplash_81ed110913c9bbd81423d6bcf3177002.pdf`
- Georgetown plain-in-law review (7% faster, 19% accuracy): `https://www.law.georgetown.edu/legal-ethics-journal/wp-content/uploads/sites/24/2024/03/GT-GJLE230044.pdf`
- Wogalter et al. adequacy factors: `https://www.safetyhumanfactors.org/wp-content/uploads/2020/07/168)Wogalter,Howe,Sifuentes,Luginbuhl(1999).pdf`

*Teach-back:*
- AHRQ Toolkit Tool 5: `https://www.ahrq.gov/health-literacy/improve/precautions/tool5.html`
- Oh et al., 30-day readmission review (OR 0.55): `https://yonsei.elsevierpure.com/en/publications/effectiveness-of-discharge-education-with-the-teach-back-method-o`
- Heart-failure meta-analysis (OR 0.40): `https://www.sciencedirect.com/science/article/abs/pii/S0738399122008254` and `https://yonsei.elsevierpure.com/en/publications/effectiveness-of-discharge-education-using-teach-back-method-on-r`
- Surgical-consent review (45% reduction citation): `https://link.springer.com/article/10.1186/s13037-022-00322-z`
- ED cohort (49%→11.9%; sidecar-extracted): `https://link.springer.com/article/10.1186/s12245-020-00306-9`
- Ha Dinh et al. 2016 JBI review: `https://cre-ckd.centre.uq.edu.au/files/702/a2016_JBI_The%20effectiveness%20of%20the%20teach-back.pdf`
- Practitioner pages: `https://cbrhl.org.au/health-services-providers/communicating-with-consumers/teach-back/`, `https://www.healthliteracyplace.org.uk/toolkit/techniques/teach-back/`, `https://www.ruralhealthinfo.org/toolkits/health-literacy/2/organization/patient-centered-communication`

---

## 8. Provenance & Next Step

**Retrieval log.** SearXNG `http://127.0.0.1:8888` reachable; `general` category returned sparse/low-relevance hits and `science` returned empty for every academic query (5/5 directions), so per skill cascade-discipline the academic load shifted to Tavily (`advanced` depth): 8 queries total (plain-language/health, jargon/fluency, curse/expertise-reversal, teach-back outcomes, personalization, legal plain language, elaborative interrogation, health-literacy prevalence). Trafilatura sidecar `http://127.0.0.1:8081/extract` verified working with verbatim full-text extraction of 3 pages (Plain language Wikipedia probe, Curse of knowledge Wikipedia, Shulman & Bullock 2020 PLOS ONE, ED teach-back Springer article — 4 fetches, 3 load-bearing). No fabricated URLs: every link above was returned by a search result or extracted live; DOIs are cited inline only where the resolving page was seen.
**Confidence discipline.** CONFIRMED = ≥2 independent peer-reviewed sources agreeing (F1, F2, F3, F5, F6, F7, F9, F10); LIKELY = single peer-reviewed program or one peer-reviewed + community (F4); UNVERIFIED = community-only (folk-Feynman brand in F8). Snippet-based peer-reviewed claims (F2 prevalences, F7 effect tables) are marked for full-text re-verification before any numeric threshold is encoded.
**Sibling-file relation.** `curse-of-knowledge-2026-10-09.md` owns the expert–novice gap mechanisms (Camerer origin, Newton, egocentrism, Birch, hindsight taxonomy, Feynman-component mapping); this file owns readability evidence, voice, precision layering, and teach-back. Overlap is by citation, not duplication.
**Next step.** The proposed INDEX row (below) is NOT applied — per instructions `docs/research/INDEX.md` was read, not touched, and no git command was run. Remaining work for the protocol author: (a) full-text verify F2/F7 numbers, (b) run the Pareto-gap study in §6.4 before encoding any simplification threshold, (c) draft the teach-back prompt pattern for high-stakes explanations as a candidate protocol fragment.

---
title: "Independent evaluation of categorical semantic modeling in the software-design substrate"
date: "2026-10-11"
model: "OpenAI GPT-6"
review_type: "architecture/semantics review of user-uploaded snapshot"
status: "EVALUATION — not an implementation verification"
prov:wasGeneratedBy: "Read primary files from four user-supplied archives; inspected selected source; ran an independent illustrative Python fixture"
prov:wasAttributedTo: "OpenAI GPT-6"
prov:used:
  - "software-design-substrate-2026-10-11(3).zip"
  - "categorical-design-substrate-2026-10-11(1).zip"
  - "specodelic-main(2).zip"
  - "wanna-main(1).zip"
source_git_commit: "UNKNOWN — uploaded archives do not establish repository Git HEAD"
software_design_archive_sha256: "54af2c389fb346e6580199b74084365ebef6643473f14b921d92daf3c97a9486"
categorical_archive_sha256: "08cbd5752370c17daaa290004df6bdd98d8f11bfe0fea18e884250eab61ecaba"
specodelic_archive_sha256: "040d0f3673adf7fd084d40bb0b03473c9d1548530bad293bf4d7862dd2a1ac69"
wanna_archive_sha256: "aeb3cb57f4f79f33b49141263d3d8d24d433d66a631407b282e89e651a1e9569"
---

# TL;DR

**Assessment:** The categorical model M1 is a sound *direction for structuring semantic obligations*, not yet a usable *operational semantics for designing software*. It separates presentations, theories, interpretations, and evidence; formalizes the institution satisfaction condition; and refuses to confuse graph structure, traces, and checking with proofs. Those are strong conceptual safeguards. Its main weaknesses are (1) no small, executable domain language for actual architecture contracts and design moves; (2) category-theoretic generality arriving before any working interpretation; (3) missing first-class assume–guarantee/interface and change semantics; (4) insufficient demonstration of translation/refinement; and (5) uncertain fit to the five-kind Specodelic schema. **Recommendation:** keep M1 as a *semantic meta-contract*, implement a narrow software-design semantic profile with explicit model and obligation semantics, test the ten scenarios, and only then stabilize/extract a shared kernel.

**Integrity:** All results below are analysis of supplied snapshots. No `spk lint`, `spk conform`, `cargo test`, `npm test`, or current-upstream comparison was run. A separate small Python trace-inclusion fixture *was* run and passed its local assertions; it is not Specodelic or Wanna code and cannot establish compatibility with either.

## 1. What is being evaluated

The target is a persistent human/agent design workspace that supports provisional sketches; top-down, bottom-up and exploratory refinement; semantic zoom; modularity and Open/Closed Principles (OCP) under declared change scenarios; model-only behavior and simulation; independent oracles; implementation tracer bullets; authority/recovery; and provenance. The baseline defines DS-001–DS-028 and E01–E10. The newer categorical dossier defines proposed SEM-001–SEM-028 and claims an **institution** `(Sig, Sen, Mod, ⊨)` as the logical core, with a structural functor/ACSet-like base and separate evaluators/judgments. Its own README explicitly marks the design as *unimplemented*.

**Context/source anchors** (paths are within the supplied archives; line references are counted from file starts):

- Baseline: `software-design-substrate/{DESIGN-ACTIVITY.md,ARCHITECTURE.md,REQUIREMENTS.md,SCENARIOS.md}`; especially `REQUIREMENTS.md:15–44` and `ARCHITECTURE.md:27–75`.
- Categorical model: `categorical-design-substrate/{SEMANTICS.md,SPECIFICATION.md,SCENARIO-MATRIX.md,ARCHITECTURE-DECISION.md,RULE-OF-5.md}`; especially `SEMANTICS.md:18–76`, `SPECIFICATION.md:18–49`, `ARCHITECTURE-DECISION.md:18–81`.
- Specodelic snapshot: `Cargo.toml:3` declares **0.8.0**. `src/packs.rs:38–45,531–545,659–666` explicitly documents registered pack checkers as declaration-only/empty checked-set. `src/acset/{schema.rs,instance.rs}` implements a fixed five-kind structural schema and stores partial reference vectors; `src/model_check.rs:95–111,242–247` distinguishes model exploration from checked invariants; `src/conform.rs:34–68,481–505` names five conformance verdicts. These are inspected *source facts*, not results of executing the CLI.
- Wanna snapshot: `README.md` describes 30 capability specs, two pilot machine implementations, and a composition-shell/review-workbench prototype; `src/event-envelope/types.ts:4–65` declares typed events/rejections; `src/human-authority/types.ts:4–123` defines authority records; `src/composition-shell/types.ts:39–99,123–171` defines revision-aware persistence/commit outcomes; `docs/reuse-comparison.md` candidly reports inconclusive marginal-LOC evidence. Its reported tests have not been reproduced here.

## 2. Overall fit by dimension

| Dimension | Judgment | Basis |
|---|---|---|
| Separation of structure / semantics / evidence | **Strong design** | Explicit raw/validated/theory/judgment boundaries in M1 |
| Formal core coherence | **Promising, incomplete** | Correct institution satisfaction equation; no concrete `Sig`, `Sen`, `Mod` or executable translation |
| Human design activity | **Under-modeled** | Design verbs and alternatives sit largely outside semantic interpretation |
| Component interfaces and obligations | **Under-modeled** | Generic typed relations lack explicit assume–guarantee/port/effect semantics |
| Hierarchy, abstraction and refinement | **Well specified as obligation, not implemented** | Explicit observation maps, non-vacuity safeguard, missing worked engine |
| Partiality/conflict handling | **Good principle, incomplete policy** | Open/closed-world scope named; no supported finite representation chosen |
| Semantic composition | **Correctly cautious, not operational** | Structural pushout separated from behavioral composition; no chosen v1 operator |
| Evidence, oracles and trust | **Strong direction** | Provenance and bounded/exploratory distinctions; invalidation semantics unresolved |
| Specodelic integration | **Architecturally plausible, unverified** | Five-kind adapter boundary; packs do not execute general semantic checks |
| Wanna integration | **Plausible, partially concrete source infrastructure** | Authority, typed events, revision conflicts, continuity already represented |
| Language/paradigm portability | **Conditional aspiration** | Requires explicit preservation adapters, not shared syntax/category labels |

## 3. Findings (ranked by architectural impact)

### F01 — HIGH: M1 defines a metalanguage, not the meaning of software design

An institution specifies how languages, models, translations and satisfaction relate. It does not by itself distinguish `decomposes`, `invokes`, `provides port`, `satisfies contract`, `observes behavior`, `implements capability`, or `decision supersedes decision`. Baseline `Thing/Relation/Region/Assertion` are storage candidates, not a semantic theory for those relation types. **Consequence:** a well-formed graph can still embody nonsensical architecture; a user can record claims but may not know which assertions are testable. **Fix:** define a small executable architecture profile with typed roles, ports, dependency direction, environmental assumptions, guarantees, effects, and observables. Keep more general interpretations optional.

### F02 — HIGH: The generalized institution core is not an executable v1 contract

`M′ ⊨ Sen(σ)(φ) ⇔ Mod(σ)(M′) ⊨ φ` is the correct institutional satisfaction condition, but neither its truth nor the existence of a useful evaluation algorithm follows from stating it. For finite test cases, property-based tests can detect broken translations, not prove the law for all eligible models. The draft's `translate(...) -> SatisfactionLawObligations` is conceptual and underspecifies failures (undefined map, version mismatch, partial translation, unmet preservation). **Fix:** choose v1 signatures and sentences explicitly, give denotations/evaluation rules, and prove or exhaustively verify the finite translation cases being promised. Return `Unsupported` rather than asserting abstract portability.

### F03 — HIGH: Interface contracts and assume–guarantee reasoning are missing

E01 (storage atomicity conflicts with parent promise), E05 (new backend without changes to stable core), E09 (boundary/coupling) and model tracer slices need more than typed arrows. A basic component contract can be a tuple `(Ports, Assumptions, Guarantees, Effects, Observations)`. Its composition checks not only wiring and type compatibility but whether environment/peer guarantees discharge each component's assumptions, and which global guarantee follows. Non-vacuity is necessary here too: an impossible assumption can make implication trivially true. **Fix:** use a deliberately narrow assume–guarantee or design-by-contract semantics as the first testable design domain.

### F04 — HIGH: Structural formalism cannot be copied into a general kernel unmodified

Specodelic's `src/acset/instance.rs` stores a `Vec<Vec<Value>>` of typed references, where missing targets are `None`, not a plain total function for each schema generating arrow. Even a well-typed optional or multi-valued relation need not be a total Set-valued functor on exactly the claimed `K`; its cardinality/partiality must be modeled explicitly, e.g. relational incidence, option-lifting, or a schema with link objects. M1 flags dirty raw instances, which is good, but *validation alone* does not turn all optional/multi-valued references into ordinary total functions. **Fix:** define the semantic structural carrier and its cardinalities independently, test round-trip and category laws, and use Specodelic's ACSet-like implementation as a possible adapter, not a guaranteed universal foundation.

### F05 — HIGH: Operations that transform designs lack semantics

The baseline is expressly about **the activity of designing**, not static models. Split, merge, propose decomposition, promote a hypothesis into a commitment, propagate contract obligations, reject/retain alternative, and revise a parent claim should have distinct preconditions, effects and pending proof obligations. M1's abstract "typed transformations" lacks concrete transformation signatures, deltas and failure behavior. **Fix:** define a small edit algebra on versioned candidate theories, including speculative branches, refusal, conflict, preservation/invalidation, and review authority. The UI gesture is merely a presentation of an edit intent; Wanna can own authorization/recovery at the effect boundary.

### F06 — HIGH: Abstraction/refinement is scoped sensibly but not demonstrated

Same-signature inclusion and observation-preserving trace inclusion are valid candidate relations for *specific* property classes. Explicitly handling empty concrete model classes is essential. Yet nothing executable maps the E02 payment example to the same abstract observables, and trace inclusion does not establish fairness, divergence freedom, deadlock freedom or substitution in all contexts. **Fix:** first implement finite labeled transition systems with explicit observation/hiding mapping and one chosen simulation/refinement method, returning bounded scope and counterexample traces. Require initial-state correspondence and meaningful concrete models; add stronger semantics only when justified.

### F07 — MEDIUM-HIGH: Language/paradigm portability goes beyond signature renaming

M1's `Sig` morphisms operate *within one institution*. Actors, dataflow, functional semantics and stateful protocols may use genuinely different sentence/model categories. They may require morphisms/comorphisms **between** institutions, or simpler partial translations into a common observational contract with an explicit loss report. A shared serialization format does not settle the issue. **Fix:** keep adapters partial and versioned, test round-trip on meaningful guarantees, and declare which safety/effect/ordering properties survive translation.

### F08 — MEDIUM-HIGH: Evidence invalidation needs semantic dependency granularity

A model digest alone is not enough. A verdict depends on assumptions, environment/model completeness, abstraction map, claim, subject revision, chosen evaluator/version, bounds and oracle interpretation. An unchanged claim may need rechecking after a profile semantics upgrade; a harmless layout edit must not invalidate it. **Fix:** create per-obligation dependency fingerprints, including semantics version, not just global model hash; record stale because of *which* dependency changed. Keep independent evidence origins, but do not confuse provenance labels with statistical/logical independence.

### F09 — MEDIUM: Incompleteness requires two orthogonal treatments

`missing` from a declared transition relation differs from `unknown whether legal`, and both differ from positive and negative claims that conflict. M1 properly warns against assuming a closed world. It should choose an explicit local closure policy and separately record support/refutation of claims (possibly four knowledge states: neither, supported, refuted, both). This is *not* a demand to adopt a universal four-valued logic. **Fix:** a partial behavioral carrier with positive, negative and closed-scope declarations, plus a truth-maintenance ledger of typed claims/evidence.

### F10 — MEDIUM: Human interaction is a separate concern, but not semantically irrelevant

Wanna already defines typed approval and event envelopes with revisions, duplicate/stale detection and uncertain-effect recovery. Reusing it for semantic commit/review is preferable to inventing human-facing authorization rules inside the institution. However approval transitions and approved design revisions must be transactionally bound; an authorized but stale revision is not a valid semantic commit. **Fix:** shared ID/version/effect protocol at the boundary, while the design core remains host-neutral.

### F11 — MEDIUM: Specodelic integration is narrower than the dossier's conceptual API

Specodelic's five base structural kinds are Intent/Constraint/State/Transition/Property. Profiles can declare additional section vocabulary but do not automatically supply executable semantic interpretation or checker proofs. Source `packs.rs` explicitly advertises the checker set as empty. Exporting architectural boundaries, partial sketches and alternatives as native kinds would silently overstate integration. **Fix:** implement a loss-reporting adapter, preserve extras in design metadata, and use registered named evaluators for semantics. Incubate within Specodelic only if semantic API imports no Markdown/CLI document types; test a second consumer before extraction.

### F12 — MEDIUM: Formal rigor may raise rather than lower design friction

The envisioned system needs people to form falsifiable partial theories quickly. Requiring complete signatures, translations and proof objects before they can sketch would undermine DS-001/004/006/026. **Fix:** progressive fidelity: sketch → typed relation → contract → model → judgment, with pending obligations and no false promotion. Evaluate usability in terms of time to the first falsifiable tracer slice and recovery from contradictions, not the number of formal concepts exposed.

## 4. Correct minimal layering

```text
Canvas / text / agent / TUI                          Specodelic authoring / CLI
          |                                                 |
          +------------- typed adapter/projection ----------+
                                  |
                    VERSIONED DESIGN WORKSPACE
                  proposals, alternatives, deltas,
                  design commitments, decisions
                                  |
                 SOFTWARE DESIGN SEMANTIC PROFILE
                entities / boundaries / ports /
              assumptions / guarantees / effects /
             behavior / abstraction / change policies
                                  |
                SEMANTIC CONTRACT / INTERPRETATIONS
               typed signatures, denotations, translation
                                  |
                  EVALUATORS AND EVIDENCE JUDGMENTS
           simulation | bounded checking | oracle | runtime
                                  |
               scoped verdicts + dependency/provenance

Wanna supplies independent authorization, interaction workflow and recovery
at the workspace/effect boundary; it does not define software behavior.
```

The foundational *mathematical* framework may use an institution; the first public *operational* API should be narrower. A candidate design profile might expose these types, without claiming they are already implemented:

```text
Entity(id, role?, boundary?, revision)
Interface(id, ports, actions, effects, assumptions, guarantees)
Claim(id, subject, sentence, scope, status, provenance)
Behavior(id, initial, positive_steps, exclusions, closure_scopes)
Abstraction(id, concrete, abstract, observation_map, relation, obligations)
DesignDelta(id, base_revision, operations, preconditions, affected_claims)
Judgment(id, obligation, model_digest, semantics_version, evaluator,
         method, bounds, verdict, dependencies, evidence_origin)
```

These are candidate *concepts*, not a mandate for seven top-level storage tables; some can be relation instances or projections. `View` remains separate from meaning. A `Scenario` should be an evidence/experiment input with provenance, not automatically an assertion about all possible behaviors.

## 5. Category theory: precise role

Retain category theory where it gives a specific law or compositional benefit:

1. **Typed structural graphs / attributed sets:** capture well-formed references and identity-preserving rewrites with correct treatment of optional/multiple links.
2. **Theories and satisfaction:** make explicit what language claims mean and when an interpretation satisfies them.
3. **Theory morphisms / institution condition:** verify *declared* preservation under supported vocabulary mappings.
4. **Refinement/simulation:** define how an implementation/design maps into abstract observations, then prove the scoped preservation relation supported by the chosen checker.
5. **Composition operators:** combine interfaces and models with explicit semantics; a structural pushout alone is not a behavioral proof.

Do **not** make every UX operation, approval, work session, or project history entry a categorical object merely for uniformity. Do **not** claim a category-theory construct verifies architecture quality without operational rules and evidence.

## 6. Ten-scenario assessment

The following are **design-coverage judgments**, not executed compatibility tests.

| Case | Coverage today (in proposal) | Missing decisive semantics |
|---|---|---|
| E01 top-down/bottom-up conflict | Partial | Contract implication, assumption discharge, conflict resolution edit |
| E02 semantic zoom | Partial | Observable mapping + executable refinement/negative oracle |
| E03 model-only tracer | Partial | Concrete transition/replay semantics + scoped closure |
| E04 independent oracle | Fairly well modeled | Independent oracle ingestion and mismatch protocol |
| E05 OCP backend extension | Partial | Declared extension class, stable region, change semantic policy |
| E06 geometry vs meaning | Well modeled in principle | Reproducible semantic identity/digest tests |
| E07 implementation tracer | Partial | Runtime event adapters, coverage loss and provenance |
| E08 human interaction/recovery | Good boundary choice | Atomic connection between approved effects and design revisions |
| E09 modularity not visual grouping | Correct negative rule | Positive modularity test and dependency-completeness contract |
| E10 model check vs conform | Strong distinction | Evidence lattice/reconciliation for simultaneous contrary findings |

## 7. Independently executed illustrative micro-fixture

A local Python script (not linked to either source repository) computed prefix-closed observable traces to depth four for the payment example:

- Abstract `Requested -pay→ Paid`: observed traces `{ε, pay}`.
- Concrete `Requested -authorize→ Authorized -capture→ Paid` with `authorize ↦ τ` and `capture ↦ pay`: observed traces `{ε, pay}`. Inclusion holds in this **tiny explicitly bounded model**.
- Add `Paid -ship→ Shipped` with `ship ↦ ship`: concrete gains `pay,ship`; observable trace inclusion fails with that counterexample.
- An absent observation map cannot justify the comparison; zero concrete models must not count as a usable refinement.
- `Cancelled -ship→ ?` absent in an incomplete model is **underspecified**, and can be forbidden only by a relevant explicit prohibition/closure.

These results show why the abstraction/closure parameters matter. They **do not** demonstrate any current Specodelic/Wanna feature, nor prove general trace inclusion algorithms correct.

## 8. Preferred v1 experiment (highest information gain)

Define one narrow *order-payment-storage* system. Make the top-level promise "accepted payment is durably recorded before shipping". Decompose into three components with typed ports and action/effect contracts. Simulate two alternative concrete payments (authorization/capture versus direct payment), mapping them to the same abstract observation. Inject an incompatible storage backend; show that its limitation produces a *challenge to the parent guarantee*, not a silent redefinition. Introduce a third compatible backend, check that designated stable processor code/contract remains unchanged under the defined extension scenario. Record one external independent example and one model-generated example separately; show that a passing corpus does not erase a model counterexample. Propose then reject a semantic revision through a Wanna-mediated authority boundary. Finally, export the subset that current Specodelic understands; record mapping losses and actual CLI outcomes.

**Acceptance gate:** One nontrivial cross-boundary scenario and a matching negative case, with explicit interface/behavior/abstraction semantics, truthful provenance and no unclaimed inference. Produce (a) a finite executable semantic fixture, (b) a real `spk lint` / model-check / conform execution if supported, (c) tests for view invariance and semantic-change invalidation, and (d) a versioned source-to-source loss report. If this cannot be accomplished cheaply, do not broaden the generic institution API yet.

## 9. Recommended decision

- **Retain:** three-layer separation, satisfaction condition, explicit abstraction, scope-sensitive refinement, non-vacuity, open-world default, proof-strength distinctions, source/evidence provenance.
- **Add before extraction:** typed software contracts and effects, a partial/closed behavioral model, explicit design transformation algebra, operational composition and trace/refinement checker, dependency fingerprints and loss-reporting translators.
- **Defer:** universal semantic pack evaluators, cross-paradigm model equivalence, full arbitrary institution plugin APIs, generic theorem proving, global claims of modularity/OCP, and independent multi-repository kernel versioning.

This is a **revision recommendation**, not an instruction to abandon category theory. The first validation target should be whether the semantic model makes architectural design *more falsifiable and easier to revise* than a conventional typed graph plus scenario tests.

## 10. Assumptions, unknowns, provenance and reproducibility

**Assumptions:** Source archives are representative snapshots; the purpose is human/AI software design, not merely spec linting; behavioral examples can be modeled as finite LTS for a v1 pilot; preservation claims are local to chosen semantics; authorizing changes is distinct from proving correctness.

**UNKNOWN:** exact upstream Git commits; whether a compatible packaged `spk` executable would accept proposed semantic representations; whether all Wanna tests pass on this snapshot; an adequate general semantic contract language across distributed/functional/dataflow systems; complexity bounds and false-positive rates of proposed evaluators; empirically measured human cognitive overhead; actual compatibility of Specodelic generated specs from the proposed adapter.

**Executed in this evaluation:** Four uploaded ZIP files read and extracted; relevant Markdown and Rust/TypeScript sources inspected; Python script `/mnt/data/eval_semantics_fixture.py` executed (output includes the passing and failing traces above). No repo test suite, binary, solver, web upstream HEAD, or formal proof checker executed.

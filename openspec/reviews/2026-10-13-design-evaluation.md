# Design, value, and feasibility evaluation

Date: 2026-10-13. Reviewed bundle: `VSD-IMPLEMENTATION-2026-10-13`, version `0.1.0-proposal`. Repository HEAD: `1ff503709970bfae62dc0a85efe021ed1cdb5492`. Review tracking: `funkalization-6ar`.

**Verdict: NEEDS_REVISION.** This is a credible research and verification agenda, but it is not yet a standalone implementation specification. A narrow local prototype is technically plausible; practical feasibility and user value remain unproven. The missing decisions are concentrated enough to resolve without replacing the architecture.

References below use bundle filenames relative to `openspec/design/visual-semantic-design-implementation-2026-10-13/`, unless a repository path is explicit. Recommendations are advisory, not adopted requirements. Beads holds follow-up status; this report records findings rather than an implementation checklist.

## What is strong

The design separates authorization from correctness, preserves incomplete proposals, scopes evidence, and treats diagrams as projections. Those are useful constraints against agents turning guesses into accepted facts. C01–C08 have explicit falsification ideas, and G0–G8 avoid pretending that passing unit tests establishes a complete visual product. The explicit exclusion of general equivalence, liveness, distributed collaboration, and a universal semantic kernel makes a small experiment credible.

The source ledger is unusually candid about uncertainty. All five committed document hashes and byte counts match `manifest.json`. Unknown source commits are not themselves fatal when exact source bytes can be retrieved; the problem here is that the referenced bytes are not supplied by this checkout.

## Value proposition

The most promising initial user is a maintainer reviewing agent-generated architecture changes in a repository already using structured specs. The recurring job is: **show what a proposed change invalidates, which obligations remain open, who accepted it, and what evidence still applies.** A concrete example is rejecting a storage replacement that loses the payment-before-shipment guarantee despite nominal interface compatibility.

The current pitch overstates the absence of authority, evidence, and history in prose (`README.md:3`). Reviewed, version-controlled specifications can already carry these. OpenSpec already distinguishes agreed specs, proposals, and archived changes. The proposed differentiation is machine-addressable semantic edits, automatically generated obligations, evidence invalidation, and equivalent actions across interfaces. That differentiation needs demonstration against the existing workflow, not just against unstructured chat. See [OpenSpec concepts](https://github.com/Fission-AI/OpenSpec/blob/main/docs/concepts.md).

There is no documented target-user study, baseline failure rate, adoption budget, or success threshold. The risk is paying twice: users maintain specifications and a formal model, while adapters and UNKNOWN results add review work. A useful experiment would compare the same seeded design-review tasks with the existing OpenSpec/text workflow and the proposed workspace. Measure missed defects, false alarms, total authoring-plus-review time, and whether users correctly interpret bounded evidence. Then compare text plus generated diagrams against interactive diagrams separately.

Suggested exploratory thresholds, to agree before testing: zero unsafe acceptances across the seeded stale/unauthorized/contradictory cases; at least 20% lower median total task time without worse defect detection; and successful unaided completion by at least four of five pilot maintainers. These are proposed experiment criteria, not market evidence or statistically conclusive targets. Distribution, willingness to pay, and commercial viability have not been established.

## Implementation feasibility

| Area | Assessment | Main uncertainty |
|---|---|---|
| Local schema, identity, event replay | Conventional engineering once the behavioral contract is fixed | Canonicalization, transaction boundary, migrations |
| Nominal binding and obligation generation | Plausible for a small explicit vocabulary | Exact generation, invalidation, and discharge rules |
| Finite trace inclusion | Plausible for small explicit closed models | Precise trace semantics, independent oracle, resource limits |
| Read-only views and a minimal action UI | Plausible on top of one action API | Interaction cost, projection completeness, stale evidence display |
| Bidirectional OpenSpec/Specodelic writes | Highest integration risk | Authority reconciliation, semantic merge, crash recovery |
| General refinement/composition or code correctness | Not established by this proposal | Explicitly outside the finite pilot or missing implementation-to-model relation |

There is no evidence-based delivery estimate yet. The repository has a Rust CLI scaffold and no runtime dependencies, not an integrated M3/M4 engine. Python prototypes are described in the source ledger, so reuse still requires choosing a subprocess boundary or a Rust port with conformance tests. Neither is inherently prohibitive, but neither is free.

Existing components can reduce engineering work. RFC 8785 provides deterministic JSON encoding; it does not decide which design fields are semantic or which arrays represent unordered sets. SQLite documents local transaction and crash-recovery mechanisms; using it would not automatically make external Markdown writes part of the same transaction. These are candidate mechanisms, not selected dependencies. See [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html) and [SQLite atomic commit](https://www.sqlite.org/atomiccommit.html).

## Findings

| Severity | Count | Meaning in this review |
|---|---|---|
| Critical | 4 | Blocks standalone implementation of the affected gate |
| High | 8 | Material behavioral, integration, or product-validation gap |
| Medium | 3 | Priority, precision, or documentation ambiguity |

**F01 — CRITICAL: G0 inputs are not reproducible from the repository.** `SOURCES.md:9–20`, `ACCEPTANCE.md:29–34`. The M2/M3/M4 archives and referenced source files are absent from this checkout; a filename search under the sibling project tree found no matching prototype archives. This is not a claim that the files do not exist elsewhere. There is no retrieval procedure for a cold reader. Publish content-addressed inputs or immutable retrieval locations, commands, dependencies, and expected baseline results. A fresh checkout must be able to obtain and verify those bytes without chat history.

**F02 — CRITICAL: the shared action/model contract is only a sketch.** `REQUIREMENTS.md:39–53`, `ACCEPTANCE.md:36–47`. Entities, typed edit variants, field types, preconditions, errors, normalization, and schema migration behavior are unspecified. A stateless implementer must invent the interfaces that CLI, UI, evaluators, and adapters are supposed to share. Define a versioned schema and transition table with golden inputs/outputs. Distinguish semantic content digest, event identity, and accepted revision; equal model content reached through different histories needs an explicit policy. These are intentional G1/G2 design tasks, but therefore the bundle is not yet executable as a fixed contract.

**F03 — CRITICAL: acceptance has no resolved authority boundary.** `README.md:69`, `REQUIREMENTS.md:43–53`, `RISKS.md:60–64`. External accepted facts remain authoritative, while workspace `decide` changes accepted state. It is undefined whether that means a local pending decision, an external commit, or both. An `authority` field also does not establish who may supply it. Define field ownership, trusted actor identity, permission checks, and acceptance states. Specify behavior when external specs change between import and acceptance; a matching workspace revision alone cannot detect external drift.

**F04 — CRITICAL: the pilot oracle does not yet establish the headline property.** `README.md:75–77`, `ACCEPTANCE.md:55–59,89–94`. The positive refinement example stops at payment; the negative example forbids `ship` without constructing the required durable-storage semantics. A model that never ships could satisfy these examples while providing no useful fulfillment behavior. Specify the actual state machine and exact trace corpus before checker implementation: successful payment, durable commit, then shipment eligibility; payment failure; failed/uncertain persistence; crash/restart; duplicate requests; shipment before persistence. If the test uses `ship` as a proxy for eligibility, define that relation. Include a successful shipping trace so banning all shipment is not accepted as the desired implementation. Distinguish durability, atomicity, and eventual visibility: a storage label alone cannot determine the verdict.

**F05 — HIGH: the proposed archive dependency still has a concrete overwrite path.** `ACCEPTANCE.md:61–65`, `RISKS.md:14,78`. Read-only inspection of neighboring `../specodelic/src/archive_companion.rs:169–210` shows verified delta files copied verbatim to deployed specs, one at a time. Lines 301–304 invoke OpenSpec with `--skip-specs`; lines 285–287 restore again on historical reruns. For a partial additive delta, this code has no semantic merge preserving unrelated requirements. Multiple copies have no enclosing rollback. The neighboring checkout declares version 0.9.0 at HEAD `427bf9a5ffefa72e85c6071a1e5f8e3c1875f623` and has unrelated working changes; it is not the archived 0.8.0 snapshot. This confirms a source-level integration concern, not a reproduced CLI incident. Run P07, historical replay, and interrupted multi-file writes in an isolated fixture before adopting this path. Reject writes or use a separately verified merge path until then.

**F06 — HIGH: durability can be waived despite persistent-product promises.** `REQUIREMENTS.md:14,17,28`, `ACCEPTANCE.md:47`, `docs/src/configuration.md:11–15`. G2 permits persistent restart/replay to be marked unsupported, while save/reopen, versioned state, and later external writes depend on it. Define a deliberately memory-only intermediate milestone if useful, but require persistent recovery before claiming the workspace or write adapter complete. Specify the commit point, two local writers, disk-full behavior, truncated event tails, and recovery after each write boundary. A sequence of individually atomic file replacements is not a transaction over all files.

**F07 — HIGH: evidence records lack a decision procedure.** `REQUIREMENTS.md:23–26,47–48`, `RISKS.md:23,72`. There is no deterministic rule for conflicting results, stale dependencies, evaluator failures, or whether manually submitted evidence can discharge an obligation. Preserve all results but define a derived status algorithm: applicable scope, dependency fingerprints, trusted evaluator identity, and explicit handling of pass plus counterexample. Specify whether accepted contradictory claims remain allowed with unresolved obligations; never silently interpret acceptance as discharge. Test changes to assumptions, mappings, evaluator versions, and unrelated facts separately.

**F08 — HIGH: finite refinement is insufficiently precise.** `REQUIREMENTS.md:57`, `ACCEPTANCE.md:22–23,55–59`, `RISKS.md:21,34`. Choose explicit trace sets or LTS semantics, prefix/termination treatment, empty-language policy, and whether the horizon counts concrete events or observable events after hiding. Define the verdict for missing mappings, empty models, hidden cycles, and exhausted exploration. Otherwise two compliant-looking evaluators can disagree. Require exact boundary fixtures at the horizon and beyond it, and explain why a bounded pass says nothing about the excluded behavior.

**F09 — HIGH: product value has no acceptance gate.** Root `README.md:3–9`, bundle `RISKS.md:25,74`, `ACCEPTANCE.md:77,124`. A walkthrough and all C01–C08 passing establish technical behavior, not a reason to adopt another tool. Add the comparative review experiment described above, including total modeling effort and a decision about retaining the UI if it offers no measured advantage. Treat a semantic engine and a visual product as separately testable value hypotheses.

**F10 — HIGH: falsification ideas are not requirement-to-test coverage.** `REQUIREMENTS.md:10–37`, `ACCEPTANCE.md:85–118`. The VSD rows and P01–P10 fixtures lack an explicit many-to-many mapping and executable payloads. Several assertions allow multiple outcomes: P02 can challenge, fail, or return unknown; P06 allows a report without defining committed effects. Specify the input conditions for each result and map every mandatory requirement to positive, negative, and boundary examples with exact outputs. `openspec validate --all --strict --no-interactive` currently finds no items; it does not validate this bundle under `openspec/design/`. Promote the chosen slice into normal change specs when preparing implementation, without presenting unimplemented behavior as deployed specs.

**F11 — MEDIUM: requirement priority and completion rules disagree.** `REQUIREMENTS.md:34–37`, `ACCEPTANCE.md:79–83,120–124`. Independent replay and oracle provenance are SHOULD items but mandatory for final completion. Make their final-release status explicit, and distinguish release requirements from optional intermediate-gate outcomes. Separate implementation guidance such as mechanism reuse from user-observable requirements.

**F12 — MEDIUM: qualifiers hide observable decisions.** `REQUIREMENTS.md:30,36–37`, `ACCEPTANCE.md:40,77`. Terms such as “appropriate views,” “as appropriate,” “basic human walkthrough,” and “where verified” have no declared selection criteria. Name the required views, meaning-bearing fields, walkthrough tasks, and evidence sufficient for reuse. The issue is not the word “support” by itself; it is the absence of a bounded behavior behind the wording.

**F13 — HIGH: operational limits and failure envelopes are missing.** `REQUIREMENTS.md:46–53`, `RISKS.md:34–36`. There are no input-size limits, evaluator time/memory budgets, graph limits, cancellation semantics, or stable machine-readable errors. These matter even in finite models, where possible length-h traces over k labels grow as k^h. Select a measured pilot envelope and specify limit/timeout outcomes as incomplete or unknown with no partial accepted mutation. Keep evaluator execution failure distinct from a semantic counterexample. Include untrusted source paths and malformed evidence in the input-validation contract.

**F14 — MEDIUM: project guidance and book claims diverge.** `openspec/project.md:3–31` remains template text. `docs/src/configuration.md:7–15` attributes a concrete file layout to a design contract whose persistence choice is explicitly unknown (`REQUIREMENTS.md:53`). Fill in the actual Rust/project conventions and either adopt that layout through a decision or label it as a separate unapproved suggestion. Otherwise agents can treat a documentation invention as authoritative architecture.

**F15 — HIGH: prototype reuse and implementation evidence have undefined boundaries.** `SOURCES.md:15–20,30`, `ACCEPTANCE.md:80`, `Cargo.toml:15`. There is no Python-to-Rust integration/port contract, and a real implementation trace is optional. A finite model checker can check the wrong abstraction perfectly. Decide how the prototype is reused and define conformance vectors. If the product promises evidence about running software, add an independently captured implementation trace with a declared abstraction map. Otherwise limit the claim to design-model consistency; the current gates cannot establish deployed software correctness.

## Review passes and convergence

| Pass | Focus | New findings | New critical findings | Decision |
|---|---|---|---|---|
| 1 | Standalone context and interfaces | F01–F03 | 3 | Continue |
| 2 | Requirements and value | F09–F11 | 0 | Continue: 3 new versus 3 previous |
| 3 | Precision and documentation | F12, F14 | 0 | Continue: 2 new versus 3 previous |
| 4 | Behavioral boundaries and failures | F04, F06–F08, F13 | 1 | Continue |
| 5 | Executability and integration | F05, F15 | 0 | Pass limit reached; 2 new versus 5 previous |

Convergence was not reached under the less-than-10%-new-findings rule. No empirical false-positive rate is available; each finding was rechecked against the full bundle, and known limitations were distinguished from newly observed implementation gaps. Missing future design decisions are blockers to their affected implementation gates, not claims that exploratory work must stop.

## Recommended sequence

Resolve ownership, actor authority, and the pilot's real state/trace oracle before finalizing the canonical model. Recover the baseline inputs and choose prototype reuse boundaries. Then implement one persistent text/API path that proposes a storage substitution, generates an obligation, records a scoped verdict, and refuses a stale decision. Generate a read-only view for the same path and measure whether users can review it effectively.

Move an isolated adapter-safety experiment earlier than the full G5 implementation so it can inform G1's schema; retain the prohibition on unverified production writes. Treat the full G0–G8 gates as completion evidence, while allowing an earlier non-production vertical experiment to retire architecture and value risks. Expand interactive editing only after that experiment demonstrates benefit.

## Validation and limits

- `just ci`: passed formatting, Clippy with warnings denied, and all three existing scaffold tests. No semantic-engine tests exist in this repository.
- `openspec list` / `openspec list --specs`: no active changes or deployed specs.
- `openspec validate --all --strict --no-interactive`: “No items found to validate.” This is no coverage, not a validated design contract.
- SHA-256 and byte-count verification: all five bundle documents match the committed manifest. External source archive hashes could not be checked because those inputs were not located.
- Tool versions observed: OpenSpec 0.19.0 and rustc 1.98.1. The declared Rust 1.88 minimum was not tested.
- Neighboring Specodelic code was inspected read-only; no archive CLI mutation or upstream compatibility suite was run. No G0–G8 completion is claimed.
- No product interviews, usability study, pricing study, or end-to-end model execution was performed. Feasibility judgments are engineering assessments, not measured delivery commitments.

The original bundle and production code were not modified by this review. Pre-existing `.gitignore`, `.beads.gate.lock`, and `.whisper/` working-tree changes were left intact.

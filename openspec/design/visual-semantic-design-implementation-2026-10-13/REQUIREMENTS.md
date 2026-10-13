---
title: "Normative proposed requirements for visual semantic design"
date: "2026-10-13"
model: "OpenAI GPT-6"
status: "PROPOSED SHALLS — not implemented"
source_commits: "UNKNOWN"
---
# Requirements

`SHALL` here denotes a proposed behavior to implement and test, not current tool behavior. Every requirement has at least one falsification case in [ACCEPTANCE.md](ACCEPTANCE.md). The named architecture and data contracts are **candidate APIs**, not existing published APIs.

| ID | Priority | SHALL | Falsification / observable check |
|---|---|---|---|
| VSD-01 | MUST | Preserve raw incomplete/contradictory proposals without fabricating resolution | Save/reopen dangling reference and contradictory requirements with diagnostics; no invented links |
| VSD-02 | MUST | Separate accepted semantic state, proposals, judgment ledger, history and revision | Proposal/rejection do not mutate accepted digest |
| VSD-03 | MUST | Apply decisions only against authorized matching revision | Stale approval rejected with zero committed change |
| VSD-04 | MUST | Define replayable, deterministic `propose`, `decide`, `report` effects under a frozen semantics version | Replay same ordered events twice, matching semantic digest |
| VSD-05 | MUST | Give IDs stable across name/layout changes, with explicit rename semantics | Rename preserves identity; unrelated ID not conflated |
| VSD-06 | MUST | Exclude layout/display data from semantic digest | Re-layout same design yields same digest |
| VSD-07 | MUST | Separate lifecycle, provenance and verification outcomes | Accepted declared link with unchecked behavior is not displayed as verified |
| VSD-08 | MUST | Represent explicit typed contract requirements/provisions and unresolved obligations | Missing required operation detected or explicitly unknown under open-world policy |
| VSD-09 | MUST | Generate preservation obligations for accepted decomposition or changed guarantees | Decompose/revise does not silently certify behavioral preservation |
| VSD-10 | MUST | Scope evaluator verdicts to model, claim, assumptions, semantics, method, horizon, evaluator/version | Change evaluator/assumption makes old verdict inapplicable or stale |
| VSD-11 | MUST | Distinguish `pass`, `fail`, `unknown`, `unsupported` without downgrading uncertainty to pass | Unknown observation map and unsupported shared-event composition never pass |
| VSD-12 | MUST | Produce reproducible witness for supported finite trace-inclusion failures | Forbidden observable trace is present and replayable |
| VSD-13 | MUST | Label finite conclusions as bounded and avoid generalized proof claims | A bounded pass is displayed with horizon and closure assumptions |
| VSD-14 | MUST | Identify authoritative representation per accepted fact and preserve non-exportable facts | Export fails or loss-reports unsupported alternatives rather than dropping them |
| VSD-15 | MUST | Protect accepted specs against archive/merge conflicts, partial failures and content loss | Additive change retains unrelated requirements; failed write leaves original unchanged |
| VSD-16 | MUST | Report extraction completeness and parse/skipped inputs in projections | Malformed input cannot yield unqualified complete graph |
| VSD-17 | MUST | Preserve exact edge endpoints and isolated components in appropriate views | Distinct contracts not merged invisibly; unconnected capability represented |
| VSD-18 | MUST | Trace diagram elements to authoritative claims and source revisions | Every semantic visual element has resolvable ID(s) |
| VSD-19 | MUST | Route CLI, visual and agent actions through identical typed operation protocol | Equivalent actions yield equal canonical state and obligations |
| VSD-20 | MUST | Keep visual navigation, proposal, decision, evaluation and artifact export distinct effects | Clicking/rearranging never auto-approves or overwrites accepted spec |
| VSD-21 | SHOULD | Include semantic before/after comparisons, independent oracle provenance and counterexample inspection | Reviewer can inspect claim delta and witness origin |
| VSD-22 | SHOULD | Provide an independently replayable order–payment–storage vertical tracer | Cold reader reproduces the pilot without hidden state |
| VSD-23 | SHOULD | Refuse to claim modularity/OCP unless the exact measurable policy and observed coverage are declared | No global guarantee from cosmetic boxes/wiring alone |
| VSD-24 | SHOULD | Reuse existing M3/M4/Specodelic mechanisms where verified; defer generalized kernels | No new generic semantic runtime required to pass initial pilot |

## Proposed minimum data protocols (not current APIs)

```text
DesignWorkspace = (accepted_design, proposals, judgments, event_history, accepted_revision)
Action = propose(id, base_revision, typed_edits) |
         decide(proposal_id, accept|reject, authority, base_revision) |
         report(scoped_evidence)
ActionResult = (outcome, accepted_revision, obligations, diagnostics)
Judgment = (obligation_id, subject_revision, semantics_version, evaluator_version,
            method, assumptions, bounds, observation_map?, verdict, witness?, evidence_origin)
Projection = (source_revision, semantic_digest, completeness, elements, diagnostics)
VisualElement = (stable_id, source_claim_ids, semantic_kind, lifecycle, evidence_refs)
```

**Protocol invariants:** unauthorized or stale decisions cannot mutate `accepted_design`; evidence reports cannot rewrite constraints; unsupported checker outcomes cannot discharge obligations; graph geometry cannot affect canonical semantic content; exports must be atomic or explicitly rejected. The exact serialization format, canonicalization algorithm, authority system and persistence backend are intentionally UNKNOWN pending G1/G2/G5.

## Semantic scope

Candidate supported v1 evaluation: finite explicit closed trace sets, known alphabet and horizon, declared observation/hiding map, nominal interface inclusion under a declared completeness rule. M4 does **not** establish arbitrary assume–guarantee composition, asynchronous liveness/fairness or universal contract substitutability. Future synchronized I/O semantics require separate specification, falsification tests and checker provenance.

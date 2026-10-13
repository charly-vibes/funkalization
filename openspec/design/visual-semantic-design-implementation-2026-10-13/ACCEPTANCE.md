---
title: "Execution gates, falsifiable claims and verification protocol"
date: "2026-10-13"
model: "OpenAI GPT-6"
status: "TEST PLAN; NEW GATES NOT RUN"
source_commits: "UNKNOWN"
---
# Acceptance / falsification plan

## Status and measurement rule

All G0–G8 outcomes are **NOT RUN** for this new implementation bundle. Prior conversations reported M2 16/16, M3 50/50, M4 18/18, plus 2,801 M3 bounded event sequences, but this bundle does not reproduce those executions; count them as historical evidence only until G0 repeats them with recorded environment and commands. A test marked "expected failure" succeeds *only if* the specified evaluator detects the injected defect and returns the specified failure verdict/witness. Every claim needs a positive, negative, and scoped case; ignored, skipped or unsupported scenarios must be surfaced explicitly.

## Main falsifiable claims

| Claim | Exact proposition | Positive oracle | Negative oracle / counterexample | Scope |
|---|---|---|---|---|
| C01 | Equal semantic actions via text and UI yield equal accepted state and obligations | Apply same bound proposal through each surface; normalized digest + obligation set equal | Deliberately omit edit in UI adapter; mismatch detected | Selected v1 action subset, same base revision |
| C02 | Layout-only edits preserve meaning | Shuffle coordinates/routes/order of view elements: digest unchanged | Change contract guarantee: digest changes | Defined canonicalization, declared semantic fields |
| C03 | Stale/rejected/unauthorized actions do not corrupt accepted state | Valid current approved change applies exactly once; rejection leaves digest | Stale/duplicate/unauthorized commit rejected; unchanged digest | Single serialized workspace with version check; distributed atomicity not claimed |
| C04 | Typed bindings/decompositions generate correct obligations | Required operations and preservation claims emitted with source IDs | Missing required provision or contradiction cannot disappear; checker not auto-green | Explicit finite interface vocabulary and obligation rules |
| C05 | Supported finite valid refinement passes; invalid refinement yields witness | `authorize→τ`, `capture→pay` within closed horizon refines abstract pay trace | Additional forbidden observable `ship` yields concrete counterexample | Explicit finite LTS / trace model; no liveness proof |
| C06 | Incomplete/unsupported behavior never looks proven | Closed finite check clearly labeled with horizon and assumptions | Missing observation map, open trace scope, shared-event composition -> UNKNOWN/UNSUPPORTED | Declared evaluator capabilities and input closure |
| C07 | Export/archive cannot silently lose accepted semantic content | Add retry requirement; atomicity requirement preserved on archive/re-read | Conflicting or unsupported edit stops and preserves original data, or reports loss | Pinned compatible versions, supported semantics only |
| C08 | Every displayed semantic claim and verdict is traceable | Resolve each diagram element / judgment to source IDs and exact model revision | Delete underlying source or corrupt ID -> incomplete projection/error | Generated structured views, not freehand annotations |

## Dependency gates (strict in the declared implementation order)

### G0 — Freeze and reproduce baseline
- **Input:** supplied M2/M3/M4, Specodelic source snapshot, OpenSpec version selected from a pinned upstream revision.
- **Work:** record SHA-256, Git commits if resolvable, Python/Rust versions, OS, exact test commands/stdout/stderr, dependencies, baseline source schema. Run existing unit suites; attempt actual Specodelic `spk` operations separately.
- **Positive:** expected reference suite counts reproduced or differences explicitly explained; all actually run tests shown.
- **Negative:** deliberately broken test or invalid fixture yields non-green run and is displayed; no "pass" from skipped suite.
- **Exit:** baseline reproducible from clean checkout/archive with machine-readable result manifest. Tool-compatibility UNKNOWN until run.

### G1 — Canonical semantic model and serialization
- **Input:** M3 event model plus M4 contract fragment and required visual identities.
- **Work:** define entities, relationships, typed edits, unresolved claims, stable IDs, canonical ordering, semantic digest, schema version and migration policy.
- **Positive:** permutation of independent declarations, display coordinates and nonsemantic styling yield equal digest.
- **Negative:** change assumption/guarantee, relationship endpoint or semantic edit history as appropriate changes identity/digest; malformed ID detected.
- **Exit:** a published schema, normalization algorithm, golden fixtures, and digest test vectors.

### G2 — M3 action protocol, authority and replay
- **Work:** implement `propose`, `decide`, `report` without requiring UI; implement immutable base-revision check, duplicates, refusal, rejected alternatives, atomic changeset commit and replay.
- **Positive:** accepted current proposal changes revision once; event replay matches reference digest.
- **Negative:** stale approval at `r+1` from base `r`, duplicate application and unauthorized action do not mutate accepted model.
- **Exit:** C03 demonstrated plus precise event and failure semantics; persistent restart/replay either demonstrated or marked NOT SUPPORTED.

### G3 — Typed obligations
- **Work:** define explicit required/provided ports, declared assumptions/guarantees/effects, coverage/closure rule, and obligation generation for bind/decompose/revise.
- **Positive:** satisfied nominal interface produces accurate named obligations (does not automatically prove behavior).
- **Negative:** missing `atomic_write` provision, parent guarantee contradicted by backend, missing abstraction map each produces challenge or unknown, not pass.
- **Exit:** obligation-generation golden tests including source/revision links; C04 scope documented.

### G4 — M4 finite evaluators
- **Work:** integrate finite closed contract and observable trace inclusion, with precise evaluator version, horizon, alphabet, model digest, assumptions, witness.
- **Positive:** concrete `Requested→Authorized→Paid` mapped via `authorize→τ`, `capture→pay` passes **for specified finite closed model**.
- **Negative:** concrete `pay,ship` (where abstract model does not allow ship) fails with replayable observable witness. Unmapped event, unsupported synchronization or open domain remains UNKNOWN/UNSUPPORTED.
- **Exit:** C05 and C06 covered without conflating finite checks with general implementation correctness.

### G5 — Source adapters and archive safety
- **Work:** pin and run real Specodelic/OpenSpec toolchain; specify exact ownership and changes; define unsupported-field loss report; use staged, validated, atomic write/rollback strategy; inspect current `archive_companion` path rather than trusting it implicitly.
- **Positive:** existing accepted requirement `atomicity` plus ADDED `retry` archives to both; archive/re-read preserves both and unrelated claims.
- **Negative:** conflicting edits, malformed delta, unrepresentable alternative, forced interrupted write, duplicate archive or historical replay produce rejection or explicit report with accepted content preserved.
- **Exit:** C07 and versioned map round-trips proven for supported subset; unresolved loss and path limitations recorded.

### G6 — Read-only projections
- **Work:** generate capability/contract/behavior/obligation/change views from normalized state; preserve full contract endpoint identities, isolated nodes, source URLs/paths, accepted/proposed status and diagnostics.
- **Positive:** fully parsed source -> complete view; every element resolves to source IDs.
- **Negative:** one invalid source file among valid files -> `partial`/`failed` status, never falsely complete; contract count-preserving rename still visible in contract-level diff.
- **Exit:** C02 and C08; rendering deterministic by semantic identity independent of layout.

### G7 — Minimal interactive UI
- **Work:** select node, inspect claim, propose binding, inspect generated obligations, accept/reject with revision/authority check, inspect counterexample. No generic drawing toolkit prerequisite.
- **Positive:** identical text/API and visual action sequences produce identical normalized state and obligation sets.
- **Negative:** UI callback bypass attempting stale approval is rejected by same backend; rearrangement never mutates accepted design.
- **Exit:** C01 and C02 with automated UI-to-API fixtures; basic human walkthrough logged separately (not yet proof of usability).

### G8 — Integrated tracer bullet
- **Work:** walk all pilot fixtures and operations: top-down model, bottom-up contradicting storage, compatible alternative, hidden-event refinement, forbidden trace, stale approval, source export, archive/re-read, independent expected behavior and optional real implementation trace.
- **Positive:** complete replay from clean environment yields expected statuses/witnesses/diagrams and source references.
- **Negative:** inject an incompatible adapter, malformed corpus, stale proposal and lossy artifact; no silent green result or data loss.
- **Exit:** all C01–C08 evidenced *within explicitly recorded limitations*; nothing deferred misrepresented as complete.

## Pilot fixture table

| ID | Scenario | Expected test oracle | Required record |
|---|---|---|---|
| P01 | Atomic storage nominally provides required interface | Nominal match; behavior/effects remain separate until checked | Obligation IDs and claim scopes |
| P02 | Eventual-only store violates durable acceptance promise | Explicit challenge/fail where coverage and semantics permit, otherwise UNKNOWN | Conflicting parent/backend claims |
| P03 | Finite authorize/capture refinement | Scoped pass under explicit observation mapping | Bound, alphabet, mapping, exact traces |
| P04 | Extra observable `ship` trace | Fail with replayable witness | Minimal counterexample trace |
| P05 | Approve stale proposed binding | Conflict; identical accepted digest pre/post | Revision pair and action log |
| P06 | Export unsupported proposed alternative | Diagnostic/loss report; no silent disappearance | Before/after semantic inventory |
| P07 | Archive additive requirement | Preserve original atomicity plus new retry | Pre/post serialized requirement set |
| P08 | Move nodes without semantic edit | Equal semantic hash and revision | Before/after hashes and layout diff |
| P09 | Malformed spec among valid files | Failed/partial extraction visible | Skipped files, parser diagnostics |
| P10 | Mutually dependent components/shared events beyond M4 scope | UNKNOWN/UNSUPPORTED (not pass) | Unsupported operator/path |

## Evidence record format — example only

```yaml
claim_id: C05
fixture: P04
status: NOT_RUN
expected_verdict: fail
actual_verdict: UNKNOWN # placeholder, not a result
subject_revision: UNKNOWN
semantics_version: UNKNOWN
evaluator_version: UNKNOWN
assumptions: [finite_closed_scope]
horizon: 4 # illustrative only, not automatically sufficient
observation_map: UNKNOWN
witness: null
command: UNKNOWN
stdout_artifact: UNKNOWN
source_commit: UNKNOWN
```

A final report must include: exact source hashes/commits, execution environment, test vectors, commands, stdout/stderr, expected vs actual verdicts, assessor notes, skipped tests, known interpretation limits and independent oracle provenance. `status=NOT_RUN` is not success. A CI zero exit alone is not formal verification.

## Go/no-go rule

**GO to a larger product only when** G0–G8 produce reproducible C01–C08 evidence and a cold reader can repeat the pilot with no invisible state. **NO-GO to generalized kernel/UI expansion** if the narrow model fails to preserve semantic authority, if integration silently loses information, or if visual manipulation cannot express the same meaning as the text interface. A semantic engine can still be worth retaining if rich visual editing yields no independently measured usability gain.

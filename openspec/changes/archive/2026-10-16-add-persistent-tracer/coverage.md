# Tracer requirement and finding coverage

This is evidence mapping, not a task list. Beads owns completion status. Test names below resolve under `tests/protocol.rs` or `tests/tracer.rs`; the tracer's named assertions are defined in `src/tracer.rs`.

## Current change requirements

| Requirement | Positive evidence | Negative/boundary evidence |
|---|---|---|
| Explicit local authority | Successful propose/evaluate/accept in cold tracer | `unauthorized_and_source_drift_have_zero_committed_effect`; strict unknown authority field in `reports_cannot_impersonate_checker_or_hide_conflicting_evidence` |
| Atomic guarded actions | `cold_tracer_runs_and_replays_in_a_second_process` | `process_death_before_commit_recovers_previous_snapshot_and_events`; `two_writers_cannot_accept_the_same_base_twice`; tracer stale/duplicate assertions |
| Scoped finite obligations | `tracer_accepts_a_real_model_change_and_labels_baseline_provenance`; temporal policy oracle | `counterexample_is_shortest_and_replayable`; `finite_boundaries_never_produce_vacuous_pass`; tracer unmapped/shared cases |
| Honest evidence and decisions | Scoped checked acceptance; `judgments_record_base_revision_and_retire_superseded_acceptance` | `reports_cannot_impersonate_checker_or_hide_conflicting_evidence`; `later_counterexample_challenges_an_accepted_binding_without_rewriting_it` |
| Safe derived artifacts | Export inventory equality and HTML creation in tracer | `malformed_limits_and_existing_destinations_are_rejected`; tracer archive/overwrite refusal; external archive probe |
| Reproducible tracer | Cold run, real model change, cross-process replay | Every report assertion compares expected and actual; mismatches return tracer_failed |
| Bounded input and version handling | Fixture seed loading and current-profile replay | `corruption_and_future_database_schema_fail_closed`; `malformed_limits_and_existing_destinations_are_rejected` |

## Historical VSD scope

| IDs | Disposition in this slice |
|---|---|
| VSD-01 | Incomplete observation maps and conflicting reports preserved. Arbitrary raw/dangling graph proposals are outside the typed binding profile. |
| VSD-02–04 | Separate proposals/evidence/revision; authorized transactional transitions and persistent replay. |
| VSD-05–06 | `canonical_bytes_match_the_published_cross_language_digest_vector` freezes canonical hashing; stable IDs and content hashing; views cannot mutate semantics. General rename/layout editing is absent. |
| VSD-07 | Lifecycle, declared reports, built-in judgments and applicability are separate. |
| VSD-08 | Two generated binding obligations: nominal interface and finite behavioral inclusion. |
| VSD-09 | Binding replacement changes scoped obligations; decomposition is explicitly unsupported. |
| VSD-10–13 | Source fingerprint, subject digest/revision, profile/evaluator, map, closure and horizon accompany judgments; finite witnesses and uncertainty tests. |
| VSD-14–15 | Source owns policy, workspace owns bindings/evidence. No source write-back; read-only fallback deliberately rejects unsafe archival. |
| VSD-16–18 | Single-source completeness, distinct proposal/obligation IDs, source-addressed projections; no multi-file corpus parser. |
| VSD-19–20 | `documented_cli_actions_match_the_direct_api_and_fail_without_effects` compares real CLI results with direct API states; views cannot approve. Interactive semantic editing and C01 are not implemented. |
| VSD-21–22 | Machine-readable inventory, witnesses and replayable tracer. Human-independent oracle and comparative study are pending. Tracer replay is mandatory for this change, regardless of the historical SHOULD. |
| VSD-23–24 | No generalized modularity/OCP claim. Recovered prototypes are reproduced separately; no generic kernel or port-equivalence claim. |

## Evaluation findings

| Finding | Resolution or explicit remaining boundary |
|---|---|
| F01 | Six exact inputs recovered; M2/M3/M4 replay and negative control passed. Two historical integration archives still missing (funkalization-8d0). |
| F02 | Typed public protocol in `src/model.rs`; explicit normalization, transition and error rules in design.md. |
| F03 | Field ownership and OS-principal boundary fixed; source drift refuses writes; no hybrid external commit. |
| F04 | Explicit positive shipment, failures, retry and restart corpus; temporal safety assertions; real storage behavior remains unverified. |
| F05 | Actual installed-tool CLI reproduced all three archive hazards; source writing disabled, export cannot overwrite. |
| F06 | Persistent SQLite commit point, restart/replay, killed-process recovery and concurrent-writer tests. Real power-loss/hardware guarantees are not independently tested. |
| F07 | Public reports cannot impersonate checker evidence; conflicts retained; later accepted-binding challenges supported; superseded evidence retired. |
| F08 | Prefix language, concrete-event horizon, total map, vacuity rules and unsupported composition explicitly selected. |
| F09 | Corrected pitch; comparative protocol and first participant identified. Actual walkthrough/measurement pending (funkalization-4e2). |
| F10 | Current requirements use OpenSpec scenarios and this test map. Unimplemented historical capabilities explicitly scoped out. |
| F11 | Current tracer requirements are mandatory; research SHOULDs do not waive current acceptance. |
| F12 | Named profile, views, fields, limits and finite outcomes replace vague implementation directions. |
| F13 | Bounded structure/snapshots, lock timeout, structured errors and no partial accepted mutation. No unbounded model exploration. |
| F14 | Project context populated; docs describe actual SQLite paths rather than an invented directory layout. |
| F15 | Deliberate M3/M4 differences documented in wai/design.md; no port equivalence or running-software correctness claim. |

The immutable original bundle remains historical evidence. These narrower implementation decisions do not retroactively complete G0–G8 or rewrite its provenance hashes.

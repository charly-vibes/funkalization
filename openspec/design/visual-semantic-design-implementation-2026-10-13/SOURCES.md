---
title: "Source register and claim provenance"
date: "2026-10-13"
model: "OpenAI GPT-6"
status: "Local uploaded source register; SRC-08 commit recovered and verified 2026-10-16"
---
# Source register

All paths refer to the named **user-uploaded ZIP snapshots**, not live checked-out Git commits. See `manifest.json` for SHA-256 of the exact mounted upload bytes. Source references point to specific bundled material; they do not establish executable integration or upstream current behavior.

| ID | Uploaded source and primary in-archive paths | Grounded role | Limits |
|---|---|---|---|
| SRC-01 | `software-design-substrate-2026-10-11(5).zip` → `software-design-substrate/{README.md,DESIGN-ACTIVITY.md,REQUIREMENTS.md,ARCHITECTURE.md,SCENARIOS.md,VALIDATION.md}` | DS-001–DS-028, E01–E10; progressive design, graphical/textual views, tracer bullets, constraints | Draft proposed requirements, not verified product |
| SRC-02 | `categorical-design-substrate-2026-10-11(3).zip` → `categorical-design-substrate/{SEMANTICS.md,SPECIFICATION.md,ARCHITECTURE-DECISION.md,SCENARIO-MATRIX.md,RULE-OF-5.md}` | M1 signatures, interpretations, satisfaction and preservation constraints | Conceptual model; not an executed general kernel |
| SRC-03 | `designing-semantics-M2-2026-10-11(1).zip` → `M2_bundle/{README.md,design_semantics_m2.py,test_design_semantics_m2.py}` | M2 revisable operational design semantics; candidate executable prototype | Fixture-level behavior; tests not independent theoretical proof |
| SRC-04 | `designing-semantics-M3-2026-10-11(1).zip` → `M3_design_semantics/{SPECIFICATION.md,VALIDATION.md,kernel.py,test_kernel.py,test_workflows.py,explore.py}` | Three-event propose/decide/report model; typed edits and obligations; bounded exploration | In-memory experimental machine, not integrated UI/Specodelic |
| SRC-05 | `designing-semantics-M4-2026-10-11(1).zip` → `M4_contracts/{SPECIFICATION.md,contracts.py,m3_bridge.py,test_contracts.py,test_bridge.py}` | Finite contract substitution / observable trace inclusion; explicitly scoped and non-decisive M3 bridge | Not general assume–guarantee composition or implementation proof |
| SRC-06 | `semantic_model_context_evaluation_2026-10-11(2).md` → sections 1–10 | Independent critical evaluation of M1 and proposed narrow semantic profile | Evaluation and Python illustration, not end-to-end integration verification |
| SRC-07 | `specodelic-openspec-visual-design-2026-10-12(1).zip` → `specodelic-openspec-visual-design-2026-10-12/{README.md,REQUIREMENTS.md,WORKFLOW.md,VISUAL-VIEWS.md,VALIDATION.md,SOURCES.md}` | Initial conservative visual workflow, source-of-truth, diagrams, pilot | Draft proposal; visual authoring not implemented |
| SRC-08 | `specodelic-main(3).zip` → `specodelic-main/{Cargo.toml,src/graph.rs,src/archive_companion.rs,src/conform.rs,src/model_check.rs,src/packs.rs,src/acset/,docs/src/tool-relationships.md}` | Available Rust snapshot for Specodelic implementation claims and integration risks | Local version `0.8.0` declared by Cargo.toml; Git commit `2939a0b2abff5f7d6d9f9959464431684a728491` recovered from the GitHub archive comment and verified in `charly-vibes/specodelic` (7 commits after v0.8.0, ancestor of upstream main); no CLI run for this bundle |

## Specific claim trace

| Claim in implementation contract | Basis / distinction |
|---|---|
| Semantic activity is not merely graph drawing | SRC-01 DESIGN-ACTIVITY, SRC-03 README, SRC-04 SPECIFICATION |
| Accepted means authorized, not proven | SRC-04 SPECIFICATION introduction |
| Event vocabulary (`propose`, `decide`, `report`) and candidate obligations | SRC-04 SPECIFICATION, `kernel.py` |
| Finite trace semantics have horizon/closure limitations | SRC-05 SPECIFICATION |
| M4 bridge results are non-decisive M3 evidence | SRC-05 SPECIFICATION and `m3_bridge.py` |
| Categorical structure alone is not an executable semantics | SRC-02 SEMANTICS, SRC-06 evaluation sections 3–5 |
| Existing Specodelic graph projection can be insufficient for architecture semantics | SRC-08 `src/graph.rs`, SRC-06 section 3, SRC-07 VISUAL-VIEWS |
| Unsafe archive overwrite is a regression hypothesis | SRC-08 `src/archive_companion.rs`, prior assessment; test exact current path G5 before making runtime claim |
| Workspace/action API and C01–C08 claims | **PROPOSED in this bundle**, synthesized from SRC-01–SRC-08; not already implemented as one system |
| Order–payment–storage end-to-end pilot | **PROPOSED** synthesis from SRC-01 scenarios, SRC-05 examples, SRC-07 storage pilot |

## Unknowns and citation policy

- **Recovery record (2026-10-16).** All eight inputs are preserved hash-verified in `research/archives/`. SRC-08's Git commit `2939a0b2abff5f7d6d9f9959464431684a728491` was recovered from the GitHub archive comment of the byte-identical snapshot (SHA-256 `040d0f36…`) and verified with `git cat-file` in `charly-vibes/specodelic`; the independent multirepo-proposal bundle pins the same SHA to the same commit. SRC-07's internal `specodelic_source_commit` remains unknown (it is a generated proposal bundle, not a repo snapshot).
- No other source Git commit can be recovered confidently from the supplied archive filenames alone: **UNKNOWN**.
- Separate prior discussions mention Wanna source, but no Wanna archive is attached in this turn. The Wanna runtime is **not re-inspected here**, and its integration compatibility is **UNKNOWN**.
- No live upstream fetch, Rust CLI run or OpenSpec run was undertaken to prepare this document. This report does not claim contemporary upstream parity.
- The uploaded `semantic_model_context_evaluation` names *earlier archive instances*, which should not be silently substituted for the current filenames; use the current manifest hashes to identify the inputs to this bundle.
- For each future executed claim, add the exact command, working tree/release revision, result, counterexample and reproducible evidence link rather than replacing this source ledger with memory-based summaries.

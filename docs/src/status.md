# Status

**Experimental: a persistent local tracer is implemented. The full visual-design contract is not complete.** The implemented profile uses Rust and SQLite, with explicit finite payment–storage fixtures.

## Implemented and tested

- Typed propose/evaluate/report/decide actions, Unix-owner and source-fingerprint guards.
- Atomic state/event commits, replay integrity, restart and process-death recovery tests.
- Separate nominal and behavioral verdicts, finite witnesses, unknown/unsupported outcomes, historical evidence and later counterexamples.
- Read-only JSON/HTML projections, new-file-only derived exports, and explicit archive refusal.
- A cold-start tracer and cross-process replay test, including an actual model substitution.

## Broader gate status

| Gate | Evidence and remaining scope |
|---|---|
| G0 | M2: 16 tests; M3: 50 tests and 2,801 event sequences; M4: 18 tests reproduced. Six input hashes verified. Two historical integration archives remain missing. |
| G1–G4 | Implemented for the declared v1 profile. General edits, decomposition and M3/M4 equivalence are not claimed. |
| G5 | Separate installed Specodelic 0.9.0 and 0.10.0 probes with OpenSpec 0.19.0 demonstrated additive overwrite, historical overwrite and partial writes in disposable repositories. Source writing is disabled. |
| G6 | Read-only source-addressed projections exist for the single-source model. A multi-file corpus adapter is not implemented. |
| G7 | No interactive semantic editor or text/UI equivalence result. |
| G8 | The local tracer is runnable; it does not complete every historical C01–C08 claim. |

## Evidence and provenance

`research/evidence/baseline-results.json` records exact historical commands and output. `research/evidence/archive-probe.json` records the external CLI failures. The current profile and requirement-to-test matrix live in `openspec/changes/add-persistent-tracer/`.

The finite corpus and separate temporal assertions were authored during implementation. They are not an independently authored human oracle. The user volunteered as first pilot reviewer; no completed walkthrough, comparative timings or adoption results are claimed until their response is recorded.

No deployed `openspec/specs/` are claimed. The change remains reviewable under `openspec/changes/` until release/deployment and archive are authorized.

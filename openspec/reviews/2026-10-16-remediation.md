# Evaluation remediation and tracer evidence

The local tracer is implemented and verified. This resolves the review's implementation-contract ambiguities for the explicit v1 slice; it does not establish completion of the broader visual product or empirical user value.

## Implemented result

The tracer proposes compatible and incompatible storage bindings, generates separate nominal/behavioral judgments, records finite witnesses, rejects stale/duplicate/unchecked acceptance, commits state plus events atomically, reopens and replays the workspace, and emits read-only HTML/JSON with source fingerprints. The compatible direct-payment backend actually changes the selected semantic model. Later counterexamples can challenge an accepted binding without rewriting it. Projection IDs reserve a separate namespace for the accepted-state node so a proposal named `accepted` cannot collide with it.

The field-ownership, serialization, canonicalization, scope, authority, persistence, limit and failure contracts are in `openspec/changes/add-persistent-tracer/design.md`; source types are in `src/model.rs`. The [coverage matrix](../changes/add-persistent-tracer/coverage.md) maps all seven current requirements and all fifteen review findings to evidence or explicit remaining boundaries. The original design bundle is preserved unchanged.

## Validation on 2026-10-16

- `just ci`: formatting, Clippy and 19 Rust tests passed (2 CLI, 14 protocol, 3 tracer). The protocol count includes the worker invoked by the actual process-kill test.
- `cargo clippy --locked --all-targets -- -D warnings`: passed.
- `openspec validate add-persistent-tracer --strict --no-interactive`: passed.
- `mdbook build`: passed.
- `cargo run --locked -- tracer target/tracer-2026-10-16`: 28/28 expected-versus-actual assertions passed; HTML, inventory, source and persistent database emitted.
- `cargo run --locked -- replay target/tracer-2026-10-16/workspace.db`: passed in a second process.
- `python3 scripts/reproduce_baselines.py`: all six available source hashes match; M2 16 tests, M3 50 tests, M4 18 tests, M3 2,801 sequences, and the negative harness control passed their expected outcomes.
- The original 0.9.0 archive probe remains recorded. Re-executing its default pin with installed 0.10.0 correctly refused the version mismatch. After source inspection, `python3 scripts/probe_archive.py --spk-version 0.10.0 --output research/evidence/archive-probe-0.10.0.json` reproduced additive overwrite, historical overwrite and partial deployment. This is negative safety evidence; source writing stays disabled.

Tests were first observed failing for the previously absent tracer command, missing judgment revision/supersession, later counterexample refusal, and the tracer's no-op acceptance. They passed after the corresponding changes. The policy corpus is separately specified from the evaluator but has the same implementation author; it is not misrepresented as a human-independent oracle.

## Remaining evidence

The two original integration archives (`specodelic-main(3).zip` and `specodelic-openspec-visual-design-2026-10-12(1).zip`) remain unavailable. Historical G0 is therefore partial despite successful M2/M3/M4 reproduction. Current installed-tool probes do not silently replace those inputs. This remains tracked in funkalization-8d0.

The user volunteered as the first pilot reviewer. The walkthrough question was sent, but no substantive response or timing has been recorded. `openspec/experiments/review-value.md` defines the formative walkthrough and a later comparative trial; funkalization-4e2 remains open. No time savings, usability improvement or adoption claim is made.

General graph editing, arbitrary decomposition, interactive semantic UI/C01, multi-file source adapters and actual payment/storage implementation verification remain outside this explicitly selected tracer profile. Power-loss hardware behavior and the declared Rust 1.88 minimum toolchain were not separately tested. These limits do not invalidate the measured local tracer results, but prevent claiming all historical G0–G8 gates passed.

## Durable context

Wai is initialized locally with a funkalization project. The canonical source distillation is `.wai/projects/funkalization/research/2026-10-16-distilled-design-research-and-tracer-decisions.md`; historical inputs and machine-readable evidence live under `research/`. Beads owns task status. No commit, remote sync, deployment or push was performed during this continuation.

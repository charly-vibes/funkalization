# Run the design-review tracer

The tracer demonstrates a storage-substitution review, including a counterexample and stale-decision refusal. Use a Unix machine with Rust, Cargo and a C compiler; run these commands from the repository root.

## Build and run

```sh
cargo build --locked
cargo run --locked -- tracer target/my-tracer
```

Both commands exit 0 on success. The tracer prints a JSON report with `"passed": true` and expected/actual assertions. The directory must not already exist; choose a new name on subsequent runs.

Open `target/my-tracer/view.html` in your browser. Inspect the `bad`, `good` and `stale` proposals. The bad backend advertises the required operations but allows shipment before persistence. The compatible replacement uses direct payment and permits shipment only within the supplied policy's allowed traces. Its pass is conditional on that finite corpus and observation map.

## Inspect evidence and replay

```sh
cargo run --locked -- replay target/my-tracer/workspace.db
cargo run --locked -- view target/my-tracer/workspace.db
```

Replay exits 0 and prints `"replayed": true` with the reconstructed state. View exits 0 and prints source-addressed JSON, including all proposal evidence, applicability and export limitations. The HTML file is a read-only snapshot; use `view DB NEW.html` to generate a refreshed one.

| Artifact | Meaning |
|---|---|
| `report.json` | Expected versus actual tracer assertions and fixture fingerprints |
| `workspace.db` | Persisted decisions, proposals and evidence |
| `source.json` | Original policy and declared initial binding |
| `export.json` | Derived inventory with an explicit loss report |
| `view.html` | Read-only inspection of accepted/proposed bindings and evidence |

## Try your own action

Initialize a separate workspace using the checked-in source fixture:

```sh
cargo run --locked -- init target/manual.db fixtures/pilot/workspace.json
```

Expected: exit 0, JSON state with revision 0. Copy one of the action examples from the [CLI reference](cli.md), then run `apply target/manual.db ACTION.json`. A failed decision exits 2 with a structured error and commits no effect.

## Troubleshooting

`destination` means the output already exists; use a fresh path. `source_changed` means the original source changed or disappeared; historical replay remains available, but mutations require restoring the exact bytes or starting a new workspace. `unresolved` means the strict acceptance conditions are unmet; inspect checker results and declared counterexamples. `stale` means the action targets an earlier accepted revision.

If bundled SQLite compilation fails because a compiler cache points to a read-only directory, give the cache a writable location, for example `CCACHE_DIR=/tmp/funkalization-ccache cargo build --locked`.

Run `just ci` for the Rust checks and `python3 scripts/reproduce_baselines.py` for the recovered Python experiments. These establish different evidence; neither establishes that a production payment service is correct.

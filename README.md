# funkalization

> **Why:** Reviewing a design change should reveal what changed, which obligations
> remain unresolved, and which evidence still applies. funkalization records typed
> proposals, guarded decisions and scoped evidence, with read-only derived views.
> **Status:** [experimental](docs/src/status.md) — persistent local tracer implemented;
> the broader visual workspace remains proposed.

[![CI](https://github.com/charly-vibes/funkalization/actions/workflows/ci.yml/badge.svg)](https://github.com/charly-vibes/funkalization/actions/workflows/ci.yml)
[![Docs](https://github.com/charly-vibes/funkalization/actions/workflows/docs.yml/badge.svg)](https://github.com/charly-vibes/funkalization)
[![crates.io](https://img.shields.io/crates/v/funkalization.svg)](https://crates.io/crates/funkalization)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-yellow.svg)](LICENSE)

A **visual semantic design workspace**: propose, revise, and inspect software
designs while preserving semantic meaning, authority, and evidence. One tool
in the [charly-vibes fleet](https://charly-vibes.github.io/dulce-de-leche/ecosystem-map.html),
installed and updated by [ddl](https://charly-vibes.github.io/dulce-de-leche/).

## Run the tracer

```sh
cargo run --locked -- tracer target/my-tracer
```

Use a new directory. Success exits 0 and prints a report with `"passed": true`.
Open `target/my-tracer/view.html`; inspect `report.json` and replay the database
with `cargo run --locked -- replay target/my-tracer/workspace.db` (exit 0).
See [the walkthrough](docs/src/getting-started.md) for prerequisites and errors.

The implemented profile is in
[add-persistent-tracer](openspec/changes/add-persistent-tracer/design.md).
[Research inputs and evidence](research/README.md) reproduce the historical
experiments and explain deliberate differences from M3/M4.

## The broader contract

The authoritative design source is the committed bundle
**`VSD-IMPLEMENTATION-2026-10-13`** in
[`openspec/design/visual-semantic-design-implementation-2026-10-13/`](openspec/design/visual-semantic-design-implementation-2026-10-13/README.md) —
an implementation and verification contract with:

- **Eight falsifiable claims** (C01–C08): equivalent text/visual behavior,
  geometry independence, safe revision decisions, correct obligations, bounded
  refinement/witnesses, honest UNKNOWN/UNSUPPORTED, lossless-or-explicitly-lossy
  export, and precise source/evidence traceability.
- **Gates G0–G8**: reproduce baseline → canonical model → design-event
  semantics → obligations → finite checks → integration safety → read-only
  diagrams → minimal UI → end-to-end tracer.
- **An order–payment–storage pilot** with fixtures P01–P10 and positive/negative
  trace oracles.

Nothing is called complete without positive **and** negative test evidence.

## Where the specs live

All specification material lives under [`openspec/`](openspec/) — design
bundle in `openspec/design/`, change proposals in `openspec/changes/`,
deployed specs in `openspec/specs/` once gates land. Specs are **not**
duplicated into `docs/` or a separate `specs/` tree.

## Development

```sh
just ci            # the ecosystem gate: fmt --check + clippy -D warnings + tests
cargo test         # just the tests
mdbook build       # the book (docs/src/)
```

## Install

Not yet published. Once `v0.1.0` is tagged: `cargo install funkalization`,
or `ddl install funkalization` from the ecosystem orchestrator.

## Ecosystem position

| | |
|---|---|
| Category | extension |
| Maturity | experimental |
| Workspace | Explicit SQLite path passed to the CLI |
| Dogfood (§4) | pretender · wai · bd · openspec · ddl (pinned in `versions.ddl.toml`) |

Fleet neighbors: **openspec** (change management), **specodelic** (spec format
+ verification), **wai** (session workflow). source write-back remains disabled after an isolated archive-safety probe exposed content loss.

## License

Apache-2.0.

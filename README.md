# funkalization

> **Why:** Design decisions live in prose, diagrams, and agent transcripts that
> carry no authority, evidence, or revision history — agents and humans cannot
> tell an accepted commitment from a casual sketch, and "looks right" replaces
> "is right". funkalization holds a versioned design workspace where accepted
> commitments, proposals, typed design edits, obligations, and scoped evidence
> are first-class events; visual diagrams are derived views that invoke the same
> semantic operations as text/CLI actions and never assign correctness to shapes.
> **Status:** [experimental](docs/src/status.md) · early scaffold — design contract committed, gates G0–G8 not yet implemented · [Motivation & design](docs/src/index.md)

[![CI](https://github.com/charly-vibes/funkalization/actions/workflows/ci.yml/badge.svg)](https://github.com/charly-vibes/funkalization/actions/workflows/ci.yml)
[![Docs](https://github.com/charly-vibes/funkalization/actions/workflows/docs.yml/badge.svg)](https://github.com/charly-vibes/funkalization)
[![crates.io](https://img.shields.io/crates/v/funkalization.svg)](https://crates.io/crates/funkalization)
[![License: Apache-2.0](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A **visual semantic design workspace**: propose, revise, and inspect software
designs while preserving semantic meaning, authority, and evidence. One tool
in the [charly-vibes fleet](https://charly-vibes.github.io/dulce-de-leche/ecosystem-map.html),
installed and updated by [ddl](https://charly-vibes.github.io/dulce-de-leche/).

## The contract, not the claims

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
| Config dir (planned) | `.funkalization/` |
| Dogfood (§4) | pretender · wai · bd · openspec · ddl (pinned in `versions.ddl.toml`) |

Fleet neighbors: **openspec** (change management), **specodelic** (spec format
+ verification), **wai** (session workflow). funkalization's adapters treat
accepted OpenSpec/Specodelic facts as authoritative for what those formats
faithfully express.

## License

Apache-2.0.

# funkalization

**Visual semantic design workspace** — a text-and-visual means of proposing,
revising, and inspecting software designs while preserving semantic meaning,
authority, and evidence.

## 30-second pitch

Design decisions today live in prose, diagrams, and agent transcripts — none
of which carry authority, evidence, or revision history. funkalization holds a
**versioned design workspace** where accepted commitments, proposals, typed
design edits, generated obligations, and scoped evidence are first-class
events. Visual diagrams are *derived views*: they never assign correctness to
shapes, and actions on them invoke the same semantic operations as text/CLI
actions.

The implementation contract is the design bundle
`VSD-IMPLEMENTATION-2026-10-13` — see
[`openspec/design/visual-semantic-design-implementation-2026-10-13/`](https://github.com/charly-vibes/funkalization/tree/main/openspec/design)
for the eight falsifiable claims (C01–C08) and gates G0–G8.

## Install

Not yet published — the repo is an early scaffold. Once `v0.1.0` is tagged:

```sh
cargo install funkalization   # crates.io
ddl install funkalization     # or via the ecosystem orchestrator
```

## Ecosystem

funkalization is one tool in the [charly-vibes fleet](https://charly-vibes.github.io/dulce-de-leche/ecosystem-map.html),
installed and updated by **ddl** (`dulce-de-leche`). Related fleet tools:
**openspec** (change management), **specodelic** (spec format + verification),
**wai** (session workflow).

## Status

Experimental — see [status.md](./status.md).

# Getting started

> **Note:** funkalization is an early scaffold implementing gates G0–G8 of the
> [design contract](https://github.com/charly-vibes/funkalization/tree/main/openspec/design).
> The commands below describe the target surface; the shipped binary currently
> exposes `--version` only.

## Build from source

```sh
git clone https://github.com/charly-vibes/funkalization.git
cd funkalization
cargo build --release
```

## Evaluate the whole repo the standard way

Every ecosystem repo answers to `just ci`:

```sh
just ci   # fmt --check + clippy -D warnings + tests
```

## First design workspace (planned, G1–G2)

```sh
funkalization init        # create a versioned design workspace
funkalization propose     # record a typed design edit as a proposal
funkalization decide      # accept/reject with obligations + evidence
funkalization views       # render deterministic (read-only) projections
```

## Where the specs live

All specification material lives under `openspec/` in the repo — the design
contract bundle in `openspec/design/`, change proposals in `openspec/changes/`,
and deployed specs in `openspec/specs/` once gates land. Specs are **not**
duplicated elsewhere; the book links into the repo instead of copying.

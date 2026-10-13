# Status

**Maturity: [experimental](https://charly-vibes.github.io/dulce-de-leche/ecosystem-map.html)** —
exploratory; may be renamed or sunset (ecosystem §5 vocabulary).

## Implemented

| Command / area | Status | Notes |
|----------------|--------|-------|
| `funkalization --version` | ✅ working | ddl-compatible semver output |
| CLI scaffold (`just ci` gate) | ✅ working | fmt + clippy + tests |
| Design contract bundle | ✅ committed | `openspec/design/visual-semantic-design-implementation-2026-10-13/` |

## In progress

| Area | Status | Notes |
|------|--------|-------|
| Gate G0 — reproduce baseline | ◻ not started | per ACCEPTANCE.md of the contract |
| Gates G1–G4 (workspace, events, obligations, finite checks) | ◻ not started | implementation contract, not yet built |
| Gates G5–G8 (integration, read-only diagrams, UI, tracer) | ◻ not started | nothing is "complete" without positive *and* negative test evidence (C01–C08) |

## Mapped to specs

No deployed `openspec/specs/` yet — the normative requirements are the
`VSD-*` SHALLs in the design contract bundle under `openspec/design/`. They
graduate into `openspec/changes/` → `openspec/specs/` as gates land. Specs
live **only** under `openspec/` (no duplicate `specs/` trees; book links into
the repo rather than copying).

## Dogfooding

funkalization is new — no other ecosystem tool consumes it yet. Its own CI
installs and pins pretender, wai, bd, openspec, and ddl (§4 matrix row), and
ddl manages it in the registry (experimental/extension).

## Known unknowns

From the contract: Git HEADs of source snapshots, tool-version compatibility,
Wanna integration behavior, human usability, general semantic soundness — all
honestly UNKNOWN. See the contract's RISKS.md.

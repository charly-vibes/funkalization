# Project Context

## Purpose
Help maintainers review design changes by preserving proposed and accepted bindings, unresolved obligations, scoped evidence and revision history. The implemented experiment is the payment–storage tracer; general architecture authoring and interactive semantic editing remain future work.

## Tech Stack
Rust 2021 (declared minimum 1.88), serde JSON, SHA-256 and bundled SQLite through rusqlite. The v1 workspace requires Unix UID authority and a local filesystem. Python 3.9+ runs the archived research suites and optional integration probe. mdBook builds user documentation.

## Architecture
`src/model.rs` defines the typed protocol, normalization, finite evaluator and state reducer. `src/store.rs` owns SQLite transactions and replay. `src/view.rs` derives JSON/HTML without semantic writes. `src/tracer.rs` exercises the public protocol using fixtures. `src/main.rs` adapts CLI arguments; it contains no alternate decision semantics.

Policy facts belong to the immutable source JSON. The workspace owns binding choices and evidence. A changed source fingerprint blocks mutations; v1 requires a new workspace for a revised source. No write-back to OpenSpec/Specodelic is enabled. All specs and decisions are under `openspec/`; executable fixtures are under `fixtures/`.

## Code and Testing
New source modules carry Purpose/Responsibilities/Rationale headers. Run `just ci` for formatting, Clippy and tests. Use failing behavioral tests before implementation. Cover negative decisions, evidence provenance, boundedness, process death, concurrency and replay. A green finite-model test is not evidence about a running storage service. The exact current profile is `openspec/changes/add-persistent-tracer/design.md`; `src/model.rs` supplies its serialization types.

## Git and Work Tracking
Use `bd` for tasks and persistent project memory; use wai for research and decision rationale. The local Beads configuration currently selects no-db mode; use the CLI rather than assuming a storage backend. Do not commit, push or sync remotes without explicit authorization under the conservative profile. Keep changes and deployments distinct: a validated proposal is not a deployed spec.

## Domain and Limits
The pilot requires accepted payment and a durable record before shipment eligibility. Nominal operation inclusion and finite observed behavior are separate obligations. Candidate labels alone do not establish actual atomicity or durability. The profile uses prefix-closed finite corpora and a concrete-event horizon; it deliberately differs from historical M4. Acceptance is a stricter review policy than M3's exploratory acceptance. Full compatibility is not claimed.

## Source Baselines
Six exact inputs recovered from `.scratch` are preserved in `research/archives/`. `python3 scripts/reproduce_baselines.py` checks the original hashes and runs M2/M3/M4 plus the M3 explorer and a negative harness control. Two historical Specodelic/visual-integration archives remain unavailable. Current installed-tool probes have distinct provenance in `research/evidence/archive-probe.json`.

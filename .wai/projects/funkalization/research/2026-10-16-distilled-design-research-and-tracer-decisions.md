---
tags: [semantics, baseline, tracer]
bead: funkalization-8d0
---

# Distilled design research and tracer decisions

Distilled 2026-10-16 from six hash-matching inputs recovered from `.scratch`, preserved unchanged in `research/archives/`. This is implementation context, not task tracking or a new normative spec. The normative slice is `openspec/changes/add-persistent-tracer/`; Beads owns work status.

## What the research establishes

The original software-design substrate describes designing as a revisable activity: top-down commitments meet bottom-up constraints; contradictory and incomplete proposals remain visible. Its E01–E10 scenarios separate model-only simulation, independent examples, implementation traces, authority and presentation. It proposes a research direction, not a working product.

M1 supplies an institution/translation/satisfaction framework and candidate laws. Its architecture decision explicitly says to define a narrow semantic profile and demonstrate a second consumer before extracting a generic kernel. Typed structural graphs and categorical vocabulary alone establish neither behavioral refinement nor modularity. The separate semantic_model_context_evaluation recommends the order–payment–storage experiment and warns about premature generality and user friction.

M2 separates accepted design, alternatives, obligations, judgments and history; its 16 tests cover operational distinctions. M3 reduces the event vocabulary to propose/decide/report, while typed edits generate obligations. Acceptance means authorization, not proof. Incompatible or inconsistent bindings may be accepted for exploration. Reports are method-scoped, and public reports cannot impersonate decisive built-in checks. M3 uses conservative global digest invalidation, not a proven selective dependency analysis; its reference machine does not implement durable replay.

M4 defines an experimental finite contract policy: consumer assumptions must be included in provider assumptions, while provider guarantees must be included in consumer guarantees. Its languages do not encode state-dependent rely/guarantee behavior. Trace refinement uses an explicit observation map; prefix closure is not implicit. Disjoint-alphabet interleavings are combinatorial examples, not a general composition theorem. The bridge sends non-decisive evidence to M3. No liveness, deadlock, realizability or running-software guarantee follows.

## Reproduced evidence

`python3 scripts/reproduce_baselines.py` verifies original manifest SHA-256 values and runs M2 16/16, M3 50/50, M4 18/18, M3 exploration of 2,801 sequences, and an intentionally failing harness control. Exact stdout/stderr and environment are in `research/evidence/baseline-results.json`. Historical Git commits remain unknown; exact bytes are known. The original Specodelic 0.8.0 ZIP and visual-integration dossier ZIP remain absent.

`python3 scripts/probe_archive.py` pins installed versions and records entrypoint hashes. With Specodelic 0.9.0 and OpenSpec 0.19.0, disposable git repositories reproduce additive-delta overwrite, historical archive overwrite of a later fact, and a partial multi-file write when a later destination fails. This is actual CLI evidence, separate from the missing 0.8.0 snapshot. The first probe lacked its own git root and did not exercise the intended path; the corrected harness asserts repository isolation before running. No real accepted specs were modified. On 2026-10-16 the installed tool had become 0.10.0; the original pin correctly refused it. After source inspection, an explicitly selected 0.10.0 probe reproduced the same three hazards and recorded separate evidence in research/evidence/archive-probe-0.10.0.json.

## Decisions in the Rust tracer

One SQLite transaction commits state and event history. The original policy owns required operations and allowed traces; the workspace owns binding choices and evidence. Source drift blocks mutation. Effective Unix UID is the local authority; the trusted owner can inspect or alter its own files, so this is not a multi-user security service. Replay embeds original source bytes and detects snapshot/event inconsistency.

The Rust profile is an explicit specialization, not a verified port: it requires scoped passes before accepting a replacement, uses prefix-closed observations, counts concrete events before hiding, and requires a successful shipping trace to avoid a never-ship demonstration. M3 permits unproved exploratory acceptance, and M4 does not assume prefix closure and describes an observable horizon. These differences must remain visible. Later counterexamples can challenge an accepted binding without automatic revocation.

The pilot compares advertised operations separately from behavior. A nominally compatible store that permits request→pay→ship fails; a compatible direct-payment replacement can pass the supplied policy language. Durability is modeled, not tested against a real storage service. The fixture corpus and separate temporal assertions were written by the same implementation author; human-independent validation is pending.

Views are read-only and source-addressed. Derived export retains the inventory and emits loss limitations; no source write adapter or semantic editor is enabled. This follows the original bundle's explicit read-only fallback. General edits, arbitrary composition, rich UI, kernel extraction and cross-language equivalence remain outside this slice.

## Value hypothesis and next evidence

The maintainer's job is reviewing a proposed substitution: identify changed commitments, open obligations and stale evidence without rereading agent transcripts. Compare against an existing OpenSpec/text workflow; include authoring time and false alarms. A technically correct graph does not establish user benefit. The user volunteered as first pilot reviewer. Use `openspec/experiments/review-value.md`; record their actual answer and timing, never an inferred result.

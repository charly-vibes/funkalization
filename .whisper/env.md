# funkalization — repo-level infra facts

- Rust CLI crate `funkalization` (0.1.0); binary exposes: init DB SOURCE.json | apply DB ACTION.json | show DB | replay DB | view DB [NEW.html] | export DB NEW.json | archive DB | tracer NEW_DIRECTORY
- Quality gate: `just ci` (fmt-check, clippy, cargo test). Never bypass.
- ddl-managed ecosystem tool #12 (extension/experimental). Compat floor >=0.1.0; `ddl install funkalization` fails until the first tag exists.
- CI + mdBook live at charly-vibes.github.io/funkalization; docs build via `just docs-build`. Release page generated at build time — never committed.
- Specs: `openspec/` is the ONLY spec home (user-mandated; no duplicate specs/ trees). Design contract: openspec/design/visual-semantic-design-implementation-2026-10-13/ (gates G0–G4+ per ACCEPTANCE.md).
- Beads (`bd`) currently uses no-db mode with a tracked JSONL export; use the CLI rather than assuming a database backend. Task status authority: bd, not markdown checkboxes.
- Turu/whisper routing: repo-local `.whisper/` (branch slug `main`, repo key `cv/charly-vibes/funkalization`). `.whisper/private/` and `notes.usage.jsonl` are machine-local, git-ignored.
- 2026-10-16T19:08:36Z [id:9a3709a6df5ce2d97d37c3bbe748e1cac42428e7db104262c3d38256509ca86a] Recovered historical inputs live in research/archives; reproduce with scripts/reproduce_baselines.py. The Rust tracer is a deliberate specialization of M3/M4, not a port-equivalence claim. Specodelic 0.9.0 and 0.10.0 archive probes show overwrite and partial-write hazards; source write-back stays disabled. Wai research and the requirement coverage matrix record provenance and boundaries.

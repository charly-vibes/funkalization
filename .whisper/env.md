# funkalization — repo-level infra facts

- Rust CLI crate `funkalization` (0.1.0); binary exposes: init DB SOURCE.json | apply DB ACTION.json | show DB | replay DB | view DB [NEW.html] | export DB NEW.json | archive DB | tracer NEW_DIRECTORY
- Quality gate: `just ci` (fmt-check, clippy, cargo test). Never bypass.
- ddl-managed ecosystem tool #12 (extension/experimental). Compat floor >=0.1.0; `ddl install funkalization` fails until the first tag exists.
- CI + mdBook live at charly-vibes.github.io/funkalization; docs build via `just docs-build`. Release page generated at build time — never committed.
- Specs: `openspec/` is the ONLY spec home (user-mandated; no duplicate specs/ trees). Design contract: openspec/design/visual-semantic-design-implementation-2026-10-13/ (gates G0–G4+ per ACCEPTANCE.md).
- beads (`bd`) runs embedded (no-db, local Dolt under .beads/, which is git-ignored here). Task status authority: bd, not markdown checkboxes.
- Turu/whisper routing: repo-local `.whisper/` (branch slug `main`, repo key `cv/charly-vibes/funkalization`). `.whisper/private/` and `notes.usage.jsonl` are machine-local, git-ignored.
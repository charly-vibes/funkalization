# funkalization

funkalization helps a maintainer review design changes: what changed, which obligations remain unresolved, and which evidence still applies. The current experimental release is a local payment–storage tracer with persistent decisions and read-only views.

Reviewed prose and version control already preserve decisions and history. This experiment adds typed changes, revision guards, explicit obligations and scoped machine-readable evidence. Whether those additions reduce review effort is being tested separately from technical correctness.

## Try the experiment

Start with [Run the design-review tracer](getting-started.md), then inspect the [CLI reference](cli.md) and [workspace storage](configuration.md). The [status page](status.md) separates the working slice from the larger proposed visual workspace.

## Design contract

The historical `VSD-IMPLEMENTATION-2026-10-13` bundle defines the broader claims C01–C08 and gates G0–G8. The implemented slice is specified by `openspec/changes/add-persistent-tracer/`; both live in the [repository's openspec directory](https://github.com/charly-vibes/funkalization/tree/main/openspec).

Interactive semantic editing, general composition and safe source write-back are not implemented. A passing model check is scoped evidence, not proof of a running implementation.

## Ecosystem

funkalization is an experimental [charly-vibes fleet](https://charly-vibes.github.io/dulce-de-leche/ecosystem-map.html) tool. OpenSpec supplies change-management conventions, Specodelic is a prospective integration, wai records research and rationale, and Beads tracks work. Build from source; no published installation is claimed here.

# Workspace storage and authority

The v1 tracer uses a SQLite file at the path you pass to the CLI. It reads no global configuration and does not automatically discover a `.funkalization/` directory.

## Persisted state

The database stores the initial snapshot, current snapshot, and ordered actions. Each successful mutation commits its event and resulting state in one transaction. Reopening and `replay` verify the stored history against the current snapshot.

The source JSON path and original bytes are retained with a SHA-256 fingerprint. Source edits block further mutations; create a new workspace for a changed source. Replay of historical events still works from the embedded source snapshot, while projections report that the live source is no longer current.

## Decision authority

The Unix effective UID at initialization owns the workspace. Action JSON cannot claim another identity. Anyone who can alter the database as that OS user is inside the trust boundary; this is not a distributed authorization service.

The source policy controls requirements and allowed traces. The workspace controls the selected binding, proposals and evidence. Accepting a binding never modifies source specifications. A human report can challenge an accepted binding without silently replacing it.

## Recovery and limits

A failed action commits neither event nor state. An interrupted uncommitted SQLite transaction rolls back. If a response is lost after commit, inspect `show` or `replay` before retrying: duplicate proposal and decision effects are refused.

Inputs and snapshots are limited to 1 MiB; a model has at most 128 traces of 32 events, and a workspace has at most 256 proposals and 1,024 evidence records. The lock wait is five seconds. Schema/profile mismatches fail explicitly; no automatic migration exists in v1. SQLite commits rely on the local filesystem honoring synchronization. Power-loss durability and network filesystems have not been independently tested.

Derived HTML and JSON exports only create new files. They refuse overwrite and are not an import or source round-trip format. Archival writes remain disabled.

# CLI reference

Shipped surface (v1 tracer bullet — persistent local design-review
workspace):

```text
funkalization init DB SOURCE.json      # create workspace; SOURCE bytes are fingerprinted immutable inputs
funkalization apply DB ACTION.json     # apply a propose/evaluate/report/decide action in one transaction
funkalization show DB                  # print current replayed state
funkalization replay DB                # reduce all events from genesis and verify digests
funkalization view DB [NEW.html]       # read-only JSON projection; with a path, write an inspectable HTML view
funkalization export DB NEW.json       # derived export (explicitly not a round-trip channel)
funkalization archive DB               # fails closed — archive/recovery is separately tracked (funkalization-91v)
funkalization tracer NEW_DIRECTORY     # self-checking tracer: fixtures, protocol run, machine-readable evidence
```

Successful operations exit 0 even when a recorded evaluator verdict is
fail/unknown; invocation and decision errors exit 2. The tracer exits
nonzero when expected outcomes do not match.

`--version` prints `funkalization <semver>`; unknown commands exit 2.
Semantics are normative in the
[design-tracer spec delta](https://github.com/charly-vibes/funkalization/tree/main/openspec/changes/add-persistent-tracer)
and the [implementation contract](https://github.com/charly-vibes/funkalization/tree/main/openspec/design).

## Action JSON examples

`apply` uses a tagged JSON object. Unknown fields are rejected. The seed and binding shapes are illustrated in `fixtures/pilot/workspace.json` and `fixtures/pilot/compatible.json`; `src/model.rs` defines the serialization types.

To propose a binding, save an object with `"action": "propose"`, a unique `"id"`, `"base_revision": 0`, and `"binding"` containing the complete binding object. Propose preserves the accepted state. Then evaluate and decide with separate files:

```json
{"action":"evaluate","proposal_id":"replacement"}
```

```json
{"action":"decide","proposal_id":"replacement","base_revision":0,"decision":"accept"}
```

Each successful call exits 0 and prints the resulting state. Acceptance requires both current checker passes and no declared failure. Use `"decision":"reject"` to record rejection without changing the accepted revision. Repeating a completed decision exits 2 with `decided`; a stale revision exits 2 with `stale`.

A declared report can be added even after acceptance:

```json
{"action":"report","proposal_id":"replacement","verdict":"fail","note":"Observed shipment before durable persistence"}
```

A successful report exits 0 and preserves the accepted binding and revision. It remains separate from checker evidence; a later counterexample is displayed as a challenge, not silently discarded or used to rewrite history. Reports on retired proposals do not transfer to newer bindings.

## Output and failure contract

Errors are JSON objects with `code` and `message` on stderr. No action accepts an `authority` field; the host obtains the Unix UID. Relevant codes include `invalid_json`, `invalid_id`, `limit`, `version`, `unauthorized`, `source_changed`, `not_found`, `duplicate`, `decided`, `stale`, `unresolved`, `integrity`, `database`, `destination` and `unsupported`.

`archive DB` always exits 2 with `unsupported`; it never invokes the unsafe archive companion. A derived export is not an archival substitute and always declares that round-trip source writes are unsupported.

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

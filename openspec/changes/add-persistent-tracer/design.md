# Persistent tracer v1 decisions

This contract specializes the historical VSD bundle. It does not claim all G0–G8 are implemented. Historical documents remain immutable provenance. The self-contained Rust baseline replaces unavailable M3/M4 runtime dependencies for this experiment; it is not a reproduction or verified port of those experiments.

## Authority and scope

Single-host, same-OS-user workspace. The UID recorded at initialization is the decision authority; callers cannot send an authority field in actions. Local users able to alter the database have the same trust as its owner; this is not a multi-user security boundary. Other UIDs cannot mutate it. Unknown JSON fields are rejected.

The imported JSON policy owns required operations, allowed traces, alphabet/closure and horizon. Its source bytes are fingerprinted and rechecked before every mutation. External edits require a new workspace, never automatic reconciliation. The workspace owns binding choices, proposals, evidence and accepted revision. Acceptance affects only this local binding choice, never the source policy or an OpenSpec file. The initial binding is a declared baseline, not verified evidence. All subsequent accepted bindings require two applicable checker passes (nominal and bounded behavioral); acceptance remains bounded, not proof about running software.

## Model and canonicalization

Authoritative types are in src/model.rs; JSON uses snake_case tagged actions and deny_unknown_fields. Schema/profile/evaluator versions are fixed strings. IDs match [A-Za-z0-9_-]{1,64}. Sets use sorted BTreeSet; maps use BTreeMap. Trace collections are lexicographically sorted and deduplicated before hashing; event order inside each trace is preserved. Typed canonical JSON has integers and strings only. SHA-256 of a profile-tagged policy/binding pair identifies semantic content; name, revision, evidence, proposals, timestamps and layout are excluded. This is a domain canonicalizer, not a claim of RFC 8785 compatibility. Policy is included in every judgment scope.

## State and transactions

SQLite stores a genesis snapshot, current snapshot and contiguous ordered event rows. BEGIN IMMEDIATE serializes writers; busy timeout is five seconds. Each successful action produces one event and snapshot in the same FULL-synchronous transaction. A failed action commits neither. Connection termination before commit rolls back. A successfully committed action may have an unobserved response; duplicate proposal/decision requests are rejected without a second effect. The CLI can show/replay state to resolve uncertainty. Schema/version mismatch fails closed; v1 has no automatic migrations. Replay reduces all ordered actions from genesis, rechecks stored digests, and compares the full reconstructed state. The source snapshot is embedded for historical replay; live source drift is reported separately.

## Actions

propose(id, base_revision, binding): validate ID, unique proposal ID and matching accepted revision; preserve semantic counterexamples and incomplete mappings as a pending proposal. Structural/size errors are rejected. No accepted-state change.

evaluate(proposal_id): pending proposal at current base only; append checker evidence from the built-in evaluator. No accepted-state change.

report(proposal_id, verdict, note): append declared human evidence only. It never discharges an obligation or impersonates checker evidence. A human fail blocks strict acceptance until a new proposal is reviewed; all origins remain visible.

decide(proposal_id, base_revision, accept|reject): pending proposal and both revision guards must match. Reject records a rejected proposal without revision change. Accept requires applicable built-in nominal+behavioral passes and no human fail, then installs binding and increments revision once. Old pending proposals become stale by revision mismatch. Duplicate decisions fail.

All effects return JSON state or a structured error code. Commands exit 0 on successful operations even if a recorded evaluator verdict is fail/unknown/unsupported; invocation/decision errors exit 2. Tracer exits nonzero when expected outcomes do not match.

## Finite evaluator and independent oracle

The policy contains explicit finite prefix-closed observations, represented by maximal traces with all prefixes accepted. Concrete events map to observable labels or tau; missing mapping yields unknown. Horizon counts concrete events before hiding. Supported limits: 1 MiB input/snapshot, 128 traces, 32 events per trace, 32 horizon, 64 operations/mappings, 256 proposals and 1024 evidence records per workspace. Exhausted semantic bounds yield unknown; malformed/oversized structure is an input error. No unbounded exploration, liveness, fairness, general composition or wall-clock semantics is attempted; unsupported shared-event composition yields unsupported. Empty concrete traces or a policy with no successful ship trace cannot pass. A declared closed finite corpus is a claim about this supplied corpus only, not completeness of a real implementation.

Nominal check is required-operation inclusion. Behavioral check maps each trace and checks membership in policy prefixes, plus requires at least one successful ship trace in the candidate to prevent the never-ship loophole. A failure returns the shortest offending concrete prefix (lexicographic tie-break) and its observable projection. The hand-written policy fixture is independent of the evaluator. A separate test-only state-machine oracle checks its safety meaning: request opens an order; pay accepts payment; persist records that acceptance durably; ship requires paid and durable; crash loses unpersisted payment and requires restart; durable acceptance survives restart. Payment/persistence failures never grant eligibility. Repeated request/persist are idempotent in the fixture's one-order scope. Eligibility is defined as paid AND durable AND running; ship tests that predicate. Real storage crash durability is not tested.

## Evidence, projections and adapters

Obligations are nominal-interface and payment-durability, with proposal ID, subject digest, source fingerprint, profile/evaluator, horizon, closure and observation mapping. Only built-in checker records in the same exact scope count. Explicit fail dominates; unknown/unsupported never pass. Stale evidence is displayed but never reused. Read-only JSON and HTML projections preserve isolated accepted/proposal nodes, distinct obligation identities and source references. HTML selection/inspection does not mutate anything; there is no editing UI or C01 claim in v1.

Export creates a new derived JSON inventory containing all proposals and evidence, with an explicit loss report for unsupported source round-tripping. It never overwrites existing files or writes accepted specs. Archive always returns unsupported without mutation. This is the historical contract's read-only fallback, not a fix to upstream Specodelic. Filesystem sandbox/symlink attacks by the trusted OS owner are outside the threat model; destination overwrite is still refused. Source inputs must be regular files.

## Product experiment

The initial job is reviewing a storage substitution proposed by an agent. Compare existing text/spec review to this workspace using the protocol in openspec/experiments/review-value.md. Technical success does not establish adoption, time savings, or commercial value. No human results are synthesized.

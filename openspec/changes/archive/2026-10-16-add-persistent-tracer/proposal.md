# Change: Persistent design-review tracer

## Why
The evaluation found unresolved authority, interfaces, persistence and pilot oracles. The user requested all findings be addressed and a tracer bullet built. This change makes a narrow, independently runnable slice and records unavailable historical inputs and human validation without claiming they ran.

## What Changes
- A Rust local-workspace CLI implements proposal, evaluation, guarded decision, evidence, replay, projection and derived export.
- A SQLite transaction commits state and ordered events together; imported source bytes are immutable inputs.
- Independent explicit trace fixtures cover payment, durable persistence, crash/restart and shipping.
- A self-checking tracer emits machine-readable evidence and an inspectable read-only HTML view.
- Archive/source write adapters fail closed; historical archive reproduction and empirical user studies remain separately tracked.

## Impact
- Affected specs: design-tracer (new proposed capability).
- Affected code: src/, tests/, fixtures/pilot/, Cargo.toml, docs/.
- Approval: user instruction “fix all and create a tracer-bullet” authorizes this remediation and implementation.

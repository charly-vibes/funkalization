---
title: "Risks, limitations, assumptions, unknowns and open decisions"
date: "2026-10-13"
model: "OpenAI GPT-6"
status: "OPEN DESIGN REVIEW"
source_commits: "UNKNOWN"
---
# Risks, limitations and unresolved decisions

## Priority risk register

| Risk | Priority | Failure mode | Test or mitigation | Status |
|---|---|---|---|---|
| R01 archive delta overwrite | CRITICAL | Existing accepted spec replaced rather than semantically merged | G5 archive/re-read of ADDED/MODIFIED/REMOVED/RENAMED; failed write injection | Source-inspection concern; CLI not verified here |
| R02 competing authoritative representations | HIGH | Workspace acceptance differs from accepted spec corpus | Ownership map per field and round-trip/loss check | OPEN |
| R03 incomplete parsing yields seemingly complete graph | HIGH | Components missing without user knowing | G6 malformed-corpus and completeness test | Source-inspection concern; live path unverified |
| R04 reduced graph loses contract identity/isolated nodes | HIGH | Critical change invisible in overview | Contract-level ID and isolated-node fixtures | Known representation limitation in prior review |
| R05 architecture edge mistaken for behavioral proof | HIGH | False substitutability | Distinct lifecycle/evidence statuses; G3/G4 | OPEN |
| R06 unsafe stale approval or duplicate effect | HIGH | Concurrent human/agent change overwrites decisions | Revision-guarded G2/G7 negative cases | OPEN |
| R07 unsupported semantic projection | HIGH | Alternatives/hypotheses/evidence dropped on export | Machine-readable loss report, rejection if destructive | OPEN |
| R08 vacuous refinement | HIGH | Empty concrete model appears to refine anything | Require non-vacuous supported model and explicit closure | OPEN |
| R09 isolated contract checks hide composition failures | HIGH | Deadlock/circular assumptions despite local success | P10 returns unsupported; later synchronized I/O model | Current M4 limitation |
| R10 evidence stale under semantic-profile change | MEDIUM | Old results inappropriately reused | Evidence dependencies include evaluator, profile and assumptions | OPEN |
| R11 conflated `missing` / `forbidden` / `unknown` | MEDIUM | Open-world incomplete spec treated as safety property | Explicit closure and support/refutation policy | OPEN |
| R12 user overload | MEDIUM | Formalism slows design more than diagrams help | Separate empirical UX study of task time, errors and correction | UNKNOWN |
| R13 language/paradigm transfer loss | MEDIUM | Same format hides incompatible semantics | Versioned partial adapters and loss reports | UNKNOWN |
| R14 path/identity renames break provenance | MEDIUM | Historic evidence points to wrong claims | Stable semantic IDs and revision-addressed links | OPEN |
| R15 scenario/oracle circularity | MEDIUM | Model-generated tests only validate themselves | Independent prewritten negative and positive oracles | OPEN |

## Edge-case coverage inventory

- **Structure:** unconnected capability; two contracts between same files; cycle in strict containment; optional requirement; dangling target; rename; dependency inversion; dynamic dependency unknown; multiple providers with distinct conditions.
- **Design activity:** competing speculative branches; contradictory accepted claims; rejected proposal; stale proposal; duplicate event; interrupted write; partial approval; semantic vs cosmetic edit; undo after source export.
- **Behavior:** empty model (vacuity); hidden event divergence; unmapped action; nondeterministic outcomes; timed-out evaluator; finite horizon miss; liveness/fairness not represented; circular peer assumptions; race and interleaving; retry after uncertain persistence; idempotency and ordering.
- **Interoperation:** delta that is partial rather than deployable specification; two changes to one capability; historical archive replay; unsupported metadata; cross-repository contract; unpinned CLI; partial read and parse error; untrusted source evidence.
- **UX:** huge graph; zoom hiding obligations; stale view after accepted revision; multiple links with same endpoints; visual selection interpreted as semantic change; absent source path; review authority differs from verification.

## Explicit assumptions

A01. The supplied snapshots represent relevant versions for design planning; they do not establish upstream HEAD or release parity.

A02. A small finite trace/transition system is an appropriate *experiment*, not a universal behavioral language.

A03. The initial pilot uses serializable, revision-guarded single-workspace decisions; full distributed collaborative transaction semantics are out of scope.

A04. Relevant claims may be incomplete and acceptance does not imply truth or realizability.

A05. OpenSpec and Specodelic are reused for what they actually represent; no magic extensibility from pack names is assumed.

A06. A design abstraction must identify which observations and properties are preserved; diagram similarity has no semantic authority.

A07. Additional claims beyond the selected finite verifier's scope must remain pending, unknown or unsupported.

## Open questions (all unresolved)

Q01. What are the exact Git commits, compatible binary/CLI versions and formal schema version of each source? **UNKNOWN.**

Q02. What single schema/serialization and canonical hash algorithm should be used for workspace state? **UNKNOWN — decide G1.**

Q03. Which data are authoritative in OpenSpec/Specodelic and which remain only in the workspace? **UNKNOWN — decide before G5.**

Q04. What guarantees atomicity of accepted design updates across workspace files and externally written specs? **UNKNOWN.**

Q05. Which specific `requires`, `part_of`, `refines` and effect concepts belong in Specodelic native fields, extension metadata or the independent workspace? **UNKNOWN — test mapping loss.**

Q06. What are the supported semantics for synchronized/shared events or state-dependent assume–guarantee composition? **UNKNOWN; not in the limited M4 profile.**

Q07. How will independently supplied oracles be gathered, labeled and evaluated? **UNKNOWN — experimental protocol required.**

Q08. Are Wanna authorization, persistence and recovery primitives compatible with this semantic revision protocol? **UNKNOWN — inspect and test, do not assume.**

Q09. How will checker outcomes be reconciled when a finite pass and a real-world counterexample coexist? **UNKNOWN — preserve both origins/scope.**

Q10. Does a rich visual interface measurably improve design reasoning versus readable structured text and generated diagrams? **UNKNOWN — UX study required.**

Q11. Which features are truly needed from a separate categorical semantics core and which can remain specification-time laws? **UNKNOWN — defer extraction until a second consumer.**

Q12. Can the existing archive companion implement safe merge semantics without major changes, or should archival be delegated to a pinned upstream OpenSpec workflow? **UNKNOWN — settle via G5 end-to-end executions.**

## Architecture decisions to record during pilot

For each decision, capture: `decision_id`, options, chosen option, competing evidence, assumptions, affected requirements/obligations, base revision, authority, rejected alternatives, and scope. Avoid global judgments like "guarantees modularity" unless a precise policy and test establish the claim.

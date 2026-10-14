## ADDED Requirements

### Requirement: Explicit local authority
The system SHALL preserve imported policy bytes and permit workspace mutations only by the recorded local OS principal against the unchanged source fingerprint.

#### Scenario: Source changes after import
- **GIVEN** a workspace initialized from a policy file
- **WHEN** that file is edited before a proposal or decision
- **THEN** the mutation fails with source_changed and no committed event or state change

#### Scenario: Caller supplies an authority field
- **WHEN** an action includes an unknown authority field
- **THEN** strict decoding rejects it

### Requirement: Atomic guarded actions
The system SHALL commit each successful typed action and its state atomically, reject stale or duplicate decisions, and reproduce the entire state by replay.

#### Scenario: Restart after acceptance
- **WHEN** a checked current proposal is accepted and the process restarts
- **THEN** its binding and revision survive and event replay reconstructs the same state

#### Scenario: Stale or duplicate acceptance
- **WHEN** a decision uses an old revision or an already decided proposal
- **THEN** no state or event is committed

#### Scenario: Interrupted transaction
- **WHEN** the process dies before transaction commit
- **THEN** reopening preserves the previous committed state and replay succeeds

### Requirement: Scoped finite obligations
The system SHALL evaluate nominal-operation inclusion and bounded observable trace inclusion independently, preserve counterexamples, and never treat unknown or unsupported as pass.

#### Scenario: Storage loses durability
- **WHEN** a backend exposes pay then ship without persist
- **THEN** behavioral evaluation fails with a replayable offending prefix even if nominal inclusion passes

#### Scenario: Valid durable shipment
- **WHEN** all closed traces map into the policy prefix language and at least one ships after durable payment
- **THEN** behavioral evaluation passes with its concrete-event horizon and source/model scope

#### Scenario: Incomplete or unsupported model
- **WHEN** a map is missing, closure is absent, a model is empty or exceeds its horizon
- **THEN** behavioral evaluation is unknown
- **AND WHEN** shared-event composition is requested
- **THEN** behavioral evaluation is unsupported

### Requirement: Honest evidence and decisions
The system SHALL distinguish declared reports from built-in checker evidence and accept only current proposals with applicable nominal and behavioral passes and no reported failure.

#### Scenario: Forged or contradictory report
- **WHEN** a human report says pass without checker evidence
- **THEN** acceptance is refused
- **AND WHEN** a human fail coexists with checker passes
- **THEN** both are retained and acceptance is refused

### Requirement: Safe derived artifacts
The system SHALL provide source-addressed read-only projections and loss-reporting derived export, refuse destination overwrite, and reject source archival in this profile.

#### Scenario: Archive attempt
- **WHEN** archive is requested
- **THEN** unsupported is returned and source, state and event log remain unchanged

#### Scenario: Export to existing destination
- **WHEN** a derived export targets an existing file
- **THEN** it is refused and the original bytes remain intact

### Requirement: Reproducible tracer
The system SHALL run a clean-directory tracer using checked-in independent fixtures, emit expected-versus-actual evidence, and fail if any assertion is unmet.

#### Scenario: Cold run
- **WHEN** a reader runs the documented tracer command in a fresh directory
- **THEN** compatible and incompatible storage, unknown and unsupported checks, stale and duplicate decisions, restart/replay, safe export and archive refusal are exercised
- **AND** a machine-readable report and inspectable HTML projection are created

### Requirement: Bounded input and version handling
The system SHALL reject oversized or malformed JSON and unsupported schema/profile versions with structured errors and no partial accepted mutation.

#### Scenario: Unknown schema version
- **WHEN** an input or persisted state uses an unsupported version
- **THEN** loading fails explicitly rather than interpreting it as the current profile

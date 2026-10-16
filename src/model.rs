//! Purpose: Define and reduce the bounded design-review protocol.
//! Responsibilities: validate typed inputs, derive scoped obligations and enforce decisions.
//! Rationale: funkalization-5fs/jg3 freeze a small deterministic profile before UI expansion.

use crate::{error, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const PROFILE: &str = "payment-storage-v1";
pub const EVALUATOR: &str = "finite-prefix-v1";
pub const MAX_BYTES: usize = 1_048_576;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub id: String,
    pub required_operations: BTreeSet<String>,
    pub horizon: usize,
    pub closed: bool,
    pub traces: Vec<Vec<String>>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub id: String,
    pub provides: BTreeSet<String>,
    pub closed: bool,
    pub composition: String,
    pub traces: Vec<Vec<String>>,
    pub observation_map: BTreeMap<String, String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Seed {
    pub schema_version: u32,
    pub policy: Policy,
    pub binding: Binding,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Pass,
    Fail,
    Unknown,
    Unsupported,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Judgment {
    pub subject_revision: u64,
    pub subject_digest: String,
    pub source_fingerprint: String,
    pub profile: String,
    pub evaluator: String,
    pub horizon: usize,
    pub closed: bool,
    pub observation_map: BTreeMap<String, String>,
    pub nominal: Verdict,
    pub behavior: Verdict,
    pub witness: Option<Vec<String>>,
    pub observable_witness: Option<Vec<String>>,
    pub diagnostics: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub verdict: Verdict,
    pub note: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Proposal {
    pub base_revision: u64,
    pub binding: Binding,
    pub lifecycle: String,
    pub judgments: Vec<Judgment>,
    pub reports: Vec<Report>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub schema_version: u32,
    pub profile: String,
    pub owner: u32,
    pub source_path: String,
    pub source_fingerprint: String,
    pub source_bytes: String,
    pub policy: Policy,
    pub accepted: Binding,
    pub revision: u64,
    pub proposals: BTreeMap<String, Proposal>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Propose {
        id: String,
        base_revision: u64,
        binding: Binding,
    },
    Evaluate {
        proposal_id: String,
    },
    Report {
        proposal_id: String,
        verdict: Verdict,
        note: String,
    },
    Decide {
        proposal_id: String,
        base_revision: u64,
        decision: Decision,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    Accept,
    Reject,
}

pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn json<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(|e| error("serialization", e))
}
pub fn decode<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    if bytes.len() > MAX_BYTES {
        return Err(error("limit", "JSON exceeds 1 MiB"));
    }
    serde_json::from_slice(bytes).map_err(|e| error("invalid_json", e))
}
fn valid_id(id: &str) -> Result<()> {
    if id.is_empty()
        || id.len() > 64
        || !id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err(error("invalid_id", id));
    }
    Ok(())
}
fn validate_traces(traces: &[Vec<String>]) -> Result<()> {
    if traces.len() > 128 || traces.iter().any(|t| t.len() > 32) {
        return Err(error("limit", "maximum 128 traces of 32 events"));
    }
    for label in traces.iter().flatten() {
        valid_id(label)?;
    }
    Ok(())
}
pub fn validate_binding(binding: &Binding) -> Result<()> {
    valid_id(&binding.id)?;
    validate_traces(&binding.traces)?;
    if binding.provides.len() > 64 || binding.observation_map.len() > 64 {
        return Err(error("limit", "maximum 64 operations/mappings"));
    }
    for id in binding
        .provides
        .iter()
        .chain(binding.observation_map.keys())
        .chain(binding.observation_map.values())
    {
        valid_id(id)?;
    }
    Ok(())
}
pub fn validate_seed(seed: &Seed) -> Result<()> {
    if seed.schema_version != 1 {
        return Err(error("version", "unsupported seed schema"));
    }
    valid_id(&seed.policy.id)?;
    validate_traces(&seed.policy.traces)?;
    if seed.policy.horizon == 0
        || seed.policy.horizon > 32
        || seed.policy.required_operations.len() > 64
    {
        return Err(error(
            "limit",
            "policy horizon must be 1..32 and operations <=64",
        ));
    }
    for id in &seed.policy.required_operations {
        valid_id(id)?;
    }
    if seed
        .policy
        .traces
        .iter()
        .any(|t| t.len() > seed.policy.horizon)
    {
        return Err(error("limit", "policy trace exceeds horizon"));
    }
    validate_binding(&seed.binding)
}
pub fn semantic_digest(policy: &Policy, binding: &Binding) -> String {
    let mut p = policy.clone();
    let mut b = binding.clone();
    p.traces.sort();
    p.traces.dedup();
    b.traces.sort();
    b.traces.dedup();
    hash(
        serde_json::to_string(&(PROFILE, p, b))
            .expect("typed model serializes")
            .as_bytes(),
    )
}

pub fn evaluate(policy: &Policy, binding: &Binding, source: &str) -> Judgment {
    let nominal = if policy.required_operations.is_subset(&binding.provides) {
        Verdict::Pass
    } else if binding.closed {
        Verdict::Fail
    } else {
        Verdict::Unknown
    };
    let mut j = Judgment {
        subject_revision: 0,
        subject_digest: semantic_digest(policy, binding),
        source_fingerprint: source.into(),
        profile: PROFILE.into(),
        evaluator: EVALUATOR.into(),
        horizon: policy.horizon,
        closed: policy.closed && binding.closed,
        observation_map: binding.observation_map.clone(),
        nominal,
        behavior: Verdict::Unknown,
        witness: None,
        observable_witness: None,
        diagnostics: vec![],
    };
    if binding.composition != "independent" {
        j.behavior = Verdict::Unsupported;
        j.diagnostics
            .push("shared/composed events are unsupported".into());
        return j;
    }
    if !j.closed
        || binding.traces.is_empty()
        || policy.traces.is_empty()
        || binding.traces.iter().any(|t| t.len() > policy.horizon)
    {
        j.diagnostics
            .push("open, empty, or beyond concrete-event horizon".into());
        return j;
    }
    if !policy.traces.iter().any(|t| t.iter().any(|x| x == "ship")) {
        j.diagnostics
            .push("policy has no successful shipment".into());
        return j;
    }
    if binding
        .traces
        .iter()
        .flatten()
        .any(|x| !binding.observation_map.contains_key(x))
    {
        j.diagnostics.push("missing observation map".into());
        return j;
    }
    let mut prefixes = BTreeSet::new();
    for trace in &policy.traces {
        for n in 0..=trace.len() {
            prefixes.insert(trace[..n].to_vec());
        }
    }
    let mut failures: Vec<(Vec<String>, Vec<String>)> = vec![];
    let mut shipped = false;
    for trace in &binding.traces {
        let mut observed = vec![];
        for (i, label) in trace.iter().enumerate() {
            let mapped = &binding.observation_map[label];
            if mapped != "tau" {
                observed.push(mapped.clone());
            }
            if !prefixes.contains(&observed) {
                failures.push((trace[..=i].to_vec(), observed));
                break;
            }
            if mapped == "ship" {
                shipped = true;
            }
        }
    }
    failures.sort_by(|a, b| a.0.len().cmp(&b.0.len()).then(a.0.cmp(&b.0)));
    if let Some((w, o)) = failures.into_iter().next() {
        j.behavior = Verdict::Fail;
        j.witness = Some(w);
        j.observable_witness = Some(o);
    } else if shipped {
        j.behavior = Verdict::Pass;
    } else {
        j.diagnostics
            .push("no successful shipment; non-vacuity not established".into());
    }
    j
}

pub fn reduce(state: &State, action: &Action, principal: u32) -> Result<State> {
    if state.schema_version != 1 || state.profile != PROFILE {
        return Err(error("version", "unsupported workspace"));
    }
    if state.owner != principal {
        return Err(error(
            "unauthorized",
            "workspace belongs to another OS principal",
        ));
    }
    let mut next = state.clone();
    match action {
        Action::Propose {
            id,
            base_revision,
            binding,
        } => {
            valid_id(id)?;
            validate_binding(binding)?;
            if *base_revision != state.revision {
                return Err(error("stale", "proposal revision mismatch"));
            }
            if next.proposals.contains_key(id) {
                return Err(error("duplicate", "proposal ID already exists"));
            }
            if next.proposals.len() >= 256 {
                return Err(error("limit", "maximum 256 proposals"));
            }
            next.proposals.insert(
                id.clone(),
                Proposal {
                    base_revision: *base_revision,
                    binding: binding.clone(),
                    lifecycle: "proposed".into(),
                    judgments: vec![],
                    reports: vec![],
                },
            );
        }
        Action::Evaluate { proposal_id }
        | Action::Report { proposal_id, .. }
        | Action::Decide { proposal_id, .. } => {
            let count: usize = next
                .proposals
                .values()
                .map(|p| p.judgments.len() + p.reports.len())
                .sum();
            let p = next
                .proposals
                .get_mut(proposal_id)
                .ok_or_else(|| error("not_found", "proposal"))?;
            if !matches!(action, Action::Report { .. }) && p.lifecycle != "proposed" {
                return Err(error("decided", "proposal already decided"));
            }
            if !matches!(action, Action::Report { .. }) && p.base_revision != state.revision {
                return Err(error("stale", "proposal is stale"));
            }
            match action {
                Action::Evaluate { .. } => {
                    if count >= 1024 {
                        return Err(error("limit", "maximum 1024 evidence records"));
                    }
                    let mut judgment =
                        evaluate(&state.policy, &p.binding, &state.source_fingerprint);
                    judgment.subject_revision = state.revision;
                    p.judgments.push(judgment);
                }
                Action::Report { verdict, note, .. } => {
                    if count >= 1024 || note.len() > 4096 {
                        return Err(error("limit", "report/evidence limit"));
                    }
                    p.reports.push(Report {
                        verdict: verdict.clone(),
                        note: note.clone(),
                    });
                }
                Action::Decide {
                    base_revision,
                    decision,
                    ..
                } => {
                    if *base_revision != state.revision {
                        return Err(error("stale", "decision revision mismatch"));
                    }
                    if *decision == Decision::Accept {
                        let mut expected =
                            evaluate(&state.policy, &p.binding, &state.source_fingerprint);
                        expected.subject_revision = state.revision;
                        if expected.nominal != Verdict::Pass
                            || expected.behavior != Verdict::Pass
                            || !p.judgments.contains(&expected)
                            || p.reports.iter().any(|r| r.verdict == Verdict::Fail)
                        {
                            return Err(error(
                                "unresolved",
                                "two scoped checker passes and no reported fail required",
                            ));
                        }
                        next.accepted = p.binding.clone();
                        next.revision += 1;
                        p.lifecycle = "accepted".into();
                    } else {
                        p.lifecycle = "rejected".into();
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    if matches!(
        action,
        Action::Decide {
            decision: Decision::Accept,
            ..
        }
    ) {
        if let Action::Decide { proposal_id, .. } = action {
            for (id, proposal) in &mut next.proposals {
                if id != proposal_id && proposal.lifecycle == "accepted" {
                    proposal.lifecycle = "superseded".into();
                }
            }
        }
    }
    if json(&next)?.len() > MAX_BYTES {
        return Err(error("limit", "workspace snapshot exceeds 1 MiB"));
    }
    Ok(next)
}

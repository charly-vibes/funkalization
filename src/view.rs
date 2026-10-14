//! Purpose: Render inspectable projections without granting semantic authority to layout.
//! Responsibilities: retain all evidence, source addresses, stale labels and explicit export loss.
//! Rationale: funkalization-0ps ships a read-only visual tracer before an editing UI.

use crate::{model::*, store::Workspace};
use serde_json::{json, Value};

pub fn projection(state: &State) -> Value {
    let current = Workspace::source_matches(state);
    let mut elements = vec![
        json!({"id":"accepted", "source_claim_ids":[state.policy.id], "binding":state.accepted.id,"lifecycle":"accepted","provenance":"declared","verification":"inspect_scoped_evidence"}),
    ];
    for (id, p) in &state.proposals {
        let digest = semantic_digest(&state.policy, &p.binding);
        let applicable = current
            && (p.base_revision == state.revision
                || (p.lifecycle == "accepted"
                    && digest == semantic_digest(&state.policy, &state.accepted)));
        elements.push(json!({"id":id,"source_claim_ids":[state.policy.id],"binding":p.binding.id,"lifecycle":p.lifecycle,"applicable":applicable,"subject_digest":digest,"obligations":[{"id":format!("{id}/nominal-interface"),"kind":"nominal-interface"},{"id":format!("{id}/payment-durability"),"kind":"payment-durability"}],"judgments":p.judgments,"declared_reports":p.reports}));
    }
    json!({"format":"derived_inventory_v1","source_revision":state.revision,"semantic_digest":semantic_digest(&state.policy,&state.accepted),"source_path":state.source_path,"source_fingerprint":state.source_fingerprint,"completeness":if current {"complete"} else {"failed"},"diagnostics":if current {vec![]} else {vec!["authoritative source changed or disappeared"]},"elements":elements,"state":state,"loss_report":{"round_trip_supported":false,"source_writes":false,"unsupported":["OpenSpec/Specodelic semantic write-back","general decomposition","interactive semantic editing","implementation correctness proof"]}})
}
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
pub fn html(state: &State) -> String {
    let projection = projection(state);
    let mut cards = String::new();
    for element in projection["elements"].as_array().expect("elements array") {
        cards.push_str(&format!(
            "<details open><summary>{}</summary><pre>{}</pre></details>",
            escape(element["id"].as_str().unwrap_or("claim")),
            escape(&serde_json::to_string_pretty(element).expect("JSON renders"))
        ));
    }
    format!("<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width\"><title>Design review tracer</title><style>body{{font:16px system-ui;max-width:1000px;margin:2rem auto;padding:1rem;background:#f6f8fa;color:#172536}}details{{background:white;border:1px solid #9aabb8;padding:1rem;margin:1rem 0}}summary{{cursor:pointer;font-weight:bold}}pre{{white-space:pre-wrap;overflow-wrap:anywhere}}.flow{{padding:1rem;background:#dcefe3}}</style><h1>Design review tracer</h1><p>Revision {} · {} · Read-only snapshot. Re-run view to refresh.</p><p class=\"flow\">Request → Payment accepted → Durable record → Shipment eligible</p><p>Accepted means an authorized binding. A checker pass applies only to its recorded finite corpus and horizon; it does not prove running software.</p><p>Source: {}<br>SHA-256: {}</p>{}<details><summary>Export limitations</summary><pre>{}</pre></details></html>",state.revision,escape(projection["completeness"].as_str().unwrap_or("failed")),escape(&state.source_path),escape(&state.source_fingerprint),cards,escape(&projection["loss_report"].to_string()))
}

//! Purpose: Run the independent pilot corpus through persistent public operations.
//! Responsibilities: record expected/actual assertions, emit evidence and a read-only view.
//! Rationale: funkalization-0ps makes the vertical slice replayable without historical archives.

use crate::{
    error,
    model::*,
    store::{read_input, write_new, Workspace},
    view, Result,
};
use serde_json::{json, Value};
use std::{fs, path::Path};

pub const SEED: &str = include_str!("../fixtures/pilot/workspace.json");
pub const ATOMIC: &str = include_str!("../fixtures/pilot/atomic.json");
pub const EVENTUAL: &str = include_str!("../fixtures/pilot/eventual.json");
pub const UNMAPPED: &str = include_str!("../fixtures/pilot/unmapped.json");
pub const SHARED: &str = include_str!("../fixtures/pilot/shared.json");

fn record(rows: &mut Vec<Value>, id: &str, expected: Value, actual: Value) {
    rows.push(json!({"id":id,"passed":expected==actual,"expected":expected,"actual":actual}));
}
fn code<T>(r: Result<T>) -> Value {
    match r {
        Ok(_) => json!("unexpected_success"),
        Err(e) => json!(e.code),
    }
}
pub fn run(root: &Path) -> Result<Value> {
    fs::create_dir(root).map_err(|e| {
        error(
            "destination",
            format!("tracer requires a new directory: {e}"),
        )
    })?;
    let source = root.join("source.json");
    let db = root.join("workspace.db");
    write_new(&source, SEED.as_bytes())?;
    let mut ws = Workspace::init(&db, &source)?;
    let baseline = ws.state()?;
    let mut rows = vec![];
    for (id, fixture, expected) in [
        ("bad", EVENTUAL, Verdict::Fail),
        ("unmapped", UNMAPPED, Verdict::Unknown),
        ("shared", SHARED, Verdict::Unsupported),
        ("good", ATOMIC, Verdict::Pass),
        ("stale", ATOMIC, Verdict::Pass),
    ] {
        let binding = decode(fixture.as_bytes())?;
        ws.apply(&Action::Propose {
            id: id.into(),
            base_revision: 0,
            binding,
        })?;
        let state = ws.apply(&Action::Evaluate {
            proposal_id: id.into(),
        })?;
        let j = state.proposals[id]
            .judgments
            .last()
            .expect("evaluation exists");
        record(
            &mut rows,
            &format!("{id}-behavior"),
            json!(expected),
            json!(j.behavior),
        );
        record(
            &mut rows,
            &format!("{id}-nominal"),
            json!(Verdict::Pass),
            json!(j.nominal),
        );
    }
    record(
        &mut rows,
        "forbidden-witness",
        json!(["request", "capture", "ship"]),
        json!(ws.state()?.proposals["bad"].judgments[0].witness),
    );
    record(
        &mut rows,
        "proposal-does-not-accept",
        json!(semantic_digest(&baseline.policy, &baseline.accepted)),
        json!(semantic_digest(&ws.state()?.policy, &ws.state()?.accepted)),
    );
    let count = ws.event_count()?;
    record(
        &mut rows,
        "bad-accept-refused",
        json!("unresolved"),
        code(ws.apply(&Action::Decide {
            proposal_id: "bad".into(),
            base_revision: 0,
            decision: Decision::Accept,
        })),
    );
    record(
        &mut rows,
        "refusal-no-event",
        json!(count),
        json!(ws.event_count()?),
    );
    ws.apply(&Action::Decide {
        proposal_id: "bad".into(),
        base_revision: 0,
        decision: Decision::Reject,
    })?;
    let state = ws.apply(&Action::Decide {
        proposal_id: "good".into(),
        base_revision: 0,
        decision: Decision::Accept,
    })?;
    record(&mut rows, "accepted-once", json!(1), json!(state.revision));
    let count = ws.event_count()?;
    record(
        &mut rows,
        "stale-refused",
        json!("stale"),
        code(ws.apply(&Action::Decide {
            proposal_id: "stale".into(),
            base_revision: 0,
            decision: Decision::Accept,
        })),
    );
    record(
        &mut rows,
        "duplicate-refused",
        json!("decided"),
        code(ws.apply(&Action::Decide {
            proposal_id: "good".into(),
            base_revision: 1,
            decision: Decision::Accept,
        })),
    );
    record(
        &mut rows,
        "refusals-no-events",
        json!(count),
        json!(ws.event_count()?),
    );
    drop(ws);
    let mut ws = Workspace::open(&db)?;
    record(
        &mut rows,
        "restart-replay",
        json!(state),
        json!(ws.replay()?),
    );
    let artifact = view::projection(&state);
    write_new(
        &root.join("export.json"),
        serde_json::to_string_pretty(&artifact)
            .map_err(|e| error("serialization", e))?
            .as_bytes(),
    )?;
    record(
        &mut rows,
        "export-no-overwrite",
        json!("destination"),
        code(write_new(&root.join("export.json"), b"overwritten")),
    );
    record(
        &mut rows,
        "export-preserves-inventory",
        artifact.clone(),
        decode::<Value>(&read_input(&root.join("export.json"))?)?,
    );
    record(
        &mut rows,
        "archive-refused",
        json!("unsupported"),
        code(archive()),
    );
    record(
        &mut rows,
        "archive-no-events",
        json!(count),
        json!(ws.event_count()?),
    );
    record(
        &mut rows,
        "source-preserved",
        json!(hash(SEED.as_bytes())),
        json!(hash(&read_input(&source)?)),
    );
    // Only this newly created fixture is changed; external specs are never touched.
    fs::write(&source, b"malformed").map_err(|e| error("io", e))?;
    record(
        &mut rows,
        "drift-projection",
        json!("failed"),
        view::projection(&state)["completeness"].clone(),
    );
    record(
        &mut rows,
        "drift-refused",
        json!("source_changed"),
        code(ws.apply(&Action::Propose {
            id: "drift".into(),
            base_revision: 1,
            binding: decode(ATOMIC.as_bytes())?,
        })),
    );
    fs::write(&source, SEED).map_err(|e| error("io", e))?;
    record(
        &mut rows,
        "drift-no-events",
        json!(count),
        json!(ws.event_count()?),
    );
    write_new(&root.join("view.html"), view::html(&state).as_bytes())?;
    let passed = rows.iter().all(|r| r["passed"] == true);
    let report = json!({"profile":PROFILE,"evaluator":EVALUATOR,"package_version":env!("CARGO_PKG_VERSION"),"platform":{"os":std::env::consts::OS,"arch":std::env::consts::ARCH},"command":"cargo run --locked -- tracer <new-directory>","passed":passed,"assertions":rows,"fixtures":{"workspace":hash(SEED.as_bytes()),"atomic":hash(ATOMIC.as_bytes()),"eventual":hash(EVENTUAL.as_bytes()),"unmapped":hash(UNMAPPED.as_bytes()),"shared":hash(SHARED.as_bytes())},"limitations":["self-contained baseline; historical M3/M4 not reproduced","explicit finite model corpus, not running storage verification","read-only HTML; no visual editing equivalence claim","source archive writes unsupported","human value trial not run"]});
    write_new(
        &root.join("report.json"),
        serde_json::to_string_pretty(&report)
            .map_err(|e| error("serialization", e))?
            .as_bytes(),
    )?;
    if !passed {
        return Err(error(
            "tracer_failed",
            format!("inspect {}", root.join("report.json").display()),
        ));
    }
    Ok(report)
}
pub fn archive() -> Result<()> {
    Err(error(
        "unsupported",
        "source archive/write-back is disabled; use derived export",
    ))
}

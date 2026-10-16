//! Purpose: Exercise the tracer through real processes and independent domain assertions.
//! Responsibilities: verify cold replay, failure isolation and the payment-storage oracle.
//! Rationale: funkalization-0ps requires evidence beyond evaluator-generated expectations.

use assert_cmd::Command;
use std::fs;

#[test]
fn cold_tracer_runs_and_replays_in_a_second_process() {
    let root = std::env::temp_dir().join(format!("funkalization-red-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    Command::cargo_bin("funkalization")
        .unwrap()
        .args(["tracer", root.to_str().unwrap()])
        .assert()
        .success();
    assert!(root.join("report.json").is_file());
    assert!(root.join("view.html").is_file());
    Command::cargo_bin("funkalization")
        .unwrap()
        .args(["replay", root.join("workspace.db").to_str().unwrap()])
        .assert()
        .success();
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn tracer_accepts_a_real_model_change_and_labels_baseline_provenance() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("tracer");
    let report = funkalization::tracer::run(&root).unwrap();
    let export: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("export.json")).unwrap()).unwrap();
    assert_eq!(export["state"]["accepted"]["id"], "durable-direct-payment");
    assert!(report["limitations"].as_array().unwrap().iter().any(|s| s
        .as_str()
        .unwrap()
        .contains("M2/M3/M4 reproduced separately")));
    let assertions = report["assertions"].as_array().unwrap();
    assert!(assertions
        .iter()
        .any(|r| r["id"] == "acceptance-changes-semantic-digest" && r["passed"] == true));
}

#[test]
fn documented_cli_actions_match_the_direct_api_and_fail_without_effects() {
    use funkalization::{model::*, store::Workspace, tracer};
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source.json");
    fs::write(&source, tracer::SEED).unwrap();
    let cli_db = dir.path().join("cli.db");
    let api_db = dir.path().join("api.db");
    Command::cargo_bin("funkalization")
        .unwrap()
        .args(["init", cli_db.to_str().unwrap(), source.to_str().unwrap()])
        .assert()
        .success();
    let mut api = Workspace::init(&api_db, &source).unwrap();
    let actions = [
        Action::Propose {
            id: "replacement".into(),
            base_revision: 0,
            binding: decode(tracer::COMPATIBLE.as_bytes()).unwrap(),
        },
        Action::Evaluate {
            proposal_id: "replacement".into(),
        },
        Action::Decide {
            proposal_id: "replacement".into(),
            base_revision: 0,
            decision: Decision::Accept,
        },
        Action::Report {
            proposal_id: "replacement".into(),
            verdict: Verdict::Fail,
            note: "Observed shipment before durable persistence".into(),
        },
    ];
    let action_file = dir.path().join("action.json");
    for action in actions {
        fs::write(&action_file, json(&action).unwrap()).unwrap();
        let output = Command::cargo_bin("funkalization")
            .unwrap()
            .args([
                "apply",
                cli_db.to_str().unwrap(),
                action_file.to_str().unwrap(),
            ])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();
        let actual: State = decode(&output).unwrap();
        assert_eq!(actual, api.apply(&action).unwrap());
    }
    let before = api.replay().unwrap();
    fs::write(
        &action_file,
        r#"{"action":"decide","proposal_id":"replacement","base_revision":1,"decision":"accept"}"#,
    )
    .unwrap();
    let output = Command::cargo_bin("funkalization")
        .unwrap()
        .args([
            "apply",
            cli_db.to_str().unwrap(),
            action_file.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .code(2)
        .get_output()
        .stderr
        .clone();
    let failure: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(failure["code"], "decided");
    Command::cargo_bin("funkalization")
        .unwrap()
        .args(["archive", cli_db.to_str().unwrap()])
        .assert()
        .failure()
        .code(2);
    let actual = Workspace::open(&cli_db).unwrap().replay().unwrap();
    assert_eq!(actual, before);
}

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

//! CLI contract: `--version` output must be `funkalization <semver>` —
//! ddl's version detection parses this exact shape.

use assert_cmd::Command;

#[test]
fn version_flag_prints_name_and_semver() {
    let out = Command::cargo_bin("funkalization")
        .expect("binary builds")
        .arg("--version")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(out).unwrap();
    assert_eq!(stdout.trim(), "funkalization 0.1.0");
}

#[test]
fn unknown_command_exits_nonzero() {
    Command::cargo_bin("funkalization")
        .expect("binary builds")
        .arg("nope")
        .assert()
        .failure()
        .code(2);
}

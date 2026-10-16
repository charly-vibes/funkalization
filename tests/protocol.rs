//! Purpose: Verify failure boundaries of the design-review protocol independently of the tracer.
//! Responsibilities: exercise finite oracles, trust separation, atomicity, concurrency and replay.
//! Rationale: funkalization-5fs/jg3 require negative evidence, including real process interruption.

use funkalization::{
    model::*,
    store::{principal, write_new, Workspace},
    tracer, view,
};
use std::{fs, path::PathBuf};

fn workspace() -> (tempfile::TempDir, PathBuf, Workspace) {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source.json");
    fs::write(&source, tracer::SEED).unwrap();
    let db = dir.path().join("workspace.db");
    let ws = Workspace::init(&db, &source).unwrap();
    (dir, db, ws)
}
fn propose(ws: &mut Workspace, id: &str, fixture: &str) {
    ws.apply(&Action::Propose {
        id: id.into(),
        base_revision: 0,
        binding: decode(fixture.as_bytes()).unwrap(),
    })
    .unwrap();
}
fn decide(id: &str, revision: u64) -> Action {
    Action::Decide {
        proposal_id: id.into(),
        base_revision: revision,
        decision: Decision::Accept,
    }
}
fn seed() -> Seed {
    decode(tracer::SEED.as_bytes()).unwrap()
}

#[test]
fn hand_written_policy_obeys_independent_temporal_oracle() {
    // A second formulation: inspect preceding event positions rather than calling the evaluator.
    let seed = seed();
    let mut shipments = 0;
    for trace in &seed.policy.traces {
        for (i, event) in trace.iter().enumerate() {
            if event != "ship" {
                continue;
            }
            shipments += 1;
            let prior = &trace[..i];
            let persisted = prior
                .iter()
                .rposition(|x| x == "persist")
                .expect("shipment needs durable record");
            let paid = prior[..persisted]
                .iter()
                .rposition(|x| x == "pay")
                .expect("record needs accepted payment");
            assert!(prior[..paid].iter().any(|x| x == "request"));
            assert!(
                !prior[paid + 1..persisted].iter().any(|x| x == "crash"),
                "unpersisted payment lost on crash"
            );
            if let Some(crash) = prior.iter().rposition(|x| x == "crash") {
                assert!(prior[crash + 1..].iter().any(|x| x == "restart"));
            }
        }
    }
    assert!(
        shipments >= 4,
        "include successful shipment, restart and retry"
    );
}

#[test]
fn finite_boundaries_never_produce_vacuous_pass() {
    let s = seed();
    let b = s.binding.clone();
    let mut cases = vec![];
    let mut x = b.clone();
    x.traces.clear();
    cases.push(x);
    let mut x = b.clone();
    x.traces = vec![vec![]];
    cases.push(x);
    let mut x = b.clone();
    x.closed = false;
    cases.push(x);
    let mut x = b.clone();
    x.observation_map.remove("capture");
    cases.push(x);
    let mut x = b.clone();
    x.traces = vec![vec!["authorize".into(); 13]];
    cases.push(x);
    let mut x = b.clone();
    x.traces = vec![vec!["request".into()]];
    cases.push(x);
    for x in cases {
        assert_eq!(evaluate(&s.policy, &x, "source").behavior, Verdict::Unknown);
    }
    let mut p = s.policy.clone();
    p.closed = false;
    assert_eq!(evaluate(&p, &b, "source").behavior, Verdict::Unknown);
    let mut x = b.clone();
    x.provides.remove("durable_write");
    assert_eq!(evaluate(&s.policy, &x, "source").nominal, Verdict::Fail);
    x.closed = false;
    assert_eq!(evaluate(&s.policy, &x, "source").nominal, Verdict::Unknown);
    // The bound counts hidden concrete actions, not only the observable projection.
    let mut x = b.clone();
    x.traces = vec![vec!["request", "capture", "persist", "ship"]
        .into_iter()
        .map(String::from)
        .collect()];
    let mut p = s.policy.clone();
    p.horizon = 4;
    assert_eq!(evaluate(&p, &x, "source").behavior, Verdict::Pass);
    x.traces[0].insert(1, "authorize".into());
    assert_eq!(evaluate(&p, &x, "source").behavior, Verdict::Unknown);
    p.horizon = 5;
    assert_eq!(evaluate(&p, &x, "source").behavior, Verdict::Pass);
}

#[test]
fn counterexample_is_shortest_and_replayable() {
    let s = seed();
    let mut b = s.binding;
    b.traces = vec![
        vec!["request", "capture", "persist", "ship", "ship"],
        vec!["request", "capture", "ship"],
    ]
    .into_iter()
    .map(|t| t.into_iter().map(String::from).collect())
    .collect();
    let j = evaluate(&s.policy, &b, "source");
    assert_eq!(j.behavior, Verdict::Fail);
    assert_eq!(
        j.witness,
        Some(vec!["request".into(), "capture".into(), "ship".into()])
    );
    assert_eq!(
        j.observable_witness,
        Some(vec!["request".into(), "pay".into(), "ship".into()])
    );
}

#[test]
fn canonical_digest_ignores_declaration_order_not_semantics() {
    let s = seed();
    let mut b = s.binding.clone();
    let mut p = s.policy.clone();
    b.traces.reverse();
    p.traces.reverse();
    b.traces.push(b.traces[0].clone());
    assert_eq!(
        semantic_digest(&s.policy, &s.binding),
        semantic_digest(&p, &b)
    );
    p.horizon -= 1;
    assert_ne!(
        semantic_digest(&s.policy, &s.binding),
        semantic_digest(&p, &b)
    );
    p = s.policy.clone();
    b.observation_map.insert("capture".into(), "tau".into());
    assert_ne!(
        semantic_digest(&s.policy, &s.binding),
        semantic_digest(&p, &b)
    );
}

#[test]
fn reports_cannot_impersonate_checker_or_hide_conflicting_evidence() {
    let (_dir, _db, mut ws) = workspace();
    propose(&mut ws, "review", tracer::ATOMIC);
    ws.apply(&Action::Report {
        proposal_id: "review".into(),
        verdict: Verdict::Pass,
        note: "I say pass".into(),
    })
    .unwrap();
    assert_eq!(
        ws.apply(&decide("review", 0)).unwrap_err().code,
        "unresolved"
    );
    ws.apply(&Action::Evaluate {
        proposal_id: "review".into(),
    })
    .unwrap();
    ws.apply(&Action::Report {
        proposal_id: "review".into(),
        verdict: Verdict::Fail,
        note: "observed counterexample".into(),
    })
    .unwrap();
    assert_eq!(
        ws.apply(&decide("review", 0)).unwrap_err().code,
        "unresolved"
    );
    let p = &ws.replay().unwrap().proposals["review"];
    assert_eq!(p.reports.len(), 2);
    assert_eq!(p.judgments.len(), 1);
    assert!(decode::<Action>(
        br#"{"action":"evaluate","proposal_id":"review","authority":"owner"}"#
    )
    .is_err());
}

#[test]
fn unauthorized_and_source_drift_have_zero_committed_effect() {
    let (dir, _db, mut ws) = workspace();
    let a = Action::Propose {
        id: "new".into(),
        base_revision: 0,
        binding: seed().binding,
    };
    assert_eq!(
        ws.apply_as(&a, principal().wrapping_add(1))
            .unwrap_err()
            .code,
        "unauthorized"
    );
    let before = ws.state().unwrap();
    fs::write(dir.path().join("source.json"), "{}").unwrap();
    assert_eq!(ws.apply(&a).unwrap_err().code, "source_changed");
    assert_eq!(ws.event_count().unwrap(), 0);
    assert_eq!(ws.replay().unwrap(), before);
    assert_eq!(view::projection(&before)["completeness"], "failed");
}

#[test]
fn corruption_and_future_database_schema_fail_closed() {
    let (_dir, db, mut ws) = workspace();
    let conn = rusqlite::Connection::open(&db).unwrap();
    let mut state = ws.state().unwrap();
    state.revision = 99;
    conn.execute(
        "UPDATE snapshots SET body=?1 WHERE id=1",
        [json(&state).unwrap()],
    )
    .unwrap();
    assert_eq!(ws.replay().unwrap_err().code, "integrity");
    conn.execute_batch("PRAGMA user_version=2").unwrap();
    assert!(matches!(Workspace::open(&db),Err(e) if e.code=="version"));
}

#[test]
fn two_writers_cannot_accept_the_same_base_twice() {
    let (_dir, db, mut ws) = workspace();
    for id in ["a", "b"] {
        propose(&mut ws, id, tracer::ATOMIC);
        ws.apply(&Action::Evaluate {
            proposal_id: id.into(),
        })
        .unwrap();
    }
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let handles: Vec<_> = ["a", "b"]
        .into_iter()
        .map(|id| {
            let path = db.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let mut ws = Workspace::open(&path).unwrap();
                barrier.wait();
                ws.apply(&decide(id, 0))
            })
        })
        .collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter_map(|r| r.as_ref().err())
            .next()
            .unwrap()
            .code,
        "stale"
    );
    assert_eq!(ws.replay().unwrap().revision, 1);
    assert_eq!(ws.event_count().unwrap(), 5);
}

#[test]
fn transaction_crash_worker() {
    use std::io::Write;
    let Ok(path) = std::env::var("FUNKALIZATION_CRASH_DB") else {
        return;
    };
    let conn = rusqlite::Connection::open(path).unwrap();
    conn.execute_batch("BEGIN IMMEDIATE; UPDATE snapshots SET body='incomplete' WHERE id=1; INSERT INTO events VALUES(1,0,'invalid','before','after');").unwrap();
    println!("READY_TO_KILL");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    panic!("parent should kill this process before commit");
}

#[test]
fn process_death_before_commit_recovers_previous_snapshot_and_events() {
    use std::{
        io::{BufRead, BufReader},
        process::{Command, Stdio},
    };
    let (_dir, db, mut ws) = workspace();
    let before = ws.state().unwrap();
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "transaction_crash_worker", "--nocapture"])
        .env("FUNKALIZATION_CRASH_DB", &db)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut reader = BufReader::new(child.stdout.take().unwrap());
    loop {
        let mut line = String::new();
        assert!(reader.read_line(&mut line).unwrap() > 0);
        if line.contains("READY_TO_KILL") {
            break;
        }
    }
    child.kill().unwrap();
    assert!(!child.wait().unwrap().success());
    assert_eq!(ws.replay().unwrap(), before);
    assert_eq!(ws.event_count().unwrap(), 0);
}

#[test]
fn malformed_limits_and_existing_destinations_are_rejected() {
    assert_eq!(
        decode::<Seed>(&vec![b' '; MAX_BYTES + 1]).unwrap_err().code,
        "limit"
    );
    let mut s = seed();
    s.schema_version = 2;
    assert_eq!(validate_seed(&s).unwrap_err().code, "version");
    s = seed();
    s.binding.traces[0] = vec!["request".into(); 33];
    assert_eq!(validate_seed(&s).unwrap_err().code, "limit");
    let (dir, db, ws) = workspace();
    let original = fs::read(&db).unwrap();
    assert!(Workspace::init(&db, &dir.path().join("source.json")).is_err());
    assert_eq!(fs::read(&db).unwrap(), original);
    assert!(write_new(&db, b"overwrite").is_err());
    assert_eq!(fs::read(&db).unwrap(), original);
    let before = ws.state().unwrap();
    let html = view::html(&before);
    assert!(html.contains("Read-only snapshot"));
    assert_eq!(ws.state().unwrap(), before);
}

#[test]
fn judgments_record_base_revision_and_retire_superseded_acceptance() {
    let (_dir, _db, mut ws) = workspace();
    propose(&mut ws, "first", tracer::ATOMIC);
    ws.apply(&Action::Evaluate {
        proposal_id: "first".into(),
    })
    .unwrap();
    ws.apply(&decide("first", 0)).unwrap();
    let mut binding = seed().binding;
    binding.id = "replacement".into();
    ws.apply(&Action::Propose {
        id: "second".into(),
        base_revision: 1,
        binding,
    })
    .unwrap();
    let evaluated = ws
        .apply(&Action::Evaluate {
            proposal_id: "second".into(),
        })
        .unwrap();
    let encoded = serde_json::to_value(&evaluated.proposals["second"].judgments[0]).unwrap();
    assert_eq!(encoded["subject_revision"], 1);
    let state = ws.apply(&decide("second", 1)).unwrap();
    assert_eq!(state.proposals["first"].lifecycle, "superseded");
    let projection = view::projection(&state);
    let first = projection["elements"]
        .as_array()
        .unwrap()
        .iter()
        .find(|x| x["id"] == "first")
        .unwrap();
    assert_eq!(first["applicable"], false);
}

#[test]
fn later_counterexample_challenges_an_accepted_binding_without_rewriting_it() {
    let (_dir, _db, mut ws) = workspace();
    propose(&mut ws, "accepted", tracer::ATOMIC);
    ws.apply(&Action::Evaluate {
        proposal_id: "accepted".into(),
    })
    .unwrap();
    let accepted = ws.apply(&decide("accepted", 0)).unwrap();
    let challenged = ws
        .apply(&Action::Report {
            proposal_id: "accepted".into(),
            verdict: Verdict::Fail,
            note: "Independent runtime observation contradicts the finite model".into(),
        })
        .unwrap();
    assert_eq!(challenged.accepted, accepted.accepted);
    assert_eq!(challenged.revision, accepted.revision);
    assert_eq!(challenged.proposals["accepted"].judgments.len(), 1);
    let projected = view::projection(&challenged);
    let node = projected["elements"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == "accepted")
        .unwrap();
    assert_eq!(node["verification"], "challenged");
    assert_eq!(ws.replay().unwrap(), challenged);
}

#[test]
fn canonical_bytes_match_the_published_cross_language_digest_vector() {
    let vector: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/pilot/digest-vector.json")).unwrap();
    let s = seed();
    assert_eq!(
        semantic_digest(&s.policy, &s.binding),
        vector["semantic_digest"]
    );
}

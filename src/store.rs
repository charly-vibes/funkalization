//! Purpose: Persist workspace snapshots and replayable actions atomically.
//! Responsibilities: enforce source/UID guards, serialize writers and verify replay integrity.
//! Rationale: funkalization-5fs uses one SQLite transaction instead of multi-file commits.

use crate::{error, model::*, Result};
use rusqlite::{params, Connection, OpenFlags, TransactionBehavior};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::Path,
    time::Duration,
};

pub fn principal() -> u32 {
    #[cfg(unix)]
    // SAFETY: geteuid has no arguments, ownership requirements or failure mode.
    unsafe {
        libc::geteuid()
    }
    #[cfg(not(unix))]
    {
        u32::MAX
    }
}
pub fn read_input(path: &Path) -> Result<Vec<u8>> {
    if !path.is_file() {
        return Err(error("input", "input must be a regular file"));
    }
    let mut bytes = vec![];
    fs::File::open(path)
        .map_err(|e| error("io", e))?
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| error("io", e))?;
    if bytes.len() > MAX_BYTES {
        return Err(error("limit", "input exceeds 1 MiB"));
    }
    Ok(bytes)
}
pub fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(parent).map_err(|e| error("io", e))?;
    tmp.write_all(bytes).map_err(|e| error("io", e))?;
    tmp.as_file().sync_all().map_err(|e| error("io", e))?;
    tmp.persist_noclobber(path)
        .map_err(|e| error("destination", e))?;
    fs::File::open(parent)
        .and_then(|f| f.sync_all())
        .map_err(|e| error("io", e))?;
    Ok(())
}
pub struct Workspace {
    conn: Connection,
}
impl Workspace {
    pub fn init(path: &Path, source: &Path) -> Result<Self> {
        if !cfg!(unix) {
            return Err(error("unsupported", "v1 requires Unix UID authority"));
        }
        let bytes = read_input(source)?;
        let seed: Seed = decode(&bytes)?;
        validate_seed(&seed)?;
        let state = State {
            schema_version: 1,
            profile: PROFILE.into(),
            owner: principal(),
            source_path: fs::canonicalize(source)
                .map_err(|e| error("io", e))?
                .to_string_lossy()
                .into_owned(),
            source_fingerprint: hash(&bytes),
            source_bytes: String::from_utf8(bytes).map_err(|e| error("invalid_json", e))?,
            policy: seed.policy,
            accepted: seed.binding,
            revision: 0,
            proposals: BTreeMap::new(),
        };
        let snapshot = json(&state)?;
        if snapshot.len() > MAX_BYTES {
            return Err(error("limit", "initial snapshot exceeds 1 MiB"));
        }
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let tmp = tempfile::NamedTempFile::new_in(parent).map_err(|e| error("io", e))?;
        {
            let conn = Connection::open(tmp.path()).map_err(db)?;
            conn.execute_batch("PRAGMA synchronous=FULL; BEGIN IMMEDIATE; PRAGMA user_version=1; CREATE TABLE snapshots (id INTEGER PRIMARY KEY, body TEXT NOT NULL); CREATE TABLE events (seq INTEGER PRIMARY KEY, principal INTEGER NOT NULL, action TEXT NOT NULL, before_hash TEXT NOT NULL, after_hash TEXT NOT NULL);").map_err(db)?;
            conn.execute("INSERT INTO snapshots VALUES (0,?1),(1,?1)", [&snapshot])
                .map_err(db)?;
            conn.execute_batch("COMMIT").map_err(db)?;
        }
        tmp.as_file().sync_all().map_err(|e| error("io", e))?;
        tmp.persist_noclobber(path)
            .map_err(|e| error("destination", e))?;
        fs::File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|e| error("io", e))?;
        Self::open(path)
    }
    pub fn open(path: &Path) -> Result<Self> {
        if !cfg!(unix) {
            return Err(error("unsupported", "v1 requires Unix UID authority"));
        }
        let conn =
            Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE).map_err(db)?;
        conn.busy_timeout(Duration::from_secs(5)).map_err(db)?;
        conn.execute_batch("PRAGMA synchronous=FULL;").map_err(db)?;
        let version: u32 = conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .map_err(db)?;
        if version != 1 {
            return Err(error("version", "unsupported database schema"));
        }
        Ok(Self { conn })
    }
    pub fn state(&self) -> Result<State> {
        snapshot(&self.conn, 1)
    }
    pub fn source_matches(state: &State) -> bool {
        read_input(Path::new(&state.source_path))
            .is_ok_and(|bytes| hash(&bytes) == state.source_fingerprint)
    }
    pub fn apply(&mut self, action: &Action) -> Result<State> {
        self.apply_as(action, principal())
    }
    // Principal is supplied by the host, never deserialized from an action.
    pub fn apply_as(&mut self, action: &Action, actor: u32) -> Result<State> {
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(db)?;
        let state = replay_connection(&tx)?;
        if !Self::source_matches(&state) {
            return Err(error(
                "source_changed",
                "source is missing, malformed, or changed; initialize a new workspace",
            ));
        }
        let next = reduce(&state, action, actor)?;
        let before = json(&state)?;
        let after = json(&next)?;
        tx.execute(
            "INSERT INTO events(principal,action,before_hash,after_hash) VALUES (?1,?2,?3,?4)",
            params![
                actor,
                json(action)?,
                hash(before.as_bytes()),
                hash(after.as_bytes())
            ],
        )
        .map_err(db)?;
        tx.execute("UPDATE snapshots SET body=?1 WHERE id=1", [&after])
            .map_err(db)?;
        tx.commit().map_err(db)?;
        Ok(next)
    }
    pub fn replay(&mut self) -> Result<State> {
        let tx = self.conn.transaction().map_err(db)?;
        let state = replay_connection(&tx)?;
        tx.commit().map_err(db)?;
        Ok(state)
    }
    pub fn event_count(&self) -> Result<u64> {
        self.conn
            .query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0))
            .map_err(db)
    }
}
fn db(e: rusqlite::Error) -> crate::Error {
    error("database", e)
}
fn snapshot(conn: &Connection, id: u32) -> Result<State> {
    let text: String = conn
        .query_row("SELECT body FROM snapshots WHERE id=?1", [id], |r| r.get(0))
        .map_err(db)?;
    let state: State = decode(text.as_bytes())?;
    if state.schema_version != 1 || state.profile != PROFILE {
        return Err(error("version", "unsupported state schema/profile"));
    }
    Ok(state)
}
fn replay_connection(conn: &Connection) -> Result<State> {
    let mut state = snapshot(conn, 0)?;
    if hash(state.source_bytes.as_bytes()) != state.source_fingerprint {
        return Err(error("integrity", "genesis source hash mismatch"));
    }
    let seed: Seed = decode(state.source_bytes.as_bytes())?;
    validate_seed(&seed)?;
    if state.policy != seed.policy
        || state.accepted != seed.binding
        || state.revision != 0
        || !state.proposals.is_empty()
    {
        return Err(error("integrity", "invalid genesis"));
    }
    let mut query = conn
        .prepare("SELECT seq,principal,action,before_hash,after_hash FROM events ORDER BY seq")
        .map_err(db)?;
    let mut rows = query.query([]).map_err(db)?;
    let mut expected = 1;
    while let Some(row) = rows.next().map_err(db)? {
        let seq: u64 = row.get(0).map_err(db)?;
        let actor: u32 = row.get(1).map_err(db)?;
        let action: String = row.get(2).map_err(db)?;
        let before: String = row.get(3).map_err(db)?;
        let after: String = row.get(4).map_err(db)?;
        if seq != expected || expected > 2048 || hash(json(&state)?.as_bytes()) != before {
            return Err(error("integrity", "event sequence/before hash mismatch"));
        }
        state = reduce(&state, &decode::<Action>(action.as_bytes())?, actor)?;
        if hash(json(&state)?.as_bytes()) != after {
            return Err(error("integrity", "event result hash mismatch"));
        }
        expected += 1;
    }
    if state != snapshot(conn, 1)? {
        return Err(error("integrity", "snapshot differs from event replay"));
    }
    Ok(state)
}

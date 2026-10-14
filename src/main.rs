//! Purpose: Expose the persistent local design-review protocol as a CLI.
//! Responsibilities: parse commands, resolve host identity and return JSON results/errors.
//! Rationale: funkalization-0ps routes all mutations through the shared transactional API.

use funkalization::{
    error,
    model::{decode, Action},
    store::{read_input, write_new, Workspace},
    tracer, view, Result,
};
use serde_json::{json, Value};
use std::path::Path;

fn run(args: &[String]) -> Result<Value> {
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["init",db,source] => Ok(json!(Workspace::init(Path::new(db),Path::new(source))?.state()?)),
        ["apply",db,action] => {
            let action:Action=decode(&read_input(Path::new(action))?)?;
            Ok(json!(Workspace::open(Path::new(db))?.apply(&action)?))
        }
        ["show",db] => Ok(json!(Workspace::open(Path::new(db))?.replay()?)),
        ["replay",db] => {
            let state=Workspace::open(Path::new(db))?.replay()?;
            Ok(json!({"replayed":true,"source_matches":Workspace::source_matches(&state),"state":state}))
        }
        ["view",db] => Ok(view::projection(&Workspace::open(Path::new(db))?.replay()?)),
        ["view",db,path] => {
            let state=Workspace::open(Path::new(db))?.replay()?;
            write_new(Path::new(path),view::html(&state).as_bytes())?;
            Ok(json!({"written":path,"read_only":true}))
        }
        ["export",db,path] => {
            let state=Workspace::open(Path::new(db))?.replay()?;
            write_new(Path::new(path),serde_json::to_string_pretty(&view::projection(&state)).map_err(|e|error("serialization",e))?.as_bytes())?;
            Ok(json!({"written":path,"round_trip_supported":false}))
        }
        ["archive",_db] => {tracer::archive()?; unreachable!()}
        ["tracer",directory] => tracer::run(Path::new(directory)),
        _ => Err(error("usage","expected init DB SOURCE | apply DB ACTION.json | show DB | replay DB | view DB [NEW.html] | export DB NEW.json | archive DB | tracer NEW_DIRECTORY")),
    }
}
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 1 && matches!(args[0].as_str(), "--version" | "-V") {
        println!("funkalization {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.is_empty() || (args.len() == 1 && matches!(args[0].as_str(), "--help" | "-h")) {
        println!("funkalization {}\nLocal design-review tracer\n\ninit DB SOURCE.json\napply DB ACTION.json\nshow DB\nreplay DB\nview DB [NEW.html]\nexport DB NEW.json\narchive DB (unsupported)\ntracer NEW_DIRECTORY",env!("CARGO_PKG_VERSION"));
        return;
    }
    match run(&args) {
        Ok(value) => println!(
            "{}",
            serde_json::to_string_pretty(&value).expect("JSON renders")
        ),
        Err(e) => {
            eprintln!("{}", serde_json::to_string(&e).expect("error renders"));
            std::process::exit(2);
        }
    }
}

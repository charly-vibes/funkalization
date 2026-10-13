//! funkalization — visual semantic design workspace.
//!
//! A text-and-visual means of proposing, revising, and inspecting software
//! designs while preserving semantic meaning, authority, and evidence.
//! The implementation contract lives in `openspec/design/` (bundle
//! VSD-IMPLEMENTATION-2026-10-13); gates G0–G8 there define what ships.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--version") | Some("-V") => println!("funkalization {}", env!("CARGO_PKG_VERSION")),
        Some("--help") | Some("-h") | None => {
            println!("funkalization {}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("Visual semantic design workspace (early scaffold).");
            println!();
            println!("USAGE:");
            println!("    funkalization --version");
        }
        Some(other) => {
            eprintln!("error: unknown command `{other}` — try `funkalization --help`");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_matches_cargo_toml() {
        assert_eq!(
            env!("CARGO_PKG_VERSION"),
            "0.1.0",
            "bump Cargo.toml and this test together"
        );
    }
}

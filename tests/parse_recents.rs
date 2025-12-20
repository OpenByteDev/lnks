use lnks::Shortcut;
use std::{env, fs, io, path::PathBuf};

fn recent_dir() -> PathBuf {
    let appdata = env::var("APPDATA").expect("APPDATA not set");
    let path = PathBuf::from(format!(r"{appdata}\Microsoft\Windows\Recent"));
    assert!(path.exists(), "recent dir not found at {}", path.display());
    path
}

#[test]
fn parse_all_recent_shortcuts() {
    let recent_dir = recent_dir();
    let entries = fs::read_dir(&recent_dir)
        .expect("failed to read recent dir")
        .collect::<io::Result<Vec<_>>>()
        .expect("failed to read recent dir entries");

    for entry in entries {
        let path = entry.path();
        let is_lnk = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("lnk"))
            .unwrap_or(false);
        if !is_lnk {
            continue;
        }

        if let Err(e) = Shortcut::load(&path) {
            eprintln!("Failed to parse {}: {e:?}", path.display());
        }
    }
}

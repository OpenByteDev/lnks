use lnks::{Icon, Shortcut, WindowState};
use std::{
    env,
    path::PathBuf,
};

use crate::temp_path::TempPath;

#[path = "temp_path.rs"]
mod temp_path;

macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of(f).rsplit("::").nth(1).unwrap_or("<unknown>")
    }};
}

fn notepad_path() -> PathBuf {
    let windir = env::var("WINDIR").expect("WINDIR not set");
    let path = PathBuf::from(format!(r"{windir}\System32\notepad.exe"));
    assert!(path.exists(), "notepad not found at {}", path.display());
    path
}

#[test]
fn roundtrip_notepad() {
    let tmp = TempPath::new(function_name!(), "lnk");

    let target = notepad_path();
    let expected = Shortcut {
        target_path: Some(target.clone()),
        arguments: Some("\"arg1\" /ARG2 --arg3 -- arg4".to_string()),
        working_dir: Some(std::env::temp_dir()),
        description: Some("roundtrip shortcut".to_string()),
        icon: Some(Icon::new(target.clone())),
        window_state: WindowState::default(),
        hotkey: None,
        #[cfg(feature = "runas")]
        run_as_admin: false,
    };
    expected
        .save(tmp.path())
        .expect("failed to create shortcut");

    let actual = Shortcut::load(tmp.path()).expect("failed to load shortcut");
    assert_eq!(expected, actual);
}

#[test]
fn roundtrip_run_as_admin_bit() {
    let tmp = TempPath::new(function_name!(), "lnk");

    let target = notepad_path();
    let expected = Shortcut::new(target);
    expected
        .save(tmp.path())
        .expect("failed to create shortcut");
    let actual = Shortcut::load(tmp.path()).expect("failed to load shortcut");
    assert_eq!(expected, actual);
}

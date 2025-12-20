use lnks::{Hotkey, Icon, Shortcut, WindowState};
use std::{
    env,
    fmt::{self, Debug},
    path::{Path, PathBuf},
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

fn canonicalize(p: impl AsRef<Path>) -> PathBuf {
    let p = p.as_ref();
    dunce::canonicalize(p).unwrap_or(p.to_path_buf())
}

pub struct CanonicalEq(pub PathBuf);

impl PartialEq for CanonicalEq {
    fn eq(&self, other: &Self) -> bool {
        let l = canonicalize(&self.0).into_os_string();
        let r = canonicalize(&other.0).into_os_string();
        l.eq_ignore_ascii_case(r)
    }
}

impl Debug for CanonicalEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

fn assert_path_option_eq(left: &Option<PathBuf>, right: &Option<PathBuf>, message: &str) {
    let left = left.as_ref().map(|l| CanonicalEq(l.to_path_buf()));
    let right = right.as_ref().map(|r| CanonicalEq(r.to_path_buf()));
    assert_eq!(left, right, "{}", message);
}

fn assert_shortcut_eq(left: &Shortcut, right: &Shortcut) {
    assert_path_option_eq(
        &left.target_path,
        &right.target_path,
        "target paths do not match",
    );
    assert_eq!(left.arguments, right.arguments);
    assert_path_option_eq(
        &left.working_dir,
        &right.working_dir,
        "working directories do not match",
    );
    assert_eq!(
        left.description, right.description,
        "descriptions do not match"
    );
    assert_eq!(
        left.icon.as_ref().map(|l| l.index),
        right.icon.as_ref().map(|r| r.index),
        "icon indices do not match"
    );
    assert_path_option_eq(
        &left.icon.clone().map(|l| l.path),
        &right.icon.clone().map(|r| r.path),
        "icon paths do not match",
    );
    assert_eq!(
        left.window_state, right.window_state,
        "window states do not match"
    );
    assert_eq!(left.hotkey, right.hotkey, "hotkeys do not match");
    assert_eq!(left.run_as_admin, right.run_as_admin, "runas do not match");
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
        run_as_admin: false,
    };
    expected
        .save(tmp.path())
        .expect("failed to create shortcut");

    let actual = Shortcut::load(tmp.path()).expect("failed to load shortcut");
    assert_shortcut_eq(&expected, &actual);
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
    assert_shortcut_eq(&expected, &actual);
}

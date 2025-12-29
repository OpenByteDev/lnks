#![cfg(windows)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::redundant_closure_for_method_calls
)]

//! `lnks` provides a high-level API for reading and writing Windows `.lnk` (Shell Link) files.
//!
//! It wraps the COM-based Shell APIs [`IShellLinkW`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishelllinkw) and [`IPersistFile`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersistfile), including support for reading and toggling the undocumented "Run as administrator" flag.
//!
//! ## Examples
//!
//! ### Load an existing shortcut
//!
//! ```no_run
//! # use std::path::Path;
//! # fn main() {
//! let path = Path::new(r"C:\Users\Public\Desktop\Notepad.lnk");
//! let shortcut = lnks::Shortcut::load(path).unwrap();
//! # }
//! ```
//!
//! ### Create a new shortcut
//!
//! ```no_run
//! # use std::path::Path;
//! # fn main() {
//! let mut shortcut = lnks::Shortcut::new(r"C:\Windows\System32\notepad.exe");
//! shortcut.arguments = Some(r"C:\Windows\win.ini".to_string());
//!
//! let out = Path::new(r"C:\Users\Public\Desktop\Notepad.lnk");
//! shortcut.save(out).unwrap();
//! # }
//! ```

mod buf_utils;
#[cfg(feature = "runas")]
mod runas;

/// Module containing COM related utils.
pub mod com;

mod error;
pub use error::*;

mod hotkey;
pub use hotkey::*;

mod icon;
pub use icon::*;

mod virtual_key;
pub use virtual_key::*;

mod shortcut;
pub use shortcut::*;

mod window_state;
pub use window_state::*;

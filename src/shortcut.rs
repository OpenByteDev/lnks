use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    ptr,
};

use num_enum::FromPrimitive;
use widestring::U16CString;
use windows::{
    Win32::{
        System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance, IPersistFile, STGM},
        UI::Shell::{IShellLinkW, SLGP_RAWPATH, ShellLink},
    },
    core::{Interface, PCWSTR},
};

use crate::{
    Hotkey, Icon, Result, WindowState,
    buf_utils::{com_get_optional_path, com_get_optional_string},
    com::{self, ComResultExt},
    runas,
};

/// Represents a Windows shortcut (`.lnk`) file.
///
/// See also <https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishelllinka>
#[derive(Debug, Clone, Default)]
pub struct Shortcut {
    /// Path to the target executable or file.
    pub target_path: Option<PathBuf>,

    /// Command-line arguments passed to the target.
    pub arguments: Option<String>,

    /// Working directory used when launching the target.
    pub working_dir: Option<PathBuf>,

    /// Human-readable shortcut description.
    pub description: Option<String>,

    /// Icon location (path + index).
    pub icon: Option<Icon>,

    /// Initial window state (normal, minimized, maximized).
    pub window_state: WindowState,

    /// Optional keyboard shortcut.
    pub hotkey: Option<Hotkey>,

    #[cfg(feature = "runas")]
    /// Whether the target should be run as administrator.
    pub run_as_admin: bool,
}

/// Options for loading a [`Shortcut`], can be specified when using [`load_with_options`](Shortcut::load_with_options).
#[derive(Debug, Clone, Copy)]
pub struct LoadOptions {
    /// If true, canonicalizes paths.
    pub canonicalize: bool,
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self { canonicalize: true } // keep current behavior as default
    }
}

/// Options for saving a [`Shortcut`], can be specified when using [`save_with_options`](Shortcut::save_with_options).
#[derive(Debug, Clone, Copy)]
pub struct SaveOptions {
    /// If true, canonicalizes paths.
    pub canonicalize: bool,
}

impl Default for SaveOptions {
    fn default() -> Self {
        Self { canonicalize: true }
    }
}

impl Shortcut {
    /// Create a new empty [`ShortcutBuilder`].
    #[must_use]
    pub fn builder() -> ShortcutBuilder {
        ShortcutBuilder::default()
    }

    /// Creates a new shortcut targeting the given executable.
    #[must_use]
    pub fn new(target_path: impl Into<PathBuf>) -> Self {
        let target_path = target_path.into();
        Self {
            target_path: Some(target_path.clone()),
            working_dir: target_path.parent().map(|p| p.to_path_buf()),
            icon: Some(Icon {
                path: target_path,
                index: 0,
            }),
            ..Default::default()
        }
    }

    /// Loads a `.lnk` file from disk and parses it into a [`Shortcut`].
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        Self::load_with_options(path, LoadOptions::default())
    }

    /// Canonicalizes all filesystem paths contained in this shortcut in-place.
    ///
    /// The following fields are affected:
    /// - [`target_path`](Self::target_path)
    /// - [`working_dir`](Self::working_dir)
    /// - [`icon.path`](Icon::path)
    ///
    /// # Base path
    /// If a path is **relative** and `base` is [`Some`], it is first resolved
    /// relative to `base` (typically the directory containing the `.lnk` file)
    /// and then canonicalized.
    /// 
    /// # Errors
    /// Errors during canonicalization are ignored.
    pub fn canonicalize(&mut self, base: Option<&Path>) {
        try_canonicalize_with_base_inplace_opt(self.target_path.as_mut(), base);
        try_canonicalize_with_base_inplace_opt(self.working_dir.as_mut(), base);
        try_canonicalize_with_base_inplace_opt(self.icon.as_mut().map(|i| &mut i.path), base);
    }

    /// Returns a canonicalized clone of this shortcut.
    ///
    /// This is a non-mutating variant of [`canonicalize`](Self::canonicalize).
    /// The original [`Shortcut`] is left unchanged.
    pub fn canonicalized(&self, base: Option<&Path>) -> Self {
        let mut clone = self.clone();
        clone.canonicalize(base);
        clone
    }

    /// Loads a `.lnk` file from disk and parses it into a [`Shortcut`], considering the given [`LoadOptions`].
    pub fn load_with_options(path: impl AsRef<Path>, options: LoadOptions) -> Result<Self> {
        let path = path.as_ref();
        com::ensure_initialized()?;

        let link: IShellLinkW = unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) }
            .context(None, "CoCreateInstance")?;
        let persist: IPersistFile = link.cast().context(Some("IUnknown"), "QueryInterface")?;

        let path = dunce::canonicalize(path)?;
        let wpath = U16CString::from_os_str(path.as_os_str())?;
        unsafe { persist.Load(PCWSTR(wpath.as_ptr()), STGM(0)) }
            .context(Some("IPersistFile"), "Load")?;

        let target_path = com_get_optional_path(|b| {
            unsafe { link.GetPath(b, ptr::null_mut(), SLGP_RAWPATH.0 as _) }
                .context(Some("IShellLinkW"), "GetPath")
        })?;
        let arguments = com_get_optional_string(|b| {
            unsafe { link.GetArguments(b) }.context(Some("IShellLinkW"), "GetArguments")
        })?;
        let working_dir = com_get_optional_path(|b| {
            unsafe { link.GetWorkingDirectory(b) }
                .context(Some("IShellLinkW"), "GetWorkingDirectory")
        })?;
        let description = com_get_optional_string(|b| {
            unsafe { link.GetDescription(b) }.context(Some("IShellLinkW"), "GetDescription")
        })?;

        let mut icon_index = 0;
        let icon_path = com_get_optional_path(|b| {
            unsafe { link.GetIconLocation(b, &raw mut icon_index) }
                .context(Some("IShellLinkW"), "GetIconLocation")
        })?;
        let icon = icon_path.map(Icon::new);

        let window_state_raw =
            unsafe { link.GetShowCmd() }.context(Some("IShellLinkW"), "GetShowCmd")?;
        let window_state = WindowState::from_primitive(window_state_raw.0);

        let hotkey_raw = unsafe { link.GetHotkey() }.context(Some("IShellLinkW"), "GetHotkey")?;
        let hotkey = Hotkey::from_raw(hotkey_raw);

        #[cfg(feature = "runas")]
        let run_as_admin = runas::read_runas_bit(&path)?;

        let mut shortcut = Shortcut {
            target_path,
            arguments,
            working_dir,
            description,
            icon,
            window_state,
            hotkey,
            #[cfg(feature = "runas")]
            run_as_admin,
        };

        if options.canonicalize {
            shortcut.canonicalize(path.parent());
        }

        Ok(shortcut)
    }

    /// Saves the shortcut to disk as a `.lnk` file.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        self.save_with_options(path, SaveOptions::default())
    }

    /// Saves the shortcut to disk as a `.lnk` file, considering the given [`SaveOptions`].
    pub fn save_with_options(&self, path: impl AsRef<Path>, options: SaveOptions) -> Result<()> {
        com::ensure_initialized()?;

        let path = maybe_try_canonicalize(path.as_ref(), options.canonicalize);
        let base = path.parent();

        let link: IShellLinkW = unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) }
            .context(None, "CoCreateInstance")?;

        if let Some(target_path) = &self.target_path {
            let target_path =
                maybe_try_canonicalize_with_base(target_path, base, options.canonicalize);
            let w = U16CString::from_os_str(target_path.as_os_str())?;
            unsafe { link.SetPath(PCWSTR(w.as_ptr())) }.context(Some("IShellLinkW"), "SetPath")?;
        }

        if let Some(arguments) = &self.arguments {
            let w = U16CString::from_str(arguments)?;
            unsafe { link.SetArguments(PCWSTR(w.as_ptr())) }
                .context(Some("IShellLinkW"), "SetArguments")?;
        }

        if let Some(working_dir) = &self.working_dir {
            let working_dir =
                maybe_try_canonicalize_with_base(working_dir, base, options.canonicalize);
            let w = U16CString::from_os_str(working_dir.as_os_str())?;
            unsafe { link.SetWorkingDirectory(PCWSTR(w.as_ptr())) }
                .context(Some("IShellLinkW"), "SetWorkingDirectory")?;
        }

        if let Some(description) = &self.description {
            let w = U16CString::from_str(description)?;
            unsafe { link.SetDescription(PCWSTR(w.as_ptr())) }
                .context(Some("IShellLinkW"), "SetDescription")?;
        }

        if let Some(icon) = &self.icon {
            let icon_path =
                maybe_try_canonicalize_with_base(&icon.path, base, options.canonicalize);
            let w = U16CString::from_os_str(icon_path.as_os_str())?;
            unsafe { link.SetIconLocation(PCWSTR(w.as_ptr()), icon.index) }
                .context(Some("IShellLinkW"), "SetIconLocation")?;
        }

        unsafe { link.SetShowCmd(self.window_state.to_raw()) }
            .context(Some("IShellLinkW"), "SetShowCmd")?;
        unsafe { link.SetHotkey(self.hotkey.map_or(0, |h| h.to_raw())) }
            .context(Some("IShellLinkW"), "SetHotkey")?;

        let persist: IPersistFile = link.cast().context(Some("IUnknown"), "QueryInterface")?;
        let wout = U16CString::from_os_str(path.as_os_str())?;
        unsafe { persist.Save(PCWSTR(wout.as_ptr()), true) }
            .context(Some("IPersistFile"), "Save")?;
        unsafe { persist.SaveCompleted(PCWSTR(wout.as_ptr())) }
            .context(Some("IPersistFile"), "SaveCompleted")?;

        #[cfg(feature = "runas")]
        if self.run_as_admin {
            runas::write_runas_bit(&path, true)?;
        }

        Ok(())
    }
}

fn cmp_path(a: &Path, b: &Path) -> bool {
    try_canonicalize(a)
        .as_os_str()
        .eq_ignore_ascii_case(try_canonicalize(b).as_os_str())
}

#[allow(clippy::ref_option)]
fn cmp_opt_path(a: &Option<PathBuf>, b: &Option<PathBuf>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => cmp_path(a.as_path(), b.as_path()),
        _ => false,
    }
}

impl PartialEq for Shortcut {
    fn eq(&self, other: &Self) -> bool {
        if !cmp_opt_path(&self.target_path, &other.target_path) {
            return false;
        }

        if self.arguments != other.arguments {
            return false;
        }

        if !cmp_opt_path(&self.working_dir, &other.working_dir) {
            return false;
        }

        if self.description != other.description {
            return false;
        }

        if self.icon != other.icon {
            return false;
        }

        if self.window_state != other.window_state {
            return false;
        }

        if self.hotkey != other.hotkey {
            return false;
        }

        #[cfg(feature = "runas")]
        if self.run_as_admin != other.run_as_admin {
            return false;
        }

        true
    }
}

/// Builder for [`Shortcut`] to support ergonomic construction.
///
/// Example:
/// ```no_run
/// # use std::path::PathBuf;
/// # fn main() {
/// let s = lnks::ShortcutBuilder::new(r"C:\Windows\system32\notepad.exe")
///     .arguments(r"C:\Windows\win.ini")
///     .description("My Shortcut")
///     .build();
/// # }
/// ```
#[derive(Debug, Clone, Default)]
pub struct ShortcutBuilder {
    inner: Shortcut,
}

impl ShortcutBuilder {
    /// Create a new builder with a target path.
    #[must_use]
    pub fn new(target_path: impl Into<PathBuf>) -> Self {
        Self {
            inner: Shortcut::new(target_path),
        }
    }

    /// Set command-line arguments.
    #[must_use]
    pub fn arguments(mut self, args: impl Into<String>) -> Self {
        self.inner.arguments = Some(args.into());
        self
    }

    /// Set the working directory.
    #[must_use]
    pub fn working_dir(mut self, wd: impl Into<PathBuf>) -> Self {
        self.inner.working_dir = Some(wd.into());
        self
    }

    /// Set the description.
    #[must_use]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.inner.description = Some(desc.into());
        self
    }

    /// Set the icon.
    #[must_use]
    pub fn icon(mut self, icon: Icon) -> Self {
        self.inner.icon = Some(icon);
        self
    }

    /// Set the window state.
    #[must_use]
    pub fn window_state(mut self, state: WindowState) -> Self {
        self.inner.window_state = state;
        self
    }

    /// Set the hotkey.
    #[must_use]
    pub fn hotkey(mut self, hotkey: Hotkey) -> Self {
        self.inner.hotkey = Some(hotkey);
        self
    }

    /// Finalize the builder and return the constructed [`Shortcut`].
    #[must_use]
    pub fn build(self) -> Shortcut {
        self.inner
    }
}

fn try_canonicalize_inplace(path: &mut PathBuf) -> bool {
    if let Ok(canon) = dunce::canonicalize(&path) {
        *path = canon;
        true
    } else {
        false
    }
}
/* 
fn try_canonicalize_inplace_opt(opt: Option<&mut PathBuf>) -> bool {
    if let Some(path) = opt {
        try_canonicalize_inplace(path)
    } else {
        false
    }
}
*/
#[allow(clippy::unnecessary_unwrap)]
fn try_canonicalize_with_base_inplace(path: &mut PathBuf, base: Option<&Path>) -> bool {
    if path.is_absolute() || base.is_none() {
        try_canonicalize_inplace(path)
    } else {
        let mut absolute = base.unwrap().join(&path);
        let success = try_canonicalize_inplace(&mut absolute);
        if success {
            *path = absolute;
        }
        success
    }
}
fn try_canonicalize_with_base_inplace_opt(opt: Option<&mut PathBuf>, base: Option<&Path>) -> bool {
    if let Some(path) = opt {
        try_canonicalize_with_base_inplace(path, base)
    } else {
        false
    }
}

fn try_canonicalize(path: &'_ Path) -> Cow<'_, Path> {
    dunce::canonicalize(path).map_or_else(|_| Cow::Borrowed(path), Cow::Owned)
}
fn maybe_try_canonicalize(path: &'_ Path, enabled: bool) -> Cow<'_, Path> {
    if enabled {
        try_canonicalize(path)
    } else {
        Cow::Borrowed(path)
    }
}

#[allow(clippy::unnecessary_unwrap)]
fn try_canonicalize_with_base<'a>(path: &'a Path, base: Option<&'_ Path>) -> Cow<'a, Path> {
    if path.is_absolute() || base.is_none() {
        try_canonicalize(path)
    } else {
        let mut absolute = base.unwrap().join(path);
        if try_canonicalize_inplace(&mut absolute) {
            Cow::Owned(absolute)
        } else {
            Cow::Borrowed(path)
        }
    }
}
fn maybe_try_canonicalize_with_base<'a>(
    path: &'a Path,
    base: Option<&'_ Path>,
    enabled: bool,
) -> Cow<'a, Path> {
    if enabled {
        try_canonicalize_with_base(path, base)
    } else {
        Cow::Borrowed(path)
    }
}

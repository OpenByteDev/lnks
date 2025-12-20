use std::{
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

    /// Whether the target should be run as administrator.
    pub run_as_admin: bool,
}

impl Shortcut {
    /// Creates a new shortcut targeting the given executable.
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
        let path = path.as_ref();
        com::ensure_initialized()?;

        let link: IShellLinkW = unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) }
            .context(None, "CoCreateInstance")?;
        let persist: IPersistFile = link.cast().context(Some("IUnknown"), "QueryInterface")?;

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

        let run_as_admin = runas::read_runas_bit(path)?;

        let shortcut = Shortcut {
            target_path,
            arguments,
            working_dir,
            description,
            icon,
            window_state,
            hotkey,
            run_as_admin,
        };
        Ok(shortcut)
    }

    /// Saves the shortcut to disk as a `.lnk` file.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        com::ensure_initialized()?;

        let link: IShellLinkW = unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) }
            .context(None, "CoCreateInstance")?;

        if let Some(tp) = &self.target_path {
            let w = U16CString::from_os_str(tp.as_os_str())?;
            unsafe { link.SetPath(PCWSTR(w.as_ptr())) }.context(Some("IShellLinkW"), "SetPath")?;
        }

        if let Some(args) = &self.arguments {
            let w = U16CString::from_str(args)?;
            unsafe { link.SetArguments(PCWSTR(w.as_ptr())) }
                .context(Some("IShellLinkW"), "SetArguments")?;
        }

        if let Some(wd) = &self.working_dir {
            let w = U16CString::from_os_str(wd.as_os_str())?;
            unsafe { link.SetWorkingDirectory(PCWSTR(w.as_ptr())) }
                .context(Some("IShellLinkW"), "SetWorkingDirectory")?;
        }

        if let Some(desc) = &self.description {
            let w = U16CString::from_str(desc)?;
            unsafe { link.SetDescription(PCWSTR(w.as_ptr())) }
                .context(Some("IShellLinkW"), "SetDescription")?;
        }

        if let Some(icon) = &self.icon {
            let w = U16CString::from_os_str(icon.path.as_os_str())?;
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

        if self.run_as_admin {
            runas::write_runas_bit(path, true)?;
        }

        Ok(())
    }
}

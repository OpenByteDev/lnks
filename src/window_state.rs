use num_enum::{self, FromPrimitive, IntoPrimitive};
use std::fmt;
use windows::Win32::UI::WindowsAndMessaging::{
    SHOW_WINDOW_CMD, SW_SHOWMAXIMIZED, SW_SHOWMINIMIZED, SW_SHOWNORMAL,
};

/// Initial window display state for a launched application.
///
/// See also <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow>
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, FromPrimitive)]
pub enum WindowState {
    /// Normal window (`SW_SHOWNORMAL`).
    ///
    /// The window is shown in its default restored size and position.
    Normal = SW_SHOWNORMAL.0,

    /// Minimized window (`SW_SHOWMINIMIZED`).
    ///
    /// The window is launched minimized to the taskbar.
    Minimized = SW_SHOWMINIMIZED.0,

    /// Maximized window (`SW_SHOWMAXIMIZED`).
    ///
    /// The window is launched maximized to fill the screen.
    Maximized = SW_SHOWMAXIMIZED.0,

    /// Another unknown window state.
    #[num_enum(catch_all)]
    Other(i32),
}

impl WindowState {
    /// Creates a [`WindowState`] from a raw Win32 [`SHOW_WINDOW_CMD`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/struct.SHOW_WINDOW_CMD.html).
    ///
    /// Unknown values are mapped to [`Self::Other`].
    #[must_use]
    pub fn from_raw(raw: SHOW_WINDOW_CMD) -> Self {
        Self::from_code(raw.0)
    }

    /// Converts this [`WindowState`] into a raw Win32 [`SHOW_WINDOW_CMD`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/struct.SHOW_WINDOW_CMD.html).
    #[must_use]
    pub fn to_raw(&self) -> SHOW_WINDOW_CMD {
        SHOW_WINDOW_CMD(self.to_code())
    }

    /// Creates a [`WindowState`] from a raw integer value.
    #[must_use]
    pub fn from_code(raw: i32) -> Self {
        Self::from_primitive(raw)
    }

    /// Returns the underlying integer representation.
    ///
    /// This corresponds directly to the Win32 `SW_*` constants.
    #[must_use]
    pub fn to_code(&self) -> i32 {
        i32::from(*self)
    }
}

impl fmt::Display for WindowState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "Normal"),
            Self::Minimized => write!(f, "Minimized"),
            Self::Maximized => write!(f, "Maximized"),
            Self::Other(raw) => write!(f, "{raw}"),
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for WindowState {
    fn default() -> Self {
        Self::Normal
    }
}

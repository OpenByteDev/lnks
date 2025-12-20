use num_enum::{self, FromPrimitive, IntoPrimitive};
use windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;

/// Initial window display state for a launched application.
///
/// See also <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow>
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, FromPrimitive)]
pub enum WindowState {
    /// Normal window (`SW_SHOWNORMAL`).
    ///
    /// The window is shown in its default restored size and position.
    Normal = 1,

    /// Minimized window (`SW_SHOWMINIMIZED`).
    ///
    /// The window is launched minimized to the taskbar.
    Minimized = 2,

    /// Maximized window (`SW_SHOWMAXIMIZED`).
    ///
    /// The window is launched maximized to fill the screen.
    Maximized = 3,

    /// Another unknown window state.
    #[num_enum(catch_all)]
    Other(i32),
}

impl WindowState {
    /// Creates a [`WindowState`] from a raw Win32 code.
    ///
    /// Unknown values are mapped to [`WindowState::Other`].
    #[must_use]
    pub fn from_raw(raw: SHOW_WINDOW_CMD) -> Self {
        Self::from_code(raw.0)
    }

    /// Converts this [`WindowState`] into a raw Win32 [`SHOW_WINDOW_CMD`].
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

#[allow(clippy::derivable_impls)]
impl Default for WindowState {
    fn default() -> Self {
        Self::Normal
    }
}

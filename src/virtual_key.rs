use num_enum::{FromPrimitive, IntoPrimitive};
use std::{borrow::Cow, fmt};
#[allow(clippy::wildcard_imports)]
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[allow(clippy::cast_possible_truncation)]
const fn convert(key: VIRTUAL_KEY) -> u8 {
    assert!(key.0 <= u8::MAX as u16);
    key.0 as u8
}

/// Represents a Win32 virtual-key code.
///
/// Unknown values are captured in the [`Other`](VirtualKey::Other) variant.
///
/// See: <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes>
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
pub enum VirtualKey {
    // 0x01 - 0x06: Mouse buttons (documented order)
    /// Left mouse button
    LButton = convert(VK_LBUTTON),
    /// Right mouse button
    RButton = convert(VK_RBUTTON),
    /// Control-break processing
    Cancel = convert(VK_CANCEL),
    /// Middle mouse button (three-button mouse)
    MButton = convert(VK_MBUTTON),
    /// X1 mouse button
    XButton1 = convert(VK_XBUTTON1),
    /// X2 mouse button
    XButton2 = convert(VK_XBUTTON2),

    // 0x08 - 0x0D: Editing keys
    /// BACKSPACE key
    Back = convert(VK_BACK),
    /// TAB key
    Tab = convert(VK_TAB),
    /// CLEAR key
    Clear = convert(VK_CLEAR),
    /// ENTER key
    Return = convert(VK_RETURN),

    // 0x10 - 0x14: Modifier and toggle keys
    /// SHIFT key
    Shift = convert(VK_SHIFT),
    /// CTRL key
    Control = convert(VK_CONTROL),
    /// ALT (MENU) key
    Menu = convert(VK_MENU),
    /// PAUSE key
    Pause = convert(VK_PAUSE),
    /// CAPS LOCK
    Capital = convert(VK_CAPITAL),
    /// Left SHIFT
    LShift = convert(VK_LSHIFT),
    /// Right SHIFT
    RShift = convert(VK_RSHIFT),
    /// Left CTRL
    LControl = convert(VK_LCONTROL),
    /// Right CTRL
    RControl = convert(VK_RCONTROL),
    /// Left ALT (Menu)
    LMenu = convert(VK_LMENU),
    /// Right ALT (Menu)
    RMenu = convert(VK_RMENU),

    // 0x20 - 0x26: Space and navigation
    /// SPACEBAR
    Space = convert(VK_SPACE),
    /// PAGE UP key
    PageUp = convert(VK_PRIOR),
    /// PAGE DOWN key
    PageDown = convert(VK_NEXT),
    /// END key
    End = convert(VK_END),
    /// HOME key
    Home = convert(VK_HOME),
    /// LEFT ARROW key
    Left = convert(VK_LEFT),
    /// UP ARROW key
    Up = convert(VK_UP),
    /// RIGHT ARROW key
    Right = convert(VK_RIGHT),
    /// DOWN ARROW key
    Down = convert(VK_DOWN),

    /// ESCAPE key
    Esc = convert(VK_ESCAPE),

    // 0x30 - 0x39: Number keys
    Key0 = convert(VK_0),
    Key1 = convert(VK_1),
    Key2 = convert(VK_2),
    Key3 = convert(VK_3),
    Key4 = convert(VK_4),
    Key5 = convert(VK_5),
    Key6 = convert(VK_6),
    Key7 = convert(VK_7),
    Key8 = convert(VK_8),
    Key9 = convert(VK_9),

    // 0x41 - 0x5A: A-Z
    A = convert(VK_A),
    B = convert(VK_B),
    C = convert(VK_C),
    D = convert(VK_D),
    E = convert(VK_E),
    F = convert(VK_F),
    G = convert(VK_G),
    H = convert(VK_H),
    I = convert(VK_I),
    J = convert(VK_J),
    K = convert(VK_K),
    L = convert(VK_L),
    M = convert(VK_M),
    N = convert(VK_N),
    O = convert(VK_O),
    P = convert(VK_P),
    Q = convert(VK_Q),
    R = convert(VK_R),
    S = convert(VK_S),
    T = convert(VK_T),
    U = convert(VK_U),
    V = convert(VK_V),
    W = convert(VK_W),
    X = convert(VK_X),
    Y = convert(VK_Y),
    Z = convert(VK_Z),

    // 0x70 - 0x87: Function keys F1-F24
    F1 = convert(VK_F1),
    F2 = convert(VK_F2),
    F3 = convert(VK_F3),
    F4 = convert(VK_F4),
    F5 = convert(VK_F5),
    F6 = convert(VK_F6),
    F7 = convert(VK_F7),
    F8 = convert(VK_F8),
    F9 = convert(VK_F9),
    F10 = convert(VK_F10),
    F11 = convert(VK_F11),
    F12 = convert(VK_F12),
    F13 = convert(VK_F13),
    F14 = convert(VK_F14),
    F15 = convert(VK_F15),
    F16 = convert(VK_F16),
    F17 = convert(VK_F17),
    F18 = convert(VK_F18),
    F19 = convert(VK_F19),
    F20 = convert(VK_F20),
    F21 = convert(VK_F21),
    F22 = convert(VK_F22),
    F23 = convert(VK_F23),
    F24 = convert(VK_F24),

    // 0x2D - 0x2E: Insert/Delete (documented order)
    /// INSERT key
    Insert = convert(VK_INSERT),
    /// DELETE key
    Delete = convert(VK_DELETE),

    // Lock and system keys
    /// Print Screen / Snapshot key
    Snapshot = convert(VK_SNAPSHOT),
    /// Scroll Lock
    Scroll = convert(VK_SCROLL),
    /// Num Lock
    NumLock = convert(VK_NUMLOCK),
    /// Applications (Context menu) key
    Apps = convert(VK_APPS),

    // OEM / punctuation keys
    /// OEM 1 (`;:` on US keyboards)
    Oem1 = convert(VK_OEM_1),
    /// OEM 2 (`/` on US keyboards)
    Oem2 = convert(VK_OEM_2),
    /// OEM 3 (`` ` `` on US keyboards)
    Oem3 = convert(VK_OEM_3),
    /// OEM 4 (`[` on US keyboards)
    Oem4 = convert(VK_OEM_4),
    /// OEM 5 (`\` on US keyboards)
    Oem5 = convert(VK_OEM_5),
    /// OEM 6 (`]` on US keyboards)
    Oem6 = convert(VK_OEM_6),
    /// OEM 7 (`'` on US keyboards)
    Oem7 = convert(VK_OEM_7),
    /// OEM 8 (misc)
    Oem8 = convert(VK_OEM_8),
    /// OEM 102 (international)
    Oem102 = convert(VK_OEM_102),

    // Catch-all for unknown values
    #[num_enum(catch_all)]
    Other(u8),
}

impl VirtualKey {
    /// Return a short, human-friendly string for this key.
    ///
    /// For known keys this returns a borrowed `&'static str`. For unknown
    /// values ([`Other`](VirtualKey::Other) this returns an owned string like `VK_0xNN`.
    #[must_use]
    pub fn to_str(&self) -> Cow<'static, str> {
        match self {
            // Mouse buttons
            Self::LButton => Cow::Borrowed("LButton"),
            Self::RButton => Cow::Borrowed("RButton"),
            Self::Cancel => Cow::Borrowed("Cancel"),
            Self::MButton => Cow::Borrowed("MButton"),
            Self::XButton1 => Cow::Borrowed("XButton1"),
            Self::XButton2 => Cow::Borrowed("XButton2"),

            // Editing and control
            Self::Back => Cow::Borrowed("Backspace"),
            Self::Tab => Cow::Borrowed("Tab"),
            Self::Clear => Cow::Borrowed("Clear"),
            Self::Return => Cow::Borrowed("Enter"),

            // Modifier and toggle keys
            Self::Shift => Cow::Borrowed("Shift"),
            Self::Control => Cow::Borrowed("Ctrl"),
            Self::Menu => Cow::Borrowed("Menu"),
            Self::Pause => Cow::Borrowed("Pause"),
            Self::Capital => Cow::Borrowed("CapsLock"),
            Self::LShift => Cow::Borrowed("LShift"),
            Self::RShift => Cow::Borrowed("RShift"),
            Self::LControl => Cow::Borrowed("LControl"),
            Self::RControl => Cow::Borrowed("RControl"),
            Self::LMenu => Cow::Borrowed("LMenu"),
            Self::RMenu => Cow::Borrowed("RMenu"),

            // Whitespace / system
            Self::Space => Cow::Borrowed("Space"),
            Self::Esc => Cow::Borrowed("Esc"),

            // Then letters and digits
            Self::A => Cow::Borrowed("A"),
            Self::B => Cow::Borrowed("B"),
            Self::C => Cow::Borrowed("C"),
            Self::D => Cow::Borrowed("D"),
            Self::E => Cow::Borrowed("E"),
            Self::F => Cow::Borrowed("F"),
            Self::G => Cow::Borrowed("G"),
            Self::H => Cow::Borrowed("H"),
            Self::I => Cow::Borrowed("I"),
            Self::J => Cow::Borrowed("J"),
            Self::K => Cow::Borrowed("K"),
            Self::L => Cow::Borrowed("L"),
            Self::M => Cow::Borrowed("M"),
            Self::N => Cow::Borrowed("N"),
            Self::O => Cow::Borrowed("O"),
            Self::P => Cow::Borrowed("P"),
            Self::Q => Cow::Borrowed("Q"),
            Self::R => Cow::Borrowed("R"),
            Self::S => Cow::Borrowed("S"),
            Self::T => Cow::Borrowed("T"),
            Self::U => Cow::Borrowed("U"),
            Self::V => Cow::Borrowed("V"),
            Self::W => Cow::Borrowed("W"),
            Self::X => Cow::Borrowed("X"),
            Self::Y => Cow::Borrowed("Y"),
            Self::Z => Cow::Borrowed("Z"),

            Self::Key0 => Cow::Borrowed("0"),
            Self::Key1 => Cow::Borrowed("1"),
            Self::Key2 => Cow::Borrowed("2"),
            Self::Key3 => Cow::Borrowed("3"),
            Self::Key4 => Cow::Borrowed("4"),
            Self::Key5 => Cow::Borrowed("5"),
            Self::Key6 => Cow::Borrowed("6"),
            Self::Key7 => Cow::Borrowed("7"),
            Self::Key8 => Cow::Borrowed("8"),
            Self::Key9 => Cow::Borrowed("9"),

            Self::F1 => Cow::Borrowed("F1"),
            Self::F2 => Cow::Borrowed("F2"),
            Self::F3 => Cow::Borrowed("F3"),
            Self::F4 => Cow::Borrowed("F4"),
            Self::F5 => Cow::Borrowed("F5"),
            Self::F6 => Cow::Borrowed("F6"),
            Self::F7 => Cow::Borrowed("F7"),
            Self::F8 => Cow::Borrowed("F8"),
            Self::F9 => Cow::Borrowed("F9"),
            Self::F10 => Cow::Borrowed("F10"),
            Self::F11 => Cow::Borrowed("F11"),
            Self::F12 => Cow::Borrowed("F12"),
            Self::F13 => Cow::Borrowed("F13"),
            Self::F14 => Cow::Borrowed("F14"),
            Self::F15 => Cow::Borrowed("F15"),
            Self::F16 => Cow::Borrowed("F16"),
            Self::F17 => Cow::Borrowed("F17"),
            Self::F18 => Cow::Borrowed("F18"),
            Self::F19 => Cow::Borrowed("F19"),
            Self::F20 => Cow::Borrowed("F20"),
            Self::F21 => Cow::Borrowed("F21"),
            Self::F22 => Cow::Borrowed("F22"),
            Self::F23 => Cow::Borrowed("F23"),
            Self::F24 => Cow::Borrowed("F24"),

            Self::Left => Cow::Borrowed("Left"),
            Self::Up => Cow::Borrowed("Up"),
            Self::Right => Cow::Borrowed("Right"),
            Self::Down => Cow::Borrowed("Down"),
            Self::Home => Cow::Borrowed("Home"),
            Self::End => Cow::Borrowed("End"),
            Self::PageUp => Cow::Borrowed("PageUp"),
            Self::PageDown => Cow::Borrowed("PageDown"),

            Self::Insert => Cow::Borrowed("Insert"),
            Self::Delete => Cow::Borrowed("Delete"),

            _ => Cow::Owned(format!("VK_0x{:02X}", self.to_raw())),
        }
    }

    /// Convert this [`VirtualKey`] into the raw [`u8`] virtual-key code.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        self.into()
    }

    /// Construct a [`VirtualKey`] from a raw [`u8`] value.
    #[must_use]
    pub fn from_raw(vk: u8) -> Self {
        Self::from_primitive(vk)
    }

    #[must_use]
    pub fn is_modifier(self) -> bool {
        matches!(
            self,
            Self::Shift
                | Self::Control
                | Self::Menu
                | Self::LShift
                | Self::RShift
                | Self::LControl
                | Self::RControl
                | Self::LMenu
                | Self::RMenu
        )
    }

    #[must_use]
    pub fn is_mouse(self) -> bool {
        matches!(
            self,
            Self::LButton | Self::RButton | Self::MButton | Self::XButton1 | Self::XButton2
        )
    }
}

impl fmt::Display for VirtualKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl TryFrom<VIRTUAL_KEY> for VirtualKey {
    type Error = ();

    fn try_from(vk: VIRTUAL_KEY) -> Result<Self, Self::Error> {
        u8::try_from(vk.0).map(Self::from_raw).map_err(|_| ())
    }
}

impl From<VirtualKey> for VIRTUAL_KEY {
    fn from(vk: VirtualKey) -> Self {
        Self(u16::from(vk.to_raw()))
    }
}

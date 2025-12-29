use crate::VirtualKey;
use enumflags2::{BitFlags, bitflags};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt;
use windows::Win32::UI::Controls::{HOTKEYF_ALT, HOTKEYF_CONTROL, HOTKEYF_EXT, HOTKEYF_SHIFT};

#[allow(clippy::cast_possible_truncation)]
const fn convert(code: u32) -> u8 {
    assert!(code <= u8::MAX as u32);
    code as u8
}

/// Hotkey modifier flags of a shortcut.
#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum HotkeyModifier {
    /// Shift key modifier.
    Shift = convert(HOTKEYF_SHIFT),
    /// Control (Ctrl) key modifier.
    Control = convert(HOTKEYF_CONTROL),
    /// Alt key modifier.
    Alt = convert(HOTKEYF_ALT),
    /// Extended key flag.
    ///
    /// This is set for certain extended keys (for example, keys on the
    /// numeric keypad or other non-standard keys).
    Ext = convert(HOTKEYF_EXT),
}

/// A set of [`HotkeyModifier`]s.
pub type HotkeyModifiers = BitFlags<HotkeyModifier>;

impl std::fmt::Display for HotkeyModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl HotkeyModifier {
    /// Return a short, human-readable string for this modifier.
    #[must_use]
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Shift => "Shift",
            Self::Control => "Ctrl",
            Self::Alt => "Alt",
            Self::Ext => "Ext",
        }
    }

    /// Return the underlying modifier bit as a [`u8`].
    #[must_use]
    pub fn to_raw(&self) -> u8 {
        *self as u8
    }

    /// Construct [`HotkeyModifiers`] from raw bits.
    #[must_use]
    pub fn many_from_raw(bits: u8) -> HotkeyModifiers {
        HotkeyModifiers::from_bits_truncate(bits)
    }

    /// Get the [`HotkeyModifier`] from raw bits if the flags represent a single variant..
    #[must_use]
    pub fn single_from_raw(bits: u8) -> Option<HotkeyModifier> {
        Self::try_from_primitive(bits).ok()
    }
}

/// Represents a Windows shortcut hotkey.
///
/// See also <https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-gethotkey>.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hotkey {
    /// Virtual key code
    key: VirtualKey,
    /// Modifier flags
    modifiers: HotkeyModifiers,
}

impl Hotkey {
    /// Create a hotkey from its parts.
    /// This returns [`None`] if the key code is `0`.
    #[must_use]
    pub fn new(key: VirtualKey, modifiers: impl Into<HotkeyModifiers>) -> Option<Self> {
        if key == VirtualKey::Other(0) {
            return None;
        }
        Some(unsafe { Self::new_unchecked(key, modifiers) })
    }

    /// Identical to [`new`](Self::new) but assumes inputs are valid.
    /// 
    /// # Safety
    /// The key has to be non-zero.
    #[must_use]
    pub unsafe fn new_unchecked(key: VirtualKey, modifiers: impl Into<HotkeyModifiers>) -> Self {
        let modifiers = modifiers.into();
        Self { key, modifiers }
    }

    /// Return the hotkey's virtual key.
    #[must_use]
    pub fn key(&self) -> VirtualKey {
        self.key
    }

    /// Return the hotkey's modifier flags.
    #[must_use]
    pub fn modifiers(&self) -> HotkeyModifiers {
        self.modifiers
    }

    /// Return whether the hotkey has any modifier flags.
    #[must_use]
    pub fn has_modifiers(&self) -> bool {
        !self.modifiers.is_empty()
    }

    /// Creates a [`Hotkey`] from the raw 16-bit representation used by Windows.
    /// ```text
    /// 0xHHLL
    ///    │└─ Low byte : virtual-key code (VK_*)
    ///    └── High byte: modifier flags
    /// ```
    /// Returns [`None`] if the input is zero.
    #[must_use]
    pub fn from_raw(word: u16) -> Option<Self> {
        if word == 0 {
            return None;
        }

        let low = (word & 0x00FF) as u8;
        let high = ((word >> 8) & 0x00FF) as u8;

        let key = VirtualKey::from_raw(low);
        let modifiers = HotkeyModifier::many_from_raw(high);
        Some(Self { key, modifiers })
    }

    /// Identical to [`from_raw`](Self::from_raw) but assumes input is valid.
    /// 
    /// # Safety
    /// The key low byte of the word has to be non-zero and the high byte has to contain only valid modifier flags.
    #[must_use]
    pub unsafe fn from_raw_unchecked(word: u16) -> Self {
        let low = (word & 0x00FF) as u8;
        let high = ((word >> 8) & 0x00FF) as u8;

        let key = VirtualKey::from_raw(low);
        let modifiers = unsafe { HotkeyModifiers::from_bits_unchecked(high) };
        Self { key, modifiers }
    }

    /// Converts this [`Hotkey`] into the raw 16-bit format used by Windows.
    #[must_use]
    pub fn to_raw(&self) -> u16 {
        let low = u16::from(self.key.to_raw());
        let high = u16::from(self.modifiers.bits()) << 8;
        high | low
    }
}

impl From<Hotkey> for u16 {
    fn from(h: Hotkey) -> Self {
        h.to_raw()
    }
}

impl fmt::Display for Hotkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mods = self
            .modifiers
            .iter()
            .map(|m| m.to_str())
            .collect::<Vec<_>>()
            .join("+");
        if self.has_modifiers() {
            write!(f, "{}", self.key)
        } else {
            write!(f, "{}+{}", mods, self.key)
        }
    }
}

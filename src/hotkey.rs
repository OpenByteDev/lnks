use std::num::NonZeroU8;

use enumflags2::{BitFlags, bitflags};

/// Hotkey modifier flags of a shortcut.
#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyMod {
    /// Shift key modifier.
    Shift = 0x01,
    /// Control (Ctrl) key modifier.
    Control = 0x02,
    /// Alt key modifier.
    Alt = 0x04,
    /// Extended key flag.
    ///
    /// This is set for certain extended keys (for example, keys on the
    /// numeric keypad or other non-standard keys).
    Ext = 0x08,
}

/// Represents a Windows shortcut hotkey.
/// If both `key_code` and `modifiers` are zero, no hotkey is assigned.
///
/// See also <https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-gethotkey>.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hotkey {
    /// Virtual key code
    pub key_code: NonZeroU8,
    /// Modifier flags
    pub modifiers: BitFlags<HotkeyMod>,
}

impl Hotkey {
    /// Create a hotkey from its parts.
    #[must_use]
    pub fn new(key_code: NonZeroU8, modifiers: impl Into<BitFlags<HotkeyMod>>) -> Self {
        let modifiers = modifiers.into();
        Self {
            key_code,
            modifiers,
        }
    }

    /// Creates a [`Hotkey`] from the raw 16-bit representation used by Windows.
    /// Low byte = virtual key code, high byte = modifier flags.
    /// Returns `None` if the input is zero.
    #[must_use]
    pub fn from_raw(word: u16) -> Option<Self> {
        if word == 0 {
            return None;
        }

        let low = (word & 0x00FF) as u8;
        let high = ((word >> 8) & 0x00FF) as u8;

        let key_code = NonZeroU8::new(low).unwrap();
        let modifiers = BitFlags::<HotkeyMod>::from_bits_truncate(high);
        Some(Self::new(key_code, modifiers))
    }

    /// Converts this [`Hotkey`] into the raw 16-bit format used by Windows.
    #[must_use]
    pub fn to_raw(&self) -> u16 {
        let low = u16::from(self.key_code.get());
        let high = u16::from(self.modifiers.bits()) << 8;
        high | low
    }
}

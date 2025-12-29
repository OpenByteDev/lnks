use thiserror::Error;

use crate::{com, runas};

/// Top-level error type for this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A Windows / COM API returned an error.
    #[error("COM error: {0}")]
    Com(
        #[from]
        #[source]
        com::Error,
    ),

    /// Filesystem IO failed.
    #[error("Filesystem error: {0}")]
    Io(
        #[from]
        #[source]
        std::io::Error,
    ),

    /// UTF-16 conversion failed.
    #[error("UTF-16 conversion error: {0}")]
    Utf16(
        #[from]
        #[source]
        widestring::error::Utf16Error,
    ),

    /// The input contained an interior NUL.
    #[error("string/path contains NUL: {0}")]
    ContainsNul(
        #[from]
        #[source]
        widestring::error::ContainsNul<u16>,
    ),

    #[cfg(feature = "runas")]
    /// Failed to get or set `runas` flag.
    #[error("failed to get/set runas flag: {0}")]
    RunAsFlag(
        #[from]
        #[source]
        runas::Error,
    ),
}

/// Result alias for this crates error type.
pub type Result<T> = std::result::Result<T, Error>;

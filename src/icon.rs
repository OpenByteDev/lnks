use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

/// Icon resource used by a Windows shortcut.
#[derive(Debug, Clone, Default)]
pub struct Icon {
    /// Path to the file containing the icon resource.
    ///
    /// This is typically an executable (`.exe`), DLL, or `.ico` file.
    pub path: PathBuf,

    /// Index of the icon within the file.
    ///
    /// An index of `0` selects the first icon. Negative values may be used by
    /// some APIs to indicate a resource ID rather than a zero-based index.
    pub index: i32,
}

impl Icon {
    /// Create a new [`Icon`] using the first icon (`index = 0`) from the given path.
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self::with_index(path, 0)
    }

    /// Create a new [`Icon`] using an explicit icon index.
    #[must_use]
    pub fn with_index(path: PathBuf, index: i32) -> Self {
        Self { path, index }
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.path.display(), self.index)
    }
}

impl From<PathBuf> for Icon {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl From<Icon> for PathBuf {
    fn from(icon: Icon) -> PathBuf {
        icon.path
    }
}

impl AsRef<Path> for Icon {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl PartialEq for Icon {
    fn eq(&self, other: &Self) -> bool {
        if self.index != other.index {
            return false;
        }

        let a = &self.path;
        let b = &other.path;
        match (fs::canonicalize(a), fs::canonicalize(b)) {
            (Ok(a), Ok(b)) => a.as_os_str().eq_ignore_ascii_case(b.as_os_str()),
            _ => a.as_os_str().eq_ignore_ascii_case(b.as_os_str()),
        }
    }
}

impl Eq for Icon {}

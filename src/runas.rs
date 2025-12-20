use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

/// Errors that can occur when reading or modifying the `runas` flag in a `.lnk` file.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to read or write the `.lnk` file.
    #[error("IO error while accessing {}: {}", path.display(), source)]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// The layout (length) of the `.lnk` was smaller than expected.
    #[error("Unexpected layout encountered while accessing {}", path.display())]
    UnexpectedLayout { path: PathBuf },
}

trait IoLayoutExt<T> {
    fn context(self, path: &Path) -> Result<T, Error>;
}

impl<T> IoLayoutExt<T> for io::Result<T> {
    fn context(self, path: &Path) -> Result<T, Error> {
        self.map_err(|source| {
            let path = path.to_path_buf();
            if source.kind() == io::ErrorKind::UnexpectedEof {
                Error::UnexpectedLayout { path }
            } else {
                Error::Io { path, source }
            }
        })
    }
}

const PATCH_BYTE_OFFSET: u64 = 0x15;
const PATCH_BIT_MASK: u8 = 0x20;

pub(crate) fn read_runas_bit(path: &Path) -> Result<bool, Error> {
    let mut file = File::open(path).context(path)?;

    file.seek(SeekFrom::Start(PATCH_BYTE_OFFSET))
        .context(path)?;
    let mut byte = [0u8; 1];
    file.read_exact(&mut byte).context(path)?;

    Ok((byte[0] & PATCH_BIT_MASK) != 0)
}

pub(crate) fn write_runas_bit(path: &Path, enable: bool) -> Result<(), Error> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .open(path)
        .context(path)?;

    file.seek(SeekFrom::Start(PATCH_BYTE_OFFSET))
        .context(path)?;
    let mut byte = [0u8; 1];
    file.read_exact(&mut byte).context(path)?;

    if enable {
        byte[0] |= PATCH_BIT_MASK;
    } else {
        byte[0] &= !PATCH_BIT_MASK;
    }

    file.seek(SeekFrom::Start(PATCH_BYTE_OFFSET))
        .context(path)?;
    file.write_all(&byte).context(path)?;

    Ok(())
}

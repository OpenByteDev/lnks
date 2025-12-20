use crate::Result;
use std::path::PathBuf;
use widestring::U16CString;
use windows::Win32::Foundation::MAX_PATH;

const INITIAL_CAPACITY: usize = MAX_PATH as _;
const MAX_CAPACITY: usize = 4 * 1024;

pub(crate) fn com_get_with_growth(
    mut f: impl FnMut(&mut [u16]) -> Result<()>,
) -> Result<U16CString> {
    let mut buf = vec![0u16; INITIAL_CAPACITY];

    loop {
        let res = f(&mut buf);

        // COM APIs do not consitently return an error and just truncate the output
        // but still include the null terminator.
        // Thus we just check if the second to last element is null to guess whether
        // the output was truncated.
        if buf[0] != 0 && buf[buf.len() - 2] != 0 {
            let new_capacity = buf.capacity().saturating_mul(2);
            if new_capacity <= MAX_CAPACITY {
                buf.resize(new_capacity, 0);
                continue;
            }
        }

        return res.map(|()| U16CString::from_vec_truncate(buf));
    }
}

pub(crate) fn com_get_string(f: impl FnMut(&mut [u16]) -> Result<()>) -> Result<String> {
    com_get_with_growth(f)?
        .to_string()
        .map_err(crate::Error::Utf16)
}

pub(crate) fn com_get_optional_string(
    f: impl FnMut(&mut [u16]) -> Result<()>,
) -> Result<Option<String>> {
    let s = com_get_string(f)?;
    if s.is_empty() { Ok(None) } else { Ok(Some(s)) }
}

#[allow(dead_code)]
pub(crate) fn com_get_path(f: impl FnMut(&mut [u16]) -> Result<()>) -> Result<PathBuf> {
    let str = com_get_with_growth(f)?;
    Ok(PathBuf::from(str.to_os_string()))
}

pub(crate) fn com_get_optional_path(
    f: impl FnMut(&mut [u16]) -> Result<()>,
) -> Result<Option<PathBuf>> {
    let s = com_get_with_growth(f)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(PathBuf::from(s.to_os_string())))
    }
}

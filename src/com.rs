use std::cell::Cell;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use windows::Win32::System::Com::{
    COINIT, COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED, CoInitializeEx, CoUninitialize,
};
use windows::core::HRESULT;

/// COM apartment model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    /// Single-Threaded Apartment (STA)
    Sta,
    /// Multi-Threaded Apartment (MTA)
    Mta,
}

impl Type {
    #[must_use]
    pub fn as_raw(&self) -> COINIT {
        match self {
            Type::Sta => COINIT_APARTMENTTHREADED,
            Type::Mta => COINIT_MULTITHREADED,
        }
    }

    #[must_use]
    pub fn as_code(&self) -> i32 {
        self.as_raw().0
    }
}

/// HRESULT returned when COM is already initialized on the current thread with a
/// different apartment model than requested.
///
/// In this situation COM remains usable, but the requested model is not applied.
const RPC_E_CHANGED_MODE: HRESULT = HRESULT(0x8001_0106_u32.cast_signed());

thread_local! {
    /// Per-thread flag that stores whether COM has been initialized.
    static COM_INIT: Cell<bool> = const { Cell::new(false) };
}

/// Result of a COM initialization attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initialization {
    /// COM was successfully initialized by this call.
    Success,
    /// COM was already initialized on this thread, either:
    /// - by a previous call to `initialize_as`
    /// - by external code
    AlreadyInitialized,
}

/// Initializes COM for the current thread.
/// This is done automatically when reading or writing shortcuts (as STA).
///
/// See also <https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex>
pub fn initialize_as(kind: Type) -> Result<Initialization, Error> {
    COM_INIT.with(|state| {
        if state.get() {
            return Ok(Initialization::AlreadyInitialized);
        }

        let coinit: COINIT = match kind {
            Type::Sta => COINIT_APARTMENTTHREADED,
            Type::Mta => COINIT_MULTITHREADED,
        };

        let hresult = unsafe { CoInitializeEx(None, coinit) };

        match hresult {
            HRESULT(0) => {
                state.set(true);
                Ok(Initialization::Success)
            }
            HRESULT(1) | RPC_E_CHANGED_MODE => {
                state.set(true);
                Ok(Initialization::AlreadyInitialized)
            }
            _ => Err(windows::core::Error::from_hresult(hresult).context(None, "CoInitializeEx")),
        }
    })
}

pub(crate) fn ensure_initialized() -> crate::Result<()> {
    match initialize_as(Type::Sta) {
        Ok(_) => Ok(()),
        Err(err) => Err(crate::Error::Com(err)),
    }
}

/// Uninitialize COM for the current thread.
///
/// See also <https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize>
pub fn uninitialize() {
    COM_INIT.with(|state| {
        if state.get() {
            unsafe { CoUninitialize() };
        }
    });
}

/// RAII wrapper around [`initialize_as`] / [`uninitialize`].
#[derive(Debug)]
pub struct Session {
    uninit_on_drop: bool,
    _no_send: PhantomData<Rc<()>>,
}

impl Session {
    /// Initializes COM for the current thread and returns a guard that will call
    /// [`uninitialize`] on drop if COM was not already initialized.
    pub fn new(kind: Type) -> Result<Self, Error> {
        let init = initialize_as(kind)?;
        Ok(Self {
            uninit_on_drop: init == Initialization::Success,
            _no_send: PhantomData,
        })
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.uninit_on_drop {
            uninitialize();
        }
    }
}

/// Identifies a COM method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodInfo<'a> {
    /// Optional COM interface name (e.g. `IShellLinkW`).
    pub interface: Option<&'a str>,
    /// COM method name (e.g. `GetPath`).
    pub method: &'a str,
}

impl fmt::Display for MethodInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.interface {
            Some(interface) => write!(f, "{}::{}", interface, self.method),
            None => write!(f, "{}", self.method),
        }
    }
}

/// Windows / COM error annotated with the COM method that failed.
///
/// This type exists to add semantic context to `windows::core::Error`
/// without losing the original error as the source.
#[derive(Debug, thiserror::Error)]
#[error("COM error from {method}: {source}")]
pub struct Error {
    /// The COM method that produced the error.
    pub method: MethodInfo<'static>,

    /// The underlying Windows error.
    #[source]
    pub source: windows::core::Error,
}

pub(crate) trait ComErrorExt {
    fn context(self, interface: Option<&'static str>, method: &'static str) -> Error;
}

impl ComErrorExt for windows::core::Error {
    fn context(self, interface: Option<&'static str>, method: &'static str) -> Error {
        Error {
            method: MethodInfo { interface, method },
            source: self,
        }
    }
}

pub(crate) trait ComResultExt<T> {
    fn context(self, interface: Option<&'static str>, method: &'static str) -> crate::Result<T>;
}

impl<T> ComResultExt<T> for windows::core::Result<T> {
    fn context(self, interface: Option<&'static str>, method: &'static str) -> crate::Result<T> {
        self.map_err(|source| crate::Error::Com(source.context(interface, method)))
    }
}

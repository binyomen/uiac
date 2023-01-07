use std::{fmt, string::FromUtf16Error};

#[cfg(not(target_os = "windows"))]
compile_error!("uiac is only supported on Windows.");

mod dump;
mod wrappers;

pub use dump::dump;

#[derive(Debug)]
pub enum UiacError {
    Message(String),
    InvalidVariantType,
    Windows(windows::core::Error),
    FromUtf16(FromUtf16Error),
}

impl fmt::Display for UiacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiacError::Message(s) => write!(f, "{s}"),
            UiacError::InvalidVariantType => write!(f, "invalid variant type"),
            UiacError::Windows(err) => write!(f, "{err}"),
            UiacError::FromUtf16(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for UiacError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UiacError::Message(_) => None,
            UiacError::InvalidVariantType => None,
            UiacError::Windows(err) => Some(err),
            UiacError::FromUtf16(err) => Some(err),
        }
    }
}

macro_rules! derive_from {
    ($from:ty, $variant:ident) => {
        impl From<$from> for UiacError {
            fn from(err: $from) -> Self {
                UiacError::$variant(err)
            }
        }
    };
}

derive_from!(windows::core::Error, Windows);
derive_from!(FromUtf16Error, FromUtf16);

pub type UiacResult<T> = Result<T, UiacError>;

pub fn opt_result<T>(result: windows::core::Result<T>) -> windows::core::Result<Option<T>> {
    match result {
        Ok(t) => Ok(Some(t)),
        Err(err) => {
            if err == windows::core::Error::OK {
                Ok(None)
            } else {
                Err(err)
            }
        }
    }
}

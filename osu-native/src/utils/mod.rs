use std::{
    ffi::{CString, FromVecWithNulError, IntoStringError},
    ptr,
};

use libosu_native_sys::ErrorCode;
use thiserror::Error as ThisError;

use crate::error::NativeError;

/// Convenience alias for the native type of `T`.
pub type NativeType<T> = <T as HasNative>::Native;

pub trait HasNative {
    type Native;
}

#[derive(Debug, ThisError)]
pub enum StringError {
    #[error("Received invalid length {0}")]
    InvalidLength(i32),
    #[error("Received invalid string")]
    InvalidNul(#[from] FromVecWithNulError),
    #[error("Received invalid utf8 string: {0:?}")]
    InvalidUtf8(CString),
    #[error("Native error")]
    Native(#[from] NativeError),
}

impl From<ErrorCode> for StringError {
    fn from(code: ErrorCode) -> Self {
        Self::Native(code.into())
    }
}

pub(crate) fn read_native_string(
    handle: i32,
    func: unsafe extern "C" fn(i32, *mut u8, *mut i32) -> ErrorCode,
) -> Result<String, StringError> {
    let mut size = 0i32;

    let code = unsafe { func(handle, ptr::null_mut(), &mut size) };

    if code != ErrorCode::BufferSizeQuery {
        return Err(code.into());
    }

    let len = size
        .try_into()
        .map_err(|_| StringError::InvalidLength(size))?;

    let mut buffer = vec![0u8, len];

    let code = unsafe { func(handle, buffer.as_mut_ptr(), &mut size) };

    if code != ErrorCode::Success {
        return Err(code.into());
    }

    CString::from_vec_with_nul(buffer)?
        .into_string()
        .map_err(IntoStringError::into_cstring)
        .map_err(StringError::InvalidUtf8)
}

#[cfg(test)]
pub fn initialize_path() -> std::path::PathBuf {
    let manifest_path = std::env!("CARGO_MANIFEST_DIR");
    let mut path = std::path::PathBuf::from(manifest_path);
    path.push("standard.osu");

    path
}

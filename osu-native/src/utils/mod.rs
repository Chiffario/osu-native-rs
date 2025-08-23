use std::{
    ffi::{CString, FromVecWithNulError, IntoStringError},
    ptr,
};

use libosu_native_sys::ErrorCode;
use thiserror::Error as ThisError;

use crate::error::NativeError;

#[macro_use]
pub mod macros;

#[derive(Debug, ThisError)]
pub enum StringError {
    #[error("Received invalid length {0}")]
    InvalidLength(i32),
    #[error("Received invalid string")]
    InvalidNul(#[from] FromVecWithNulError),
    #[error("Received invalid utf8 string")]
    InvalidUtf8(#[from] IntoStringError),
    #[error("Native error")]
    Native(#[from] NativeError),
}

impl From<ErrorCode> for StringError {
    fn from(code: ErrorCode) -> Self {
        Self::Native(code.into())
    }
}

type StringFn<H> = unsafe extern "C" fn(H, *mut u8, *mut i32) -> ErrorCode;

pub(crate) fn read_native_string<H>(handle: H, func: StringFn<H>) -> Result<String, StringError>
where
    H: Copy,
{
    fn new_buffer(code: ErrorCode, size: i32) -> Result<Vec<u8>, StringError> {
        if code != ErrorCode::BufferSizeQuery {
            return Err(code.into());
        }

        let len = size
            .try_into()
            .map_err(|_| StringError::InvalidLength(size))?;

        Ok(vec![0u8; len])
    }

    fn into_result(code: ErrorCode, buffer: Vec<u8>) -> Result<String, StringError> {
        if code != ErrorCode::Success {
            return Err(code.into());
        }

        CString::from_vec_with_nul(buffer)?
            .into_string()
            .map_err(Into::into)
    }

    let mut size = 0i32;

    let code = unsafe { func(handle, ptr::null_mut(), &mut size) };

    let mut buffer = new_buffer(code, size)?;

    let code = unsafe { func(handle, buffer.as_mut_ptr(), &mut size) };

    into_result(code, buffer)
}

#[cfg(test)]
pub fn initialize_path() -> std::path::PathBuf {
    let manifest_path = std::env!("CARGO_MANIFEST_DIR");
    let mut path = std::path::PathBuf::from(manifest_path);
    path.push("standard.osu");

    path
}

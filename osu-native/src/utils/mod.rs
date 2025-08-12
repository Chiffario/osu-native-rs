use std::{ffi::CString, path::PathBuf, ptr::null_mut};

use libosu_native_sys::ErrorCode;

use crate::error::{OsuError, error_code_to_osu};

pub(crate) fn read_native_string(
    handle: i32,
    func: unsafe extern "C" fn(i32, *mut u8, *mut i32) -> ErrorCode,
) -> Result<String, OsuError> {
    let mut size = 0i32;

    unsafe {
        match func(handle, null_mut(), &raw mut size) {
            ErrorCode::BufferSizeQuery => {}
            e => return Err(error_code_to_osu(e).into()),
        }
    }
    println!("{size}");

    let mut buffer = Vec::with_capacity(size.try_into().unwrap());
    unsafe {
        match func(handle, buffer.as_mut_ptr(), &raw mut size) {
            ErrorCode::Success => {
                buffer.set_len(size as usize);
                println!("{buffer:?}");
                let string = CString::from_vec_with_nul(buffer).unwrap();
                let string = string.into_string().unwrap();
                return Ok(string);
            }
            e => return Err(error_code_to_osu(e).into()),
        }
    }
}

pub fn initialize_path() -> PathBuf {
    let manifest_path = std::env!("CARGO_MANIFEST_DIR");
    let mut path = PathBuf::from(manifest_path);
    path.push("standard.osu");
    path
}

use std::fmt::Debug;

use libosu_native_sys::ErrorCode;

/// A wrapper around a [`Native`] type.
pub trait NativeWrapper {
    // `Debug` bound necessary for error message in case `Drop` fails
    type Native: Native<Handle: Debug>;
}

pub(crate) type DestroyFn<H> = unsafe extern "C" fn(H) -> ErrorCode;

/// A native type coming from C#.
pub trait Native {
    type Handle;

    const DESTROY: DestroyFn<Self::Handle>;

    fn handle(&self) -> Self::Handle;
}

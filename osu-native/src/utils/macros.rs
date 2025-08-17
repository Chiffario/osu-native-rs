/// Implements [`NativeWrapper`], [`Deref`], [`From<Native>`] and [`Drop`] for
/// native wrappers.
///
/// [`NativeWrapper`]: crate::traits::NativeWrapper
/// [`Deref`]: std::ops::Deref
/// [`From<Native>`]: std::convert::From
macro_rules! declare_native_wrapper {
    // Entrypoint for simple tuple wrappers
    (
        #[$meta:meta]
        $vis:vis struct $name:ident($native:ident);
    ) => {
        #[$meta]
        #[doc = declare_native_wrapper!(@DOC $native)]
        $vis struct $name($native);

        impl std::ops::Deref for $name {
            type Target = $native;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<($native, crate::ruleset::Ruleset)> for $name {
            fn from((native, _): ($native, crate::ruleset::Ruleset)) -> Self {
                Self(native)
            }
        }

        declare_native_wrapper!(@NATIVE $name: $native);
        declare_native_wrapper!(@DROP $name);
    };
    // Entrypoint for named field structs
    (
        #[$meta:meta]
        $vis:vis struct $name:ident {
            $( $field_vis:vis $field_name:ident: $field_type:ty, )*
        }
    ) => {
        declare_native_wrapper! {
            #[$meta]
            $vis struct $name
                [declare_native_wrapper!(@FIND_NATIVE $( $field_name $field_type, )*)]
            {
                $( $field_vis $field_name: $field_type, )*
            }
        }
    };
    // Actual handling of named field structs and their native type
    (
        #[$meta:meta]
        $vis:vis struct $name:ident [$native:ty] {
            $( $field_vis:vis $field_name:ident: $field_type:ty, )*
        }
    ) => {
        #[$meta]
        #[doc = declare_native_wrapper!(@DOC $native)]
        $vis struct $name {
            $( $field_vis $field_name: $field_type, )*
        }

        impl std::ops::Deref for $name {
            type Target = $native;

            fn deref(&self) -> &Self::Target {
                &self.native
            }
        }

        declare_native_wrapper!(@NATIVE $name: $native);
        declare_native_wrapper!( @DROP $name );
    };
    // Found field called `native` so we return its type
    ( @FIND_NATIVE native $field_type:ty, $( $rest:tt )* ) => {
        $field_type
    };
    // Current field is not called `native` so keep looking
    ( @FIND_NATIVE $field_name:ident $field_type:ty, $( $rest:tt )* ) => {
        declare_native_wrapper!(@FIND_NATIVE $( $rest )*)
    };
    // No field called `native` found
    ( @FIND_NATIVE ) => {
        compile_error!("must contain field called `native`");
    };
    // Documentation for wrapper type
    ( @DOC $native:ty ) => {
        concat!("Wrapper around [`", stringify!($native), "`].")
    };
    // `NativeWrapper` trait implementation
    ( @NATIVE $name:ident: $native:ty ) => {
        impl crate::traits::NativeWrapper for $name {
            type Native = $native;
        }
    };
    // `Drop` trait implementation
    ( @DROP $name:ident ) => {
        impl Drop for $name {
            fn drop(&mut self) {
                let code = unsafe {
                    <<Self as crate::traits::NativeWrapper>::Native as Native>::DESTROY(
                        self.handle(),
                    )
                };

                if code != ::libosu_native_sys::ErrorCode::Success {
                    eprintln!("Failed to destroy object {:?}: {code:?}", self.handle());
                }
            }
        }
    }
}

/// Implements [`Native`] for native types.
///
/// [`Native`]: crate::traits::Native
macro_rules! impl_native {
    ( $name:ident: $handle:ident, $destroy:ident ) => {
        impl crate::traits::Native for $name {
            type Handle = $handle;

            const DESTROY: crate::traits::DestroyFn<Self::Handle> = $destroy;

            fn handle(&self) -> Self::Handle {
                self.handle
            }
        }
    };
}

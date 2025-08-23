/// Implements [`NativeWrapper`], [`Deref`], [`From<Native>`] (only for simple
/// tuple wrappers) and [`Drop`] for native wrappers.
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

        impl From<$native> for $name {
            fn from(native: $native) -> Self {
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

/// Implements [`DifficultyCalculator`].
///
/// [`DifficultyCalculator`]: crate::calculator::DifficultyCalculator
macro_rules! impl_calculator {
    (
        $name:ident {
            attributes: $attrs:ty,
            handle: $handle:ident,
            create: $create:ident,
            calculate: $calc:ident,
        }
    ) => {
        impl crate::calculator::DifficultyCalculator for $name {
            type Attributes = $attrs;

            fn create(
                ruleset: crate::ruleset::Ruleset,
                beatmap: &crate::beatmap::Beatmap,
            ) -> Result<Self, crate::error::NativeError> {
                let mut calculator = MaybeUninit::uninit();

                let code =
                    unsafe { $create(ruleset.handle(), beatmap.handle(), calculator.as_mut_ptr()) };

                if code != ErrorCode::Success {
                    return Err(code.into());
                }

                let native = unsafe { calculator.assume_init() };

                Ok(Self {
                    native,
                    ruleset,
                    mods: crate::mods::GameMods::default(),
                })
            }

            fn mods(
                mut self,
                mods: impl crate::mods::IntoGameMods,
            ) -> Result<Self, crate::mods::GameModsError> {
                self.mods = crate::mods::IntoGameMods::into_mods(mods)?;

                Ok(self)
            }

            fn calculate(&self) -> Result<Self::Attributes, crate::error::OsuError> {
                let mods = crate::mods::native::ModCollection::new()?;

                let mods_vec = self
                    .mods
                    .0
                    .iter()
                    .map(|gamemod| {
                        let m = crate::mods::native::Mod::new(gamemod.acronym.as_str())?;
                        m.apply_settings(&gamemod.settings)?;

                        Ok(m)
                    })
                    .collect::<Result<Vec<_>, crate::error::OsuError>>()?;

                for gamemod in mods_vec.iter() {
                    mods.add(gamemod)?;
                }

                let mut attributes = std::mem::MaybeUninit::uninit();

                let code = unsafe {
                    $calc(
                        self.handle(),
                        self.ruleset.handle(),
                        mods.handle(),
                        attributes.as_mut_ptr(),
                    )
                };

                if code != ::libosu_native_sys::ErrorCode::Success {
                    return Err(code.into());
                }

                let native = unsafe { attributes.assume_init() };

                Ok(native.into())
            }
        }
    };
}

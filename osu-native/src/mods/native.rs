use std::{
    collections::HashMap,
    ffi::{CString, NulError},
    mem::MaybeUninit,
};

use libosu_native_sys::{
    ErrorCode, Mod_Create, Mod_Destroy, Mod_SetSetting, ModsCollection_Add, ModsCollection_Create,
    ModsCollection_Destroy, NativeModCollectionHandle, NativeModHandle,
};
use rosu_mods::simple::SettingSimple;
use thiserror::Error as ThisError;

use crate::{
    error::NativeError,
    mods::{GameModsError, IntoGameMods},
};

pub struct ModCollection {
    pub handle: NativeModCollectionHandle,
    mods: Vec<Mod>,
}

#[derive(Debug, ThisError)]
pub enum ModCollectionError {
    #[error("Native error")]
    Native(#[from] NativeError),
    #[error("Game mods error")]
    ModsError(#[from] GameModsError),
    #[error("Mod error")]
    Mod(#[from] ModError),
}

impl ModCollection {
    pub fn handle(&self) -> NativeModCollectionHandle {
        self.handle
    }

    /// Creates an instance of [`ModCollection`]
    ///
    /// # Example
    /// ```no_run
    /// # use osu_native::mods::native::ModCollection;
    /// let mods = ModCollection::new()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    /// Returns [`NativeError`] if osu-native returns an error
    pub fn new() -> Result<Self, NativeError> {
        let mut collection = MaybeUninit::uninit();

        let code = unsafe { ModsCollection_Create(collection.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let handle = unsafe { collection.assume_init() };

        Ok(Self {
            handle,
            mods: vec![],
        })
    }

    pub fn with_game_mods(
        mut self,
        gamemods: impl IntoGameMods,
    ) -> Result<Self, ModCollectionError> {
        let mods = gamemods
            .into_mods()?
            .0
            .iter()
            .map(|gamemod| {
                let m = Mod::new(gamemod.acronym.as_str())?;
                m.apply_settings(&gamemod.settings)?;

                Ok(m)
            })
            .collect::<Result<Vec<_>, ModCollectionError>>()?;

        for gamemod in mods.into_iter() {
            self.add(&gamemod)?;
            self.mods.push(gamemod);
        }
        Ok(self)
    }

    pub fn add(&self, gamemod: &Mod) -> Result<(), NativeError> {
        let code = unsafe { ModsCollection_Add(self.handle, gamemod.handle()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        Ok(())
    }
}

impl Drop for ModCollection {
    fn drop(&mut self) {
        unsafe { ModsCollection_Destroy(self.handle) };
    }
}

pub struct Mod {
    handle: NativeModHandle,
}

#[derive(Debug, ThisError)]
pub enum ModError {
    #[error("Native error")]
    Native(#[from] NativeError),
    #[error("Acronym error")]
    Acronym(#[from] NulError),
}

impl From<ErrorCode> for ModError {
    fn from(code: ErrorCode) -> Self {
        Self::Native(code.into())
    }
}

impl Mod {
    pub fn handle(&self) -> NativeModHandle {
        self.handle
    }

    pub fn new(acronym: &str) -> Result<Self, ModError> {
        let acronym = CString::new(acronym)?;
        let acronym_ptr = acronym.as_ptr();

        let mut gamemod = MaybeUninit::uninit();

        let code = unsafe { Mod_Create(acronym_ptr, gamemod.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let handle = unsafe { gamemod.assume_init() };

        Ok(Self { handle })
    }

    #[expect(unused)]
    pub fn apply_settings(
        &self,
        settings: &HashMap<Box<str>, SettingSimple>,
    ) -> Result<(), ModError> {
        let handle = self.handle();

        for (key, value) in settings {
            let key = CString::new(key.as_ref())?;
            let key_ptr = key.as_ptr();

            let code = unsafe {
                match value {
                    SettingSimple::Bool(value) => continue, // TODO
                    SettingSimple::Number(value) => Mod_SetSetting(handle, key_ptr, *value),
                    SettingSimple::String(value) => continue, // TODO
                }
            };

            if code != ErrorCode::Success {
                return Err(code.into());
            }
        }

        Ok(())
    }
}

impl Drop for Mod {
    fn drop(&mut self) {
        unsafe { Mod_Destroy(self.handle) };
    }
}

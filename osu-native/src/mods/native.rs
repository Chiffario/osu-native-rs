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

use crate::error::NativeError;

pub(crate) struct ModCollection {
    pub handle: NativeModCollectionHandle,
}

impl ModCollection {
    pub fn handle(&self) -> NativeModCollectionHandle {
        self.handle
    }

    pub fn new() -> Result<Self, NativeError> {
        let mut collection = MaybeUninit::uninit();

        let code = unsafe { ModsCollection_Create(collection.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let handle = unsafe { collection.assume_init() };

        Ok(Self { handle })
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

pub(crate) struct Mod {
    pub handle: NativeModHandle,
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

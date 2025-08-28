use std::{ffi::CString, mem::MaybeUninit};

use libosu_native_sys::{
    ErrorCode, NativeRuleset, Ruleset_CreateFromId, Ruleset_CreateFromShortName,
    Ruleset_GetShortName,
};
use thiserror::Error as ThisError;

use crate::{
    error::NativeError,
    utils::{HasNative, NativeType, StringError, read_native_string},
};

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum RulesetKind {
    #[default]
    Osu = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

impl From<RulesetKind> for i32 {
    fn from(kind: RulesetKind) -> Self {
        match kind {
            RulesetKind::Osu => 0,
            RulesetKind::Taiko => 1,
            RulesetKind::Catch => 2,
            RulesetKind::Mania => 3,
        }
    }
}

#[derive(Debug, ThisError)]
#[error("Invalid ruleset ID {0}")]
pub struct InvalidRulesetId(i32);

impl TryFrom<i32> for RulesetKind {
    type Error = InvalidRulesetId;

    fn try_from(id: i32) -> Result<Self, Self::Error> {
        match id {
            0 => Ok(RulesetKind::Osu),
            1 => Ok(RulesetKind::Taiko),
            2 => Ok(RulesetKind::Catch),
            3 => Ok(RulesetKind::Mania),
            _ => Err(InvalidRulesetId(id)),
        }
    }
}

#[derive(Debug, ThisError)]
pub enum RulesetError {
    #[error(transparent)]
    InvalidRuleset(#[from] InvalidRulesetId),
    #[error("Invalid string passed as ruleset name")]
    StringError,
    #[error("Native error")]
    Native(#[from] NativeError),
}

impl From<ErrorCode> for RulesetError {
    fn from(code: ErrorCode) -> Self {
        Self::Native(code.into())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ruleset {
    handle: i32,
    pub kind: RulesetKind,
}

impl Ruleset {
    pub fn handle(&self) -> i32 {
        self.handle
    }
}

impl Ruleset {
    pub fn new(kind: RulesetKind) -> Result<Self, RulesetError> {
        let mut ruleset: MaybeUninit<NativeType<Self>> = MaybeUninit::uninit();

        let code = unsafe { Ruleset_CreateFromId(kind.into(), ruleset.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { ruleset.assume_init() };

        Ok(Self {
            kind: native.id.try_into()?,
            handle: native.handle,
        })
    }

    pub fn from_short_name(name: String) -> Result<Self, RulesetError> {
        let mut ruleset: MaybeUninit<NativeType<Self>> = MaybeUninit::uninit();

        let Ok(name_cstr) = CString::new(name) else {
            return Err(RulesetError::StringError);
        };

        let code = unsafe { Ruleset_CreateFromShortName(name_cstr.as_ptr(), ruleset.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { ruleset.assume_init() };
        Ok(Self {
            kind: native.id.try_into()?,
            handle: native.handle,
        })
    }

    pub fn short_name(&self) -> Result<String, StringError> {
        read_native_string(self.handle, Ruleset_GetShortName)
    }
}

impl HasNative for Ruleset {
    type Native = NativeRuleset;
}

#[cfg(test)]
mod tests {
    use crate::generate_ruleset_tests;

    use super::{Ruleset, RulesetKind};

    generate_ruleset_tests!(Osu, "osu");
    generate_ruleset_tests!(Taiko, "taiko");
    generate_ruleset_tests!(Mania, "mania");
    generate_ruleset_tests!(Catch, "fruits");
}

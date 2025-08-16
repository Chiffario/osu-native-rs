use std::mem::MaybeUninit;

use libosu_native_sys::{ErrorCode, NativeRuleset, Ruleset_CreateFromId, Ruleset_GetShortName};
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
    #[error("Native error")]
    Native(#[from] NativeError),
}

impl From<ErrorCode> for RulesetError {
    fn from(code: ErrorCode) -> Self {
        Self::Native(code.into())
    }
}

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
        let mut native: MaybeUninit<NativeType<Self>> = MaybeUninit::uninit();

        let code = unsafe { Ruleset_CreateFromId(kind.into(), native.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { native.assume_init() };

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
    use super::{Ruleset, RulesetKind};

    #[test]
    fn test_create_osu() {
        let osu = Ruleset::new(RulesetKind::Osu).unwrap();
        assert_eq!(osu.kind, RulesetKind::Osu);
    }

    #[test]
    fn test_create_taiko() {
        let taiko = Ruleset::new(RulesetKind::Taiko).unwrap();
        assert_eq!(taiko.kind, RulesetKind::Taiko);
    }

    #[test]
    fn test_create_catch() {
        let catch = Ruleset::new(RulesetKind::Catch).unwrap();
        assert_eq!(catch.kind, RulesetKind::Catch);
    }

    #[test]
    fn test_create_mania() {
        let mania = Ruleset::new(RulesetKind::Mania).unwrap();
        assert_eq!(mania.kind, RulesetKind::Mania);
    }

    #[test]
    fn test_get_name_osu() {
        let osu = Ruleset::new(RulesetKind::Osu).unwrap();
        assert_eq!(osu.short_name().unwrap(), "osu");
    }

    #[test]
    fn test_get_name_taiko() {
        let taiko = Ruleset::new(RulesetKind::Taiko).unwrap();
        assert_eq!(taiko.short_name().unwrap(), "taiko");
    }

    #[test]
    fn test_get_name_catch() {
        let catch = Ruleset::new(RulesetKind::Catch).unwrap();
        assert_eq!(catch.short_name().unwrap(), "fruits");
    }

    #[test]
    fn test_get_name_mania() {
        let mania = Ruleset::new(RulesetKind::Mania).unwrap();
        assert_eq!(mania.short_name().unwrap(), "mania");
    }
}

use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, NativeRuleset, NativeRulesetHandle, Ruleset_CreateFromId, Ruleset_Destroy,
    Ruleset_GetShortName,
};
use thiserror::Error as ThisError;

use crate::{
    error::NativeError,
    traits::Native,
    utils::{StringError, read_native_string},
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

declare_native_wrapper! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Ruleset(NativeRuleset);
}

impl_native!(NativeRuleset: NativeRulesetHandle, Ruleset_Destroy);

impl Ruleset {
    pub fn from_kind(kind: RulesetKind) -> Result<Self, NativeError> {
        let mut native = MaybeUninit::uninit();
        let code = unsafe { Ruleset_CreateFromId(kind.into(), native.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { native.assume_init() };

        Ok(Self(native))
    }

    pub fn short_name(&self) -> Result<String, StringError> {
        read_native_string(self.handle(), Ruleset_GetShortName)
    }

    pub fn kind(&self) -> Result<RulesetKind, InvalidRulesetId> {
        self.id.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::{Ruleset, RulesetKind};

    #[test]
    fn test_create_osu() {
        let osu = Ruleset::from_kind(RulesetKind::Osu).unwrap();
        assert_eq!(osu.kind().unwrap(), RulesetKind::Osu);
    }

    #[test]
    fn test_create_taiko() {
        let taiko = Ruleset::from_kind(RulesetKind::Taiko).unwrap();
        assert_eq!(taiko.kind().unwrap(), RulesetKind::Taiko);
    }

    #[test]
    fn test_create_catch() {
        let catch = Ruleset::from_kind(RulesetKind::Catch).unwrap();
        assert_eq!(catch.kind().unwrap(), RulesetKind::Catch);
    }

    #[test]
    fn test_create_mania() {
        let mania = Ruleset::from_kind(RulesetKind::Mania).unwrap();
        assert_eq!(mania.kind().unwrap(), RulesetKind::Mania);
    }

    #[test]
    fn test_get_name_osu() {
        let osu = Ruleset::from_kind(RulesetKind::Osu).unwrap();
        assert_eq!(osu.short_name().unwrap(), "osu");
    }

    #[test]
    fn test_get_name_taiko() {
        let taiko = Ruleset::from_kind(RulesetKind::Taiko).unwrap();
        assert_eq!(taiko.short_name().unwrap(), "taiko");
    }

    #[test]
    fn test_get_name_catch() {
        let catch = Ruleset::from_kind(RulesetKind::Catch).unwrap();
        assert_eq!(catch.short_name().unwrap(), "fruits");
    }

    #[test]
    fn test_get_name_mania() {
        let mania = Ruleset::from_kind(RulesetKind::Mania).unwrap();
        assert_eq!(mania.short_name().unwrap(), "mania");
    }
}

use std::mem::MaybeUninit;

use libosu_native_sys::{ErrorCode, NativeRuleset, Ruleset_CreateFromId, Ruleset_GetShortName};
use thiserror::Error as ThisError;

use crate::{
    error::{NativeError, OsuError},
    utils::read_native_string,
};

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum Rulesets {
    Standard = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

impl From<Rulesets> for i32 {
    fn from(value: Rulesets) -> Self {
        match value {
            Rulesets::Standard => 0,
            Rulesets::Taiko => 1,
            Rulesets::Catch => 2,
            Rulesets::Mania => 3,
        }
    }
}

impl TryFrom<i32> for Rulesets {
    type Error = RulesetError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rulesets::Standard),
            1 => Ok(Rulesets::Taiko),
            2 => Ok(Rulesets::Catch),
            3 => Ok(Rulesets::Mania),
            _ => Err(RulesetError::InvalidRuleset(value)),
        }
    }
}

#[derive(Debug, ThisError)]
pub enum RulesetError {
    #[error("Invalid ruleset ID {0}")]
    InvalidRuleset(i32),
    #[error("Native error")]
    GenericError(#[from] OsuError),
}

impl From<NativeError> for RulesetError {
    fn from(value: NativeError) -> Self {
        Self::GenericError(value.into())
    }
}

pub struct Ruleset {
    handle: i32,
    ruleset: Rulesets,
}

impl Ruleset {
    pub fn get_handle(&self) -> i32 {
        self.handle
    }
}

impl Ruleset {
    pub fn new_from_variant(variant: Rulesets) -> Result<Self, RulesetError> {
        let mut ruleset: MaybeUninit<NativeRuleset> = MaybeUninit::uninit();
        let ruleset = unsafe {
            match Ruleset_CreateFromId(variant.into(), ruleset.as_mut_ptr()) {
                ErrorCode::Success => Ok(ruleset.assume_init()),
                e => Err(RulesetError::GenericError(e.into())),
            }
        };

        ruleset.map(|r| Ruleset {
            handle: r.handle,
            ruleset: r.id.try_into().unwrap(),
        })
    }

    pub fn get_short_name(&self) -> Result<String, RulesetError> {
        read_native_string(self.handle, Ruleset_GetShortName).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::{Ruleset, Rulesets};

    #[test]
    fn test_create_standard() {
        let osu = Ruleset::new_from_variant(Rulesets::Standard).unwrap();
        assert_eq!(osu.ruleset, Rulesets::Standard);
    }

    #[test]
    fn test_create_taiko() {
        let taiko = Ruleset::new_from_variant(Rulesets::Taiko).unwrap();
        assert_eq!(taiko.ruleset, Rulesets::Taiko);
    }

    #[test]
    fn test_create_catch() {
        let catch = Ruleset::new_from_variant(Rulesets::Catch).unwrap();
        assert_eq!(catch.ruleset, Rulesets::Catch);
    }

    #[test]
    fn test_create_mania() {
        let mania = Ruleset::new_from_variant(Rulesets::Mania).unwrap();
        assert_eq!(mania.ruleset, Rulesets::Mania);
    }

    #[test]
    fn test_get_name_standard() {
        let osu = Ruleset::new_from_variant(Rulesets::Standard).unwrap();
        let _ = osu.get_short_name().unwrap();
    }

    #[test]
    fn test_get_name_taiko() {
        let taiko = Ruleset::new_from_variant(Rulesets::Taiko).unwrap();

        assert_eq!(
            taiko.get_short_name().unwrap(),
            String::from("taiko"),
            "Displayed {:?}",
            taiko.get_short_name()
        );
    }

    #[test]
    fn test_get_name_catch() {
        let catch = Ruleset::new_from_variant(Rulesets::Catch).unwrap();

        assert_eq!(
            catch.get_short_name().unwrap(),
            String::from("fruits"),
            "Displayed {:?}",
            catch.get_short_name()
        );
    }

    #[test]
    fn test_get_name_mania() {
        let mania = Ruleset::new_from_variant(Rulesets::Mania).unwrap();

        assert_eq!(
            mania.get_short_name().unwrap(),
            String::from("mania"),
            "Displayed {:?}",
            mania.get_short_name()
        );
    }
}

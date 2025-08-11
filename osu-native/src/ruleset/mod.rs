use std::{error::Error, ffi::CString, fmt::Display, mem::MaybeUninit, ptr::null_mut};

use libosu_native_sys::{ErrorCode, NativeRuleset, Ruleset_CreateFromId, Ruleset_GetShortName};

use crate::error::{NativeError, error_code_to_native};

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
            _ => Err(Self::Error::InvalidRuleset),
        }
    }
}

#[derive(Debug)]
pub enum RulesetError {
    InvalidRuleset,
    NativeError(NativeError),
}
impl Display for RulesetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RulesetError::InvalidRuleset => writeln!(f, "Invalid ruleset ID"),
            RulesetError::NativeError(native_error) => writeln!(f, "Native error: {native_error}"),
        }
    }
}

impl Error for RulesetError {}

impl From<NativeError> for RulesetError {
    fn from(value: NativeError) -> Self {
        Self::NativeError(value)
    }
}
struct Ruleset {
    handle: i32,
    ruleset: Rulesets,
}

impl Ruleset {
    pub fn new_from_variant(variant: Rulesets) -> Result<Self, RulesetError> {
        let mut ruleset: MaybeUninit<NativeRuleset> = MaybeUninit::uninit();
        let ruleset = unsafe {
            match Ruleset_CreateFromId(variant.into(), ruleset.as_mut_ptr()) {
                ErrorCode::Success => Ok(ruleset.assume_init()),
                e => Err(error_code_to_native(e).into()),
            }
        };
        ruleset.map(|r| Ruleset {
            handle: r.handle,
            ruleset: r.id.try_into().unwrap(),
        })
    }
    pub fn get_short_name(&self) -> Result<String, RulesetError> {
        let mut size = 0i32;

        unsafe {
            match Ruleset_GetShortName(self.handle, null_mut(), &raw mut size) {
                ErrorCode::BufferSizeQuery => {}
                e => return Err(error_code_to_native(e).into()),
            }
        }
        println!("{size}");

        let mut buffer = Vec::with_capacity(size.try_into().unwrap());
        unsafe {
            match Ruleset_GetShortName(self.handle, buffer.as_mut_ptr(), &raw mut size) {
                ErrorCode::Success => {
                    buffer.set_len(size as usize);
                    println!("{buffer:?}");
                    let string = CString::from_vec_with_nul(buffer).unwrap();
                    let string = string.into_string().unwrap();
                    return Ok(string);
                }
                e => return Err(error_code_to_native(e).into()),
            }
        }
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

use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, ManiaDifficultyCalculator_Calculate, ManiaDifficultyCalculator_Create,
    ManiaDifficultyCalculator_Destroy, NativeManiaDifficultyAttributes,
};

use crate::{
    beatmap::Beatmap,
    error::OsuError,
    ruleset::Ruleset,
    utils::{HasNative, NativeType},
};

use super::DifficultyCalculator;

#[derive(PartialEq, Eq)]
pub struct ManiaDifficultyCalculator {
    handle: i32,
}

impl Drop for ManiaDifficultyCalculator {
    fn drop(&mut self) {
        unsafe { ManiaDifficultyCalculator_Destroy(self.handle) };
    }
}

impl DifficultyCalculator for ManiaDifficultyCalculator {
    type Attributes = ManiaDifficultyAttributes;

    fn new(ruleset: Ruleset, beatmap: Beatmap) -> Result<Self, OsuError> {
        let mut handle = 0;

        let code = unsafe {
            ManiaDifficultyCalculator_Create(ruleset.handle(), beatmap.handle(), &mut handle)
        };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        Ok(Self { handle })
    }

    fn calculate(&self) -> Result<Self::Attributes, OsuError> {
        let mut attributes: MaybeUninit<NativeType<Self::Attributes>> = MaybeUninit::uninit();

        let code =
            unsafe { ManiaDifficultyCalculator_Calculate(self.handle, attributes.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { attributes.assume_init() };

        Ok(native.into())
    }
}

impl From<NativeManiaDifficultyAttributes> for ManiaDifficultyAttributes {
    fn from(value: NativeManiaDifficultyAttributes) -> Self {
        Self {
            star_rating: value.star_rating,
            max_combo: value.max_combo as usize,
        }
    }
}

pub struct ManiaDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: usize,
}

impl HasNative for ManiaDifficultyAttributes {
    type Native = NativeManiaDifficultyAttributes;
}

#[cfg(test)]
mod tests {
    use crate::{
        beatmap::Beatmap,
        calculator::{DifficultyCalculator, mania::ManiaDifficultyCalculator},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_convert_mania() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Mania).unwrap();
        let calculator = ManiaDifficultyCalculator::new(ruleset, beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_ne!(attributes.star_rating, 0.0);
        assert_eq!(attributes.max_combo, 1463);
    }
}

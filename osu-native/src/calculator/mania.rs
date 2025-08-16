use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, ManiaDifficultyCalculator_Calculate, ManiaDifficultyCalculator_Create,
    ManiaDifficultyCalculator_Destroy, NativeManiaDifficultyAttributes,
};

use crate::error::OsuError;

use super::DifficultyCalculator;

struct ManiaDifficultyCalculator {
    handle: i32,
}

impl Drop for ManiaDifficultyCalculator {
    fn drop(&mut self) {
        unsafe { ManiaDifficultyCalculator_Destroy(self.handle) };
    }
}

impl DifficultyCalculator for ManiaDifficultyCalculator {
    type Attributes = ManiaDifficultyAttributes;
    type NativeAttributes = NativeManiaDifficultyAttributes;

    fn new(
        ruleset: crate::ruleset::Ruleset,
        beatmap: crate::beatmap::Beatmap,
    ) -> Result<Self, OsuError> {
        let mut handle = 0;

        unsafe {
            match ManiaDifficultyCalculator_Create(
                ruleset.get_handle(),
                beatmap.get_handle(),
                &raw mut handle,
            ) {
                ErrorCode::Success => Ok(Self { handle }),
                e => Err(e.into()),
            }
        }
    }

    fn calculate(&self) -> Result<Self::Attributes, OsuError> {
        let mut attributes: MaybeUninit<Self::NativeAttributes> = MaybeUninit::uninit();

        let attributes = unsafe {
            match ManiaDifficultyCalculator_Calculate(self.handle, attributes.as_mut_ptr()) {
                ErrorCode::Success => Ok(attributes.assume_init().into()),
                e => Err(e.into()),
            }
        };

        attributes
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

#[cfg(test)]
mod tests {
    use crate::{
        beatmap::Beatmap,
        calculator::{DifficultyCalculator, mania::ManiaDifficultyCalculator},
        ruleset::{Ruleset, Rulesets},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_convert_mania() {
        let beatmap = Beatmap::new_from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new_from_variant(Rulesets::Mania).unwrap();
        let calculator = ManiaDifficultyCalculator::new(ruleset, beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_eq!(attributes.star_rating, 3.8214955335276204);
        assert_eq!(attributes.max_combo, 1463);
    }
}

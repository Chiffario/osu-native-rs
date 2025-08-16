use std::mem::MaybeUninit;

use libosu_native_sys::{
    CatchDifficultyCalculator_Calculate, CatchDifficultyCalculator_Create,
    CatchDifficultyCalculator_Destroy, ErrorCode, NativeCatchDifficultyAttributes,
};

use crate::error::OsuError;

use super::DifficultyCalculator;

struct CatchDifficultyCalculator {
    handle: i32,
}

impl Drop for CatchDifficultyCalculator {
    fn drop(&mut self) {
        unsafe { CatchDifficultyCalculator_Destroy(self.handle) };
    }
}

impl DifficultyCalculator for CatchDifficultyCalculator {
    type Attributes = CatchDifficultyAttributes;
    type NativeAttributes = NativeCatchDifficultyAttributes;

    fn new(
        ruleset: crate::ruleset::Ruleset,
        beatmap: crate::beatmap::Beatmap,
    ) -> Result<Self, OsuError> {
        let mut handle = 0;

        unsafe {
            match CatchDifficultyCalculator_Create(
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
            match CatchDifficultyCalculator_Calculate(self.handle, attributes.as_mut_ptr()) {
                ErrorCode::Success => Ok(attributes.assume_init().into()),
                e => Err(e.into()),
            }
        };

        attributes
    }
}

impl From<NativeCatchDifficultyAttributes> for CatchDifficultyAttributes {
    fn from(value: NativeCatchDifficultyAttributes) -> Self {
        Self {
            star_rating: value.star_rating,
            max_combo: value.max_combo as usize,
        }
    }
}

pub struct CatchDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: usize,
}

#[cfg(test)]
mod tests {
    use crate::{
        beatmap::Beatmap,
        calculator::{DifficultyCalculator, catch::CatchDifficultyCalculator},
        ruleset::{Ruleset, Rulesets},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_convert_catch() {
        let beatmap = Beatmap::new_from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new_from_variant(Rulesets::Catch).unwrap();
        let calculator = CatchDifficultyCalculator::new(ruleset, beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_eq!(attributes.star_rating, 3.8496726424352428);
        assert_eq!(attributes.max_combo, 717);
    }
}

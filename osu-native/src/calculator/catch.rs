use std::mem::MaybeUninit;

use libosu_native_sys::{
    CatchDifficultyCalculator_Calculate, CatchDifficultyCalculator_Create,
    CatchDifficultyCalculator_Destroy, ErrorCode, NativeCatchDifficultyAttributes,
};

use crate::{
    beatmap::Beatmap,
    error::OsuError,
    ruleset::Ruleset,
    utils::{HasNative, NativeType},
};

use super::DifficultyCalculator;

#[derive(PartialEq, Eq)]
pub struct CatchDifficultyCalculator {
    handle: i32,
}

impl Drop for CatchDifficultyCalculator {
    fn drop(&mut self) {
        unsafe { CatchDifficultyCalculator_Destroy(self.handle) };
    }
}

impl DifficultyCalculator for CatchDifficultyCalculator {
    type Attributes = CatchDifficultyAttributes;

    fn new(ruleset: Ruleset, beatmap: Beatmap) -> Result<Self, OsuError> {
        let mut handle = 0;

        let code = unsafe {
            CatchDifficultyCalculator_Create(ruleset.handle(), beatmap.handle(), &mut handle)
        };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        Ok(Self { handle })
    }

    fn calculate(&self) -> Result<Self::Attributes, OsuError> {
        let mut attributes: MaybeUninit<NativeType<Self::Attributes>> = MaybeUninit::uninit();

        let code =
            unsafe { CatchDifficultyCalculator_Calculate(self.handle, attributes.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { attributes.assume_init() };

        Ok(native.into())
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

impl HasNative for CatchDifficultyAttributes {
    type Native = NativeCatchDifficultyAttributes;
}

#[cfg(test)]
mod tests {
    use crate::{
        beatmap::Beatmap,
        calculator::{DifficultyCalculator, catch::CatchDifficultyCalculator},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_convert_catch() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Catch).unwrap();
        let calculator = CatchDifficultyCalculator::new(ruleset, beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_eq!(attributes.star_rating, 3.8496726424352428);
        assert_eq!(attributes.max_combo, 717);
    }
}

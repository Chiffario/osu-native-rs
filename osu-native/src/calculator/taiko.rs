use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, NativeTaikoDifficultyAttributes, TaikoDifficultyCalculator_Calculate,
    TaikoDifficultyCalculator_Create, TaikoDifficultyCalculator_Destroy,
};

use crate::{
    beatmap::Beatmap,
    error::OsuError,
    ruleset::Ruleset,
    utils::{HasNative, NativeType},
};

use super::DifficultyCalculator;

#[derive(PartialEq, Eq)]
pub struct TaikoDifficultyCalculator {
    handle: i32,
}

impl Drop for TaikoDifficultyCalculator {
    fn drop(&mut self) {
        unsafe { TaikoDifficultyCalculator_Destroy(self.handle) };
    }
}

impl DifficultyCalculator for TaikoDifficultyCalculator {
    type Attributes = TaikoDifficultyAttributes;

    fn new(ruleset: Ruleset, beatmap: Beatmap) -> Result<Self, OsuError> {
        let mut handle = 0;

        let code = unsafe {
            TaikoDifficultyCalculator_Create(ruleset.handle(), beatmap.handle(), &mut handle)
        };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        Ok(Self { handle })
    }

    fn calculate(&self) -> Result<Self::Attributes, OsuError> {
        let mut attributes: MaybeUninit<NativeType<Self::Attributes>> = MaybeUninit::uninit();

        let code =
            unsafe { TaikoDifficultyCalculator_Calculate(self.handle, attributes.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { attributes.assume_init() };

        Ok(native.into())
    }
}

pub struct TaikoDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: usize,
    pub rhythm_difficulty: f64,
    pub reading_difficulty: f64,
    pub colour_difficulty: f64,
    pub stamina_difficulty: f64,
    pub mono_stamina_factor: f64,
    pub rhythm_top_strains: f64,
    pub colour_top_strains: f64,
    pub stamina_top_strains: f64,
}

impl HasNative for TaikoDifficultyAttributes {
    type Native = NativeTaikoDifficultyAttributes;
}

impl From<NativeTaikoDifficultyAttributes> for TaikoDifficultyAttributes {
    fn from(value: NativeTaikoDifficultyAttributes) -> Self {
        Self {
            star_rating: value.star_rating,
            max_combo: value.max_combo as usize,
            rhythm_difficulty: value.rhythm_difficulty,
            reading_difficulty: value.reading_difficulty,
            colour_difficulty: value.colour_difficulty,
            stamina_difficulty: value.stamina_difficulty,
            mono_stamina_factor: value.mono_stamina_factor,
            rhythm_top_strains: value.rhythm_top_strains,
            colour_top_strains: value.colour_top_strains,
            stamina_top_strains: value.stamina_top_strains,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        beatmap::Beatmap,
        calculator::{DifficultyCalculator, taiko::TaikoDifficultyCalculator},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_convert_taiko() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Taiko).unwrap();
        let calculator = TaikoDifficultyCalculator::new(ruleset, beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_eq!(attributes.star_rating, 3.4385310622836567);
        assert_eq!(attributes.max_combo, 709);
        // assert_eq!(attributes.rhythm_difficulty, 0.6085760732532105);
        // assert_eq!(attributes.reading_difficulty, 0.0);
        // assert_eq!(attributes.colour_difficulty, 0.0);
        // assert_eq!(attributes.stamina_difficulty, 0.0);
        assert_eq!(attributes.mono_stamina_factor, 0.33588227030500867);
        // assert_eq!(attributes.rhythm_top_strains, 0.0);
        // assert_eq!(attributes.colour_top_strains, 0.0);
        // assert_eq!(attributes.stamina_top_strains, 0.0);
    }
}

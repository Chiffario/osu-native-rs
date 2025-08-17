use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, NativeOsuDifficultyAttributes, OsuDifficultyCalculator_Calculate,
    OsuDifficultyCalculator_Create, OsuDifficultyCalculator_Destroy,
};

use crate::{
    beatmap::Beatmap,
    error::OsuError,
    ruleset::Ruleset,
    utils::{HasNative, NativeType},
};

use super::DifficultyCalculator;

#[derive(PartialEq, Eq)]
pub struct OsuDifficultyCalculator {
    handle: i32,
}

impl Drop for OsuDifficultyCalculator {
    fn drop(&mut self) {
        unsafe { OsuDifficultyCalculator_Destroy(self.handle) };
    }
}

impl DifficultyCalculator for OsuDifficultyCalculator {
    type Attributes = OsuDifficultyAttributes;

    fn new(ruleset: Ruleset, beatmap: Beatmap) -> Result<Self, OsuError> {
        let mut handle = 0;

        let code = unsafe {
            OsuDifficultyCalculator_Create(ruleset.handle(), beatmap.handle(), &mut handle)
        };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        Ok(Self { handle })
    }

    fn calculate(&self) -> Result<Self::Attributes, OsuError> {
        let mut attributes: MaybeUninit<NativeType<Self::Attributes>> = MaybeUninit::uninit();

        let code =
            unsafe { OsuDifficultyCalculator_Calculate(self.handle, attributes.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { attributes.assume_init() };

        Ok(native.into())
    }
}

pub struct OsuDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: usize,
    pub aim_difficulty: f64,
    pub aim_difficulty_slider_count: f64,
    pub speed_difficulty: f64,
    pub speed_note_count: f64,
    pub flashlight_difficulty: f64,
    pub slider_factor: f64,
    pub aim_difficult_strain_count: f64,
    pub speed_difficult_strain_count: f64,
    pub drain_rate: f64,
    pub hit_circle_count: i32,
    pub slider_count: i32,
    pub spinner_count: i32,
}

impl HasNative for OsuDifficultyAttributes {
    type Native = NativeOsuDifficultyAttributes;
}

impl From<NativeOsuDifficultyAttributes> for OsuDifficultyAttributes {
    fn from(value: NativeOsuDifficultyAttributes) -> Self {
        Self {
            star_rating: value.star_rating,
            max_combo: value.max_combo as usize,
            aim_difficulty: value.aim_difficulty,
            aim_difficulty_slider_count: value.aim_difficulty_slider_count,
            speed_difficulty: value.speed_difficulty,
            speed_note_count: value.speed_note_count,
            flashlight_difficulty: value.flashlight_difficulty,
            slider_factor: value.slider_factor,
            aim_difficult_strain_count: value.aim_difficult_strain_count,
            speed_difficult_strain_count: value.speed_difficult_strain_count,
            drain_rate: value.drain_rate,
            hit_circle_count: value.hit_circle_count,
            slider_count: value.slider_count,
            spinner_count: value.spinner_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OsuDifficultyCalculator;
    use crate::{
        beatmap::Beatmap,
        calculator::DifficultyCalculator,
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_standard() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Osu).unwrap();
        let calculator = OsuDifficultyCalculator::new(ruleset, beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_ne!(attributes.star_rating, 0.0);
        assert_eq!(attributes.max_combo, 719);
        assert_eq!(attributes.flashlight_difficulty, 0.0);
        assert_ne!(attributes.aim_difficulty, 0.0);
        assert_ne!(attributes.aim_difficulty_slider_count, 0.0);
        assert_ne!(attributes.speed_difficulty, 0.0);
        assert_ne!(attributes.speed_note_count, 0.0);
        assert_ne!(attributes.slider_factor, 0.0);
        assert_ne!(attributes.aim_difficult_strain_count, 0.0);
        assert_ne!(attributes.speed_difficult_strain_count, 0.0);
        assert_ne!(attributes.hit_circle_count, 0);
        assert_ne!(attributes.slider_count, 0);
        assert_ne!(attributes.spinner_count, 0);
    }
}

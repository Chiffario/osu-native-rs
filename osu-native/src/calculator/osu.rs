use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, NativeOsuDifficultyAttributes, OsuDifficultyCalculator_Calculate,
    OsuDifficultyCalculator_Create, OsuDifficultyCalculator_Destroy,
};

use crate::{beatmap, error::OsuError, ruleset};

use super::DifficultyCalculator;

struct OsuDifficultyCalculator {
    handle: i32,
}

impl Drop for OsuDifficultyCalculator {
    fn drop(&mut self) {
        unsafe { OsuDifficultyCalculator_Destroy(self.handle) };
    }
}

impl DifficultyCalculator for OsuDifficultyCalculator {
    type Attributes = OsuDifficultyAttributes;
    type NativeAttributes = NativeOsuDifficultyAttributes;

    fn new(ruleset: ruleset::Ruleset, beatmap: beatmap::Beatmap) -> Result<Self, OsuError> {
        let mut handle = 0;
        unsafe {
            match OsuDifficultyCalculator_Create(
                ruleset.handle(),
                beatmap.handle(),
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
            match OsuDifficultyCalculator_Calculate(self.handle, attributes.as_mut_ptr()) {
                ErrorCode::Success => Ok(attributes.assume_init().into()),
                e => Err(e.into()),
            }
        };

        attributes
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
        ruleset::{Ruleset, Rulesets},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_standard() -> () {
        let beatmap = Beatmap::new_from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new_from_variant(Rulesets::Standard).unwrap();
        let calculator = OsuDifficultyCalculator::new(ruleset, beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_eq!(attributes.star_rating, 5.249653517949988);
        assert_eq!(attributes.max_combo, 719);
        assert_eq!(attributes.aim_difficulty, 2.5911260941792102);
        assert_eq!(attributes.aim_difficulty_slider_count, 98.87994344139403);
        assert_eq!(attributes.speed_difficulty, 2.4120488152035953);
        assert_eq!(attributes.speed_note_count, 269.9710712919464);
        assert_eq!(attributes.flashlight_difficulty, 0.0);
        assert_eq!(attributes.slider_factor, 0.9874380702413679);
        assert_eq!(attributes.aim_difficult_strain_count, 125.82956561063801);
        assert_eq!(attributes.speed_difficult_strain_count, 113.37620139075446);
        assert_eq!(attributes.hit_circle_count, 343);
        assert_eq!(attributes.slider_count, 177);
        assert_eq!(attributes.spinner_count, 2);
    }
}

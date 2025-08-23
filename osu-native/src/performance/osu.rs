use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, NativeOsuPerformanceAttributes, NativeScore, OsuPerformanceCalculator_Calculate,
    OsuPerformanceCalculator_Create, OsuPerformanceCalculator_Destroy,
};
use rosu_mods::GameModSimple;

use crate::{
    difficulty::osu::OsuDifficultyAttributes,
    mods::{IntoGameMods, native::ModCollection},
    performance::{PerformanceCalculator, ScoreStatistics},
    ruleset::Ruleset,
    utils::HasNative,
};

#[derive(Debug)]
pub struct OsuPerformanceCalculator {
    handle: i32,
}

impl Drop for OsuPerformanceCalculator {
    fn drop(&mut self) {
        unsafe {
            OsuPerformanceCalculator_Destroy(self.handle);
        }
    }
}

impl PerformanceCalculator for OsuPerformanceCalculator {
    type DifficultyAttributes = OsuDifficultyAttributes;

    type Attributes = OsuPerformanceAttributes;

    fn new() -> Result<Self, crate::error::OsuError> {
        let mut handle = 0;

        let code = unsafe { OsuPerformanceCalculator_Create(&mut handle) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        Ok(Self { handle })
    }

    fn calculate(
        &self,
        ruleset: &Ruleset,
        score: &ScoreStatistics,
        mods: impl IntoGameMods,
        difficulty_attributes: &Self::DifficultyAttributes,
    ) -> Result<Self::Attributes, crate::error::OsuError> {
        let mods = ModCollection::new()?.with_game_mods(mods)?;
        let mut attributes = MaybeUninit::uninit();
        let score = NativeScore {
            mods_handle: mods.handle(),
            max_combo: score.max_combo,
            accuracy: score.accuracy,
            count_miss: score.count_miss,
            count_meh: score.count_meh,
            count_ok: score.count_ok,
            count_good: score.count_good,
            count_great: score.count_great,
            count_perfect: score.count_perfect,
            count_slider_tail_hit: score.count_slider_tail_hit,
            count_large_tick_miss: score.count_large_tick_miss,
        };
        let code = unsafe {
            OsuPerformanceCalculator_Calculate(
                self.handle,
                ruleset.handle(),
                score.into(),
                difficulty_attributes.into(),
                attributes.as_mut_ptr(),
            )
        };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        Ok(unsafe { attributes.assume_init().into() })
    }
}

#[derive(Debug)]
pub struct OsuPerformanceAttributes {
    pub pp: f64,
    pub aim: f64,
    pub speed: f64,
    pub accuracy: f64,
    pub flashlight: f64,
    pub effective_miss_count: f64,
}

impl HasNative for OsuPerformanceAttributes {
    type Native = NativeOsuPerformanceAttributes;
}

impl From<NativeOsuPerformanceAttributes> for OsuPerformanceAttributes {
    fn from(value: NativeOsuPerformanceAttributes) -> Self {
        Self {
            pp: value.total,
            aim: value.aim,
            speed: value.speed,
            accuracy: value.accuracy,
            flashlight: value.flashlight,
            effective_miss_count: value.effective_miss_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        beatmap::Beatmap,
        difficulty::{DifficultyCalculator, osu::OsuDifficultyCalculator},
        mods::native::ModCollection,
        performance::{PerformanceCalculator, ScoreStatistics, osu::OsuPerformanceCalculator},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_performance_osu() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Osu).unwrap();
        let calculator = OsuDifficultyCalculator::new(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();

        let mods = calculator.mods();

        let score = ScoreStatistics {
            max_combo: attributes.max_combo as i32,
            accuracy: 1.0,
            count_miss: 0,
            count_meh: 0,
            count_ok: 0,
            count_good: 0,
            count_great: attributes.hit_circle_count
                + attributes.slider_count
                + attributes.spinner_count,
            count_perfect: 0,
            count_slider_tail_hit: attributes.slider_count,
            count_large_tick_miss: 0,
        };

        let ruleset = Ruleset::new(RulesetKind::Osu).unwrap();
        let perfcalc = OsuPerformanceCalculator::new().unwrap();
        let attributes = perfcalc
            .calculate(&ruleset, &score, mods.0, &attributes)
            .unwrap();
        println!("attributes: {attributes:#?}");

        assert_ne!(attributes.pp, 0.0);
    }

    #[test]
    fn test_performance_with_different_accuracy_osu() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Osu).unwrap();
        let calculator = OsuDifficultyCalculator::new(ruleset, &beatmap).unwrap();
        let difficulty_attributes = calculator.calculate().unwrap();

        let ss = ScoreStatistics {
            max_combo: difficulty_attributes.max_combo as i32,
            accuracy: 1.0,
            count_miss: 0,
            count_meh: 0,
            count_ok: 0,
            count_good: 0,
            count_great: difficulty_attributes.hit_circle_count
                + difficulty_attributes.slider_count
                + difficulty_attributes.spinner_count,
            count_perfect: 0,
            count_slider_tail_hit: difficulty_attributes.slider_count,
            count_large_tick_miss: 0,
        };
        let worse = ScoreStatistics {
            max_combo: difficulty_attributes.max_combo as i32,
            accuracy: 0.9869,
            count_miss: 0,
            count_meh: 0,
            count_ok: 12,
            count_good: 0,
            count_great: difficulty_attributes.hit_circle_count
                + difficulty_attributes.slider_count
                + difficulty_attributes.spinner_count
                - 12,
            count_perfect: 0,
            count_slider_tail_hit: difficulty_attributes.slider_count,
            count_large_tick_miss: 0,
        };

        let ruleset = Ruleset::new(RulesetKind::Osu).unwrap();
        let perfcalc = OsuPerformanceCalculator::new().unwrap();
        let mods = calculator.mods();
        let ss_attributes = perfcalc
            .calculate(&ruleset, &ss, mods.0, &difficulty_attributes)
            .unwrap();
        let mods = calculator.mods();
        let worse_attributes = perfcalc
            .calculate(&ruleset, &worse, mods.0, &difficulty_attributes)
            .unwrap();

        assert!(ss_attributes.pp > worse_attributes.pp);
    }
}

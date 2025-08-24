use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, NativeScore, NativeTaikoPerformanceAttributes, TaikoPerformanceCalculator_Calculate,
    TaikoPerformanceCalculator_Create, TaikoPerformanceCalculator_Destroy,
};
use rosu_mods::GameModSimple;

use crate::{
    beatmap::Beatmap,
    difficulty::taiko::TaikoDifficultyAttributes,
    mods::{IntoGameMods, native::ModCollection},
    performance::{PerformanceCalculator, ScoreStatistics},
    ruleset::Ruleset,
    utils::HasNative,
};

#[derive(Debug)]
pub struct TaikoPerformanceCalculator {
    handle: i32,
}

impl Drop for TaikoPerformanceCalculator {
    fn drop(&mut self) {
        unsafe {
            TaikoPerformanceCalculator_Destroy(self.handle);
        }
    }
}

impl PerformanceCalculator for TaikoPerformanceCalculator {
    type DifficultyAttributes = TaikoDifficultyAttributes;

    type Attributes = TaikoPerformanceAttributes;

    fn new() -> Result<Self, crate::error::OsuError> {
        let mut handle = 0;

        let code = unsafe { TaikoPerformanceCalculator_Create(&mut handle) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        Ok(Self { handle })
    }

    fn calculate(
        &self,
        ruleset: &Ruleset,
        score: &ScoreStatistics,
        beatmap: &Beatmap,
        mods: impl IntoGameMods,
        difficulty_attributes: &Self::DifficultyAttributes,
    ) -> Result<Self::Attributes, crate::error::OsuError> {
        let mods = ModCollection::new()?.with_game_mods(mods)?;
        let mut attributes = MaybeUninit::uninit();
        let score = NativeScore {
            mods_handle: mods.handle(),
            ruleset_handle: ruleset.handle(),
            beatmap_handle: beatmap.handle(),
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
            TaikoPerformanceCalculator_Calculate(
                self.handle,
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
pub struct TaikoPerformanceAttributes {
    pub pp: f64,
    pub difficulty: f64,
    pub accuracy: f64,
    pub effective_miss_count: f64,
    pub estimated_unstable_rate: Option<f64>,
}

impl HasNative for TaikoPerformanceAttributes {
    type Native = NativeTaikoPerformanceAttributes;
}

impl From<NativeTaikoPerformanceAttributes> for TaikoPerformanceAttributes {
    fn from(value: NativeTaikoPerformanceAttributes) -> Self {
        Self {
            pp: value.total,
            difficulty: value.difficulty,
            accuracy: value.accuracy,
            effective_miss_count: value.effective_miss_count,
            estimated_unstable_rate: value.estimated_unstable_rate.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rosu_mods::{Acronym, GameModSimple};

    use crate::{
        beatmap::Beatmap,
        difficulty::{
            DifficultyCalculator,
            taiko::{TaikoDifficultyAttributes, TaikoDifficultyCalculator},
        },
        performance::{PerformanceCalculator, ScoreStatistics, taiko::TaikoPerformanceCalculator},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_performance_taiko() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Taiko).unwrap();
        let calculator = TaikoDifficultyCalculator::new(ruleset, &beatmap).unwrap();
        let calculator = calculator
            .with_mods(vec![GameModSimple {
                acronym: unsafe { Acronym::from_str_unchecked("CL") },
                settings: Default::default(),
            }])
            .unwrap();
        let attributes: TaikoDifficultyAttributes = calculator.calculate().unwrap();
        println!("Attributes: {attributes:#?}");

        let mods = calculator.mods();

        let score = ScoreStatistics {
            max_combo: attributes.max_combo as i32,
            accuracy: 0.9706,
            count_miss: 0,
            count_meh: 0,
            count_ok: 55,
            count_good: 0,
            count_great: 882,
            count_perfect: 0,
            count_slider_tail_hit: 0,
            count_large_tick_miss: 0,
        };

        let ruleset = Ruleset::new(RulesetKind::Taiko).unwrap();
        let perfcalc = TaikoPerformanceCalculator::new().unwrap();
        let attributes = perfcalc
            .calculate(&ruleset, &score, &beatmap, mods.0, &attributes)
            .unwrap();
        println!("attributes: {attributes:#?}");

        assert_ne!(attributes.pp, 0.0);
        assert!(attributes.estimated_unstable_rate.is_some());
        assert!(attributes.estimated_unstable_rate.unwrap() > 0.0);
    }
}

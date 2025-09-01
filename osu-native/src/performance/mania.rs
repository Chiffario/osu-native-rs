use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, ManiaPerformanceCalculator_Calculate, ManiaPerformanceCalculator_Create,
    ManiaPerformanceCalculator_Destroy, NativeManiaPerformanceAttributes, NativeScore,
};

use crate::{
    beatmap::Beatmap,
    difficulty::mania::ManiaDifficultyAttributes,
    mods::{IntoGameMods, native::ModCollection},
    performance::{PerformanceCalculator, ScoreStatistics},
    ruleset::Ruleset,
    utils::HasNative,
};

#[derive(Debug)]
pub struct ManiaPerformanceCalculator {
    handle: i32,
}

impl Drop for ManiaPerformanceCalculator {
    fn drop(&mut self) {
        unsafe {
            ManiaPerformanceCalculator_Destroy(self.handle);
        }
    }
}

impl PerformanceCalculator for ManiaPerformanceCalculator {
    type DifficultyAttributes = ManiaDifficultyAttributes;

    type Attributes = ManiaPerformanceAttributes;

    fn new() -> Result<Self, crate::error::NativeError> {
        let mut handle = 0;

        let code = unsafe { ManiaPerformanceCalculator_Create(&mut handle) };

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
            ManiaPerformanceCalculator_Calculate(
                self.handle,
                score,
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
pub struct ManiaPerformanceAttributes {
    pub pp: f64,
    pub difficulty: f64,
}

impl HasNative for ManiaPerformanceAttributes {
    type Native = NativeManiaPerformanceAttributes;
}

impl From<NativeManiaPerformanceAttributes> for ManiaPerformanceAttributes {
    fn from(value: NativeManiaPerformanceAttributes) -> Self {
        Self {
            pp: value.total,
            difficulty: value.difficulty,
        }
    }
}

impl From<ManiaPerformanceAttributes> for NativeManiaPerformanceAttributes {
    fn from(val: ManiaPerformanceAttributes) -> Self {
        NativeManiaPerformanceAttributes {
            total: val.pp,
            difficulty: val.difficulty,
        }
    }
}
#[cfg(test)]
mod tests {
    use rosu_mods::{Acronym, GameModSimple};

    use crate::{
        beatmap::Beatmap,
        difficulty::{DifficultyCalculator, mania::*},
        performance::{PerformanceCalculator, ScoreStatistics, mania::ManiaPerformanceCalculator},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_performance_mania() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Mania).unwrap();
        let calculator = ManiaDifficultyCalculator::new(ruleset, &beatmap).unwrap();
        let calculator = calculator
            .with_mods(vec![GameModSimple {
                acronym: unsafe { Acronym::from_str_unchecked("4K") },
                settings: Default::default(),
            }])
            .unwrap();
        let attributes: ManiaDifficultyAttributes = calculator.calculate().unwrap();
        println!("Attributes: {attributes:#?}");

        let mods = calculator.mods();

        let score = ScoreStatistics {
            max_combo: attributes.max_combo as i32,
            accuracy: 0.9996,
            count_miss: 0,
            count_meh: 0,
            count_ok: 0,
            count_good: 0,
            count_great: 16,
            count_perfect: 823,
            count_slider_tail_hit: 0,
            count_large_tick_miss: 0,
        };

        let ruleset = Ruleset::new(RulesetKind::Mania).unwrap();
        let perfcalc = ManiaPerformanceCalculator::new().unwrap();
        let attributes = perfcalc
            .calculate(&ruleset, &score, &beatmap, mods.0, &attributes)
            .unwrap();

        assert_ne!(attributes.pp, 0.0);
    }
}

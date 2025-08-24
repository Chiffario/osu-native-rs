use std::mem::MaybeUninit;

use libosu_native_sys::{
    CatchPerformanceCalculator_Calculate, CatchPerformanceCalculator_Create,
    CatchPerformanceCalculator_Destroy, ErrorCode, NativeCatchPerformanceAttributes, NativeScore,
};

use crate::{
    beatmap::Beatmap,
    difficulty::catch::CatchDifficultyAttributes,
    mods::{IntoGameMods, native::ModCollection},
    performance::{PerformanceCalculator, ScoreStatistics},
    ruleset::Ruleset,
    utils::HasNative,
};

#[derive(Debug)]
pub struct CatchPerformanceCalculator {
    handle: i32,
}

impl Drop for CatchPerformanceCalculator {
    fn drop(&mut self) {
        unsafe {
            CatchPerformanceCalculator_Destroy(self.handle);
        }
    }
}

impl PerformanceCalculator for CatchPerformanceCalculator {
    type DifficultyAttributes = CatchDifficultyAttributes;

    type Attributes = CatchPerformanceAttributes;

    fn new() -> Result<Self, crate::error::OsuError> {
        let mut handle = 0;

        let code = unsafe { CatchPerformanceCalculator_Create(&mut handle) };

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
            CatchPerformanceCalculator_Calculate(
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
pub struct CatchPerformanceAttributes {
    pub pp: f64,
}

impl HasNative for CatchPerformanceAttributes {
    type Native = NativeCatchPerformanceAttributes;
}

impl From<NativeCatchPerformanceAttributes> for CatchPerformanceAttributes {
    fn from(value: NativeCatchPerformanceAttributes) -> Self {
        Self { pp: value.total }
    }
}

impl Into<NativeCatchPerformanceAttributes> for CatchPerformanceAttributes {
    fn into(self) -> NativeCatchPerformanceAttributes {
        NativeCatchPerformanceAttributes { total: self.pp }
    }
}
#[cfg(test)]
mod tests {
    use rosu_mods::{Acronym, GameModSimple};

    use crate::{
        beatmap::Beatmap,
        difficulty::{DifficultyCalculator, catch::*},
        performance::{PerformanceCalculator, ScoreStatistics, catch::CatchPerformanceCalculator},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_performance_catch() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Catch).unwrap();
        let calculator = CatchDifficultyCalculator::new(ruleset, &beatmap).unwrap();
        let calculator = calculator
            .with_mods(vec![GameModSimple {
                acronym: unsafe { Acronym::from_str_unchecked("HD") },
                settings: Default::default(),
            }])
            .unwrap();
        let attributes: CatchDifficultyAttributes = calculator.calculate().unwrap();
        println!("Attributes: {attributes:#?}");

        let mods = calculator.mods();

        let score = ScoreStatistics {
            max_combo: attributes.max_combo as i32,
            accuracy: 1.0,
            count_miss: 0,
            count_meh: 177,
            count_ok: 4,
            count_good: 0,
            count_great: 713,
            count_perfect: 0,
            count_slider_tail_hit: 130,
            count_large_tick_miss: 0,
        };

        let ruleset = Ruleset::new(RulesetKind::Catch).unwrap();
        let perfcalc = CatchPerformanceCalculator::new().unwrap();
        let attributes = perfcalc
            .calculate(&ruleset, &score, &beatmap, mods.0, &attributes)
            .unwrap();

        assert_ne!(attributes.pp, 0.0);
    }
}

use libosu_native_sys::{ErrorCode, NativeScore, OsuPerformanceCalculator_Create};
use rosu_mods::GameModSimple;

use crate::{
    mods::{IntoGameMods, native::ModCollection},
    ruleset::Ruleset,
    utils::HasNative,
};

pub mod osu;
trait PerformanceCalculator: Sized {
    type DifficultyAttributes: HasNative;

    type Attributes: HasNative;

    fn new() -> Result<Self, crate::error::OsuError>;

    fn calculate(
        &self,
        ruleset: &Ruleset,
        score: &ScoreStatistics,
        mods: impl IntoGameMods,
        difficulty_attributes: &Self::DifficultyAttributes,
    ) -> Result<Self::Attributes, crate::error::OsuError>;
}

pub struct ScoreStatistics {
    pub max_combo: i32,
    pub accuracy: f64,
    pub count_miss: i32,
    pub count_meh: i32,
    pub count_ok: i32,
    pub count_good: i32,
    pub count_great: i32,
    pub count_perfect: i32,
    pub count_slider_tail_hit: i32,
    pub count_large_tick_miss: i32,
}

// impl Into<NativeScore> for ScoreStatistics {
//     fn into(self) -> NativeScore {
//         NativeScore {
//             mods_handle: ModCollection::new()
//                 .unwrap()
//                 .with_game_mods(self.mods)
//                 .unwrap()
//                 .handle(),
//             max_combo: self.max_combo,
//             accuracy: self.accuracy,
//             count_miss: self.count_miss,
//             count_meh: self.count_meh,
//             count_ok: self.count_ok,
//             count_good: self.count_good,
//             count_great: self.count_great,
//             count_perfect: self.count_perfect,
//             count_slider_tail_hit: self.count_slider_tail_hit,
//             count_large_tick_miss: self.count_large_tick_miss,
//         }
//     }
// }

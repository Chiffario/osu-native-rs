use std::{marker::PhantomData, path::Path};

use libosu_native_sys::{ErrorCode, NativeScore, OsuPerformanceCalculator_Create};
use rosu_mods::GameModSimple;

use crate::{
    beatmap::{Beatmap, BeatmapError},
    difficulty::{
        DifficultyCalculator,
        osu::{OsuDifficultyAttributes, OsuDifficultyCalculator},
        taiko::{TaikoDifficultyAttributes, TaikoDifficultyCalculator},
    },
    error::OsuError,
    mods::{
        IntoGameMods,
        native::{ModCollection, ModCollectionError},
    },
    performance::{
        osu::{OsuPerformanceAttributes, OsuPerformanceCalculator},
        taiko::{TaikoPerformanceAttributes, TaikoPerformanceCalculator},
    },
    ruleset::{Ruleset, RulesetError, RulesetKind},
    utils::HasNative,
};

pub mod catch;
pub mod osu;
pub mod taiko;
trait PerformanceCalculator: Sized {
    type DifficultyAttributes: HasNative;

    type Attributes: HasNative;

    fn new() -> Result<Self, crate::error::OsuError>;

    fn calculate(
        &self,
        ruleset: &Ruleset,
        score: &ScoreStatistics,
        beatmap: &Beatmap,
        mods: impl IntoGameMods,
        difficulty_attributes: &Self::DifficultyAttributes,
    ) -> Result<Self::Attributes, crate::error::OsuError>;
}

#[derive(Debug)]
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

impl Default for ScoreStatistics {
    fn default() -> Self {
        Self {
            max_combo: Default::default(),
            accuracy: 1.0,
            count_miss: Default::default(),
            count_meh: Default::default(),
            count_ok: Default::default(),
            count_good: Default::default(),
            count_great: Default::default(),
            count_perfect: Default::default(),
            count_slider_tail_hit: Default::default(),
            count_large_tick_miss: Default::default(),
        }
    }
}

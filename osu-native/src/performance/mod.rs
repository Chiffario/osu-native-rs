#![allow(dead_code)]
use std::{marker::PhantomData, path::Path};

use rosu_mods::GameModSimple;

use crate::{
    beatmap::{Beatmap, BeatmapError},
    difficulty::{
        DifficultyCalculator, catch::CatchDifficultyCalculator, mania::ManiaDifficultyCalculator,
        osu::OsuDifficultyCalculator, taiko::TaikoDifficultyCalculator,
    },
    error::OsuError,
    mods::{IntoGameMods, native::ModCollectionError},
    performance::{
        catch::CatchPerformanceCalculator, mania::ManiaPerformanceCalculator,
        osu::OsuPerformanceCalculator, taiko::TaikoPerformanceCalculator,
    },
    ruleset::{Ruleset, RulesetError, RulesetKind},
    utils::HasNative,
};

pub mod catch;
pub mod mania;
pub mod osu;
pub mod taiko;
pub trait PerformanceCalculator: Sized {
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
    pub count_meh: i32,     // n50
    pub count_ok: i32,      // n100
    pub count_good: i32,    // n200
    pub count_great: i32,   // n300
    pub count_perfect: i32, // n320
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

pub struct Empty;
pub struct WithBeatmap;

pub struct Osu;
pub struct Taiko;
pub struct Mania;
pub struct Catch;
pub trait RulesetTrait {
    type PerformanceCalculatorTy: PerformanceCalculator;
    type DifficultyCalculatorTy: DifficultyCalculator;
    const KIND: RulesetKind;
}
impl RulesetTrait for Osu {
    type PerformanceCalculatorTy = OsuPerformanceCalculator;
    type DifficultyCalculatorTy = OsuDifficultyCalculator;
    const KIND: RulesetKind = RulesetKind::Osu;
}
impl RulesetTrait for Taiko {
    type PerformanceCalculatorTy = TaikoPerformanceCalculator;
    type DifficultyCalculatorTy = TaikoDifficultyCalculator;
    const KIND: RulesetKind = RulesetKind::Taiko;
}
impl RulesetTrait for Mania {
    type PerformanceCalculatorTy = ManiaPerformanceCalculator;
    type DifficultyCalculatorTy = ManiaDifficultyCalculator;
    const KIND: RulesetKind = RulesetKind::Mania;
}
impl RulesetTrait for Catch {
    type PerformanceCalculatorTy = CatchPerformanceCalculator;
    type DifficultyCalculatorTy = CatchDifficultyCalculator;
    const KIND: RulesetKind = RulesetKind::Catch;
}
pub struct WithRuleset<T: RulesetTrait> {
    _marker: PhantomData<T>,
}

pub struct PerformanceCalculatorBuilder<T: RulesetTrait> {
    beatmap: Beatmap,
    ruleset: Ruleset,
    mods: Vec<GameModSimple>,
    difficulty_attributes: <<T as RulesetTrait>::PerformanceCalculatorTy as PerformanceCalculator>::DifficultyAttributes,
    score_state: ScoreStatistics,
    _marker: PhantomData<T>,
}

pub struct CalculatorBuilder<T> {
    beatmap: Option<Beatmap>,
    ruleset: Option<Ruleset>,
    mods: Option<Vec<GameModSimple>>,
    _marker: PhantomData<T>,
}

impl CalculatorBuilder<WithBeatmap> {
    pub fn from_path(map: impl AsRef<Path>) -> Result<Self, BeatmapError> {
        let beatmap = Beatmap::from_path(map)?;
        Ok(CalculatorBuilder {
            beatmap: Some(beatmap),
            ruleset: None,
            mods: None,
            _marker: PhantomData::<WithBeatmap>,
        })
    }

    pub fn from_text(string: String) -> Result<Self, BeatmapError> {
        let beatmap = Beatmap::from_text(string)?;
        Ok(CalculatorBuilder {
            beatmap: Some(beatmap),
            ruleset: None,
            mods: None,
            _marker: PhantomData::<WithBeatmap>,
        })
    }
}

macro_rules! implement_ruleset {
    ($name:ident, $ty:ident) => {
        fn $name(self) -> Result<CalculatorBuilder<WithRuleset<$ty>>, RulesetError> {
            let ruleset = Ruleset::new($ty::KIND)?;
            Ok(CalculatorBuilder {
                beatmap: self.beatmap,
                ruleset: Some(ruleset),
                mods: None,
                _marker: PhantomData::<WithRuleset<$ty>>,
            })
        }
    };
}

impl CalculatorBuilder<WithBeatmap> {
    implement_ruleset!(osu, Osu);
    implement_ruleset!(taiko, Taiko);
    implement_ruleset!(mania, Mania);
    implement_ruleset!(catch, Catch);
}

impl<T> CalculatorBuilder<WithRuleset<T>>
where
    T: RulesetTrait,
{
    fn mods(
        mut self,
        mods: impl IntoGameMods,
    ) -> Result<CalculatorBuilder<WithRuleset<T>>, ModCollectionError> {
        self.mods = Some(mods.into_mods()?.0);
        Ok(self)
    }
}

trait Difficulty {
    type Calculator: DifficultyCalculator;
    type RulesetType: RulesetTrait;
    fn difficulty(self) -> Result<Self::Calculator, OsuError>;
    fn performance(self) -> Result<PerformanceCalculatorBuilder<Self::RulesetType>, OsuError>;
}

macro_rules! implement_difficulty {
    ($ruleset:ty) => {
        impl Difficulty for CalculatorBuilder<WithRuleset<$ruleset>> {
            type Calculator = <$ruleset as RulesetTrait>::DifficultyCalculatorTy;
            type RulesetType = $ruleset;

            fn difficulty(self) -> Result<Self::Calculator, OsuError> {
                Self::Calculator::new(self.ruleset.unwrap(), self.beatmap.as_ref().unwrap())
            }
            fn performance(
                self,
            ) -> Result<PerformanceCalculatorBuilder<Self::RulesetType>, OsuError> {
                let ruleset = Ruleset::new(self.ruleset.as_ref().unwrap().kind).unwrap();
                let attr =
                    Self::Calculator::new(self.ruleset.unwrap(), self.beatmap.as_ref().unwrap())?
                        .calculate()?;
                let mut score = ScoreStatistics::default();
                score.max_combo = attr.max_combo;
                Ok(PerformanceCalculatorBuilder {
                    beatmap: self.beatmap.unwrap(),
                    ruleset,
                    mods: self.mods.unwrap(),
                    difficulty_attributes: attr,
                    score_state: score,
                    _marker: PhantomData::<Self::RulesetType>,
                })
            }
        }
    };
}

implement_difficulty!(Osu);
implement_difficulty!(Taiko);
implement_difficulty!(Mania);
implement_difficulty!(Catch);

impl<T: RulesetTrait> PerformanceCalculatorBuilder<T> {
    fn with_score_state(mut self, score: ScoreStatistics) -> Self {
        self.score_state = score;
        self
    }
    fn max_combo(mut self, n: i32) -> Self {
        self.score_state.max_combo = n;
        self
    }
    fn misses(mut self, n: i32) -> Self {
        self.score_state.count_miss = n;
        self
    }
    fn accuracy(mut self, acc: f64) -> Self {
        self.score_state.accuracy = acc;
        self
    }
    fn calculator(
        self,
    ) -> Result<<T::PerformanceCalculatorTy as PerformanceCalculator>::Attributes, OsuError> {
        let calc = T::PerformanceCalculatorTy::new().unwrap().calculate(
            &self.ruleset,
            &self.score_state,
            &self.beatmap,
            self.mods,
            &self.difficulty_attributes,
        )?;
        Ok(calc)
    }
}

macro_rules! implement_setter {
    // Match patterns like: alias -> field, alias -> field, ...
    {$($alias:ident -> $field:ident),+ $(,)?} => {
        $(
            fn $alias(mut self, n: i32) -> Self {
                self.score_state.$field = n;
                self
            }
        )+
    };

    // Support for different types, not just i32
    {$($alias:ident -> $field:ident : $type:ty),+ $(,)?} => {
        $(
            fn $alias(mut self, value: $type) -> Self {
                self.score_state.$field = value;
                self
            }
        )+
    };
}

impl PerformanceCalculatorBuilder<Osu> {
    implement_setter! {
        n300 -> count_great,
        n100 -> count_ok,
        n50 -> count_meh,
        slider_tick_hits -> count_slider_tail_hit,
    }
    fn slider_tick_misses(mut self, n: i32) -> Self {
        self.score_state.count_slider_tail_hit = self.difficulty_attributes.slider_count - n;
        self
    }
}

impl PerformanceCalculatorBuilder<Taiko> {
    implement_setter! {
        n300 -> count_great,
        n100 -> count_ok,
    }
}

impl PerformanceCalculatorBuilder<Mania> {
    implement_setter! {
        n320 -> count_perfect,
        n300 -> count_great,
        n200 -> count_good,
        n100 -> count_ok,
        n50 -> count_meh,
    }
}
impl PerformanceCalculatorBuilder<Catch> {
    implement_setter! {
        hits -> count_great,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        difficulty::DifficultyCalculator,
        performance::{CalculatorBuilder, Difficulty},
        utils::initialize_path,
    };

    #[test]
    fn test_typestate() -> Result<(), Box<dyn std::error::Error>> {
        let calc = CalculatorBuilder::from_path(initialize_path())?
            .osu()?
            .mods(vec![])?
            .difficulty()?
            .calculate()?;
        assert!(calc.star_rating > 0.0);
        assert_eq!(calc.max_combo, 719);
        let calc = CalculatorBuilder::from_path(initialize_path())?
            .osu()?
            .mods(vec![])?
            .performance()?
            .max_combo(250)
            .n100(10)
            .accuracy(0.98)
            .calculator()?;
        assert!(calc.pp > 0.0);
        Ok(())
    }
}

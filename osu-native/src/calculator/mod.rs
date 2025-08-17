use crate::{beatmap::Beatmap, error::OsuError, ruleset::Ruleset, utils::HasNative};

pub mod catch;
pub mod mania;
pub mod osu;
pub mod taiko;

pub trait DifficultyCalculator: Sized {
    type Attributes: HasNative;

    fn new(ruleset: Ruleset, beatmap: Beatmap) -> Result<Self, OsuError>;

    fn calculate(&self) -> Result<Self::Attributes, OsuError>;
}

use crate::{beatmap::Beatmap, error::OsuError, ruleset::Ruleset};

pub mod catch;
pub mod mania;
pub mod osu;
pub mod taiko;

trait DifficultyCalculator: Sized {
    type Attributes;
    type NativeAttributes;

    fn new(ruleset: Ruleset, beatmap: Beatmap) -> Result<Self, OsuError>;

    fn calculate(&self) -> Result<Self::Attributes, OsuError>;
}

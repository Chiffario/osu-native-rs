use crate::{
    beatmap::Beatmap,
    error::{NativeError, OsuError},
    ruleset::Ruleset,
};
pub mod catch;
pub mod mania;
pub mod osu;
pub mod taiko;

trait DifficultyCalculator {
    type Attributes;
    type NativeAttributes;
    fn new(ruleset: Ruleset, beatmap: Beatmap) -> Result<Self, OsuError>
    where
        Self: Sized;
    fn calculate(&self) -> Result<Self::Attributes, OsuError>;
}

use crate::{
    beatmap::Beatmap,
    error::{NativeError, OsuError},
    ruleset::Ruleset,
};
pub mod osu;

trait DifficultyCalculator {
    type Attributes;
    fn new(ruleset: Ruleset, beatmap: Beatmap) -> Result<Self, OsuError>
    where
        Self: Sized;
    fn calculate(&self) -> Result<Self::Attributes, OsuError>;
}

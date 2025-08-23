use crate::{
    beatmap::Beatmap,
    error::{NativeError, OsuError},
    mods::{GameModsError, IntoGameMods},
    ruleset::Ruleset,
    traits::NativeWrapper,
};

pub mod catch;
pub mod mania;
pub mod osu;
pub mod taiko;

pub trait DifficultyCalculator: Sized + NativeWrapper {
    type Attributes;

    fn create(ruleset: Ruleset, beatmap: &Beatmap) -> Result<Self, NativeError>;

    fn mods(self, mods: impl IntoGameMods) -> Result<Self, GameModsError>;

    fn calculate(&self) -> Result<Self::Attributes, OsuError>;
}

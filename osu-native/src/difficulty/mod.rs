use crate::{
    beatmap::Beatmap,
    error::OsuError,
    mods::{GameMods, GameModsError, IntoGameMods},
    ruleset::Ruleset,
    utils::HasNative,
};

pub mod catch;
pub mod mania;
pub mod osu;
pub mod taiko;

pub trait DifficultyCalculator: Sized {
    type DifficultyAttributes: HasNative;

    fn new(ruleset: Ruleset, beatmap: &Beatmap) -> Result<Self, OsuError>;

    fn mods(&self) -> GameMods;
    fn with_mods(self, mods: impl IntoGameMods) -> Result<Self, GameModsError>;

    fn calculate(&self) -> Result<Self::DifficultyAttributes, OsuError>;
}

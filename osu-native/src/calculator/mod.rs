use crate::{
    beatmap::Beatmap,
    error::OsuError,
    mods::{GameModsError, IntoGameMods},
    utils::HasNative,
};

pub mod catch;
pub mod mania;
pub mod osu;
pub mod taiko;

pub trait DifficultyCalculator: Sized {
    type Attributes: HasNative;

    fn new(beatmap: &Beatmap) -> Result<Self, OsuError>;

    fn mods(self, mods: impl IntoGameMods) -> Result<Self, GameModsError>;

    fn calculate(&self) -> Result<Self::Attributes, OsuError>;
}

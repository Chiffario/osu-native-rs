use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result as FmtResult},
};

use rosu_mods::{GameModSimple, GameMods as GameModsLazer, GameModsIntermode, GameModsLegacy};
use serde_json::Error as JsonError;
use thiserror::Error as ThisError;

pub mod native;

#[derive(Clone, PartialEq)]
pub struct GameMods(pub(crate) Vec<GameModSimple>);

/// Convenience trait to turn a type into [`GameMods`]
///
/// Should be useful because using raw native mods is annoying and
/// very error prone (due to requiring manual verification of setting names).
/// Implementation over [`GameModsLazer`] and [`GameModsIntermode`] allows for
/// full rosu-mods integration
pub trait IntoGameMods {
    fn into_mods(self) -> Result<GameMods, GameModsError>;
}

impl GameMods {
    pub(crate) const DEFAULT: Self = Self(Vec::new());
}

impl Debug for GameMods {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}

impl Default for GameMods {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, ThisError)]
pub enum GameModsError {
    #[error("Failed to serialize mods")]
    Serialization(#[source] JsonError),
    #[error("Failed to deserialize mods")]
    Deserialization(#[source] JsonError),
}

impl IntoGameMods for &GameModsLazer {
    fn into_mods(self) -> Result<GameMods, GameModsError> {
        let serialized = serde_json::to_vec(self).map_err(GameModsError::Serialization)?;
        let mods = serde_json::from_slice(&serialized).map_err(GameModsError::Deserialization)?;

        Ok(GameMods(mods))
    }
}

impl IntoGameMods for &GameModsIntermode {
    fn into_mods(self) -> Result<GameMods, GameModsError> {
        let mods = self
            .iter()
            .map(|m| GameModSimple {
                acronym: m.acronym(),
                settings: HashMap::new(),
            })
            .collect();

        Ok(GameMods(mods))
    }
}

impl IntoGameMods for GameModsLegacy {
    fn into_mods(self) -> Result<GameMods, GameModsError> {
        self.bits().into_mods()
    }
}

impl IntoGameMods for u32 {
    fn into_mods(self) -> Result<GameMods, GameModsError> {
        IntoGameMods::into_mods(&GameModsIntermode::from_bits(self))
    }
}

impl IntoGameMods for Vec<GameModSimple> {
    fn into_mods(self) -> Result<GameMods, GameModsError> {
        Ok(GameMods(self))
    }
}

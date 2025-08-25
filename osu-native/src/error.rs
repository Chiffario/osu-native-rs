use libosu_native_sys::ErrorCode;
use thiserror::Error as ThisError;

use crate::mods::native::{ModCollectionError, ModError};

#[derive(Debug, ThisError)]
pub enum NativeError {
    #[error("Native object not found")]
    ObjectNotFound,
    #[error("Specified ruleset is unavailable")]
    RulesetUnavailable,
    #[error("Specified ruleset isn't expected in operation context")]
    UnexpectedRuleset,
    #[error("Beatmap file with specified path was not found")]
    BeatmapFileNotFound,
    #[error("Unknown error code: {0:?}")]
    UnknownError(ErrorCode),
}

impl From<ErrorCode> for NativeError {
    fn from(code: ErrorCode) -> Self {
        match code {
            ErrorCode::ObjectNotFound => Self::ObjectNotFound,
            ErrorCode::RulesetUnavailable => Self::RulesetUnavailable,
            ErrorCode::UnexpectedRuleset => Self::UnexpectedRuleset,
            ErrorCode::BeatmapFileNotFound => Self::BeatmapFileNotFound,
            _ => Self::UnknownError(code),
        }
    }
}

// TODO: split this type up into whatever is needed
#[derive(Debug, ThisError)]
pub enum OsuError {
    #[error("Library error, contact the developer")]
    LogicError,
    #[error("GameMod error")]
    Mods(#[from] ModError),
    #[error("Mod collection error")]
    ModCollection(#[from] ModCollectionError),
    #[error("Native error")]
    NativeError(#[from] NativeError),
    #[error("Unknown error")]
    UnknownError,
}

impl From<ErrorCode> for OsuError {
    fn from(code: ErrorCode) -> Self {
        Self::NativeError(code.into())
    }
}

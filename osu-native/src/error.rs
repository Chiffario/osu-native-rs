use libosu_native_sys::ErrorCode;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum NativeError {
    #[error("Native object not found")]
    ObjectNotFound,
    #[error("Specified ruleset is unavailable")]
    RulesetUnavailable,
    #[error("Beatmap file with specified path was not found")]
    BeatmapFileNotFound,
    #[error("Unknown error")]
    UnknownError,
}

impl From<ErrorCode> for NativeError {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::ObjectNotFound => Self::ObjectNotFound,
            ErrorCode::RulesetUnavailable => Self::RulesetUnavailable,
            ErrorCode::BeatmapFileNotFound => Self::BeatmapFileNotFound,
            _ => Self::UnknownError,
        }
    }
}

#[derive(Debug, ThisError)]
pub enum OsuError {
    #[error("Library error, contact the developer")]
    LogicError,
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

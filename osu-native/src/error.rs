use std::fmt::Display;

use libosu_native_sys::ErrorCode;
#[derive(Debug)]
pub enum NativeError {
    ObjectNotFound,
    RulesetUnavailable,
    BeatmapFileNotFound(String),
    UnknownError,
}
impl Display for NativeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NativeError::ObjectNotFound => writeln!(f, "Native object not found"),
            NativeError::RulesetUnavailable => writeln!(f, "Specified ruleset is unavailable"),
            NativeError::BeatmapFileNotFound(v) => {
                writeln!(f, "Beatmap file with specified path ({v}) is not found")
            }
            NativeError::UnknownError => writeln!(f, "Unknown error"),
        }
    }
}
impl std::error::Error for NativeError {}
impl From<ErrorCode> for NativeError {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::ObjectNotFound => Self::ObjectNotFound,
            ErrorCode::RulesetUnavailable => Self::RulesetUnavailable,
            ErrorCode::BeatmapFileNotFound => Self::BeatmapFileNotFound(String::new()),
            _ => Self::UnknownError,
        }
    }
}
pub fn error_code_to_osu(error_code: ErrorCode) -> OsuError {
    match error_code {
        ErrorCode::ObjectNotFound => OsuError::NativeError(NativeError::ObjectNotFound),
        ErrorCode::RulesetUnavailable => OsuError::NativeError(NativeError::RulesetUnavailable),
        ErrorCode::BeatmapFileNotFound => {
            OsuError::NativeError(NativeError::BeatmapFileNotFound(String::new()))
        }
        _ => OsuError::NativeError(NativeError::UnknownError),
    }
}

#[derive(Debug)]
pub enum OsuError {
    LogicError,
    NativeError(NativeError),
    UnknownError,
}
impl Display for OsuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OsuError::LogicError => writeln!(f, "Library error, contact the developer"),
            OsuError::NativeError(e) => writeln!(f, "Native error: {e}"),
            OsuError::UnknownError => writeln!(f, "Unknown error"),
        }
    }
}
impl std::error::Error for OsuError {}

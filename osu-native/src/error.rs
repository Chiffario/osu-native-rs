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
pub fn error_code_to_native(error_code: ErrorCode) -> NativeError {
    match error_code {
        ErrorCode::ObjectNotFound => NativeError::ObjectNotFound,
        ErrorCode::RulesetUnavailable => NativeError::RulesetUnavailable,
        ErrorCode::BeatmapFileNotFound => NativeError::BeatmapFileNotFound(String::new()),
        _ => NativeError::UnknownError,
    }
}

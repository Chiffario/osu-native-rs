use libosu_native_sys::ErrorCode;

pub enum Error {
    ObjectNotFound,
    RulesetUnavailable,
    BeatmapFileNotFound,
    UnknownFailure,
}

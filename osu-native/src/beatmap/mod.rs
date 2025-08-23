use std::{ffi::CString, mem::MaybeUninit, path::Path};

use libosu_native_sys::{
    Beatmap_CreateFromFile, Beatmap_Destroy, Beatmap_GetArtist, Beatmap_GetTitle,
    Beatmap_GetVersion, ErrorCode, NativeBeatmap, NativeBeatmapHandle,
};
use thiserror::Error as ThisError;

use crate::{
    error::NativeError,
    traits::Native,
    utils::{StringError, read_native_string},
};

impl_native!(NativeBeatmap: NativeBeatmapHandle, Beatmap_Destroy);

declare_native_wrapper! {
    #[derive(Debug, PartialEq)]
    pub struct Beatmap(NativeBeatmap);
}

#[derive(Debug, ThisError)]
pub enum BeatmapError {
    #[error("Specified path is invalid")]
    PathError,
    #[error("Native error")]
    Native(#[from] NativeError),
}

impl From<ErrorCode> for BeatmapError {
    fn from(value: ErrorCode) -> Self {
        Self::Native(value.into())
    }
}

impl Beatmap {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, BeatmapError> {
        let Some(Ok(path_cstr)) = path.as_ref().to_str().map(CString::new) else {
            return Err(BeatmapError::PathError);
        };

        let mut beatmap = MaybeUninit::uninit();
        let code = unsafe { Beatmap_CreateFromFile(path_cstr.as_ptr(), beatmap.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { beatmap.assume_init() };

        Ok(Self(native))
    }

    pub fn title(&self) -> Result<String, StringError> {
        read_native_string(self.handle(), Beatmap_GetTitle)
    }

    pub fn artist(&self) -> Result<String, StringError> {
        read_native_string(self.handle(), Beatmap_GetArtist)
    }

    pub fn version(&self) -> Result<String, StringError> {
        read_native_string(self.handle(), Beatmap_GetVersion)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::initialize_path;

    use super::Beatmap;

    #[test]
    fn test_create_map_from_path() {
        let map = Beatmap::from_path(initialize_path()).unwrap();
        assert_eq!(map.approach_rate, 9.2);
        assert_eq!(map.overall_difficulty, 8.3);
        assert_eq!(map.drain_rate, 5.0);
        assert_eq!(map.circle_size, 4.0);
        assert_eq!(map.slider_multiplier, 2.0);
        assert_eq!(map.slider_tick_rate, 1.0);
    }

    #[test]
    fn test_get_strings() {
        let map = Beatmap::from_path(initialize_path()).unwrap();
        let title = map.title().unwrap();
        assert_eq!(title, "Toy Box");
        let artist = map.artist().unwrap();
        assert_eq!(artist, "John Grant");
        let version = map.version().unwrap();
        assert_eq!(version, "Expert");
    }
}

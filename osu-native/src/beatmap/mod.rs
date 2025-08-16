use std::{ffi::CString, mem::MaybeUninit, path::Path};

use libosu_native_sys::{
    Beatmap_CreateFromFile, Beatmap_Destroy, Beatmap_GetArtist, Beatmap_GetTitle,
    Beatmap_GetVersion, ErrorCode, NativeBeatmap,
};
use thiserror::Error as ThisError;

use crate::{
    error::{NativeError, OsuError},
    utils::read_native_string,
};

pub struct Beatmap {
    handle: i32,
    pub approach_rate: f32,
    pub drain_rate: f32,
    pub overall_difficulty: f32,
    pub circle_size: f32,
    pub slider_multiplier: f64,
    pub slider_tick_rate: f64,
}

impl Beatmap {
    pub fn handle(&self) -> i32 {
        self.handle
    }
}

impl Drop for Beatmap {
    fn drop(&mut self) {
        unsafe { Beatmap_Destroy(self.handle) };
    }
}

impl From<NativeBeatmap> for Beatmap {
    fn from(value: NativeBeatmap) -> Self {
        Self {
            handle: value.handle,
            approach_rate: value.approach_rate,
            drain_rate: value.drain_rate,
            overall_difficulty: value.overall_difficulty,
            circle_size: value.circle_size,
            slider_multiplier: value.slider_multiplier,
            slider_tick_rate: value.slider_tick_rate,
        }
    }
}

#[derive(Debug, ThisError)]
pub enum BeatmapError {
    #[error("Specified path is invalid")]
    PathError,
    #[error("Native error")]
    GenericError(#[from] OsuError),
    #[error("Unknown error")]
    UnknownError,
}

impl From<NativeError> for BeatmapError {
    fn from(value: NativeError) -> Self {
        Self::GenericError(value.into())
    }
}

impl Beatmap {
    pub fn new_from_path(path: impl AsRef<Path>) -> Result<Self, BeatmapError> {
        let path_str = path.as_ref().to_str().ok_or(BeatmapError::PathError)?;
        let path_cstr = CString::new(path_str).map_err(|_| BeatmapError::PathError)?;
        let mut beatmap: MaybeUninit<NativeBeatmap> = MaybeUninit::uninit();

        let beatmap: Result<Beatmap, NativeError> = unsafe {
            match Beatmap_CreateFromFile(path_cstr.as_ptr(), beatmap.as_mut_ptr()) {
                ErrorCode::Success => Ok(beatmap.assume_init().into()),
                e => Err(e.into()),
            }
        };

        beatmap.map_err(Into::into)
    }

    pub fn title(&self) -> Result<String, BeatmapError> {
        read_native_string(self.handle, Beatmap_GetTitle).map_err(Into::into)
    }

    pub fn artist(&self) -> Result<String, BeatmapError> {
        read_native_string(self.handle, Beatmap_GetArtist).map_err(Into::into)
    }

    pub fn version(&self) -> Result<String, BeatmapError> {
        read_native_string(self.handle, Beatmap_GetVersion).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::initialize_path;

    use super::Beatmap;

    #[test]
    fn test_create_map_from_path() {
        let map = Beatmap::new_from_path(initialize_path()).unwrap();
        assert_eq!(map.approach_rate, 9.2);
        assert_eq!(map.overall_difficulty, 8.3);
        assert_eq!(map.drain_rate, 5.0);
        assert_eq!(map.circle_size, 4.0);
        assert_eq!(map.slider_multiplier, 2.0);
        assert_eq!(map.slider_tick_rate, 1.0);
    }

    #[test]
    fn test_get_strings() {
        let map = Beatmap::new_from_path(initialize_path()).unwrap();
        let title = map.title().unwrap();
        assert_eq!(title, String::from("Toy Box"));
        let artist = map.artist().unwrap();
        assert_eq!(artist, String::from("John Grant"));
        let version = map.version().unwrap();
        assert_eq!(version, String::from("Expert"));
    }
}

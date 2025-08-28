use std::{ffi::CString, mem::MaybeUninit, path::Path};

use libosu_native_sys::{
    Beatmap_CreateFromFile, Beatmap_CreateFromText, Beatmap_Destroy, Beatmap_GetArtist,
    Beatmap_GetTitle, Beatmap_GetVersion, ErrorCode, NativeBeatmap,
};
use thiserror::Error as ThisError;

use crate::{
    error::NativeError,
    utils::{HasNative, NativeType, StringError, read_native_string},
};

/// osu! Beatmap. Contains general ruleset-independent attributes
/// Also contains methods for getting extra data
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
        // Ensure resources are freed on osu-native's side
        unsafe { Beatmap_Destroy(self.handle) };
    }
}

impl HasNative for Beatmap {
    type Native = NativeBeatmap;
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
    #[error("Specified string is invalid")]
    StringError,
    #[error("Native error")]
    Native(#[from] NativeError),
}

impl From<ErrorCode> for BeatmapError {
    fn from(value: ErrorCode) -> Self {
        Self::Native(value.into())
    }
}

impl Beatmap {
    /// Creates a new [`Beatmap`] from a path to a .osu file
    ///
    /// # Examples
    /// ```no_run
    /// # use osu_native::beatmap::Beatmap;
    /// # let get_path = || "../../standard.osu";
    /// let path = get_path();
    /// let beatmap = Beatmap::from_path(path).unwrap();
    /// println!("{}", beatmap.approach_rate);
    /// ```
    ///
    /// # Errors
    /// Returns a [`BeatmapError::StringError`] if the path can't be correctly converted to [`CString`]
    /// Returns a [`BeatmapError::NativeError`] if there is an error on osu-native's side
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, BeatmapError> {
        let Some(Ok(path_cstr)) = path.as_ref().to_str().map(CString::new) else {
            return Err(BeatmapError::StringError);
        };

        let mut beatmap: MaybeUninit<NativeType<Self>> = MaybeUninit::uninit();

        let code = unsafe { Beatmap_CreateFromFile(path_cstr.as_ptr(), beatmap.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { beatmap.assume_init() };

        Ok(native.into())
    }

    /// Creates a new [`Beatmap`] from an .osu file in a [`String`]
    ///
    /// # Examples
    /// ```no_run
    /// # use osu_native::beatmap::Beatmap;
    /// # let get_path = || "../../standard.osu";
    /// let path = get_path();
    /// let beatmap = Beatmap::from_path(path).unwrap();
    /// println!("{}", beatmap.approach_rate);
    /// ```
    ///
    /// # Errors
    /// Returns a [`BeatmapError::StringError`] if the path can't be correctly converted to [`CString`]
    /// Returns a [`BeatmapError::NativeError`] if there is an error on osu-native's side
    pub fn from_text(string: String) -> Result<Self, BeatmapError> {
        let Ok(map_cstr) = CString::new(string) else {
            return Err(BeatmapError::StringError);
        };

        let mut beatmap: MaybeUninit<NativeType<Self>> = MaybeUninit::uninit();

        let code = unsafe { Beatmap_CreateFromText(map_cstr.as_ptr(), beatmap.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { beatmap.assume_init() };

        Ok(native.into())
    }

    /// Creates a [`String`] with beatmap's romanized title by fetching it from osu-native
    ///
    /// # Examples
    /// ```no_run
    /// # use osu_native::beatmap::Beatmap;
    /// # let path = "../../standard.osu";
    /// # let beatmap = Beatmap::from_path(path).unwrap();
    /// let title = beatmap.title().unwrap();
    /// println!("{title}");
    /// ```
    ///
    /// # Errors
    /// See [`read_native_string()`]
    pub fn title(&self) -> Result<String, StringError> {
        read_native_string(self.handle, Beatmap_GetTitle)
    }

    /// Creates a [`String`] with beatmap's romanized artist name by fetching it from osu-native
    ///
    /// # Examples
    /// ```no_run
    /// # use osu_native::beatmap::Beatmap;
    /// # let path = "../../standard.osu";
    /// # let beatmap = Beatmap::from_path(path).unwrap();
    /// let title = beatmap.artist().unwrap();
    /// println!("{title}");
    /// ```
    ///
    /// # Errors
    /// See [`read_native_string()`]
    pub fn artist(&self) -> Result<String, StringError> {
        read_native_string(self.handle, Beatmap_GetArtist)
    }

    /// Creates a [`String`] with beatmap's difficulty name by fetching it from osu-native
    ///
    /// # Examples
    /// ```no_run
    /// # use osu_native::beatmap::Beatmap;
    /// # let path = "../../standard.osu";
    /// # let beatmap = Beatmap::from_path(path).unwrap();
    /// let title = beatmap.version().unwrap();
    /// println!("{title}");
    /// ```
    ///
    /// # Errors
    /// See [`read_native_string()`]
    pub fn version(&self) -> Result<String, StringError> {
        read_native_string(self.handle, Beatmap_GetVersion)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::utils::initialize_path;
    use crate::{generate_beatmap_field_tests, generate_beatmap_method_tests};

    use super::Beatmap;
    generate_beatmap_field_tests! {
        approach_rate == 9.2,
        overall_difficulty == 8.3,
        drain_rate == 5.0,
        circle_size == 4.0,
        slider_multiplier == 2.0,
        slider_tick_rate == 1.0,
    }

    generate_beatmap_method_tests! {
        title() == "Toy Box",
        artist() == "John Grant",
        version() == "Expert",
    }
}

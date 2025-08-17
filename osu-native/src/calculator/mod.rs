use std::mem::MaybeUninit;

use libosu_native_sys::{ErrorCode, NativeBeatmapHandle, NativeRulesetHandle};

use crate::{
    beatmap::Beatmap,
    error::{NativeError, OsuError},
    mods::{GameModsError, IntoGameMods},
    ruleset::Ruleset,
    traits::{Native, NativeWrapper},
};

pub mod catch;
pub mod mania;
pub mod osu;
pub mod taiko;

type CreateFn<N> =
    unsafe extern "C" fn(NativeRulesetHandle, NativeBeatmapHandle, *mut N) -> ErrorCode;

pub trait DifficultyCalculator: Sized + NativeWrapper + From<(Self::Native, Ruleset)> {
    type Attributes;

    const CREATE: CreateFn<Self::Native>;

    fn calculate(&self) -> Result<Self::Attributes, OsuError>;

    fn mods(self, mods: impl IntoGameMods) -> Result<Self, GameModsError>;

    fn create(ruleset: Ruleset, beatmap: &Beatmap) -> Result<Self, NativeError> {
        let mut calculator = MaybeUninit::uninit();

        let code =
            unsafe { Self::CREATE(ruleset.handle(), beatmap.handle(), calculator.as_mut_ptr()) };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { calculator.assume_init() };

        Ok(Self::from((native, ruleset)))
    }
}

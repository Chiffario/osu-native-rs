use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, ManiaDifficultyCalculator_CalculateMods, ManiaDifficultyCalculator_Create,
    ManiaDifficultyCalculator_Destroy, NativeManiaDifficultyAttributes,
};

use crate::{
    beatmap::Beatmap,
    error::OsuError,
    mods::{
        GameMods, GameModsError, IntoGameMods,
        native::{Mod, ModCollection},
    },
    ruleset::Ruleset,
    utils::HasNative,
};

use super::DifficultyCalculator;

#[derive(PartialEq)]
pub struct ManiaDifficultyCalculator {
    handle: i32,
    ruleset: Ruleset,
    mods: GameMods,
}

impl Drop for ManiaDifficultyCalculator {
    fn drop(&mut self) {
        unsafe { ManiaDifficultyCalculator_Destroy(self.handle) };
    }
}

impl DifficultyCalculator for ManiaDifficultyCalculator {
    type Attributes = ManiaDifficultyAttributes;

    fn new(ruleset: Ruleset, beatmap: &Beatmap) -> Result<Self, OsuError> {
        let mut handle = 0;

        let code = unsafe {
            ManiaDifficultyCalculator_Create(ruleset.handle(), beatmap.handle(), &mut handle)
        };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        Ok(Self {
            handle,
            ruleset,
            mods: GameMods::default(),
        })
    }

    fn with_mods(mut self, mods: impl IntoGameMods) -> Result<Self, GameModsError> {
        self.mods = mods.into_mods()?;

        Ok(self)
    }

    fn calculate(&self) -> Result<Self::Attributes, OsuError> {
        let mods = ModCollection::new()?;

        let mods_vec = self
            .mods
            .0
            .iter()
            .map(|gamemod| {
                let m = Mod::new(gamemod.acronym.as_str())?;
                m.apply_settings(&gamemod.settings)?;

                Ok(m)
            })
            .collect::<Result<Vec<_>, OsuError>>()?;

        for gamemod in mods_vec.iter() {
            mods.add(gamemod)?;
        }

        let mut attributes = MaybeUninit::uninit();

        let code = unsafe {
            ManiaDifficultyCalculator_CalculateMods(
                self.handle,
                self.ruleset.handle(),
                mods.handle(),
                attributes.as_mut_ptr(),
            )
        };

        if code != ErrorCode::Success {
            return Err(code.into());
        }

        let native = unsafe { attributes.assume_init() };

        Ok(native.into())
    }
}

impl From<NativeManiaDifficultyAttributes> for ManiaDifficultyAttributes {
    fn from(value: NativeManiaDifficultyAttributes) -> Self {
        Self {
            star_rating: value.star_rating,
            max_combo: value.max_combo as usize,
        }
    }
}

pub struct ManiaDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: usize,
}

impl HasNative for ManiaDifficultyAttributes {
    type Native = NativeManiaDifficultyAttributes;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rosu_mods::{Acronym, GameModSimple};

    use crate::{
        beatmap::Beatmap,
        calculator::{DifficultyCalculator, mania::ManiaDifficultyCalculator},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_convert_mania() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Mania).unwrap();
        let calculator = ManiaDifficultyCalculator::new(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_ne!(attributes.star_rating, 0.0);
        assert_eq!(attributes.max_combo, 1463);
    }
    #[test]
    fn test_toy_box_mania_with_mods() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Mania).unwrap();
        let calculator = ManiaDifficultyCalculator::new(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        let ruleset = Ruleset::new(RulesetKind::Mania).unwrap();
        let mods: GameModSimple = GameModSimple {
            acronym: Acronym::from_str("DT").unwrap(),
            settings: Default::default(),
        };
        let calculator_with_mods = ManiaDifficultyCalculator::new(ruleset, &beatmap)
            .unwrap()
            .with_mods(vec![mods])
            .unwrap();
        let attributes_with_mods = calculator_with_mods.calculate().unwrap();

        assert!(attributes_with_mods.star_rating > attributes.star_rating);
        assert!(attributes_with_mods.max_combo == attributes.max_combo);
    }
}

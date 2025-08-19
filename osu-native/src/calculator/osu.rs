use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, NativeOsuDifficultyAttributes, OsuDifficultyCalculator_CalculateMods,
    OsuDifficultyCalculator_Create, OsuDifficultyCalculator_Destroy,
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
pub struct OsuDifficultyCalculator {
    handle: i32,
    ruleset: Ruleset,
    mods: GameMods,
}

impl Drop for OsuDifficultyCalculator {
    fn drop(&mut self) {
        unsafe { OsuDifficultyCalculator_Destroy(self.handle) };
    }
}

impl DifficultyCalculator for OsuDifficultyCalculator {
    type Attributes = OsuDifficultyAttributes;

    fn new(ruleset: Ruleset, beatmap: &Beatmap) -> Result<Self, OsuError> {
        let mut handle = 0;

        let code = unsafe {
            OsuDifficultyCalculator_Create(ruleset.handle(), beatmap.handle(), &mut handle)
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

    fn mods(mut self, mods: impl IntoGameMods) -> Result<Self, GameModsError> {
        self.mods = mods.into_mods()?;

        Ok(self)
    }

    fn calculate(&self) -> Result<Self::Attributes, OsuError> {
        let mod_collection = ModCollection::new()?;

        let mods = self
            .mods
            .0
            .iter()
            .map(|gamemod| {
                let m = Mod::new(gamemod.acronym.as_str())?;
                m.apply_settings(&gamemod.settings)?;

                Ok(m)
            })
            .collect::<Result<Vec<_>, OsuError>>()?;

        for gamemod in mods.iter() {
            mod_collection.add(gamemod)?;
        }

        let mut attributes = MaybeUninit::uninit();

        let code = unsafe {
            OsuDifficultyCalculator_CalculateMods(
                self.handle,
                self.ruleset.handle(),
                mod_collection.handle(),
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

#[derive(Debug, PartialEq)]
pub struct OsuDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: usize,
    pub aim_difficulty: f64,
    pub aim_difficulty_slider_count: f64,
    pub speed_difficulty: f64,
    pub speed_note_count: f64,
    pub flashlight_difficulty: f64,
    pub slider_factor: f64,
    pub aim_difficult_strain_count: f64,
    pub speed_difficult_strain_count: f64,
    pub drain_rate: f64,
    pub hit_circle_count: i32,
    pub slider_count: i32,
    pub spinner_count: i32,
}

impl HasNative for OsuDifficultyAttributes {
    type Native = NativeOsuDifficultyAttributes;
}

impl From<NativeOsuDifficultyAttributes> for OsuDifficultyAttributes {
    fn from(value: NativeOsuDifficultyAttributes) -> Self {
        Self {
            star_rating: value.star_rating,
            max_combo: value.max_combo as usize,
            aim_difficulty: value.aim_difficulty,
            aim_difficulty_slider_count: value.aim_difficulty_slider_count,
            speed_difficulty: value.speed_difficulty,
            speed_note_count: value.speed_note_count,
            flashlight_difficulty: value.flashlight_difficulty,
            slider_factor: value.slider_factor,
            aim_difficult_strain_count: value.aim_difficult_strain_count,
            speed_difficult_strain_count: value.speed_difficult_strain_count,
            drain_rate: value.drain_rate,
            hit_circle_count: value.hit_circle_count,
            slider_count: value.slider_count,
            spinner_count: value.spinner_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rosu_mods::{Acronym, GameModSimple};

    use super::OsuDifficultyCalculator;
    use crate::{
        beatmap::Beatmap,
        calculator::DifficultyCalculator,
        mods::native::{Mod, ModCollection},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_osu() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Osu).unwrap();
        let calculator = OsuDifficultyCalculator::new(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_ne!(attributes.star_rating, 0.0);
        assert_eq!(attributes.max_combo, 719);
        assert_eq!(attributes.flashlight_difficulty, 0.0);
        assert_ne!(attributes.aim_difficulty, 0.0);
        assert_ne!(attributes.aim_difficulty_slider_count, 0.0);
        assert_ne!(attributes.speed_difficulty, 0.0);
        assert_ne!(attributes.speed_note_count, 0.0);
        assert_ne!(attributes.slider_factor, 0.0);
        assert_ne!(attributes.aim_difficult_strain_count, 0.0);
        assert_ne!(attributes.speed_difficult_strain_count, 0.0);
        assert_ne!(attributes.hit_circle_count, 0);
        assert_ne!(attributes.slider_count, 0);
        assert_ne!(attributes.spinner_count, 0);
    }

    #[test]
    fn test_toy_box_osu_with_mods() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Osu).unwrap();
        let calculator = OsuDifficultyCalculator::new(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();

        let mods: GameModSimple = GameModSimple {
            acronym: Acronym::from_str("DT").unwrap(),
            settings: Default::default(),
        };
        let ruleset = Ruleset::new(RulesetKind::Osu).unwrap();
        let calculator_with_mods = OsuDifficultyCalculator::new(ruleset, &beatmap)
            .unwrap()
            .mods(vec![mods])
            .unwrap();
        let attributes_with_mods = calculator_with_mods.calculate().unwrap();

        assert!(attributes_with_mods.star_rating > attributes.star_rating);
        assert!(attributes_with_mods.max_combo == attributes.max_combo);
    }

    #[test]
    #[should_panic]
    fn test_calculator_ruleset_mismatch() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Taiko).unwrap();
        // Panics because of ruleset and calculator don't match
        let _ = OsuDifficultyCalculator::new(ruleset, &beatmap).unwrap();
    }
}

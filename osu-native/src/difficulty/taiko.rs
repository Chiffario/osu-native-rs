use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, NativeTaikoDifficultyAttributes, TaikoDifficultyCalculator_CalculateMods,
    TaikoDifficultyCalculator_Create, TaikoDifficultyCalculator_Destroy,
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
pub struct TaikoDifficultyCalculator {
    handle: i32,
    ruleset: Ruleset,
    mods: GameMods,
}

impl Drop for TaikoDifficultyCalculator {
    fn drop(&mut self) {
        unsafe { TaikoDifficultyCalculator_Destroy(self.handle) };
    }
}

impl DifficultyCalculator for TaikoDifficultyCalculator {
    type DifficultyAttributes = TaikoDifficultyAttributes;

    /// Create an instance of [`TaikoDifficultyCalculator`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use osu_native::{
    /// #     ruleset::{Ruleset, RulesetKind},
    /// #     beatmap::Beatmap,
    /// #     difficulty::{DifficultyCalculator, taiko::TaikoDifficultyCalculator},
    /// # };
    /// # let get_map = || "../../standard.osu";
    /// let beatmap = Beatmap::from_path(get_map())?;
    /// let ruleset = Ruleset::new(RulesetKind::Taiko)?;
    /// let taiko_calc = TaikoDifficultyCalculator::new(ruleset, &beatmap)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    ///
    /// Returns [`OsuError::NativeError`] if osu-native returns an error
    fn new(ruleset: Ruleset, beatmap: &Beatmap) -> Result<Self, OsuError> {
        let mut handle = 0;

        let code = unsafe {
            TaikoDifficultyCalculator_Create(ruleset.handle(), beatmap.handle(), &mut handle)
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

    fn mods(&self) -> GameMods {
        self.mods.clone()
    }

    fn with_mods(mut self, mods: impl IntoGameMods) -> Result<Self, GameModsError> {
        self.mods = mods.into_mods()?;

        Ok(self)
    }

    /// Calculates [`TaikoDifficultyAttributes`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use osu_native::{
    /// #     ruleset::{Ruleset, RulesetKind},
    /// #     beatmap::Beatmap,
    /// #     difficulty::{DifficultyCalculator, taiko::TaikoDifficultyCalculator},
    /// # };
    /// # let beatmap = Beatmap::from_path("../../standard.osu")?;
    /// # let ruleset = Ruleset::new(RulesetKind::Mania)?;
    /// let taiko_calc = TaikoDifficultyCalculator::new(ruleset, &beatmap)?;
    /// let attributes = taiko_calc.calculate()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    ///
    /// Returns [`OsuError::NativeError`] if osu-native returns an error
    fn calculate(&self) -> Result<Self::DifficultyAttributes, OsuError> {
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
            TaikoDifficultyCalculator_CalculateMods(
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

#[derive(Debug)]
pub struct TaikoDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: i32,
    pub rhythm_difficulty: f64,
    pub reading_difficulty: f64,
    pub colour_difficulty: f64,
    pub stamina_difficulty: f64,
    pub mono_stamina_factor: f64,
    pub rhythm_top_strains: f64,
    pub colour_top_strains: f64,
    pub stamina_top_strains: f64,
}

impl HasNative for TaikoDifficultyAttributes {
    type Native = NativeTaikoDifficultyAttributes;
}

impl From<NativeTaikoDifficultyAttributes> for TaikoDifficultyAttributes {
    fn from(value: NativeTaikoDifficultyAttributes) -> Self {
        Self {
            star_rating: value.star_rating,
            max_combo: value.max_combo,
            rhythm_difficulty: value.rhythm_difficulty,
            reading_difficulty: value.reading_difficulty,
            colour_difficulty: value.colour_difficulty,
            stamina_difficulty: value.stamina_difficulty,
            mono_stamina_factor: value.mono_stamina_factor,
            rhythm_top_strains: value.rhythm_top_strains,
            colour_top_strains: value.colour_top_strains,
            stamina_top_strains: value.stamina_top_strains,
        }
    }
}

impl From<&TaikoDifficultyAttributes> for NativeTaikoDifficultyAttributes {
    fn from(val: &TaikoDifficultyAttributes) -> Self {
        NativeTaikoDifficultyAttributes {
            star_rating: val.star_rating,
            max_combo: val.max_combo,
            rhythm_difficulty: val.rhythm_difficulty,
            reading_difficulty: val.reading_difficulty,
            colour_difficulty: val.colour_difficulty,
            stamina_difficulty: val.stamina_difficulty,
            mono_stamina_factor: val.mono_stamina_factor,
            rhythm_top_strains: val.rhythm_top_strains,
            colour_top_strains: val.colour_top_strains,
            stamina_top_strains: val.stamina_top_strains,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rosu_mods::{Acronym, GameModSimple};

    use crate::{
        beatmap::Beatmap,
        difficulty::{DifficultyCalculator, taiko::TaikoDifficultyCalculator},
        generate_diffcalc_field_tests,
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    generate_diffcalc_field_tests! {
        "taiko",
        star_rating!,
        max_combo == 709,
        rhythm_difficulty!,
        reading_difficulty!,
        colour_difficulty!,
        stamina_difficulty!,
        mono_stamina_factor!,
        rhythm_top_strains!,
        colour_top_strains!,
        stamina_top_strains!,
    }

    #[test]
    fn test_toy_box_taiko_with_mods() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::new(RulesetKind::Taiko).unwrap();
        let calculator = TaikoDifficultyCalculator::new(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();

        let mods: GameModSimple = GameModSimple {
            acronym: Acronym::from_str("DT").unwrap(),
            settings: Default::default(),
        };
        let ruleset = Ruleset::new(RulesetKind::Taiko).unwrap();
        let calculator_with_mods = TaikoDifficultyCalculator::new(ruleset, &beatmap)
            .unwrap()
            .with_mods(vec![mods])
            .unwrap();
        let attributes_with_mods = calculator_with_mods.calculate().unwrap();

        assert!(attributes_with_mods.star_rating > attributes.star_rating);
        assert!(attributes_with_mods.max_combo == attributes.max_combo);
    }
}

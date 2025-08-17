use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, NativeTaikoDifficultyAttributes, NativeTaikoDifficultyCalculator,
    NativeTaikoDifficultyCalculatorHandle, TaikoDifficultyCalculator_CalculateMods,
    TaikoDifficultyCalculator_Create, TaikoDifficultyCalculator_Destroy,
};

use crate::{
    calculator::CreateFn,
    error::OsuError,
    mods::{
        GameMods, GameModsError, IntoGameMods,
        native::{Mod, ModCollection},
    },
    ruleset::Ruleset,
    traits::Native,
};

use super::DifficultyCalculator;

declare_native_wrapper! {
    #[derive(Debug, PartialEq)]
    pub struct TaikoDifficultyCalculator {
        native: NativeTaikoDifficultyCalculator,
        ruleset: Ruleset,
        mods: GameMods,
    }
}

impl From<(NativeTaikoDifficultyCalculator, Ruleset)> for TaikoDifficultyCalculator {
    fn from((native, ruleset): (NativeTaikoDifficultyCalculator, Ruleset)) -> Self {
        Self {
            native,
            ruleset,
            mods: GameMods::default(),
        }
    }
}

impl_native!(
    NativeTaikoDifficultyCalculator:
        NativeTaikoDifficultyCalculatorHandle, TaikoDifficultyCalculator_Destroy
);

impl DifficultyCalculator for TaikoDifficultyCalculator {
    type Attributes = TaikoDifficultyAttributes;

    const CREATE: CreateFn<Self::Native> = TaikoDifficultyCalculator_Create;

    fn mods(mut self, mods: impl IntoGameMods) -> Result<Self, GameModsError> {
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

pub struct TaikoDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: usize,
    pub rhythm_difficulty: f64,
    pub reading_difficulty: f64,
    pub colour_difficulty: f64,
    pub stamina_difficulty: f64,
    pub mono_stamina_factor: f64,
    pub rhythm_top_strains: f64,
    pub colour_top_strains: f64,
    pub stamina_top_strains: f64,
}

impl From<NativeTaikoDifficultyAttributes> for TaikoDifficultyAttributes {
    fn from(value: NativeTaikoDifficultyAttributes) -> Self {
        Self {
            star_rating: value.star_rating,
            max_combo: value.max_combo as usize,
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rosu_mods::{Acronym, GameModSimple};

    use crate::{
        beatmap::Beatmap,
        calculator::{DifficultyCalculator, taiko::TaikoDifficultyCalculator},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_convert_taiko() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::from_kind(RulesetKind::Taiko).unwrap();
        let calculator = TaikoDifficultyCalculator::create(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_ne!(attributes.star_rating, 0.0);
        assert_eq!(attributes.max_combo, 709);
        // assert_eq!(attributes.rhythm_difficulty, 0.6085760732532105);
        // assert_eq!(attributes.reading_difficulty, 0.0);
        // assert_eq!(attributes.colour_difficulty, 0.0);
        // assert_eq!(attributes.stamina_difficulty, 0.0);
        assert_ne!(attributes.mono_stamina_factor, 0.0);
        // assert_eq!(attributes.rhythm_top_strains, 0.0);
        // assert_eq!(attributes.colour_top_strains, 0.0);
        // assert_eq!(attributes.stamina_top_strains, 0.0);
    }
    #[test]
    fn test_toy_box_taiko_with_mods() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::from_kind(RulesetKind::Taiko).unwrap();
        let calculator = TaikoDifficultyCalculator::create(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();

        let mods: GameModSimple = GameModSimple {
            acronym: Acronym::from_str("DT").unwrap(),
            settings: Default::default(),
        };
        let ruleset = Ruleset::from_kind(RulesetKind::Taiko).unwrap();
        let calculator_with_mods = TaikoDifficultyCalculator::create(ruleset, &beatmap)
            .unwrap()
            .mods(vec![mods])
            .unwrap();
        let attributes_with_mods = calculator_with_mods.calculate().unwrap();

        assert!(attributes_with_mods.star_rating > attributes.star_rating);
        assert!(attributes_with_mods.max_combo == attributes.max_combo);
    }
}

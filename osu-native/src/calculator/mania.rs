use std::mem::MaybeUninit;

use libosu_native_sys::{
    ErrorCode, ManiaDifficultyCalculator_CalculateMods, ManiaDifficultyCalculator_Create,
    ManiaDifficultyCalculator_Destroy, NativeManiaDifficultyAttributes,
    NativeManiaDifficultyCalculator, NativeManiaDifficultyCalculatorHandle,
};

use crate::{mods::GameMods, ruleset::Ruleset, traits::Native};

impl_native!(
    NativeManiaDifficultyCalculator:
        NativeManiaDifficultyCalculatorHandle, ManiaDifficultyCalculator_Destroy
);

declare_native_wrapper! {
    #[derive(Debug, PartialEq)]
    pub struct ManiaDifficultyCalculator {
        native: NativeManiaDifficultyCalculator,
        ruleset: Ruleset,
        mods: GameMods,
    }
}

impl_calculator! {
    ManiaDifficultyCalculator {
        attributes: ManiaDifficultyAttributes,
        handle: NativeManiaDifficultyCalculatorHandle,
        create: ManiaDifficultyCalculator_Create,
        calculate: ManiaDifficultyCalculator_CalculateMods,
    }
}

pub struct ManiaDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: usize,
}

impl From<NativeManiaDifficultyAttributes> for ManiaDifficultyAttributes {
    fn from(value: NativeManiaDifficultyAttributes) -> Self {
        Self {
            star_rating: value.star_rating,
            max_combo: value.max_combo as usize,
        }
    }
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
        let ruleset = Ruleset::from_kind(RulesetKind::Mania).unwrap();
        let calculator = ManiaDifficultyCalculator::create(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_ne!(attributes.star_rating, 0.0);
        assert_eq!(attributes.max_combo, 1463);
    }

    #[test]
    fn test_toy_box_mania_with_mods() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::from_kind(RulesetKind::Mania).unwrap();
        let calculator = ManiaDifficultyCalculator::create(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        let ruleset = Ruleset::from_kind(RulesetKind::Mania).unwrap();
        let mods: GameModSimple = GameModSimple {
            acronym: Acronym::from_str("DT").unwrap(),
            settings: Default::default(),
        };
        let calculator_with_mods = ManiaDifficultyCalculator::create(ruleset, &beatmap)
            .unwrap()
            .mods(vec![mods])
            .unwrap();
        let attributes_with_mods = calculator_with_mods.calculate().unwrap();

        assert!(attributes_with_mods.star_rating > attributes.star_rating);
        assert!(attributes_with_mods.max_combo == attributes.max_combo);
    }
}

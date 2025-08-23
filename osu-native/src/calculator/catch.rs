use libosu_native_sys::{
    CatchDifficultyCalculator_CalculateMods, CatchDifficultyCalculator_Create,
    CatchDifficultyCalculator_Destroy, NativeCatchDifficultyAttributes,
    NativeCatchDifficultyCalculator, NativeCatchDifficultyCalculatorHandle,
};

use crate::{mods::GameMods, ruleset::Ruleset, traits::Native};

impl_native!(
    NativeCatchDifficultyCalculator:
        NativeCatchDifficultyCalculatorHandle, CatchDifficultyCalculator_Destroy
);

declare_native_wrapper! {
    #[derive(Debug, PartialEq)]
    pub struct CatchDifficultyCalculator {
        native: NativeCatchDifficultyCalculator,
        ruleset: Ruleset,
        mods: GameMods,
    }
}

impl_calculator! {
    CatchDifficultyCalculator {
        attributes: CatchDifficultyAttributes,
        handle: NativeCatchDifficultyCalculatorHandle,
        create: CatchDifficultyCalculator_Create,
        calculate: CatchDifficultyCalculator_CalculateMods,
    }
}

pub struct CatchDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: usize,
}

impl From<NativeCatchDifficultyAttributes> for CatchDifficultyAttributes {
    fn from(value: NativeCatchDifficultyAttributes) -> Self {
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
        calculator::{DifficultyCalculator, catch::CatchDifficultyCalculator},
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_convert_catch() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::from_kind(RulesetKind::Catch).unwrap();
        let calculator = CatchDifficultyCalculator::create(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();
        assert_ne!(attributes.star_rating, 0.0);
        assert_eq!(attributes.max_combo, 717);
    }

    #[test]
    fn test_toy_box_catch_with_mods() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::from_kind(RulesetKind::Catch).unwrap();
        let calculator = CatchDifficultyCalculator::create(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();

        let mods: GameModSimple = GameModSimple {
            acronym: Acronym::from_str("DT").unwrap(),
            settings: Default::default(),
        };
        let ruleset = Ruleset::from_kind(RulesetKind::Catch).unwrap();
        let calculator_with_mods = CatchDifficultyCalculator::create(ruleset, &beatmap)
            .unwrap()
            .mods(vec![mods])
            .unwrap();
        let attributes_with_mods = calculator_with_mods.calculate().unwrap();

        assert!(attributes_with_mods.star_rating > attributes.star_rating);
        assert!(attributes_with_mods.max_combo == attributes.max_combo);
    }
}

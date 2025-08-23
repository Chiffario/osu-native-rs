use libosu_native_sys::{
    NativeOsuDifficultyAttributes, NativeOsuDifficultyCalculator,
    NativeOsuDifficultyCalculatorHandle, OsuDifficultyCalculator_CalculateMods,
    OsuDifficultyCalculator_Create, OsuDifficultyCalculator_Destroy,
};

use crate::{mods::GameMods, ruleset::Ruleset, traits::Native};

impl_native!(
    NativeOsuDifficultyCalculator:
        NativeOsuDifficultyCalculatorHandle, OsuDifficultyCalculator_Destroy
);

declare_native_wrapper! {
    #[derive(Debug, PartialEq)]
    pub struct OsuDifficultyCalculator {
        native: NativeOsuDifficultyCalculator,
        ruleset: Ruleset,
        mods: GameMods,
    }
}

impl_calculator! {
    OsuDifficultyCalculator {
        attributes: OsuDifficultyAttributes,
        handle: NativeOsuDifficultyCalculatorHandle,
        create: OsuDifficultyCalculator_Create,
        calculate: OsuDifficultyCalculator_CalculateMods,
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
        ruleset::{Ruleset, RulesetKind},
        utils::initialize_path,
    };

    #[test]
    fn test_toy_box_osu() {
        let beatmap = Beatmap::from_path(initialize_path()).unwrap();
        let ruleset = Ruleset::from_kind(RulesetKind::Osu).unwrap();
        let calculator = OsuDifficultyCalculator::create(ruleset, &beatmap).unwrap();
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
        let ruleset = Ruleset::from_kind(RulesetKind::Osu).unwrap();
        let calculator = OsuDifficultyCalculator::create(ruleset, &beatmap).unwrap();
        let attributes = calculator.calculate().unwrap();

        let mods: GameModSimple = GameModSimple {
            acronym: Acronym::from_str("DT").unwrap(),
            settings: Default::default(),
        };
        let ruleset = Ruleset::from_kind(RulesetKind::Osu).unwrap();
        let calculator_with_mods = OsuDifficultyCalculator::create(ruleset, &beatmap)
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
        let ruleset = Ruleset::from_kind(RulesetKind::Taiko).unwrap();
        // Panics because of ruleset and calculator don't match
        let _ = OsuDifficultyCalculator::create(ruleset, &beatmap).unwrap();
    }
}

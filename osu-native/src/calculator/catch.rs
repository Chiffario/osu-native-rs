use std::mem::MaybeUninit;

use libosu_native_sys::{
    CatchDifficultyCalculator_CalculateMods, CatchDifficultyCalculator_Create,
    CatchDifficultyCalculator_Destroy, ErrorCode, NativeCatchDifficultyAttributes,
    NativeCatchDifficultyCalculator, NativeCatchDifficultyCalculatorHandle,
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
    pub struct CatchDifficultyCalculator {
        native: NativeCatchDifficultyCalculator,
        ruleset: Ruleset,
        mods: GameMods,
    }
}

impl From<(NativeCatchDifficultyCalculator, Ruleset)> for CatchDifficultyCalculator {
    fn from((native, ruleset): (NativeCatchDifficultyCalculator, Ruleset)) -> Self {
        Self {
            native,
            ruleset,
            mods: GameMods::default(),
        }
    }
}

impl_native!(
    NativeCatchDifficultyCalculator:
        NativeCatchDifficultyCalculatorHandle, CatchDifficultyCalculator_Destroy
);

impl DifficultyCalculator for CatchDifficultyCalculator {
    type Attributes = CatchDifficultyAttributes;

    const CREATE: CreateFn<Self::Native> = CatchDifficultyCalculator_Create;

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
            CatchDifficultyCalculator_CalculateMods(
                self.handle(),
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

impl From<NativeCatchDifficultyAttributes> for CatchDifficultyAttributes {
    fn from(value: NativeCatchDifficultyAttributes) -> Self {
        Self {
            star_rating: value.star_rating,
            max_combo: value.max_combo as usize,
        }
    }
}

pub struct CatchDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: usize,
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

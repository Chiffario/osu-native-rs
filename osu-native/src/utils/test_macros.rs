#[macro_export]
#[cfg(test)]
macro_rules! generate_ruleset_tests {
    ($variant:ident, $expected_name:expr) => {
        pastey::paste! {
            #[test]
            fn [<test_create_ $variant:lower>]() {
                let ruleset = Ruleset::new(RulesetKind::$variant).unwrap();
                assert_eq!(ruleset.kind, RulesetKind::$variant);
            }

            #[test]
            fn [<test_get_name_ $variant:lower>]() {
                let ruleset = Ruleset::new(RulesetKind::$variant).unwrap();
                assert_eq!(ruleset.short_name().unwrap(), $expected_name);
            }
        }
    };
}

#[macro_export]
#[cfg(test)]
macro_rules! generate_beatmap_field_tests {
    ($($field:ident == $expected:expr),* $(,)?) => {
        $(
            pastey::paste! {
                #[test]
                fn [<test_beatmap_ $field _equals_expected>]() {
                    let beatmap = Beatmap::from_path(initialize_path()).unwrap();
                    assert_eq!(beatmap.$field, $expected);
                }
            }
        )*
    };
}

#[macro_export]
#[cfg(test)]
macro_rules! generate_beatmap_method_tests {
    ($($method:ident() == $expected:expr),* $(,)?) => {
        $(
            pastey::paste! {
                #[test]
                fn [<test_beatmap_ $method _equals_expected>]() {
                    let beatmap = Beatmap::from_path(initialize_path()).unwrap();
                    assert_eq!(beatmap.$method().unwrap(), $expected);
                }
            }
        )*
    };
}
#[macro_export]
#[cfg(test)]
macro_rules! generate_diffcalc_field_tests {
    ($name:expr, $($field:ident $op:tt $($expected:expr)?),* $(,)?) => {
        $(
            pastey::paste! {
                generate_diffcalc_field_tests!(@rename $name, $field, $op, {
                    let beatmap = Beatmap::from_path(initialize_path()).unwrap();
                    let ruleset = Ruleset::new(RulesetKind::[<$name:camel>]).unwrap();
                    let calculator = [<$name:camel DifficultyCalculator>]::new(ruleset, &beatmap).unwrap();
                    let attributes = calculator.calculate().unwrap();
                    generate_diffcalc_field_tests!(@assert attributes.$field, $op $(, $expected)?);
                });
            }
        )*
    };

    // Internal rule for exact equality
    (@assert $field_access:expr, ==, $expected:expr) => {
        assert_eq!($field_access, $expected);
    };

    // Internal rule for approximate equality (floats with epsilon)
    (@assert $field_access:expr, ~=, $expected:expr) => {
        assert!(($field_access - $expected).abs() < f64::EPSILON.max(f32::EPSILON as f64),
               "Expected {} to be approximately {}, but got {}",
               stringify!($field_access), $expected, $field_access);
    };

    // Internal rule for non-zero check
    (@assert $field_access:expr, !) => {
        assert_ne!($field_access, 0.0,
                  "Expected {} to be non-zero, but got {}",
                  stringify!($field_access), $field_access);
    };

    // Internal rule for checking options
    (@assert $field_access:expr, ?) => {
        assert!($field_access.is_some(),
                  "Expected {} to be some, but got {}",
                  stringify!($field_access), $field_access);
    };

    // Internal rule for renaming non-zero tests
    (@rename $name:expr, $field:ident, !, $block:block) => {
        pastey::paste! {
            #[test]
            fn [<test_beatmap_ $name _ $field _is_not_zero>]()
                $block
        }
    };

    // Internal rule for renaming equality tests
    (@rename $name:expr, $field:ident, == , $block:block) => {
        pastey::paste! {
            #[test]
            fn [<test_beatmap_ $name _ $field _equals_expected>]()
                $block
        }
    }
}

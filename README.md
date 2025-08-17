# osu-native-rs

Provides native bindings for [osu-native](https://github.com/minisbett/osu-native-new)

NOTE: Pre-pre-alpha, osu-native itself isn't stable and neither is this crate. Until osu-native is stable, this crate will NOT be correctly versioned, as I really can't bother doing that before I have some stability guarantees from the main dependency

# Usage

```rust
let beatmap = Beatmap::new_from_path("/path/to/map")?;
let title = beatmap.get_title()?;
println!("{title}");

let ruleset = Ruleset::from_kind_from_variant(Rulesets::Standard)?;
let short_name = ruleset.get_short_name()?;
println!("{short_name}")

let calculator = OsuDifficultyCalculator::new(ruleset, beatmap)?;
let attributes = calculator.calculate()?;
println!("{}", attributes.star_rating);
```

# Testing

All public APIs have standard test coverage, `cargo test` and `cargo nextest run` are your friends 

# osu-native-rs

Provides native bindings for [osu-native](https://github.com/minisbett/osu-native-new)

# Usage

```rust
let beatmap = Beatmap::new_from_path("/path/to/map")?;
let title = beatmap.get_title()?;
println!("{title}");

let ruleset = Ruleset::new_from_variant(Rulesets::Standard)?;
let short_name = ruleset.get_short_name()?;
println!("{short_name}")
```

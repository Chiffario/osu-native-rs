# osu-native-rs

Provides native bindings for [osu-native](https://github.com/minisbett/osu-native-new)

NOTE: Pre-pre-alpha, osu-native itself isn't stable and neither is this crate. Until osu-native is stable, this crate will NOT be correctly versioned, as I really can't bother doing that before I have some stability guarantees from the main dependency

# Completeness

Note: This is *raw* completeness, aka API coverage of osu-native. Not "idiomatic" coverage, *plain* coverage

## Beatmap 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| Beatmap_CreateFromFile                  |      y     |
| Beatmap_CreateFromText                  |      n     |
| Beatmap_Destroy                         |      y     |
| Beatmap_GetTitle                        |      y     |
| Beatmap_GetArtist                       |      y     |
| Beatmap_GetVersion                      |      y     |
| Beatmap_Destroy                         |      y     |

## Ruleset

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| Ruleset_CreateFromId                    |      y     |
| Ruleset_CreateFromShortName             |      y     |
| Ruleset_GetShortName                    |      y     |
| Ruleset_Destroy                         |      y     |

## Mod

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| Mod_Create                              |      y     |
| Mod_SetSetting                          |      y     |
| Mod_Debug                               |      n     |
| Mod_Destroy                             |      y     |

## ModsCollection

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| ModsCollection_Create                   |      y     |
| ModsCollection_Add                      |      y     |
| ModsCollection_Remove                   |      n     |
| ModsCollection_Destroy                  |      y     | 

## Difficulty calculators

### OsuDifficultyCalculator

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| OsuDifficultyCalculator_Create          |      y     |
| OsuDifficultyCalculator_Calculate       |      ?     |
| OsuDifficultyCalculator_CalculateMods   |      y     |
| OsuDifficultyCalculator_Destroy         |      y     |

### TaikoDifficultyCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| TaikoDifficultyCalculator_Create        |      y     |
| TaikoDifficultyCalculator_Calculate     |      ?     |
| TaikoDifficultyCalculator_CalculateMods |      y     |
| TaikoDifficultyCalculator_Destroy       |      y     |

### ManiaDifficultyCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| ManiaDifficultyCalculator_Create        |      y     |
| ManiaDifficultyCalculator_Calculate     |      ?     |
| ManiaDifficultyCalculator_CalculateMods |      y     |
| ManiaDifficultyCalculator_Destroy       |      y     |

### CatchDifficultyCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| CatchDifficultyCalculator_Create        |      y     |
| CatchDifficultyCalculator_Calculate     |      ?     |
| CatchDifficultyCalculator_CalculateMods |      y     |
| CatchDifficultyCalculator_Destroy       |      y     |

## Performance calculators

### OsuCalculatorCalculator

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| OsuCalculatorCalculator_Create          |      y     |
| OsuCalculatorCalculator_Calculate       |      y     |
| OsuCalculatorCalculator_Destroy         |      y     |

### TaikoCalculatorCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| TaikoCalculatorCalculator_Create        |      y     |
| TaikoCalculatorCalculator_Calculate     |      y     |
| TaikoCalculatorCalculator_Destroy       |      y     |

### ManiaCalculatorCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| ManiaCalculatorCalculator_Create        |      y     |
| ManiaCalculatorCalculator_Calculate     |      y     |
| ManiaCalculatorCalculator_Destroy       |      y     |

### CatchCalculatorCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| CatchCalculatorCalculator_Create        |      y     |
| CatchCalculatorCalculator_Calculate     |      y     |
| CatchCalculatorCalculator_Destroy       |      y     |

# Testing

All public APIs have standard test coverage, `cargo test` and `cargo nextest run` are your friends 

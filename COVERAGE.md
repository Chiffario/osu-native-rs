# Completeness

Note: This is *raw* completeness, aka API coverage of osu-native. Not "idiomatic" coverage, *plain* coverage

Current compliance:

lazer: 2025.1029.0
osu-native: 37dbd06

## Beatmap 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| Beatmap_CreateFromFile                  |      +     |
| Beatmap_CreateFromText                  |      +     |
| Beatmap_Destroy                         |      +     |
| Beatmap_GetTitle                        |      +     |
| Beatmap_GetArtist                       |      +     |
| Beatmap_GetVersion                      |      +     |
| Beatmap_Destroy                         |      +     |

## Ruleset

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| Ruleset_CreateFromId                    |      +     |
| Ruleset_CreateFromShortName             |      +     |
| Ruleset_GetShortName                    |      +     |
| Ruleset_Destroy                         |      +     |

## Mod

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| Mod_Create                              |      +     |
| Mod_SetSetting                          |      +     |
| Mod_Debug                               |      -     |
| Mod_Destroy                             |      +     |

## ModsCollection

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| ModsCollection_Create                   |      +     |
| ModsCollection_Add                      |      +     |
| ModsCollection_Remove                   |      -     |
| ModsCollection_Destroy                  |      +     | 

## Difficulty calculators

### OsuDifficultyCalculator

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| OsuDifficultyCalculator_Create          |      +     |
| OsuDifficultyCalculator_Calculate       |      ?     |
| OsuDifficultyCalculator_CalculateMods   |      +     |
| OsuDifficultyCalculator_Destroy         |      +     |

### TaikoDifficultyCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| TaikoDifficultyCalculator_Create        |      +     |
| TaikoDifficultyCalculator_Calculate     |      ?     |
| TaikoDifficultyCalculator_CalculateMods |      +     |
| TaikoDifficultyCalculator_Destroy       |      +     |

### ManiaDifficultyCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| ManiaDifficultyCalculator_Create        |      +     |
| ManiaDifficultyCalculator_Calculate     |      ?     |
| ManiaDifficultyCalculator_CalculateMods |      +     |
| ManiaDifficultyCalculator_Destroy       |      +     |

### CatchDifficultyCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| CatchDifficultyCalculator_Create        |      +     |
| CatchDifficultyCalculator_Calculate     |      ?     |
| CatchDifficultyCalculator_CalculateMods |      +     |
| CatchDifficultyCalculator_Destroy       |      +     |

## Performance calculators

### OsuCalculatorCalculator

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| OsuCalculatorCalculator_Create          |      +     |
| OsuCalculatorCalculator_Calculate       |      +     |
| OsuCalculatorCalculator_Destroy         |      +     |

### TaikoCalculatorCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| TaikoCalculatorCalculator_Create        |      +     |
| TaikoCalculatorCalculator_Calculate     |      +     |
| TaikoCalculatorCalculator_Destroy       |      +     |

### ManiaCalculatorCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| ManiaCalculatorCalculator_Create        |      +     |
| ManiaCalculatorCalculator_Calculate     |      +     |
| ManiaCalculatorCalculator_Destroy       |      +     |

### CatchCalculatorCalculator 

| Native function                         | Is wrapped |
| --------------------------------------- | ---------- |
| CatchCalculatorCalculator_Create        |      +     |
| CatchCalculatorCalculator_Calculate     |      +     |
| CatchCalculatorCalculator_Destroy       |      +     |

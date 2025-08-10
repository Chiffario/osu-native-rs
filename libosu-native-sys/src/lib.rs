#![allow(dead_code)]
#[repr(C)]
struct NativeOsuDifficultyAttributes {
    star_rating: f64,
    max_combo: f64,
    aim_difficulty: f64,
    aim_difficulty_slider_count: f64,
    speed_difficulty: f64,
    speed_note_count: f64,
    flashlight_difficulty: f64,
    slider_factor: f64,
    aim_difficult_strain_count: f64,
    speed_difficult_strain_count: f64,
    drain_rate: f64,
    hit_circle_count: i32,
    slider_count: i32,
    spinner_count: i32,
}

#[repr(C)]
struct NativeManiaDifficultyAttributes {
    star_rating: f64,
    max_combo: f64,
}

#[repr(C)]
struct NativeTaikoDifficultyAttributes {
    star_rating: f64,
    max_combo: f64,
    rhythm_difficulty: f64,
    reading_difficulty: f64,
    colour_difficulty: f64,
    stamina_difficulty: f64,
    mono_stamina_factor: f64,
    rhythm_top_strains: f64,
    colour_top_strains: f64,
    stamina_top_strains: f64,
}

#[repr(C)]
struct NativeCatchDifficultyAttributes {
    star_rating: f64,
    max_combo: f64,
}

#[repr(C)]
struct NativeScoreHitStatistics {
    miss: i32,
    meh: i32,
    ok: i32,
    good: i32,
    great: i32,
    perfect: i32,
    slider_tail_hit: i32,
    large_tick_miss: i32,
}

#[repr(i8)]
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum ErrorCode {
    BufferSizeQuery = -1,
    Success = 0,
    ObjectNotFound = 1,
    RulesetUnavailable = 2,
    BeatmapFileNotFound = 3,
    Failure = 127,
}

pub type NativeModHandle = i32;
pub type NativeModCollectionHandle = i32;
pub type NativeBeatmapHandle = i32;
pub type NativeRulesetHandle = i32;
pub type NativeOsuDifficultyCalculatorHandle = i32;
pub type NativeTaikoDifficultyCalculatorHandle = i32;
pub type NativeManiaDifficultyCalculatorHandle = i32;
pub type NativeCatchDifficultyCalculatorHandle = i32;
#[repr(C)]
struct NativeRuleset {
    handle: NativeRulesetHandle,
    id: i32,
}
#[repr(C)]
struct NativeBeatmap {
    handle: NativeBeatmapHandle,
    approach_rate: f32,
    drain_rate: f32,
    overall_difficulty: f32,
    slider_multiplier: f64,
    slider_tick_rate: f64,
}
#[link(name = "osu.Native.so", modifiers = "+verbatim")]
unsafe extern "C" {
    // Mods
    fn Mod_Create(acronym: *const i8, mod_handle_ptr: *mut NativeModHandle) -> ErrorCode;
    fn Mod_SetSetting(mod_handle: NativeModHandle, key: *const i8, value: f64) -> ErrorCode;
    fn Mod_Debug(mod_handle: NativeModHandle) -> ErrorCode;
    fn Mod_Destroy(mod_handle: NativeModHandle) -> ErrorCode;
    // Mod collections
    fn ModsCollection_Create(mod_collection_ptr: *mut NativeModCollectionHandle) -> ErrorCode;
    fn ModsCollection_Add(
        mod_collection_handle: NativeModCollectionHandle,
        mod_handle: NativeModHandle,
    ) -> ErrorCode;
    fn ModsCollection_Remove(
        mod_collection_handle: NativeModCollectionHandle,
        mod_handle: NativeModHandle,
    ) -> ErrorCode;
    fn ModsCollection_Destroy(mod_collection_handle: NativeModCollectionHandle) -> ErrorCode;
    // Rulesets
    fn Ruleset_CreateFromId(ruleset_id: i32, ruleset_handle_ptr: *mut NativeRuleset) -> ErrorCode;
    fn Ruleset_CreateFromShortName(
        short_name: *const i8,
        ruleset_handle_ptr: *mut NativeRuleset,
    ) -> ErrorCode;
    fn Ruleset_GetShortName(
        ruleset_handle: NativeRulesetHandle,
        buffer: *mut u8,
        size: *mut i32,
    ) -> ErrorCode;
    fn Ruleset_Destroy(ruleset_handle: NativeRulesetHandle) -> ErrorCode;
    // Beatmaps
    fn Beatmap_CreateFromFile(path: *const i8, beatmap_ptr: *mut NativeBeatmap) -> ErrorCode;
    fn Beatmap_CreateFromText(text: *const i8, beatmap_ptr: *mut NativeBeatmap) -> ErrorCode;
    fn Beatmap_GetTitle(
        beatmap_handle: NativeBeatmapHandle,
        buffer: *mut i8,
        size: *mut i32,
    ) -> ErrorCode;
    fn Beatmap_GetArtist(
        beatmap_handle: NativeBeatmapHandle,
        buffer: *mut i8,
        size: *mut i32,
    ) -> ErrorCode;
    fn Beatmap_GetVersion(
        beatmap_handle: NativeBeatmapHandle,
        buffer: *mut i8,
        size: *mut i32,
    ) -> ErrorCode;
    fn Beatmap_Destroy(beatmap_handle: NativeBeatmapHandle) -> ErrorCode;
    /// Difficulty Calculator Objects (CDO)
    // ODCO
    fn OsuDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeOsuDifficultyCalculatorHandle,
    ) -> ErrorCode;
    fn OsuDifficultyCalculator_Calculate(
        calculator_handle: NativeOsuDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeOsuDifficultyAttributes,
    ) -> ErrorCode;
    fn OsuDifficultyCalculator_CalculateMods(
        calculator_handle: NativeOsuDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mods_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeOsuDifficultyAttributes,
    ) -> ErrorCode;
    fn OsuDifficultyCalculator_Destroy(calculator_handle: NativeOsuDifficultyCalculatorHandle);
    // TDCO
    fn TaikoDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeTaikoDifficultyCalculatorHandle,
    ) -> ErrorCode;
    fn TaikoDifficultyCalculator_Calculate(
        calculator_handle: NativeTaikoDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeTaikoDifficultyAttributes,
    ) -> ErrorCode;
    fn TaikoDifficultyCalculator_CalculateMods(
        calculator_handle: NativeTaikoDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mods_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeTaikoDifficultyAttributes,
    ) -> ErrorCode;
    fn TaikoDifficultyCalculator_Destroy(calculator_handle: NativeTaikoDifficultyCalculatorHandle);
    // MDCO
    fn ManiaDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeManiaDifficultyCalculatorHandle,
    ) -> ErrorCode;
    fn ManiaDifficultyCalculator_Calculate(
        calculator_handle: NativeManiaDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeManiaDifficultyAttributes,
    ) -> ErrorCode;
    fn ManiaDifficultyCalculator_CalculateMods(
        calculator_handle: NativeManiaDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mods_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeManiaDifficultyAttributes,
    ) -> ErrorCode;
    fn ManiaDifficultyCalculator_Destroy(calculator_handle: NativeManiaDifficultyCalculatorHandle);
    // CDCO
    fn CatchDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeCatchDifficultyCalculatorHandle,
    ) -> ErrorCode;
    fn CatchDifficultyCalculator_Calculate(
        calculator_handle: NativeCatchDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeCatchDifficultyAttributes,
    ) -> ErrorCode;
    fn CatchDifficultyCalculator_CalculateMods(
        calculator_handle: NativeCatchDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mods_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeCatchDifficultyAttributes,
    ) -> ErrorCode;
    fn CatchDifficultyCalculator_Destroy(calculator_handle: NativeCatchDifficultyCalculatorHandle);

}

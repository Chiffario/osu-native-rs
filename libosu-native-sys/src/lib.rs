#![allow(dead_code)]
#[repr(C)]
pub struct NativeOsuDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: f64,
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

#[repr(C)]
pub struct NativeManiaDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: f64,
}

#[repr(C)]
pub struct NativeTaikoDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: f64,
    pub rhythm_difficulty: f64,
    pub reading_difficulty: f64,
    pub colour_difficulty: f64,
    pub stamina_difficulty: f64,
    pub mono_stamina_factor: f64,
    pub rhythm_top_strains: f64,
    pub colour_top_strains: f64,
    pub stamina_top_strains: f64,
}

#[repr(C)]
pub struct NativeCatchDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: f64,
}

#[repr(C)]
pub struct NativeScoreHitStatistics {
    pub miss: i32,
    pub meh: i32,
    pub ok: i32,
    pub good: i32,
    pub great: i32,
    pub perfect: i32,
    pub slider_tail_hit: i32,
    pub large_tick_miss: i32,
}

#[repr(i8)]
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ErrorCode {
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
pub struct NativeRuleset {
    pub handle: NativeRulesetHandle,
    pub id: i32,
}
#[repr(C)]
pub struct NativeBeatmap {
    pub handle: NativeBeatmapHandle,
    pub approach_rate: f32,
    pub drain_rate: f32,
    pub overall_difficulty: f32,
    pub circle_size: f32,
    pub slider_multiplier: f64,
    pub slider_tick_rate: f64,
}
#[link(name = "osu.Native.so", modifiers = "+verbatim")]
unsafe extern "C" {
    // Mods
    pub fn Mod_Create(acronym: *const i8, mod_handle_ptr: *mut NativeModHandle) -> ErrorCode;
    pub fn Mod_SetSetting(mod_handle: NativeModHandle, key: *const i8, value: f64) -> ErrorCode;
    pub fn Mod_Debug(mod_handle: NativeModHandle) -> ErrorCode;
    pub fn Mod_Destroy(mod_handle: NativeModHandle) -> ErrorCode;
    // Mod collections
    pub fn ModsCollection_Create(mod_collection_ptr: *mut NativeModCollectionHandle) -> ErrorCode;
    pub fn ModsCollection_Add(
        mod_collection_handle: NativeModCollectionHandle,
        mod_handle: NativeModHandle,
    ) -> ErrorCode;
    pub fn ModsCollection_Remove(
        mod_collection_handle: NativeModCollectionHandle,
        mod_handle: NativeModHandle,
    ) -> ErrorCode;
    pub fn ModsCollection_Destroy(mod_collection_handle: NativeModCollectionHandle) -> ErrorCode;
    // Rulesets
    pub fn Ruleset_CreateFromId(
        ruleset_id: i32,
        ruleset_handle_ptr: *mut NativeRuleset,
    ) -> ErrorCode;
    pub fn Ruleset_CreateFromShortName(
        short_name: *const u8,
        ruleset_handle_ptr: *mut NativeRuleset,
    ) -> ErrorCode;
    pub fn Ruleset_GetShortName(
        ruleset_handle: NativeRulesetHandle,
        buffer: *mut u8,
        size: *mut i32,
    ) -> ErrorCode;
    pub fn Ruleset_Destroy(ruleset_handle: NativeRulesetHandle) -> ErrorCode;
    // Beatmaps
    pub fn Beatmap_CreateFromFile(path: *const i8, beatmap_ptr: *mut NativeBeatmap) -> ErrorCode;
    pub fn Beatmap_CreateFromText(text: *const i8, beatmap_ptr: *mut NativeBeatmap) -> ErrorCode;
    pub fn Beatmap_GetTitle(
        beatmap_handle: NativeBeatmapHandle,
        buffer: *mut u8,
        size: *mut i32,
    ) -> ErrorCode;
    pub fn Beatmap_GetArtist(
        beatmap_handle: NativeBeatmapHandle,
        buffer: *mut u8,
        size: *mut i32,
    ) -> ErrorCode;
    pub fn Beatmap_GetVersion(
        beatmap_handle: NativeBeatmapHandle,
        buffer: *mut u8,
        size: *mut i32,
    ) -> ErrorCode;
    pub fn Beatmap_Destroy(beatmap_handle: NativeBeatmapHandle) -> ErrorCode;
    /// Difficulty Calculator Objects (CDO)
    // ODCO
    pub fn OsuDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeOsuDifficultyCalculatorHandle,
    ) -> ErrorCode;
    pub fn OsuDifficultyCalculator_Calculate(
        calculator_handle: NativeOsuDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeOsuDifficultyAttributes,
    ) -> ErrorCode;
    pub fn OsuDifficultyCalculator_CalculateMods(
        calculator_handle: NativeOsuDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mods_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeOsuDifficultyAttributes,
    ) -> ErrorCode;
    pub fn OsuDifficultyCalculator_Destroy(calculator_handle: NativeOsuDifficultyCalculatorHandle);
    // TDCO
    pub fn TaikoDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeTaikoDifficultyCalculatorHandle,
    ) -> ErrorCode;
    pub fn TaikoDifficultyCalculator_Calculate(
        calculator_handle: NativeTaikoDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeTaikoDifficultyAttributes,
    ) -> ErrorCode;
    pub fn TaikoDifficultyCalculator_CalculateMods(
        calculator_handle: NativeTaikoDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mods_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeTaikoDifficultyAttributes,
    ) -> ErrorCode;
    pub fn TaikoDifficultyCalculator_Destroy(
        calculator_handle: NativeTaikoDifficultyCalculatorHandle,
    );
    // MDCO
    pub fn ManiaDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeManiaDifficultyCalculatorHandle,
    ) -> ErrorCode;
    pub fn ManiaDifficultyCalculator_Calculate(
        calculator_handle: NativeManiaDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeManiaDifficultyAttributes,
    ) -> ErrorCode;
    pub fn ManiaDifficultyCalculator_CalculateMods(
        calculator_handle: NativeManiaDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mods_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeManiaDifficultyAttributes,
    ) -> ErrorCode;
    pub fn ManiaDifficultyCalculator_Destroy(
        calculator_handle: NativeManiaDifficultyCalculatorHandle,
    );
    // CDCO
    pub fn CatchDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeCatchDifficultyCalculatorHandle,
    ) -> ErrorCode;
    pub fn CatchDifficultyCalculator_Calculate(
        calculator_handle: NativeCatchDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeCatchDifficultyAttributes,
    ) -> ErrorCode;
    pub fn CatchDifficultyCalculator_CalculateMods(
        calculator_handle: NativeCatchDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mods_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeCatchDifficultyAttributes,
    ) -> ErrorCode;
    pub fn CatchDifficultyCalculator_Destroy(
        calculator_handle: NativeCatchDifficultyCalculatorHandle,
    );

}

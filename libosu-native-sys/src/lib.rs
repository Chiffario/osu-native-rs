#[repr(C)]
pub struct NativeOsuDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: i32,
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
    pub max_combo: i32,
}

#[repr(C)]
pub struct NativeTaikoDifficultyAttributes {
    pub star_rating: f64,
    pub max_combo: i32,
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
    pub max_combo: i32,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(i8)]
pub enum ErrorCode {
    BufferSizeQuery = -1,
    Success = 0,
    ObjectNotFound = 1,
    RulesetUnavailable = 2,
    UnexpectedRuleset = 3,
    BeatmapFileNotFound = 4,
    Failure = 127,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct NativeOsuDifficultyCalculator {
    pub handle: NativeOsuDifficultyCalculatorHandle,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeOsuDifficultyCalculatorHandle(i32);

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct NativeTaikoDifficultyCalculator {
    pub handle: NativeTaikoDifficultyCalculatorHandle,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeTaikoDifficultyCalculatorHandle(i32);

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct NativeCatchDifficultyCalculator {
    pub handle: NativeCatchDifficultyCalculatorHandle,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeCatchDifficultyCalculatorHandle(i32);

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct NativeManiaDifficultyCalculator {
    pub handle: NativeManiaDifficultyCalculatorHandle,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeManiaDifficultyCalculatorHandle(i32);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeModHandle(i32);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeModCollectionHandle(i32);

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct NativeRuleset {
    pub handle: NativeRulesetHandle,
    pub id: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeRulesetHandle(i32);

#[derive(Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NativeBeatmapHandle(i32);

#[cfg_attr(target_os = "windows", link(name = "osu.Native", kind = "dylib"))]
#[cfg_attr(
    not(target_os = "windows"),
    link(name = "osu.Native.so", modifiers = "+verbatim")
)]
unsafe extern "C" {
    // Mod
    pub fn Mod_Create(acronym: *const i8, mod_handle_ptr: *mut NativeModHandle) -> ErrorCode;
    pub fn Mod_SetSetting(mod_handle: NativeModHandle, key: *const i8, value: f64) -> ErrorCode;
    pub fn Mod_Debug(mod_handle: NativeModHandle) -> ErrorCode;
    pub fn Mod_Destroy(mod_handle: NativeModHandle) -> ErrorCode;

    // Mod collection
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
    pub fn Ruleset_CreateFromId(ruleset_id: i32, ruleset_ptr: *mut NativeRuleset) -> ErrorCode;
    pub fn Ruleset_CreateFromShortName(
        short_name: *const u8,
        ruleset_ptr: *mut NativeRuleset,
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
        calculator_ptr: *mut NativeOsuDifficultyCalculator,
    ) -> ErrorCode;
    pub fn OsuDifficultyCalculator_Calculate(
        calculator_handle: NativeOsuDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeOsuDifficultyAttributes,
    ) -> ErrorCode;
    pub fn OsuDifficultyCalculator_CalculateMods(
        calculator_handle: NativeOsuDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mod_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeOsuDifficultyAttributes,
    ) -> ErrorCode;
    pub fn OsuDifficultyCalculator_Destroy(
        calculator_handle: NativeOsuDifficultyCalculatorHandle,
    ) -> ErrorCode;

    // TDCO
    pub fn TaikoDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeTaikoDifficultyCalculator,
    ) -> ErrorCode;
    pub fn TaikoDifficultyCalculator_Calculate(
        calculator_handle: NativeTaikoDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeTaikoDifficultyAttributes,
    ) -> ErrorCode;
    pub fn TaikoDifficultyCalculator_CalculateMods(
        calculator_handle: NativeTaikoDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mod_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeTaikoDifficultyAttributes,
    ) -> ErrorCode;
    pub fn TaikoDifficultyCalculator_Destroy(
        calculator_handle: NativeTaikoDifficultyCalculatorHandle,
    ) -> ErrorCode;

    // MDCO
    pub fn ManiaDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeManiaDifficultyCalculator,
    ) -> ErrorCode;
    pub fn ManiaDifficultyCalculator_Calculate(
        calculator_handle: NativeManiaDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeManiaDifficultyAttributes,
    ) -> ErrorCode;
    pub fn ManiaDifficultyCalculator_CalculateMods(
        calculator_handle: NativeManiaDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mod_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeManiaDifficultyAttributes,
    ) -> ErrorCode;
    pub fn ManiaDifficultyCalculator_Destroy(
        calculator_handle: NativeManiaDifficultyCalculatorHandle,
    ) -> ErrorCode;

    // CDCO
    pub fn CatchDifficultyCalculator_Create(
        ruleset_handle: NativeRulesetHandle,
        beatmap_handle: NativeBeatmapHandle,
        calculator_ptr: *mut NativeCatchDifficultyCalculator,
    ) -> ErrorCode;
    pub fn CatchDifficultyCalculator_Calculate(
        calculator_handle: NativeCatchDifficultyCalculatorHandle,
        attributes_ptr: *mut NativeCatchDifficultyAttributes,
    ) -> ErrorCode;
    pub fn CatchDifficultyCalculator_CalculateMods(
        calculator_handle: NativeCatchDifficultyCalculatorHandle,
        ruleset_handle: NativeRulesetHandle,
        mod_collection_handle: NativeModCollectionHandle,
        attributes_ptr: *mut NativeCatchDifficultyAttributes,
    ) -> ErrorCode;
    pub fn CatchDifficultyCalculator_Destroy(
        calculator_handle: NativeCatchDifficultyCalculatorHandle,
    ) -> ErrorCode;
}

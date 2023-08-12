pub mod objects;
pub mod parse;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use self::objects::HitObject;
#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParserDifficulty
{
    pub aim_strain:          f64,
    pub speed_strain:        f64,
    pub flashlight_strain:   f64,
    pub slider_strain_ratio: f64,
    pub speed_note_count:    f64,
    pub ar:                  f64,
    pub od:                  f64,
    pub hp:                  f64,
    pub circles:             u32,
    pub sliders:             u32,
    pub spinners:            u32,
    pub stars:               f64,
    pub max_combo:           u32,
}
#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParserPerformance
{
    pub difficulty:           ParserDifficulty,
    pub pp:                   f64,
    pub pp_acc:               f64,
    pub pp_aim:               f64,
    pub pp_flashlight:        f64,
    pub pp_speed:             f64,
    pub effective_miss_count: f64,
}
#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParserStrains
{
    pub difficulty:  Option<Vec<ParserDifficulty>>,
    pub performance: Option<Vec<ParserPerformance>>,
}
#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParserBeatmapAttributes
{
    pub difficulty:  Option<ParserDifficulty>,
    pub performance: Option<ParserPerformance>,
}
#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParserBeatmap
{
    pub hash:              String,
    pub title:             String,
    pub artist:            String,
    pub tags:              Vec<String>,
    pub combo_colors:      Vec<Color>,
    pub circles:           u32,
    pub sliders:           u32,
    pub spinners:          u32,
    pub ar:                f32,
    pub od:                f32,
    pub cs:                f32,
    pub hp:                f32,
    pub slider_multiplier: f64,
    pub tick_rate:         f64,
    pub difficulty:        Option<ParserDifficulty>,
    pub hit_objects:       Option<Vec<HitObject>>,
}
#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Color
{
    pub red: u8,

    pub green: u8,

    pub blue: u8,
}

impl From<libosu::prelude::Color> for Color
{
    fn from(value: libosu::prelude::Color) -> Self
    {
        Self {
            red:   value.red,
            green: value.green,
            blue:  value.blue,
        }
    }
}

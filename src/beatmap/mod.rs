use rosu_pp::{osu::OsuDifficultyAttributes, Beatmap, BeatmapExt, OsuPP};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::replay::ParserResult;
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
pub struct ParserBeatmap
{
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
}

impl ParserBeatmap
{
    pub fn parse(beatmap: &mut [u8]) -> ParserResult<Self>
    {
        let parsed = Beatmap::parse(&mut beatmap.as_ref())?;
        Ok(parsed.into())
    }
    pub fn parse_extra(beatmap: &mut [u8]) -> ParserResult<Self>
    {
        let parsed = Beatmap::parse(&mut beatmap.as_ref())?;

        let diff_result = OsuPP::new(&parsed).calculate().difficulty;

        let mut res = ParserBeatmap::from(parsed);
        res.difficulty = Some(ParserDifficulty::from(diff_result));

        Ok(res)
    }
}

impl From<rosu_pp::Beatmap> for ParserBeatmap
{
    fn from(value: rosu_pp::Beatmap) -> Self
    {
        Self {
            circles:           value.n_circles,
            sliders:           value.n_sliders,
            spinners:          value.n_spinners,
            ar:                value.ar,
            od:                value.od,
            cs:                value.cs,
            hp:                value.hp,
            slider_multiplier: value.slider_mult,
            tick_rate:         value.tick_rate,
            difficulty:        None,
        }
    }
}
impl From<OsuDifficultyAttributes> for ParserDifficulty
{
    fn from(value: OsuDifficultyAttributes) -> Self
    {
        Self {
            aim_strain:          value.aim,
            speed_strain:        value.speed,
            flashlight_strain:   value.flashlight,
            slider_strain_ratio: value.slider_factor,
            speed_note_count:    value.speed_note_count,
            ar:                  value.ar,
            od:                  value.od,
            hp:                  value.hp,
            circles:             value.n_circles as _,
            sliders:             value.n_sliders as _,
            spinners:            value.n_spinners as _,
            stars:               value.stars,
            max_combo:           value.max_combo as _,
        }
    }
}

//! wasm osu replay and beatmap parser
pub mod beatmap;
pub mod macros;
pub mod replay;

mod utils;

use beatmap::{ParserBeatmap, ParserBeatmapAttributes, ParserStrains};
use wasm_bindgen::prelude::*;

use crate::{beatmap::ParserScoreState, replay::Replay};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Parses the provided replay for basic information.
/// # Example
///
/// ```
///  const reader = new FileReader();
///
///  reader.onloadend = (evt) => {
///    const replaydata = new Uint8Array(evt.target.result);
///    console.log(wasm.parseReplay(replaydata));
/// }
/// ```
#[wasm_bindgen(js_name = parseReplay)]
pub fn parse_replay(replay: &mut [u8]) -> Result<Replay, JsError> {
    let parsed = Replay::parse(&mut replay.as_ref(), false)?;
    Ok(parsed)
}

/// Parses the provided replay for advanced information.
///
/// Includes everything from [`parse_replay`] plus raw replay data and replay frame data containing cursor and button information.
/// # Example
///
/// ```
///  const reader = new FileReader();
///
///  reader.onloadend = (evt) => {
///    const replaydata = new Uint8Array(evt.target.result);
///    console.log(wasm.parseReplayExtra(replaydata, beatmapdata));
/// }
/// ```
#[wasm_bindgen(js_name = parseReplayExtra)]
pub fn parse_replay_extra(replay: &mut [u8], beatmap: &mut [u8]) -> Result<Replay, JsError> {
    let extras = Replay::parse_extra(&mut replay.as_ref(), &mut beatmap.as_ref())?;
    Ok(extras)
}

#[derive(Copy, Clone)]
#[wasm_bindgen]
pub struct ParserScore {
    pub mods: Option<u32>,
    pub combo: Option<usize>,
    pub judgements: Option<replay::Judgements>,
    pub passed_objects: Option<usize>, //? for partial plays like fails but i dont think this is relevant in a tourney context
    pub clock_rate: Option<f64>, //? if theres any rate changes in the pool this could be used to calc pp from the original map
    pub accuracy: Option<f64>,
}
/// Parses the provided beatmap
///
/// # Example
///
/// ```
///  const reader = new FileReader();
///
///  reader.onloadend = (evt) => {
///    const beatmapdata = new Uint8Array(evt.target.result);
///    console.log(wasm.parseBeatmapExtra(beatmapdata));
/// }
/// ```
#[wasm_bindgen(js_name = parseBeatmap)]
pub fn parse_beatmap(beatmap: &mut [u8]) -> Result<ParserBeatmap, JsError> {
    let parsed = ParserBeatmap::parse(&mut beatmap.as_ref())?;
    Ok(parsed)
}
/// Parses the provided beatmap and calculates difficulty and performance attributes.
///
/// If a [`ParserScore`] is provided, it will use the provided score state for calculation.
/// # Example
///
/// ```
///  const reader = new FileReader();
///
///  reader.onloadend = (evt) => {
///    const beatmapdata = new Uint8Array(evt.target.result);
///    console.log(wasm.parseBeatmapAttributes(beatmapdata));
/// }
/// ```
#[wasm_bindgen(js_name = parseBeatmapAttributes)]
pub fn parse_beatmap_attributes(
    score: Option<ParserScore>,
    beatmap: &mut [u8],
) -> Result<ParserBeatmapAttributes, JsError> {
    Ok(ParserBeatmap::parse_beatmap_attributes(
        score,
        &mut beatmap.as_ref(),
    )?)
}
/// Parses the provided beatmap and calculates an array of gradual difficulty and optionally performance strains.
///
/// If an array of [`ParserScoreState`] is provided, it will also calculate gradual performance strains.
/// # Example
///
/// ```
///  const reader = new FileReader();
///
///  reader.onloadend = (evt) => {
///    const beatmapdata = new Uint8Array(evt.target.result);
///    console.log(wasm.parseBeatmapStrains(beatmapdata,scorearray));
/// }
/// ```
#[wasm_bindgen(js_name = parseBeatmapStrains)]
pub fn parse_beatmap_strains(
    beatmap: &mut [u8],
    score_states: Option<Vec<JsValue>>,
    mods: Option<u32>,
) -> Result<ParserStrains, JsError> {
    Ok(ParserBeatmap::parse_beatmap_strains(
        &mut beatmap.as_ref(),
        match score_states {
            Some(states) => {
                let scorestates: Vec<ParserScoreState> = states
                    .iter()
                    .map(|x| serde_wasm_bindgen::from_value(x.clone()).unwrap())
                    .collect();
                Some(scorestates)
            }
            None => None,
        },
        mods,
    )?)
}
/// Call on init for better panic reports when debugging, not required.
#[wasm_bindgen]
pub fn init_panic_hook() {
    utils::set_panic_hook();
}

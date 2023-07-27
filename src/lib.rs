pub mod beatmap;
pub mod macros;
pub mod replay;

mod utils;

use beatmap::ParserBeatmap;

use wasm_bindgen::prelude::*;

use crate::replay::Replay;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() { alert("Hello, replay-parser!"); }

//^ returns js object containing replay metadata
#[wasm_bindgen(js_name = parseReplay)]
pub fn parse_replay(replay: &mut [u8]) -> Result<Replay, JsError>
{
    let parsed = Replay::parse(&mut replay.as_ref(), false)?;
    Ok(parsed)
}

//^ return js object containing everything from parse_replay + replaydata as an array of hitobjects w hit/cursor/tap info
#[wasm_bindgen(js_name = parseReplayExtra)]
pub fn parse_replay_extra(replay: &mut [u8], beatmap: &mut [u8]) -> Result<Replay, JsError>
{
    let extras = Replay::parse_extra(&mut replay.as_ref(), &mut beatmap.as_ref())?;
    Ok(extras)
}

//^ returns js object containing map metadata
#[wasm_bindgen(js_name = parseBeatmap)]
pub fn parse_beatmap(beatmap: &mut [u8]) -> Result<ParserBeatmap, JsError>
{
    let parsed = ParserBeatmap::parse(&mut beatmap.as_ref())?;
    Ok(parsed)
}

/*
struct Score {
    pub mods: u32,
    pub combo: usize,
    pub Judgements: Judgements,
    pub passed_objects: Option<usize>, //? for partial plays like fails but i dont think this is relevant in a tourney context
    pub clock_rate: Option<f64>, //? if theres any rate changes in the pool this could be used to calc pp from the original map
    pub accuracy: f64,
}
*/

//^ return js object containing aim/speed pp and sr + total, need to figure out how to pass ar/od/cs/hp if there are any non-osu-mod modifications from the base map
#[wasm_bindgen(js_name = parseBeatmapExtra)]
pub fn parse_beatmap_extra(_score: &mut [u8], _beatmap: &mut [u8]) -> JsValue { todo!() }

#[wasm_bindgen]
pub fn init_panic_hook() { utils::set_panic_hook(); }

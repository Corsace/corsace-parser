pub mod macros;
pub mod replay;

mod utils;

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

#[wasm_bindgen]
pub fn parse_replay(replay: &mut [u8]) -> JsValue
{
    let parsed = Replay::parse(&mut replay.as_ref()).unwrap();
    serde_wasm_bindgen::to_value(&parsed).unwrap()
}
#[wasm_bindgen]
pub fn init_panic_hook() { utils::set_panic_hook(); }

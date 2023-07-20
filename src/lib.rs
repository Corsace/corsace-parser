pub mod macros;
pub mod replay;
mod utils;

use wasm_bindgen::prelude::*;

use crate::replay::Judgements;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, replay-parser!");
}
#[wasm_bindgen]
pub fn parse_replay(replay: &mut [u8]) {
    let len = replay.len();
    let judgement_test = Judgements::default();

    console_log!(
        "judgements: {:#?}",
        serde_wasm_bindgen::to_value(&judgement_test).unwrap()
    );
}

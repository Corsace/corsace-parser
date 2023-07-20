pub mod replay;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, replay-parser!");
}
#[wasm_bindgen]
pub fn parse_replay(replay: &mut [u8]) {
    let len = replay.len();
    alert(format!("replay length: {}", len).as_str());
}

//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use web_time::Instant;

// Since this library is for npm, the line below is not needed.
// wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

use wasm_replay_parser_rs;

#[wasm_bindgen_test]
fn test_parse_beatmap() {
    let mut beatmap = include_bytes!("./beatmap.osu").to_owned();
    let result = wasm_replay_parser_rs::parse_beatmap(&mut beatmap);
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_parse_replay() {
    let mut replay = include_bytes!("./replay.osr").to_owned();
    let result = wasm_replay_parser_rs::parse_replay(&mut replay);
    assert!(result.is_ok());
}

// Benchmarks

#[wasm_bindgen_test]
fn bench_parse_beatmap() {
    let mut beatmap = include_bytes!("./beatmap.osu").to_owned();

    let start = Instant::now();
    for _ in 0..100 {
        let result = wasm_replay_parser_rs::parse_beatmap(&mut beatmap);
        assert!(result.is_ok());
    }
    let duration = start.elapsed();
    wasm_bindgen_test::console_log!("Benchmark: parse_beatmap x100 took {:?}", duration);
}

#[wasm_bindgen_test]
fn bench_parse_replay() {
    let mut replay = include_bytes!("./replay.osr").to_owned();

    let start = Instant::now();
    for _ in 0..100 {
        let result = wasm_replay_parser_rs::parse_replay(&mut replay);
        assert!(result.is_ok());
    }
    let duration = start.elapsed();
    wasm_bindgen_test::console_log!("Benchmark: parse_replay x100 took {:?}", duration);
}

#![cfg(target_arch = "wasm32")]

use corsace_parser;
use wasm_bindgen_test::*;
use web_time::Instant;

// Since this library is for npm, the line below is not needed.
// wasm_bindgen_test_configure!(run_in_browser);

fn calculate_stddev(durations: &[web_time::Duration], mean: f64) -> f64 {
    let variance: f64 = durations
        .iter()
        .map(|d| {
            let diff = d.as_secs_f64() - mean;
            diff * diff
        })
        .sum::<f64>()
        / durations.len() as f64;

    variance.sqrt()
}

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_parse_beatmap() {
    let mut beatmap = include_bytes!("./beatmap.osu").to_owned();
    let result = corsace_parser::parse_beatmap(&mut beatmap);
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_parse_replay() {
    let mut replay = include_bytes!("./replay.osr").to_owned();
    let result = corsace_parser::parse_replay(&mut replay);
    assert!(result.is_ok());
}

// Benchmarks

#[wasm_bindgen_test]
fn bench_parse_beatmap() {
    let mut beatmap = include_bytes!("./beatmap.osu").to_owned();

    let mut durations = Vec::new();
    for _ in 0..100 {
        let start = Instant::now();
        let result = corsace_parser::parse_beatmap(&mut beatmap);
        assert!(result.is_ok());
        let duration = start.elapsed();
        durations.push(duration);
    }

    let total_duration: f64 = durations.iter().map(|d| d.as_secs_f64()).sum();
    let avg_duration = total_duration / durations.len() as f64;
    let stddev = calculate_stddev(&durations, avg_duration);

    wasm_bindgen_test::console_log!(
        "Benchmark: parse_beatmap x100 took total: {:.3?}s, average: {:.3?}s, stddev: {:.3?}s",
        total_duration,
        avg_duration,
        stddev
    );
}

#[wasm_bindgen_test]
fn bench_parse_replay() {
    let mut replay = include_bytes!("./replay.osr").to_owned();

    let mut durations = Vec::new();
    for _ in 0..100 {
        let start = Instant::now();
        let result = corsace_parser::parse_replay(&mut replay);
        assert!(result.is_ok());
        let duration = start.elapsed();
        durations.push(duration);
    }

    let total_duration: f64 = durations.iter().map(|d| d.as_secs_f64()).sum();
    let avg_duration = total_duration / durations.len() as f64;
    let stddev = calculate_stddev(&durations, avg_duration);

    wasm_bindgen_test::console_log!(
        "Benchmark: parse_replay x100 took total: {:.3?}s, average: {:.3?}s, stddev: {:.3?}s",
        total_duration,
        avg_duration,
        stddev
    );
}

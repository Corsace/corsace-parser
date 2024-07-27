import initWasm, * as wasm from "replay-parser";

const replayInput = document.getElementById("replay");
replayInput.addEventListener("change", handleFile, false);

const beatmapInput = document.getElementById("beatmap");
beatmapInput.addEventListener("change", handleFileBeatmap, false);

const replayExtrasEnabled = document.getElementById("replayextras");

let beatmapdata = [];

(async () => {
  try {
      await initWasm();
      console.log("WASM module initialized successfully");
  } catch (e) {
      console.error("Error initializing WASM module:", e);
  }
})();

function handleFile(e) {
  const file = e.currentTarget.files[0];
  if (!(file instanceof Blob)) return;

  const reader = new FileReader();

  reader.onloadend = (evt) => {
    const data = new Uint8Array(evt.target.result);

    console.log("data size: " + data.length);

    if (replayExtrasEnabled.checked && beatmapdata.length > 0) {
      console.log("calling parse_replayextra");
      console.log(wasm.parseReplayExtra(data, beatmapdata));
    } else {
      console.log("calling parse_replay");
      console.log(wasm.parseReplay(data));
    }
  };
  reader.readAsArrayBuffer(file);
}

let arrayOfObjects = [
  {
    max_combo: 0,
    n_geki: 99,
    n_katu: 88,
    n300: 1,
    n100: 1,
    n50: 1,
    n_misses: 1,
  },
  {
    max_combo: 0,
    n_geki: 99,
    n_katu: 88,
    n300: 1,
    n100: 1,
    n50: 1,
    n_misses: 1,
  },
  {
    max_combo: 0,
    n_geki: 99,
    n_katu: 88,
    n300: 1,
    n100: 1,
    n50: 1,
    n_misses: 1,
  },
  {
    max_combo: 0,
    n_geki: 99,
    n_katu: 88,
    n300: 1,
    n100: 1,
    n50: 1,
    n_misses: 1,
  },
];
function handleFileBeatmap(e) {
  const file = e.currentTarget.files[0];
  if (!(file instanceof Blob)) return;

  const reader = new FileReader();

  reader.onloadend = (evt) => {
    const data = new Uint8Array(evt.target.result);

    console.log("data size: " + data.length);
    beatmapdata = data;
    console.log("calling parse_beatmap");
    console.log(wasm.parseBeatmap(data));
    console.log(wasm.parseBeatmapStrains(data, arrayOfObjects));
  };
  reader.readAsArrayBuffer(file);
}

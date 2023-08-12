import * as wasm from "replay-parser";

const replayInput = document.getElementById("replay");
replayInput.addEventListener("change", handleFile, false);

const beatmapInput = document.getElementById("beatmap");
beatmapInput.addEventListener("change", handleFileBeatmap, false);

const replayExtrasEnabled = document.getElementById("replayextras");
const beatmapExtrasEnabled = document.getElementById("beatmapextras");

let beatmapdata = [];

wasm.init_panic_hook();
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

function handleFileBeatmap(e) {
  const file = e.currentTarget.files[0];
  if (!(file instanceof Blob)) return;

  const reader = new FileReader();

  reader.onloadend = (evt) => {
    const data = new Uint8Array(evt.target.result);

    console.log("data size: " + data.length);
    beatmapdata = data;
    if (beatmapExtrasEnabled.checked) {
      console.log("calling parse_beatmapextra");
      console.log(wasm.parseBeatmapExtra(data));
    } else {
      console.log("calling parse_beatmap");
      console.log(wasm.parseBeatmap(data));
      console.log(wasm.parseBeatmapStrains(data));
    }
  };
  reader.readAsArrayBuffer(file);
}

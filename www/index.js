import * as wasm from "replay-parser";

const replayInput = document.getElementById("replay");
replayInput.addEventListener("change", handleFile, false);

const beatmapInput = document.getElementById("beatmap");
beatmapInput.addEventListener("change", handleFileBeatmap, false);

wasm.init_panic_hook();
function handleFile(e) {
  const file = e.currentTarget.files[0];
  if (!(file instanceof Blob)) return;

  const reader = new FileReader();

  reader.onloadend = (evt) => {
    const data = new Uint8Array(evt.target.result);

    console.log("data size: " + data.length);
    console.log("calling parse_replay");
    console.log(wasm.parseReplay(data));
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
    console.log("calling parse_beatmap");
    console.log(wasm.parseBeatmap(data));
  };
  reader.readAsArrayBuffer(file);
}

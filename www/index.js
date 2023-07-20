import * as wasm from "replay-parser";

const inputElement = document.querySelector("input");
inputElement.addEventListener("change", handleFile, false);

function handleFile(e) {

    const file = e.currentTarget.files[0];
    if(!(file instanceof Blob)) return;

    const reader = new FileReader();

    reader.onloadend = evt => {
        const data = new Uint8Array(evt.target.result);

        console.log("data size: " + data.length);
        console.log("calling parse_replay");    
        wasm.parse_replay(data);
    }
    reader.readAsArrayBuffer(file);
}
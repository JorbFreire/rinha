import * as wasm from "rinha";

export function loadJson(event) {
  const container = document.querySelector("#container");
  const [inputFile] = event.target.files;
  container.remove();
  wasm.load_json(inputFile);
}


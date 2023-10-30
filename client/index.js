import * as wasm from "rinha";

export function loadJson() {
  const container = document.querySelector("#container");
  container.remove();
  wasm.load_json();
}


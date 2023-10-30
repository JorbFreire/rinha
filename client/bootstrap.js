// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
import("./index.js").then((main_module) => {
  const file_input = document.querySelector("#file-input")
  file_input.addEventListener("change", main_module.loadJson)
}).catch(
  e => console.error("Error importing `index.js`:", e)
);

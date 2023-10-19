mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rinha!");
}

// #[wasm_bindgen]
// pub fn display_new_row(row_name: &str) {
#[wasm_bindgen(start)]
fn display_new_row() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let new_div = document.create_element("div")? ;
    new_div.set_text_content(Some("Hello from Rust!"));
    // new_div.set_inner_html("Hello from Rust!");
    body.append_child(&new_div);

    Ok(())
}

#[wasm_bindgen]
pub fn display() {
    display_new_row();
}

// #[wasm_bindgen(start)]
// fn run() -> Result<(), JsValue> {
//     // Use `web_sys`'s global `window` function to get a handle on the global
//     // window object.
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let body = document.body().expect("document should have a body");

//     // Manufacture the element we're gonna append
//     let val = document.create_element("p")?;

//     body.append_child(&val)?;

//     Ok(())
// }

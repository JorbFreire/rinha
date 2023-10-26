mod utils;

use wasm_bindgen::prelude::*;
use serde_json::Value;
use serde_json::json;
use fix_fn::fix_fn;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rinha!");
}

#[wasm_bindgen]
pub fn render_new_row(row_name: &str) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let new_div = document.create_element("div")? ;
    new_div.set_text_content(Some(row_name));
    // new_div.set_inner_html("Hello from Rust!");
    let _ = body.append_child(&new_div);

    Ok(())
}

#[wasm_bindgen]
pub fn load_json() {
    fn check_is_iterable(object_to_check: &Value) -> bool {
        let is_object = object_to_check.is_object();
        let is_array = object_to_check.is_array();
        return is_object || is_array;
    }

    // recursive function that shall load the full JSON file
    let read_iterable_object = fix_fn!(
        |read_iterable_object, iterable_object: &Value| -> bool {
            log("ok, we are reading something at least");
            if iterable_object.is_array() {
                for value in iterable_object.as_array().unwrap().iter() {
                    log("ja foi 1");
                    render_new_row(value.as_str().unwrap());
                    if check_is_iterable(value) {
                        log("iterate again");
                        read_iterable_object(value);
                    }
                }
            }
            if iterable_object.is_object() {
                for (key, value) in iterable_object.as_object().unwrap().iter() {
                    render_new_row(value.as_str().unwrap());
                    render_new_row(key);
                    if check_is_iterable(value) {
                        read_iterable_object(value);
                    }
                }
            }
            return true;
        }
    );

    let phones = json!({
        "4": "+44 1234564",
        "5": "+44 1234565",
        "3": "+44 1234563",
        "2": "+44 1234562",
        "1": "+44 1234561",
        "7": "+44 1234567",
        "8": "+44 2345678"
    });

    read_iterable_object(&phones);
}

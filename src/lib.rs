// todo 
// ?[] - test with bigger json, from drive files
// ?[] - json input on html
// ?[] - minimal css
// ?[] - acessilibity improvement
// ?[] - performance improvement
// ?[] - Refactor "window", "document", "body" into a class or something

mod utils;

use wasm_bindgen::prelude::*;
use serde_json::Value;
use serde_json::json;
use fix_fn::fix_fn;
use web_sys::{Element, Window, Document, HtmlElement};

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
pub fn render_new_row(row: &Element) -> Result<(), JsValue> {
    // todo: shall be refactored into a class
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let body: HtmlElement = document.body().expect("document should have a body");
    // todo: end "todo"
    let _ = body.append_child(&row);
    Ok(())
}

#[wasm_bindgen]
pub fn generate_new_row(row_name: &str) -> Result<Element, JsValue> {
    // todo: shall be refactored into a class
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    // todo: end "todo"
    let new_collapsible_row = document.create_element("details")?;
    let row_summary = document.create_element("summary")?;
    let _ = row_summary.set_text_content(Some(row_name));
    let _ = new_collapsible_row.append_child(&row_summary);

    Ok(new_collapsible_row)
}

#[wasm_bindgen]
pub fn generate_nameless_row() -> Result<Element, JsValue> {
    // todo: shall be refactored into a class
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    // todo: end "todo"
    let new_div = document.create_element("div")?;

    Ok(new_div)
}

#[wasm_bindgen]
pub fn generate_new_item(item_name: &str) -> Result<Element, JsValue> {
    // todo: shall be refactored into a class
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    // todo: end "todo"
    let new_item = document.create_element("p")?;
    let _ = new_item.set_text_content(Some(item_name));

    Ok(new_item)
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
        |read_iterable_object, iterable_object: &Value| -> Result<Element, JsValue> {
            let mut this_row: Element = generate_new_row("temp_row").ok().unwrap();

            if iterable_object.is_array() {
                this_row = generate_new_row("array").ok().unwrap();
                for value in iterable_object.as_array().unwrap().iter() {
                    if check_is_iterable(value) {
                        let array_row = read_iterable_object(value).ok().unwrap();
                        let _ = this_row.append_child(&array_row);
                    } else {
                        let array_row_item = generate_new_item(value.as_str().unwrap()).ok();
                        let array_row_item_unwraped = array_row_item.as_deref().unwrap();
                        let _ = this_row.append_child(&array_row_item_unwraped);
                    }
                }
            }
            if iterable_object.is_object() {
                this_row = generate_nameless_row().ok().unwrap();
                for (key, value) in iterable_object.as_object().unwrap().iter() {
                    let sub_row = generate_new_row(key).ok().unwrap();
                    if check_is_iterable(value) {
                        let object_row = read_iterable_object(value).ok().unwrap();
                        let _ = this_row.append_child(&object_row);
                    } else {
                        let object_row_item = generate_new_item(value.as_str().unwrap()).ok();
                        let object_row_item_unwarped = object_row_item.as_deref().unwrap();
                        let _ = sub_row.append_child(&object_row_item_unwarped);
                    }
                    let _ = this_row.append_child(&sub_row);
                }
            }
            Ok(this_row)
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

    let r = read_iterable_object(&phones);
    let _ = render_new_row(&r.as_ref().ok().unwrap());
}

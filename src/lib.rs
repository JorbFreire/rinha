// todo 
// ?[] - test with bigger json, from drive files
// ?[] - json input on html
// ?[] - minimal css
// ?[] - acessilibity improvement
// ?[] - performance improvement

mod utils;
pub mod browser;

use wasm_bindgen::prelude::*;
use serde_json::Value;
use serde_json::from_str;
use fix_fn::fix_fn;
use web_sys::{Element, File};
use wasm_bindgen_futures;

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
pub fn render_element(element: &Element) -> Result<(), JsValue> {
    let body = browser::browser::Browser::new().body;
    let _ = body.append_child(&element);
    Ok(())
}

#[wasm_bindgen]
pub fn generate_new_row(row_name: &str) -> Result<Element, JsValue> {
    let document = browser::browser::Browser::new().document;

    let new_collapsible_row = document.create_element("details")?;
    let row_summary = document.create_element("summary")?;
    let _ = row_summary.set_text_content(Some(row_name));
    let _ = new_collapsible_row.append_child(&row_summary);

    Ok(new_collapsible_row)
}

#[wasm_bindgen]
pub fn generate_nameless_row() -> Result<Element, JsValue> {
    let document = browser::browser::Browser::new().document;

    let new_div = document.create_element("div")?;

    Ok(new_div)
}

#[wasm_bindgen]
pub fn generate_new_item(item_name: &str) -> Result<Element, JsValue> {
    let document = browser::browser::Browser::new().document;

    let new_item = document.create_element("p")?;
    let _ = new_item.set_text_content(Some(item_name));

    Ok(new_item)
}


#[wasm_bindgen]
pub async fn load_json(file: &File) {
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

    async fn read_json_input(file_to_read: &File) -> Result<Value, JsValue> {
        log("start de reader function");
        let promise = file_to_read.text();
        let okthen = wasm_bindgen_futures::JsFuture::from(promise).await;
        let file_as_string = &okthen.ok().unwrap().as_string().unwrap();
        log("finish");

        let file_as_json: Value = from_str(&file_as_string).unwrap();
        Ok(file_as_json)
    }

    let file_type = file.type_();
    if &file_type != "application/json" {
        log("should trown an error!");
    }

    let file_name = file.name();
    let file_size = file.size().to_string();
    // let promise = js_sys::Promise::resolve(&42.into());

    let file_as_json = read_json_input(&file).await;

    log("&file_name");
    log(&file_name);
    log("&file_size");
    log(&file_size);

    let json_result = read_iterable_object(&file_as_json.ok().unwrap());
    let json_result_as_html = json_result.as_ref().ok().unwrap();
    json_result_as_html.set_id("container");

    let _ = render_element(&json_result_as_html);
}

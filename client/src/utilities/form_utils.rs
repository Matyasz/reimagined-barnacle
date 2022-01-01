use gloo::console::log;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCollection, HtmlInputElement};

pub fn process_form_field(col: &HtmlCollection, field: &str) -> String {
    let value = col
        .named_item(field)
        .unwrap()
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .value();

    log!(JsValue::from(&value));
    value
}

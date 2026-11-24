use wasm_bindgen::JsValue;

pub fn describe_error(error: &JsValue) -> String {
    js_sys::Reflect::get(error, &JsValue::from_str("name"))
        .ok()
        .and_then(|name| name.as_string())
        .or_else(|| error.as_string())
        .unwrap_or_else(|| "unknown error".to_string())
}
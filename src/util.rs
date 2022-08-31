use serde_json::Value;
use wasm_bindgen::JsValue;

pub fn log_json(json: Value) {
    web_sys::console::log_1(&JsValue::from_str(&json.to_string()));
}

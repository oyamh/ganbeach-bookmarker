use thiserror::Error;
// use wasm_bindgen::JsValue;
use js_sys::Object;
use wasm_bindgen::{JsCast, JsValue};

impl From<MessageError> for domain::DomainError {
    fn from(error: MessageError) -> Self {
        Self::Message(error.to_string())
    }
}

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("content message error: {}", js_object_display(.0))]
    Content(JsValue),

    #[error("popup message error: {}", js_object_display(.0))]
    Popup(JsValue),

    #[error("background message error: {}", js_object_display(.0))]
    Background(JsValue),
}

// impl From<JsValue> for MessageError {
//     fn from(js_value: JsValue) -> Self {
//         Self::Content(format!("{:?}", js_value))
//     }
// }

fn js_object_display(js_value: &JsValue) -> String {
    let object: &Object = js_value.unchecked_ref();
    ToString::to_string(&object.to_string())
}

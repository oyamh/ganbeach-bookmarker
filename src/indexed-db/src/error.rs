use js_sys::Object;
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::DomException;

impl From<DatabaseError> for domain::DomainError {
    fn from(error: DatabaseError) -> Self {
        Self::Database(error.to_string())
    }
}

impl From<DomException> for DatabaseError {
    fn from(error: DomException) -> Self {
        Self::DomException(error)
    }
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("DOM Exception error: {}", js_object_display(.0))]
    DomException(DomException),

    #[error("unexpected JS type. expected: {}, found: {}", .0, js_object_display(.1))]
    UnexpectedJsType(&'static str, JsValue),

    #[error("serialize error: {0}")]
    ConvertJsValue(#[from] serde_wasm_bindgen::Error),

    #[error("empty js value: {0}")]
    EmptyValue(&'static str),
}

fn js_object_display(option: &JsValue) -> String {
    let object: &Object = option.unchecked_ref();
    ToString::to_string(&object.to_string())
}

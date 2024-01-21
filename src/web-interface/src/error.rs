use gloo_storage::errors::StorageError;
use js_sys::Object;
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};

impl From<WebInterfaceError> for domain::DomainError {
    fn from(error: WebInterfaceError) -> Self {
        Self::WebInterface(error.to_string())
    }
}

#[derive(Debug, Error)]
pub enum WebInterfaceError {
    #[error("storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("cookie error: {0}")]
    Cookie(#[from] CookieError),

    #[error("notification error: {0}")]
    Notification(#[from] NotificationError),

    #[error("bookmarks error: {0}")]
    Bookmarks(#[from] BookmarksError),

    #[error("tabs error: {0}")]
    Tabs(#[from] TabsError),

    #[error("page data error: {0}")]
    PageDataError(#[from] PageDataError),

    #[error("history error: {0}")]
    History(#[from] HistoryError),
}

#[derive(Debug, Error)]
pub enum CookieError {
    #[error("serde-wasm-bindgen error: {0}")]
    Serde(#[from] serde_wasm_bindgen::Error),

    #[error("js_value error: {0}")]
    JsValue(String),

    #[error("unknown changed cause: {0}")]
    UnknownChangedCause(String),

    #[error("value is null")]
    Null,
}

// #[error("js_error: {0}")]
// JsError(#[from] JsError),
// pub(crate) fn cookie_js_to_error(js_value: JsValue) -> WebInterfaceError {
//     match js_value.dyn_into::<js_sys::Error>() {
//         Ok(err) => WebInterfaceError::Cookie(CookieError::JsError(JsError::from(err))),
//         Err(_) => unreachable!("JsValue passed is not an Error type - this is a bug"),
//     }
// }

//enum内部のJsValue(String)の表現に必要。
impl From<CookieError> for JsValue {
    fn from(err: CookieError) -> Self {
        JsValue::from_str(&format!("{err:?}"))
    }
}

impl From<JsValue> for CookieError {
    fn from(js_value: JsValue) -> Self {
        CookieError::JsValue(format!("{:?}", js_value))
    }
}

#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("serde-wasm-bindgen error: {0}")]
    Serde(#[from] serde_wasm_bindgen::Error),

    #[error("js_value error: {}", js_object_display(.0))]
    JsValue(JsValue),
    // #[error("cookie value is null")]
    // Null,
    #[error("no promise")]
    NoPromise,
}

#[derive(Debug, Error)]
pub enum BookmarksError {
    #[error("not found current folder")]
    NotFoundCurrentFolder,

    #[error("serde-wasm-bindgen error: {0}")]
    Serde(#[from] serde_wasm_bindgen::Error),

    #[error("from js error: {0}")]
    FromJsError(#[from] FromJsError),
}

#[derive(Debug, Error)]
pub enum TabsError {
    #[error("serde-wasm-bindgen error: {0}")]
    Serde(#[from] serde_wasm_bindgen::Error),

    #[error("js_value error: {0}")]
    FromJsError(#[from] FromJsError),

    #[error("url parse error: {0}")]
    ParseUrl(String),

    #[error("no active tab")]
    NoActiveTab,
}

#[derive(Debug, Error)]
pub enum PageDataError {
    #[error("no location")]
    NoLocation,

    #[error("js_value error: {0}")]
    FromJsError(#[from] FromJsError),

    #[error("url parse error: {0}")]
    ParseUrl(String),
}

#[derive(Debug, Error)]
pub enum HistoryError {
    #[error("serde-wasm-bindgen error: {0}")]
    Serde(#[from] serde_wasm_bindgen::Error),

    #[error("js_value error: {0}")]
    FromJsError(#[from] FromJsError),
}

#[derive(Debug, Error)]
pub enum FromJsError {
    #[error("js_sys::Error: {}", .0.to_string())]
    JsSysError(js_sys::Error),

    #[error("JsValue: {}", js_sys::JsString::from(.0.clone()))]
    JsValue(JsValue),
}

impl From<JsValue> for FromJsError {
    fn from(js_value: JsValue) -> Self {
        match js_value.dyn_into::<js_sys::Error>() {
            Ok(js_error) => Self::JsSysError(js_error),
            Err(js_value) => Self::JsValue(js_value),
        }
    }
}

fn js_object_display(js_value: &JsValue) -> String {
    let object: &Object = js_value.unchecked_ref();
    ToString::to_string(&object.to_string())
}

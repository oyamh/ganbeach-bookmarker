use js_sys::Object;
use std::fmt::Debug;
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::DomException;

use crate::LoginUrl;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("failed to get access token. so get login_url instead of access token login_url={0}")]
    LoginUrlInstaedOfToken(LoginUrl),

    #[error("failed to get access token. error={0}")]
    GetAccessToken(String),

    #[error("failed to parse url string error={0}")]
    ParseUrl(#[from] url::ParseError),

    #[error("failed to parse auth provider string auth_provider_str={0}")]
    ParseAuthProvider(String),

    #[error("failed to parse bookmark id string bookmark_id={0}")]
    ParseBookmarkId(String),

    // #[error("failed to parse js value js_value={}", js_object_display(.0))]
    #[error("failed to parse js value")]
    ParseJsValue(String),

    #[error("invalid notification id id_str={0}")]
    InvalidNotificationId(String),

    #[error("failed to create bookmarks error={0}")]
    CreateBookmarks(String),

    #[error("server error={0}")]
    Server(String),

    #[error("database error={0}")]
    Database(String),

    #[error("web interface error={0}")]
    WebInterface(String),

    #[error("message error={0}")]
    Message(String),

    #[error("not found parent id")]
    NotFoundParentId,

    #[error("not found new folder")]
    NotFoundNewFolder,

    // DatabaseError
    #[error("DOM Exception error: {}", js_object_display(.0))]
    DomException(DomException),

    #[error("unexpected JS type. expected: {}, found: {}", .0, js_object_display(.1))]
    UnexpectedJsType(&'static str, JsValue),

    #[error("serialize error: {0}")]
    ConvertJsValue(#[from] serde_wasm_bindgen::Error),

    #[error("empty value: {0}")]
    EmptyValue(&'static str),

    #[error("empty value: {}", js_object_display(.0))]
    EmptyJsValue(JsValue),

    #[error("js_value error: {0}")]
    JsValue(String),
}

fn js_object_display(option: &JsValue) -> String {
    let object: &Object = option.unchecked_ref();
    ToString::to_string(&object.to_string())
}

impl From<DomException> for DomainError {
    fn from(error: DomException) -> Self {
        Self::DomException(error)
    }
}

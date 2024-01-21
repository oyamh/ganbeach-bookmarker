use serde::{Deserialize, Serialize};

/// runtime.MessageSender
/// https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/MessageSender
/// 例
/// {
///    "contextId":687194767533,
///    "id":"ba46a277b5886fa100d95f02f70897e9dde5b41a@temporary-addon",
///    "envType":"content_child",
///    "url":"moz-extension://3e790633-6481-4f0a-8dd2-36a4e4ce8db2/html/popup.html",
///    "tab":{
///       "id":5,
///       "index":3,
///       "windowId":1,
///       "highlighted":true,
///       "active":true,
///       "attention":false,
///       "pinned":true,
///       "status":"complete",
///       "hidden":false,
///       "discarded":false,
///       "incognito":false,
///       "width":1366,
///       "height":682,
///       "lastAccessed":1674741765568,
///       "audible":false,
///       "mutedInfo":{
///          "muted":false
///       },
///       "isArticle":false,
///       "isInReaderMode":false,
///       "sharingState":{
///          "camera":false,
///          "microphone":false
///       },
///       "successorTabId":-1,
///       "cookieStoreId":"firefox-default",
///       "url":"https://example.com/",
///       "title":"Example Domain"
///    },
///    "frameId":10737418256
/// }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenderInfo {
    pub tab: Option<Tab>,
    #[serde(rename(serialize = "frameId", deserialize = "frameId"))]
    pub frame_id: Option<i64>,
}

impl From<i32> for SenderInfo {
    fn from(id: i32) -> Self {
        Self {
            tab: Some(Tab::new(id)),
            frame_id: None,
        }
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod sender_test {
    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    //wasm-pack test --firefox --headless --lib -- sender_test::should_serialize_sender_info
    //wasm-pack test --chrome --lib -- sender_test::should_serialize_sender_info
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn should_serialize_sender_info() {
        let sender_info = SenderInfo::from(10);
        let result = serde_wasm_bindgen::to_value(&sender_info);
        dbg!("result: {result:#?}");
        assert!(result.is_ok());
        // assert_eq!(wasm_bindgen::JsValue::from_f64(10 as f64), result.unwrap());
        let sender_info = SenderInfo {
            tab: Some(Tab::new(11)),
            frame_id: Some(2),
        };
        let result = serde_wasm_bindgen::to_value(&sender_info);
        assert_eq!(wasm_bindgen::JsValue::from_f64(10 as f64), result.unwrap());
    }
}

/// https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/Tab
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub active: bool,
    pub id: Option<i32>,
    pub title: Option<String>,
    pub url: Option<String>,
    #[serde(rename(deserialize = "windowId"))]
    pub window_id: i32,
}

impl Tab {
    pub fn new(id: i32) -> Self {
        Self {
            active: false,
            id: Some(id),
            title: None,
            url: None,
            window_id: 0,
        }
    }
}

//from JsValue to MessageSender
// serde_wasm_bindgen::from_value::<MessageSender>(sender)
//     .map(|sender| sender.tab.id)
//     .ok()
//     .flatten()

use domain::{DomainError, MessageSender, MessageToBackground, SenderInfo};
use gloo_utils::window;
use serde::Serialize;
use wasm_bindgen::JsValue; // Closureのimport方法が不明なのでprelude::*を使う。
use wasm_bindgen_futures::JsFuture;
use web_interface::{get_url, send_browser_message, send_tab_message};
use web_sys::Window;

use crate::MessageError;

/// https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/sendMessage#parameters
#[derive(Debug, Serialize)]
struct Options {
    #[serde(rename(serialize = "frameId"))]
    frame_id: Option<i64>,
}

#[derive(Debug, Default)]
pub struct Sender;

impl Sender {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MessageSender for Sender {
    fn send_to_parent_frame<T>(&self, message: T) -> Result<(), DomainError>
    where
        T: Serialize,
    {
        log::debug!("Sender::send_to_parent_frame");
        // , target_origin: &str
        let message = serde_wasm_bindgen::to_value(&message).unwrap();
        window()
            .parent()
            .unwrap()
            .unwrap()
            //.post_message(&message, &window().location().origin().unwrap())
            .post_message(&message, "*")
            .map_err(|js_value| MessageError::Content(js_value).into())
    }

    fn send_to_child_frame<T>(
        &self,
        message: T,
        content_window: &Window,
        target_uri: impl AsRef<str>,
    ) -> Result<(), DomainError>
    where
        T: Serialize,
    {
        log::debug!("send_to_child_frame");
        let message = serde_wasm_bindgen::to_value(&message).unwrap();
        log::debug!("target_uri: {:?}", target_uri.as_ref());
        // Url::try_from(target_uri.as_ref())?; //Err(Parse("relative URL without a base")) // example: popup.html
        let inner_url = get_url(target_uri.as_ref());
        content_window
            .post_message(&message, &inner_url)
            .map_err(|js_value| MessageError::Popup(js_value).into())
    }

    async fn send_to_tab<T>(&self, message: T, sender_info: SenderInfo) -> Result<(), DomainError>
    where
        T: Serialize,
    {
        // log::debug!("send_to_tab");
        let tab_id = sender_info
            .tab
            .ok_or(MessageError::Background(JsValue::from_str("no tab")))?
            .id
            .ok_or(MessageError::Background(JsValue::from_str("no tab_id")))?;
        let message = serde_wasm_bindgen::to_value(&message).unwrap();
        let options = Options {
            frame_id: sender_info.frame_id,
        };
        let options = serde_wasm_bindgen::to_value(&options).unwrap();
        let promise = send_tab_message(tab_id, message, options);
        JsFuture::from(promise)
            .await
            .map(|_| ())
            .map_err(|js_value| MessageError::Background(js_value).into())
    }

    async fn send_to_background(&self, message: MessageToBackground) -> Result<(), DomainError> {
        log::debug!("send_inner_message");
        dbg!("test");
        let promise = send_browser_message(serde_wasm_bindgen::to_value(&message).unwrap());
        // let _result = JsFuture::from(promise).await;
        JsFuture::from(promise)
            .await
            .map(|_| ())
            .map_err(|js_value| MessageError::Background(js_value).into())
    }
}

// pub struct Sender;

// impl Sender {
//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn post_message_to_content<T>(&self, message: T) -> Result<(), JsValue>
//     where
//         T: Serialize,
//     {
//         // , target_origin: &str
//         let message = serde_wasm_bindgen::to_value(&message).unwrap();
//         window()
//             .parent()
//             .unwrap()
//             .unwrap()
//             //.post_message(&message, &window().location().origin().unwrap())
//             .post_message(&message, "*")
//     }

//     pub fn post_message_to_popup<T>(&self, content_window: Window, message: T)
//     where
//         T: Serialize,
//     {
//         let message = serde_wasm_bindgen::to_value(&message).unwrap();
//         let inner_url = browser.runtime().get_url("popup.html");
//         content_window.post_message(&message, &inner_url).unwrap();
//     }

//     pub async fn post_message_to_background<T>(&self, message: T)
//     where
//         T: Serialize,
//     {
//         let promise = browser
//             .runtime()
//             .send_message(serde_wasm_bindgen::to_value(&message).unwrap());
//         let _result = JsFuture::from(promise).await;
//     }
// }

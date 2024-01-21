use domain::MessageReceiver;
use serde::Deserialize;
use wasm_bindgen::{prelude::*, JsCast, JsValue}; // Closureのimport方法が不明なのでprelude::*を使う。
use web_interface::on_browser_message;

#[derive(Debug, Default)]
pub struct Receiver;

impl Receiver {
    pub fn new() -> Self {
        Self {}
    }
}

impl MessageReceiver for Receiver {
    /// background <=> popupをlistenする。
    /// background => contentの受信時にもcontent側で使った。
    /// NOTE: 引数callbackにはasync関数は使えない。JsFutureを返す関数を使う。[詳細](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/onMessage)
    /// JsFutureはspawn_local関数を使うと簡単に扱える。
    fn on_extension_message<M, M2, T>(&self, callback: T)
    where
        M: for<'de> Deserialize<'de>,
        M2: for<'de> Deserialize<'de>,
        T: Fn(M, M2) + 'static,
    {
        let closure = Closure::new(Box::new(move |message, sender| {
            log::debug!("on_extension_message closure");
            let message = match serde_wasm_bindgen::from_value::<M>(message) {
                Ok(message) => message,
                Err(error) => {
                    if let Some(_from_js) = error.to_string().find("fromJS") {
                        log::debug!("from js message");
                        return;
                    }
                    log::error!("message deserialize error: {error:?}");
                    return;
                }
            };
            // log::debug!("sender: {:#?}", &sender);
            let sender = match serde_wasm_bindgen::from_value::<M2>(sender) {
                Ok(sender) => sender,
                Err(error) => {
                    log::error!("sender deserialize error: {error:?}");
                    return;
                }
            };
            callback(message, sender);
        })) as Closure<dyn Fn(JsValue, JsValue)>;
        on_browser_message(closure.as_ref().unchecked_ref());
        closure.forget();
    }

    // /// content <=> popupをlistenする。
    // /// contentでbind_message_listenerとして使われていた。app.rsで。
    // fn on_window_message<M, T>(&self, callback: T) -> EventListener
    // where
    //     M: for<'de> Deserialize<'de>,
    //     T: Fn(M) + 'static,
    // {
    //     EventListener::new(&window(), "message", move |e: &Event| {
    //         // log::debug!("on_window_message");
    //         let event = e.dyn_ref::<MessageEvent>().unwrap();
    //         //TODO: eventに付与されたデータから、event生成元のurlをチェック
    //         // let data = event.data();
    //         // log::debug!("on_window_message: event.data()={:?}", &data);
    //         if let Ok(message) = serde_wasm_bindgen::from_value::<M>(event.data()) {
    //             callback(message);
    //         }
    //     })
    // }

    // /// content <=> popupを"一度だけ"listenする。
    // fn once_window_message<M, T>(&self, callback: WindowOnceMessageCallback<M, T>) -> EventListener
    // where
    //     // T: Fn(&MessageEvent) + 'static,
    //     M: for<'de> Deserialize<'de> + 'static,
    //     T: FnOnce(M) + Clone + Copy + 'static,
    // {
    //     EventListener::new(&window(), "message", move |e: &Event| {
    //         let event = e.dyn_ref::<MessageEvent>().unwrap();
    //         if let Ok(message) = serde_wasm_bindgen::from_value::<M>(event.data()) {
    //             callback(message);
    //         }
    //     })
    // }
}

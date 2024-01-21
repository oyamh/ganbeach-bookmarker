use crate::{
    create_notification,
    NotificationError, //on_clicked_notification, on_closed_notification,
    WebInterfaceError,
};
// use domain::NotificationEvent;
use domain::{DomainError, NotificationEmitter, NotificationId, NotificationOptions};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Default)]
pub struct NotificationSender;

impl NotificationSender {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NotificationEmitter for NotificationSender {
    async fn notify(
        &self,
        id: NotificationId,
        message: impl AsRef<str>,
        title: impl AsRef<str>,
    ) -> Result<NotificationId, DomainError> {
        let options = NotificationOptions::new(message, title);
        let js_options = serde_wasm_bindgen::to_value(&options).map_err(|js_error| {
            WebInterfaceError::Notification(NotificationError::Serde(js_error))
        })?;
        // let promise = create_notification(id.into(), js_options).ok_or(
        //     WebInterfaceError::Notification(NotificationError::NoPromise),
        // )?;
        let id_clone = id.clone();
        let Some(promise) = create_notification(id.into(), js_options) else {
            return Ok(id_clone);
        };

        JsFuture::from(promise)
            .await
            .and_then(|js_value| {
                let id = serde_wasm_bindgen::from_value::<String>(js_value)?;
                Ok(NotificationId::parse(id))
            })
            .map_err(|js_error| {
                WebInterfaceError::Notification(NotificationError::JsValue(js_error)).into()
            })
    }
}

// /// notification_idごとの動作の定義を行うための関数を実行する。
// #[derive(Debug, Default)]
// pub struct NotificationReceiver;

// impl NotificationListener for NotificationReceiver {
//     fn listener(&self, event: NotificationEvent, callback: impl Fn(NotificationId) + 'static) {
//         let closure: Closure<dyn Fn(JsValue)> = Closure::new(Box::new(move |js_value| {
//             log::debug!("NotificationReceiver closure");
//             let notification_id = match serde_wasm_bindgen::from_value::<String>(js_value) {
//                 Ok(raw_id) => NotificationId::parse(raw_id),
//                 Err(error) => {
//                     log::debug!("NotificationReceiver error: {error}");
//                     return;
//                 }
//             };
//             callback(notification_id);
//         }) as Box<dyn Fn(JsValue)>);

//         match event {
//             NotificationEvent::OnClicked => {
//                 on_clicked_notification(closure.as_ref().unchecked_ref())
//             }
//             NotificationEvent::OnClosed => on_closed_notification(closure.as_ref().unchecked_ref()),
//         }
//         closure.forget();
//     }
// }

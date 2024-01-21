use crate::{delete_url, get_url, on_visited, FromJsError, HistoryError};
use domain::HistoryCleaner;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[derive(Serialize)]
struct UrlDetails {
    url: String,
}

#[derive(Deserialize)]
struct HistoryItem {
    url: String,
}

async fn delete_history(url: impl AsRef<str> + 'static) -> Result<(), HistoryError> {
    let details = serde_wasm_bindgen::to_value(&UrlDetails {
        url: url.as_ref().to_string(),
    })?;
    let promise = Promise::from(delete_url(&details));
    let result = JsFuture::from(promise)
        .await
        .map_err(Into::<FromJsError>::into)?;
    log::debug!("result={result:#?}");
    Ok(())
}

#[derive(Debug, Default)]
pub struct HistoryObserver;

impl HistoryCleaner for HistoryObserver {
    fn register(&self) {
        log::debug!("HistoryObserver::register");
        let closure = Closure::new(move |js_value| {
            let history_item = match serde_wasm_bindgen::from_value::<HistoryItem>(js_value) {
                Ok(history_item) => history_item,
                Err(error) => {
                    log::error!("{error}");
                    return;
                }
            };
            match history_item.url.starts_with(get_url("").as_str()) {
                true => spawn_local(async move {
                    if let Err(error) = delete_history(history_item.url).await {
                        log::error!("{error}");
                        return;
                    };
                }),
                false => {}
            };
        }) as Closure<dyn Fn(JsValue)>;
        on_visited(closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

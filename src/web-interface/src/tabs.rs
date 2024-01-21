use crate::{create_tab, query_tab, FromJsError, TabsError}; //, query_tab, send_tab_message
use domain::{AuthAgent, DomainError, LinkOpener, LoginUrl, Tab};
use serde::Serialize;
use url::Url;
use wasm_bindgen_futures::JsFuture;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::JsFuture;

#[derive(Default)]
pub struct TabController;

impl TabController {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AuthAgent for TabController {
    async fn login(&self, login_url: &LoginUrl) -> Result<(), DomainError> {
        open_tab(login_url.into())
            .await
            .map_err(|error| DomainError::WebInterface(error.to_string()))
    }
}

impl LinkOpener for TabController {
    async fn open(&self, url: &domain::Url) -> Result<(), DomainError> {
        open_tab(url.into())
            .await
            .map_err(|error| DomainError::WebInterface(error.to_string()))
    }
}

/// test
#[derive(Serialize)]
struct CreateProperties {
    url: String,
}

async fn open_tab(url: Url) -> Result<(), serde_wasm_bindgen::Error> {
    let properties = serde_wasm_bindgen::to_value(&CreateProperties { url: url.into() })?;
    let _promise = create_tab(properties);
    Ok(())
}

#[derive(Debug, Default, Serialize)]
pub(crate) struct TabQuery {
    active: bool,
    #[serde(rename(serialize = "lastFocusedWindow"))]
    last_focused_window: bool,
}

impl TabQuery {
    pub(crate) fn current() -> Self {
        Self {
            active: true,
            last_focused_window: true,
        }
    }
}

pub(crate) async fn get_current_tab(query: TabQuery) -> Result<Tab, TabsError> {
    let promise = query_tab(serde_wasm_bindgen::to_value(&query)?);
    let result = JsFuture::from(promise)
        .await
        .map_err(Into::<FromJsError>::into)?;
    log::debug!("result={result:#?}");
    let vec_tabs = serde_wasm_bindgen::from_value::<Vec<Tab>>(result)?;
    log::debug!("vec_tabs={vec_tabs:#?}");
    let tab = vec_tabs
        .iter()
        .find(|&t| t.active)
        .ok_or(TabsError::NoActiveTab)?;
    log::debug!("tab={tab:#?}");
    Ok(tab.clone())
}

// async fn post_frontend_message(tab_id: i32, message: JsValue) -> Result<JsValue, JsValue> {
//     let promise = send_tab_message(tab_id, message);
//     JsFuture::from(promise).await
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct TabsQueryOptions {
//     active: bool,
//     // current_window: bool,
// }

// #[derive(Debug, Deserialize)]
// struct Tab {
//     pub active: bool,
//     pub id: Option<i32>,
// }

// async fn get_current_tab_id() -> Option<i32> {
//     let options = TabsQueryOptions {
//         active: true,
//         // current_window: true,
//     };
//     let promise = query_tab(serde_wasm_bindgen::to_value(&options).unwrap());

//     let result = JsFuture::from(promise).await.ok()?;
//     let vec_tabs = serde_wasm_bindgen::from_value::<Vec<Tab>>(result).ok()?;
//     let tab = vec_tabs.iter().find(|&t| t.active)?;
//     tab.id
// }

// // pub fn get_current_tab_id() -> Option<i32> {
// //     browser.tabs().tab().id()
// // }

// // pub type TabId = i32;

// // #[wasm_bindgen]
// // extern "C" {
// //     pub type Tab;
// //     pub static tab: Tab;

// //     #[wasm_bindgen(method, getter)]
// //     pub fn active(this: &Tab) -> bool;
// //     #[wasm_bindgen(method, getter)]
// //     pub fn id(this: &Tab) -> Option<i32>;
// // }

// // pub fn get_current_tab_id() -> Option<i32> {
// //     // browser.tabs().tab().id()
// //     let tabs = browser.tabs();
// //     // log::debug!("tabs: {tabs:?}");
// //     let tab1 = tabs.tab();
// //     // log::debug!("tab1: {tab1:?}");
// //     let id = tab1.id();
// //     log::debug!("id: {id:?}");
// //     id
// // }

// #[derive(Debug, Serialize, Deserialize)]
// struct TabsQueryOptions {
//     active: bool,
//     current_window: bool,
// }

// #[derive(Debug, Deserialize)]
// struct Tab {
//     pub active: bool,
//     pub id: Option<i32>,
// }

// pub async fn get_current_tab_id() -> i32 {
//     log::debug!("get_current_tab_id");
//     let options = TabsQueryOptions {
//         active: true,
//         current_window: true,
//     };
//     let promise = browser
//         .tabs()
//         .query(serde_wasm_bindgen::to_value(&options).unwrap());

//     // let o = Object::from(serde_wasm_bindgen::to_value(&options).unwrap());
//     // let promise = browser.tabs().query(&o);

//     let result = JsFuture::from(promise).await.unwrap();
//     // log::debug!("result: {result:?}");
//     // .expect("translate Promise to JsFuture");
//     let vec_tabs = serde_wasm_bindgen::from_value::<Vec<Tab>>(result).unwrap();
//     // .expect("serialize JsValue into Vec<Tab>");
//     let tab = vec_tabs.iter().find(|&t| t.active).unwrap();
//     // .expect("find active tab");
//     tab.id.unwrap()
//     // .expect("get tab id")
// }

// // domainに定義して、extract_tab_id処理はconvert.rsファイルの型変換の定義を使いつつのmessageパッケージに任せた。
// // #[derive(Debug, Deserialize)]
// // struct MessageSender {
// //     pub tab: Tab,
// // }

// // pub fn extract_tab_id(sender: JsValue) -> Option<i32> {
// //     // sender
// //     //     .into_serde::<MessageSender>()
// //     //     .map(|sender| sender.tab.id)
// //     //     .ok()
// //     //     .flatten()

// //     serde_wasm_bindgen::from_value::<MessageSender>(sender)
// //         .map(|sender| sender.tab.id)
// //         .ok()
// //         .flatten()

// //     // serde_wasm_bindgen::from_value::<MessageSender>(sender)
// //     //     .and_then(|sender| Ok(sender.tab.id))
// //     //     .ok()
// //     //     .flatten()
// // }

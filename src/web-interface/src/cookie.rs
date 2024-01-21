use crate::{get_browser_cookie, remove_browser_cookie, set_panic_hook};
use domain::{DomainError, SecretAccessor};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
// use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

/// https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/cookies/Cookie
#[derive(Debug, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    /*
    以下のプロパティは、今回の用途には必要ないと判断してコメントアウトした。
    host_only,http_onlyやsecureなどの属性はブラウザ内部での扱いを設定するために使われる。
    このプログラムでの用途は、一時的に送信Clientのheaderに値を使うだけで、ブラウザ内部に保存し直したりしないし、自動的に送信したりもしない。
    */
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub domain: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub path: String,
    // #[serde(default)]
    // pub secure: bool,
    // #[serde(default, rename = "hostOnly")]
    // pub host_only: bool,
    // #[serde(default, rename = "httpOnly")]
    // pub http_only: bool,
    // #[serde(default)]
    // pub session: bool,
    // #[serde(default, rename = "sameSite")]
    // pub same_site: String,

    // #[serde(skip_serializing_if = "Option::is_none", rename = "expirationDate")]
    // pub expiration_date: Option<u32>,
    // #[serde(default, rename = "storeId")]
    // pub store_id: String,
    // //
    // // firefoxだけ。firefoxでも基本は空だった。
    // // #[serde(default, rename = "firstPartyDomain")]
    // // pub first_party_domain: String,
    // // firefoxだけ。firefoxでも基本はnullだった。
    // // #[serde(skip_serializing_if = "Option::is_none", rename = "partitionKey")]
    // // pub partition_key: Option<>, // JsValue? Object? // 初期値はnull
}

//JsValue(Object({"name":"refresh-token","value":"","domain":"localhost","hostOnly":true,"path":"/","secure":false,"httpOnly":true,"sameSite":"no_restriction","session":false,"firstPartyDomain":"","partitionKey":null,"expirationDate":1668645034,"storeId":"firefox-default"}))

#[derive(Serialize, Debug, Default)]
pub struct CookieAccessor;

impl CookieAccessor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SecretAccessor for CookieAccessor {
    type Error = DomainError;
    async fn get<T>(&self, details: T) -> Result<String, Self::Error>
    where
        T: Serialize,
    {
        set_panic_hook();
        let js_details = serde_wasm_bindgen::to_value(&details)?;
        let promise = Promise::from(get_browser_cookie(js_details));
        let result = JsFuture::from(promise).await.map_err(|error| {
            DomainError::JsValue(
                error
                    .as_string()
                    .unwrap_or("CookieAccessor::get".to_string()),
            )
        })?;
        if result.is_null() {
            let js_details = serde_wasm_bindgen::to_value(&details)?;
            return Err(DomainError::EmptyJsValue(js_details));
        }
        let cookie = serde_wasm_bindgen::from_value::<Cookie>(result)?;
        Ok(cookie.value)
    }

    async fn delete<T>(&self, details: T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        set_panic_hook();
        let js_details = serde_wasm_bindgen::to_value(&details)?;
        let promise = Promise::from(remove_browser_cookie(js_details));
        let result = JsFuture::from(promise).await.map_err(|error| {
            DomainError::JsValue(
                error
                    .as_string()
                    .unwrap_or("CookieAccessor::delete".to_string()),
            )
        })?;
        if result.is_null() {
            let js_details = serde_wasm_bindgen::to_value(&details)?;
            return Err(DomainError::EmptyJsValue(js_details));
        }
        Ok(())
    }
}

// #[derive(Debug, Deserialize)]
// pub struct ChangeInfo {
//     removed: bool,
//     cookie: Cookie,
//     // on_changed_cause: OnChangedCause,
// }

// impl From<JsValue> for ChangeInfo {
//     fn from(src: JsValue) -> Self {
//         serde_wasm_bindgen::from_value::<ChangeInfo>(src)
//             .expect("failed to convert ChangeInfo from JsValue")
//     }
// }

// // /// https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/cookies/OnChangedCause
// // #[derive(Debug, Deserialize)]
// // enum OnChangedCause {
// //     Evicted,
// //     Expired,
// //     Explicit,
// //     ExpiredOverwrite,
// //     Overwrite,
// // }
//
// // impl TryFrom<String> for OnChangedCause {
// //     type Error = CookieError;
// //     fn try_from(input: String) -> Result<Self, Self::Error> {
// //         let output = match input.as_str() {
// //             "evicted" => Self::Evicted,
// //             "expired" => Self::Expired,
// //             "explicit" => Self::Explicit,
// //             "expired_overwrite" => Self::ExpiredOverwrite,
// //             "overwrite" => Self::Overwrite,
// //             _ => return Err(Self::Error::UnknownChangedCause(input)),
// //         };
// //         Ok(output)
// //     }
// // }

// #[derive(Debug, Default)]
// pub struct CookieObserver;

// impl CookieObserver {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

// impl RefreshTokenObserver for CookieObserver {
//     //TODO: httpOnlyのCookieは検知できない可能性がある。(セキュリティリスクにより)
//     fn on_changed(&self, callback: impl Fn(Option<RefreshToken>) + 'static) {
//         log::debug!("RefreshTokenObserver::on_changed");
//         let closure = Closure::new(move |js_value: JsValue| {
//             log::debug!("RefreshTokenObserver::on_changed closure");
//             let change_info = match serde_wasm_bindgen::from_value::<ChangeInfo>(js_value) {
//                 Ok(change_info) => change_info,
//                 Err(error) => {
//                     log::error!("{error}");
//                     return;
//                 }
//             };
//             log::debug!("1");
//             if change_info.cookie.name != REFRESH_TOKEN_NAME {
//                 return;
//             }
//             log::debug!("2");
//             if change_info.removed || change_info.cookie.value.is_empty() {
//                 callback(None);
//                 return;
//             }
//             log::debug!("3");
//             // OnChangedCauseを使う？
//             //いらないか？ほとんどがremovedで解決可能な気がする。一旦使ってみて、問題があれば使う。
//             let refresh_token = RefreshToken::from(change_info.cookie);
//             callback(Some(refresh_token));
//             log::debug!("4");
//         }) as Closure<dyn Fn(JsValue)>;
//         on_change_cookie(closure.as_ref().unchecked_ref());
//         closure.forget();
//     }
// }

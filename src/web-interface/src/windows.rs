use std::fmt::Display;

use crate::{create_window, get_current_tab, get_url, TabQuery};
use crate::{get_window, TabsError};
use domain::{DomainError, PopupOpener, PopupOrigin, Tab, Url, UrlQueryBuilder};
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Default)]
pub struct WindowsController;

impl WindowsController {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PopupOpener for WindowsController {
    async fn open_popup(&self, tab: Option<Tab>) -> Result<(), DomainError> {
        open_window(tab)
            .await
            .map_err(|error| DomainError::WebInterface(error.to_string()))
    }
}

#[derive(Serialize)]
struct CreateData {
    url: String,
    focused: bool,
    #[serde(rename(serialize = "type"))]
    kind: String,
    width: i32,
    height: i32,
    top: i32,
    left: i32,
}

const POPUP_WINDOW_WIDTH: i32 = 330;

async fn open_window(tab: Option<Tab>) -> Result<(), TabsError> {
    let query = TabQuery::current();
    log::debug!("query={query:#?}");
    let tab = match tab {
        Some(tab) => tab,
        None => get_current_tab(query).await?,
    };
    log::debug!("tab={tab:#?}");

    let window = get_window_by_id(tab.window_id).await?;
    log::debug!("window={window:#?}");

    let left = if let Some(width) = window.width {
        width - POPUP_WINDOW_WIDTH - 50
    } else {
        75
    };

    let base_url = Url::try_from(get_url("html/popup.html"))
        .map_err(|error| TabsError::ParseUrl(error.to_string()))?;
    let query_params = PopupOrigin::from(tab);
    let popup_url = PopupUrl {
        base_url,
        query_params,
    };
    log::debug!("popup_url={popup_url:?}");

    let window_data = &CreateData {
        url: popup_url.to_string(),
        focused: true,
        kind: "popup".to_string(),
        width: POPUP_WINDOW_WIDTH,
        height: 440,
        top: 75,
        left,
    };

    let data = serde_wasm_bindgen::to_value(window_data)?;
    let _promise = create_window(data);
    Ok(())
}

#[derive(Debug, Default, Deserialize)]
struct Window {
    width: Option<i32>,
}

async fn get_window_by_id(window_id: i32) -> Result<Window, serde_wasm_bindgen::Error> {
    let promise = get_window(window_id);
    let result = JsFuture::from(promise).await?;
    serde_wasm_bindgen::from_value(result)
}

#[derive(Debug)]
struct PopupUrl<T>
where
    T: UrlQueryBuilder,
{
    base_url: Url,
    query_params: T,
}

impl<T> Display for PopupUrl<T>
where
    T: UrlQueryBuilder,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = self
            .query_params
            .append_pairs(self.base_url.clone())
            .unwrap();
        write!(f, "{}", url)
    }
}

#[cfg(test)]
mod window_url {
    use domain::{Url, UrlQueryBuilder};

    use crate::windows::PopupUrl;
    use domain::PopupOrigin;
    #[test]
    fn should_get_popup_url() {
        let base_url =
            Url::try_from("chrome-extension://fmjeomnojofhdmiifbmibbigjmclbnpl/html/popup.html");
        assert!(base_url.is_ok());
        let base_url = base_url.unwrap();
        let query_params = PopupOrigin {
            title: "無料アニメ動画｜ニコニコのアニメサイト：Nアニメ".to_string(),
            url: "https://anime.nicovideo.jp/free/".to_string(),
            tab_id: 201396,
            window_id: 201395,
        };
        let popup_url = PopupUrl {
            base_url,
            query_params,
        };
        println!("{popup_url}");
        assert_eq!(popup_url.to_string(), "chrome-extension://fmjeomnojofhdmiifbmibbigjmclbnpl/html/popup.html?title=%E7%84%A1%E6%96%99%E3%82%A2%E3%83%8B%E3%83%A1%E5%8B%95%E7%94%BB%EF%BD%9C%E3%83%8B%E3%82%B3%E3%83%8B%E3%82%B3%E3%81%AE%E3%82%A2%E3%83%8B%E3%83%A1%E3%82%B5%E3%82%A4%E3%83%88%EF%BC%9AN%E3%82%A2%E3%83%8B%E3%83%A1&url=https%3A%2F%2Fanime.nicovideo.jp%2Ffree%2F&tab=201396&window=201395".to_string());

        let params = PopupOrigin::try_from(popup_url.to_string()).unwrap();
        println!("{params:#?}");

        // let decoded_url = Url::try_from(popup_url.to_string());
        // assert!(decoded_url.is_ok());
        // let decoded_url = decoded_url.unwrap();
        // let mut pairs = decoded_url.query_pairs();
        // println!("pairs count={}", pairs.count());
        // assert_eq!(pairs.count(), 4);
        // println!("{:?}", pairs.next());
        // println!("{:?}", pairs.next());
        // println!("{:?}", pairs.next());
        // println!("{:?}", pairs.next());
    }

    #[test]
    fn should_append_pair() {
        //chrome-extension://fmjeomnojofhdmiifbmibbigjmclbnpl/html/popup.html?title=無料アニメ動画｜ニコニコのアニメサイト：Nアニメ&url=https://anime.nicovideo.jp/free/&tab=201396&window=201395
        let base_url =
            Url::try_from("chrome-extension://fmjeomnojofhdmiifbmibbigjmclbnpl/html/popup.html")
                .unwrap();
        let params = PopupOrigin {
            title: "無料アニメ動画｜ニコニコのアニメサイト：Nアニメ".to_string(),
            url: "https://anime.nicovideo.jp/free/".to_string(),
            tab_id: 201396,
            window_id: 201395,
        };
        println!("{params:#?}");
        let result = params.append_pairs(base_url);
        println!("{result:#?}");
        assert!(result.is_ok());
        let url = result.unwrap();

        let mut pairs = url.query_pairs();
        println!("pairs count={}", pairs.count());
        println!("{:?}", pairs.next());
        println!("{:?}", pairs.next());
        println!("{:?}", pairs.next());
        println!("{:?}", pairs.next());
    }
}

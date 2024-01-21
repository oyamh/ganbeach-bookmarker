use std::fmt::Display;

use crate::{DomainError, Tab, Url, UrlQueryBuilder};

#[derive(Debug, Default)]
pub struct PopupOrigin {
    pub title: String,
    pub url: String,
    pub tab_id: i32,
    pub window_id: i32,
}

impl UrlQueryBuilder for PopupOrigin {
    fn append_pairs(&self, mut base_url: Url) -> Result<Url, DomainError> {
        base_url
            .query_pairs_mut()
            .append_pair("title", self.title.as_str())
            .append_pair("url", self.url.as_str())
            .append_pair("tab", self.tab_id.to_string().as_str())
            .append_pair("window", self.window_id.to_string().as_str())
            .finish();
        Ok(base_url)
    }
}

impl From<Tab> for PopupOrigin {
    fn from(src: Tab) -> Self {
        Self {
            title: src.title.unwrap_or_default(),
            url: src.url.unwrap_or_default(),
            tab_id: src.id.unwrap_or_default(),
            window_id: src.window_id,
        }
    }
}

impl TryFrom<String> for PopupOrigin {
    type Error = DomainError;
    fn try_from(src: String) -> Result<Self, Self::Error> {
        let url = Url::try_from(src)?;
        Ok(PopupOrigin::from(url))
    }
}

impl From<Url> for PopupOrigin {
    fn from(src: Url) -> Self {
        src.query_pairs()
            .into_iter()
            .fold(Self::default(), |mut params, (key, value)| {
                match key.as_ref() {
                    "title" => params.title = value.as_ref().to_string(),
                    "url" => params.url = value.as_ref().to_string(),
                    "tab" => params.tab_id = value.as_ref().parse().unwrap_or_default(),
                    "window" => params.window_id = value.as_ref().parse().unwrap_or_default(),
                    _ => {}
                };
                params
            })
    }
}

impl Display for PopupOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "title={} url={} tab={} window={}",
            self.title, self.url, self.tab_id, self.window_id
        )
    }
}

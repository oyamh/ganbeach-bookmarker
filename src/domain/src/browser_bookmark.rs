use serde::{Deserialize, Serialize};

use crate::{Title, Url};

/// [CreateDetails](https://developer.mozilla.org/ja/docs/Mozilla/Add-ons/WebExtensions/API/bookmarks/CreateDetails)
#[derive(Debug, Serialize)]
pub struct CreateDetails {
    #[serde(rename(serialize = "parentId", deserialize = "parentId"))]
    parent_id: Option<String>,
    // index: Option<i64>,
    title: Option<Title>,
    url: Option<Url>,
}

impl CreateDetails {
    pub fn new(
        title: impl AsRef<str>,
        url: impl AsRef<str>,
        parent_id: Option<impl AsRef<str>>,
    ) -> Self {
        let url = url.as_ref().try_into().ok();
        let parent_id = parent_id.map(|id| id.as_ref().to_string());
        Self {
            title: Some(title.as_ref().into()),
            url,
            parent_id,
        }
    }
}

/// [Query](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/bookmarks/search)
#[derive(Debug, Serialize)]
pub struct BrowserBookmarksQuery {
    title: Option<Title>,
}

impl BrowserBookmarksQuery {
    pub fn new(title: impl AsRef<str>) -> Self {
        Self {
            title: Some(title.as_ref().into()),
        }
    }
}

/// [bookmarks.BookmarkTreeNode](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/bookmarks/BookmarkTreeNode)
#[derive(Debug, Default, Deserialize)]
pub struct BookmarkTreeNode {
    // title: String,
    id: Option<String>,
    url: Option<String>,
    // Defaults to "bookmark" unless url is omitted, in which case it defaults to "folder".
    // #[serde(rename(deserialize = "type"))]
    // node_type: Option<String>,
}

impl BookmarkTreeNode {
    pub fn id(&self) -> &Option<String> {
        &self.id
    }
    pub fn is_folder(&self) -> bool {
        match self.url {
            Some(_) => false,
            None => true,
        }
    }
}

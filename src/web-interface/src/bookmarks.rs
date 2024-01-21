use domain::Title;
use domain::{
    BookmarkTreeNode, BrowserBookmarker, BrowserBookmarksQuery, CreateDetails, DomainError, PageUrl,
};
use wasm_bindgen_futures::JsFuture;

use crate::{create_bookmark, search_bookmark, BookmarksError, FromJsError, WebInterfaceError};

#[derive(Debug, Default)]
pub struct Bookmarks;

impl Bookmarks {
    pub fn new() -> Self {
        Self {}
    }

    async fn create(
        &self,
        title: Title,
        url: PageUrl,
        parent_title: Title,
    ) -> Result<(), BookmarksError> {
        log::debug!("Bookmarks.create");

        let query = BrowserBookmarksQuery::new(parent_title);
        let js_query = serde_wasm_bindgen::to_value(&query)?;
        let promise = search_bookmark(&js_query);

        let current_folder = JsFuture::from(promise)
            .await
            .and_then(|js_value| {
                Ok(serde_wasm_bindgen::from_value::<Vec<BookmarkTreeNode>>(
                    js_value,
                )?)
            })
            .map_err(Into::<FromJsError>::into)?
            .into_iter()
            .find(|bookmark| bookmark.is_folder());

        let parent_id = current_folder
            .as_ref()
            .and_then(|folder| folder.id().as_ref());
        let details = CreateDetails::new(title, url, parent_id);

        let promise = create_bookmark(&serde_wasm_bindgen::to_value(&details)?);
        JsFuture::from(promise)
            .await
            .map_err(Into::<FromJsError>::into)?;

        Ok(())
    }

    async fn create_in_root(&self, title: Title, url: PageUrl) -> Result<(), BookmarksError> {
        log::debug!("Bookmarks.create_in_root");
        let details = CreateDetails::new(title, url, None::<String>);
        let promise = create_bookmark(&serde_wasm_bindgen::to_value(&details)?);
        JsFuture::from(promise)
            .await
            .map_err(Into::<FromJsError>::into)?;
        Ok(())
    }
}

impl BrowserBookmarker for Bookmarks {
    async fn create(
        &self,
        title: Title,
        url: PageUrl,
        parent_title: Option<Title>,
    ) -> Result<(), DomainError> {
        let result = match parent_title {
            Some(parent_title) => self.create(title, url, parent_title).await,
            None => self.create_in_root(title, url).await,
        };
        Ok(result.map_err(Into::<WebInterfaceError>::into)?)
        // Ok(self
        //     .create(title, url, parent_title)
        //     .await
        //     .map_err(Into::<WebInterfaceError>::into)?)
    }
}

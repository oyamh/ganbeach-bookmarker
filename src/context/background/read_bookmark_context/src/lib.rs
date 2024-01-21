#![allow(refining_impl_trait)]
use domain::BookmarkReaderProvider;
use grpc_interface::BookmarkGetterClient;

#[derive(Default)]
pub struct ReadBookmarkContext {
    bookmark_getter_client: BookmarkGetterClient,
}

impl ReadBookmarkContext {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl BookmarkReaderProvider for ReadBookmarkContext {
    fn provide(&self) -> &BookmarkGetterClient {
        &self.bookmark_getter_client
    }
}

#![allow(refining_impl_trait)]
use config::{DATABASE_VERSION, HISTORY_DATABASE_NAME};
use domain::{
    BookmarkCreatorProvider, BrowserBookmarkerProvider, DatabaseAccessorProvider, DomainError,
};
use grpc_interface::BookmarkCreatorClient;
use repository::Repository;
use web_interface::Bookmarks;

pub struct CreateBookmarkContext {
    creator: BookmarkCreatorClient,
    bookmarks: Bookmarks,
    history_repository: Repository,
}

impl CreateBookmarkContext {
    pub fn new() -> Self {
        Self {
            creator: BookmarkCreatorClient::new(),
            bookmarks: Bookmarks::new(),
            history_repository: Repository::new(HISTORY_DATABASE_NAME, DATABASE_VERSION),
        }
    }
}

impl BookmarkCreatorProvider for CreateBookmarkContext {
    fn provide(&self) -> &BookmarkCreatorClient {
        &self.creator
    }
}

impl BrowserBookmarkerProvider for CreateBookmarkContext {
    fn provide(&self) -> &Bookmarks {
        &self.bookmarks
    }
}

impl DatabaseAccessorProvider for CreateBookmarkContext {
    type Error = DomainError;
    fn provide(&self) -> &impl domain::DatabaseAccessor<Error = Self::Error> {
        &self.history_repository
    }
}

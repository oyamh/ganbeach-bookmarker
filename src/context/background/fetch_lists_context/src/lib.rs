#![allow(refining_impl_trait)]
use config::{DATABASE_VERSION, LISTS_DATABASE_NAME};
use domain::{BookmarkReaderProvider, DatabaseAccessorProvider, DomainError};
use grpc_interface::BookmarkGetterClient;
use repository::Repository;

/// Listsの取得を行うためのContext。
/// サーバーと通信して全Listsを取得し、データベースに保管する。
/// データは常にputで上書きされる。
pub struct FetchListsContext {
    bookmark_getter_client: BookmarkGetterClient,
    lists_repository: Repository,
}

impl FetchListsContext {
    pub fn new() -> Self {
        Self {
            bookmark_getter_client: BookmarkGetterClient::new(),
            lists_repository: Repository::new(LISTS_DATABASE_NAME, DATABASE_VERSION),
        }
    }
}

impl BookmarkReaderProvider for FetchListsContext {
    fn provide(&self) -> &BookmarkGetterClient {
        &self.bookmark_getter_client
    }
}

impl DatabaseAccessorProvider for FetchListsContext {
    type Error = DomainError;
    fn provide(&self) -> &Repository {
        &self.lists_repository
    }
}

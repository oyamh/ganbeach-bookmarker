#![allow(refining_impl_trait)]
use config::{DATABASE_VERSION, USER_DATA_DATABASE_NAME};
use domain::{AccounterProvider, DatabaseAccessorProvider, DomainError, SecretAccessorProvider};
use grpc_interface::AccounterClient;
use repository::Repository;
use web_interface::CookieAccessor;

pub struct LoadAccessTokenContext {
    accounter_client: AccounterClient,
    userdata_repository: Repository,
    secret_accessor: CookieAccessor,
}

impl LoadAccessTokenContext {
    pub fn new() -> Self {
        Self {
            accounter_client: AccounterClient::new(),
            userdata_repository: Repository::new(USER_DATA_DATABASE_NAME, DATABASE_VERSION),
            secret_accessor: CookieAccessor::new(),
        }
    }
}

impl AccounterProvider for LoadAccessTokenContext {
    fn provide(&self) -> &AccounterClient {
        &self.accounter_client
    }
}

impl DatabaseAccessorProvider for LoadAccessTokenContext {
    type Error = DomainError;
    fn provide(&self) -> &Repository {
        &self.userdata_repository
    }
}

impl SecretAccessorProvider for LoadAccessTokenContext {
    type Error = DomainError;
    fn provide(&self) -> &CookieAccessor {
        &self.secret_accessor
    }
}

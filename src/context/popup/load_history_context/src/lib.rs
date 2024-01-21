#![allow(refining_impl_trait)]
use config::{DATABASE_VERSION, HISTORY_DATABASE_NAME};
use domain::{DatabaseAccessorProvider, DomainError};
use repository::Repository;

pub struct LoadHistoryContext {
    history_repository: Repository,
}

impl LoadHistoryContext {
    pub fn new() -> Self {
        Self {
            history_repository: Repository::new(HISTORY_DATABASE_NAME, DATABASE_VERSION),
        }
    }
}

impl DatabaseAccessorProvider for LoadHistoryContext {
    type Error = DomainError;
    fn provide(&self) -> &impl domain::DatabaseAccessor<Error = Self::Error> {
        &self.history_repository
    }
}

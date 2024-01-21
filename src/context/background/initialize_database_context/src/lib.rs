#![allow(refining_impl_trait)]
use domain::DatabaseInitializerProvider;
use indexed_db::Initializer;

#[derive(Default)]
pub struct InitializeDatabaseContext {
    initializer: Initializer,
}

impl InitializeDatabaseContext {
    pub fn new() -> Self {
        Default::default()
    }
}

impl DatabaseInitializerProvider for InitializeDatabaseContext {
    fn provide(&self) -> &Initializer {
        &self.initializer
    }
}

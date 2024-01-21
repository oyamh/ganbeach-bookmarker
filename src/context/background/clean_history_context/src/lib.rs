#![allow(refining_impl_trait)]
use domain::HistoryCleanerProvider;
use web_interface::HistoryObserver;

#[derive(Debug, Default)]
pub struct CleanHistoryContext {
    cleaner: HistoryObserver,
}

impl CleanHistoryContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HistoryCleanerProvider for CleanHistoryContext {
    fn provide(&self) -> &HistoryObserver {
        &self.cleaner
    }
}

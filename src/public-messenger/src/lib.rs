mod error_message_map;
mod error_title_map;

use domain::{DomainError, ErrorMessenger};
use error_message_map::{ErrorMessageMatcher, Language};
use error_title_map::ErrorTitleMatcher;

#[derive(Debug, Default)]
pub struct Messenger {
    language: Language,
}

impl Messenger {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl ErrorMessenger for Messenger {
    fn error_message(&self, error: &DomainError) -> &'static str {
        ErrorMessageMatcher::get_message(error, &self.language)
    }

    fn error_title(&self, error: &DomainError) -> &'static str {
        ErrorTitleMatcher::get_title(error)
    }
}

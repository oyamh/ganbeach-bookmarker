use std::fmt;

use config::{SERVER_URL_ACCOUNT, SERVER_URL_BASE, SERVER_URL_BOOKMARK};

use crate::Url;

#[derive(Debug, Default, Clone, Copy)]
pub enum ServerUrl {
    #[default]
    Home,
    Accounter,
    BookmarkGetter,
    BookmarkCreator,
}

impl ServerUrl {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Home => SERVER_URL_BASE,
            Self::Accounter => SERVER_URL_ACCOUNT,
            Self::BookmarkGetter => SERVER_URL_BOOKMARK,
            Self::BookmarkCreator => SERVER_URL_BOOKMARK,
        }
    }
}

impl AsRef<str> for ServerUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ServerUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<Url> for ServerUrl {
    fn into(self) -> Url {
        Url::try_from(Self::Home.as_ref()).unwrap()
    }
}

use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};
use url::{form_urlencoded, UrlQuery};

use crate::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url(url::Url);

// impl ToOwned for Url {
//     type Owned = Url;
//     fn to_owned(&self) -> Self::Owned {
//         Self(self.0.to_owned())
//     }
// }

impl Url {
    fn parse(url_str: impl AsRef<str>) -> Result<Self, DomainError> {
        Ok(Self(
            url::Url::parse(url_str.as_ref()).map_err(|error| DomainError::ParseUrl(error))?,
        ))
    }

    pub fn query(&self) -> Option<&str> {
        self.0.query()
    }

    pub fn query_pairs(&self) -> form_urlencoded::Parse<'_> {
        self.0.query_pairs()
    }

    pub fn query_pairs_mut(&mut self) -> form_urlencoded::Serializer<'_, UrlQuery<'_>> {
        self.0.query_pairs_mut()
    }
}

impl TryFrom<&str> for Url {
    type Error = DomainError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for Url {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

#[test]
fn should_decode_parse_encoded() {
    // let url = Url::parse("https://test.com/path?title=Test Title&query=クエリ");
    let url = Url::parse("chrome-extension://fmjeomnojofhdmiifbmibbigjmclbnpl/html/popup.html?title=thisistesttitle&url=https://example.com");
    println!("{url:#?}");
    assert!(url.is_ok());
    let url = url.unwrap();
    println!("{url}");

    let mut pairs = url.query_pairs();
    println!("pairs count={}", pairs.count());
    println!("{:?}", pairs.next());
    println!("{:?}", pairs.next());
    println!("{:?}", pairs.next());
    println!("{:?}", pairs.next());
}

#[test]
fn should_parse_extension_url() {
    let url_str = "moz-extension://231cbde7-25e9-4463-9073-7904f12848fe/popup.html";
    let result = Url::try_from(url_str);
    assert!(result.is_ok());
    println!("{result:#?}");

    let url_str = "chrome-extension://231cbde7-25e9-4463-9073-7904f12848fe/popup.html";
    let result = Url::try_from(url_str);
    assert!(result.is_ok());
    println!("{result:#?}");
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0.as_ref()
    }
}

impl From<url::Url> for Url {
    fn from(inner: url::Url) -> Self {
        Self(inner)
    }
}

impl Into<url::Url> for Url {
    fn into(self) -> url::Url {
        self.0
    }
}

impl Into<url::Url> for &Url {
    fn into(self) -> url::Url {
        self.0.to_owned()
    }
}

impl Deref for Url {
    type Target = url::Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

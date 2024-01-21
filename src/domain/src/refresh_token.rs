use std::fmt::Display;

use config::SERVER_URL_ACCOUNT;
use serde::{Deserialize, Serialize};

use crate::{CookieDetails, DomainError, MetadataHeader, SecretAccessor, SecretAccessorProvider};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RefreshToken(String);

impl RefreshToken {
    pub fn new(token_str: &str) -> Self {
        Self(token_str.to_string())
    }

    pub fn is_valid(&self) -> bool {
        // self.0.is_valid()
        false
    }
}

const COOKIE_DOMAIN_URL: &'static str = SERVER_URL_ACCOUNT;
const REFRESH_TOKEN_NAME: &'static str = "refresh-token";

impl RefreshToken {
    pub async fn load<T>(ctx: &T) -> Result<Self, DomainError>
    where
        T: SecretAccessorProvider<Error = DomainError>,
    {
        let accessor = SecretAccessorProvider::provide(ctx);
        let details = CookieDetails {
            name: REFRESH_TOKEN_NAME,
            url: COOKIE_DOMAIN_URL,
        };
        Ok(accessor
            .get(details)
            .await
            .map(Into::<RefreshToken>::into)?)
    }

    // pub async fn delete<T>(accessor: &T) -> Result<(), DomainError>
    // where
    //     T: SecretAccessor<Error = DomainError>,
    // {
    //     let details = CookieDetails {
    //         name: REFRESH_TOKEN_NAME,
    //         url: COOKIE_DOMAIN_URL,
    //     };
    //     Ok(accessor.delete(details).await?)
    // }
}

impl Display for RefreshToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for RefreshToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<RefreshToken> for String {
    fn from(token: RefreshToken) -> Self {
        token.0
    }
}

impl AsRef<str> for RefreshToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl MetadataHeader for RefreshToken {
    fn as_header(&self) -> (&str, String) {
        ("cookie", self.to_string())
    }
}

use std::{fmt::Display, ops::Deref};

use url::Url;

use crate::{AuthAgent, AuthAgentProvider, DomainError};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct LoginUrl(Url);

impl LoginUrl {
    // pub fn new(url_str: impl AsRef<str>) -> Self {
    //     Self(Url::parse(url_str.as_ref()).unwrap())
    // }

    pub fn parse(url_str: impl AsRef<str>) -> Result<Self, DomainError> {
        Ok(Self(
            Url::parse(url_str.as_ref()).map_err(|error| DomainError::ParseUrl(error))?,
        ))
    }

    // pub fn valid(&self) -> bool {
    //     AuthProvider::is_valid_domain(&self.0)
    // }

    pub async fn login<T>(&self, ctx: &T) -> Result<(), DomainError>
    where
        T: AuthAgentProvider,
    {
        let auth_agent = AuthAgentProvider::provide(ctx);
        Ok(auth_agent.login(self).await?)
    }
}

impl Into<Url> for LoginUrl {
    fn into(self) -> Url {
        self.0
    }
}

impl Into<Url> for &LoginUrl {
    fn into(self) -> Url {
        self.0.to_owned()
    }
}

impl Deref for LoginUrl {
    type Target = Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for LoginUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

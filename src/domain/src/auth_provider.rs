use std::fmt::Display;

use url::Url;

use crate::DomainError;

#[derive(Debug, PartialEq, Eq)]
pub enum AuthProvider {
    Github,
    Google,
}

const AUTH_PROVIDER_GITHUB: &'static str = "github";
const AUTH_PROVIDER_GOOGLE: &'static str = "google";

const AUTH_DOMAIN_GITHUB: &'static str = "github.com";
const AUTH_DOMAIN_GOOGLE: &'static str = "accounts.google.com";
const AUTH_DOMAINS: [&'static str; 2] = [AUTH_DOMAIN_GITHUB, AUTH_DOMAIN_GOOGLE];

impl AuthProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Github => AUTH_PROVIDER_GITHUB,
            Self::Google => AUTH_PROVIDER_GOOGLE,
        }
    }

    pub fn is_provider(&self, s: String) -> bool {
        Self::try_from(s).is_ok()
    }

    pub fn is_valid_domain(url: &Url) -> bool {
        url.domain()
            .map(|domain| AUTH_DOMAINS.contains(&domain))
            .unwrap_or(false)
    }
}

impl AsRef<str> for AuthProvider {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<String> for AuthProvider {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            AUTH_PROVIDER_GITHUB => Ok(Self::Github),
            AUTH_PROVIDER_GOOGLE => Ok(Self::Google),
            _ => Err(DomainError::ParseAuthProvider(value)),
        }
    }
}

impl Display for AuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

use crate::{
    DatabaseAccessor, DatabaseAccessorProvider, DomainError, LoginUrl, MetadataHeader, NameIndexer,
};
use base64::URL_SAFE_NO_PAD;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum AccessTokenValue {
    Token(AccessToken),
    LoginUrl(LoginUrl),
}

impl AccessTokenValue {
    pub fn token(self) -> Result<AccessToken, DomainError> {
        match self {
            Self::Token(access_token) => Ok(access_token),
            Self::LoginUrl(login_url) => Err(DomainError::LoginUrlInstaedOfToken(login_url)),
        }
    }

    pub fn login_url(self) -> Option<LoginUrl> {
        match self {
            Self::LoginUrl(login_url) => Some(login_url),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(token_str: &str) -> Self {
        Self(token_str.to_string())
    }

    pub fn is_valid(&self) -> bool {
        let splited_token = self.0.split(".").collect::<Vec<&str>>();
        let Some(encoded_payload) = splited_token.get(1) else {
            return false;
        };
        let decoded_payload = match base64::decode_config(encoded_payload, URL_SAFE_NO_PAD) {
            Ok(decoded_payload) => decoded_payload,
            Err(error) => {
                log::debug!("error={:#?}", error);
                return false;
            }
        };
        let claims = match serde_json::from_slice::<RegisterdClaims>(&decoded_payload) {
            Ok(claims) => claims,
            Err(error) => {
                log::debug!("error={:#?}", error);
                return false;
            }
        };

        if claims.is_expired() {
            log::debug!("access_token claims is expired");
            return false;
        }

        true
    }

    pub async fn store<T>(&self, ctx: &T) -> Result<(), DomainError>
    where
        T: DatabaseAccessorProvider<Error = DomainError>,
    {
        let accessor = DatabaseAccessorProvider::provide(ctx);
        let user_data = NameIndexer::new(ACCESS_TOKEN_KEY, self);
        let js_value: JsValue = user_data.into();
        accessor.put(js_value).await?;
        Ok(())
    }

    pub async fn load<T>(ctx: &T) -> Result<Self, DomainError>
    where
        T: DatabaseAccessorProvider<Error = DomainError>,
    {
        let accessor = DatabaseAccessorProvider::provide(ctx);
        let js_value = accessor.get(ACCESS_TOKEN_KEY).await?;
        let user_data: NameIndexer<AccessToken> = NameIndexer::try_from(js_value)?;
        Ok(user_data.to_value())
    }

    pub async fn delete<T>(accessor: &T) -> Result<(), DomainError>
    where
        T: DatabaseAccessor<Error = DomainError>,
    {
        accessor.delete(ACCESS_TOKEN_KEY).await?;
        Ok(())
    }
}

impl From<&str> for AccessToken {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for AccessToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<AccessToken> for String {
    fn from(token: AccessToken) -> Self {
        token.0
    }
}

impl MetadataHeader for AccessToken {
    fn as_header(&self) -> (&str, String) {
        ("authorization", format!("Bearer {}", self.0))
    }
}

const ACCESS_TOKEN_KEY: &'static str = "access_token";

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

type Epoch = u64;

#[derive(Debug, Clone, Deserialize)]
struct RegisterdClaims {
    #[serde(rename = "exp")]
    expires_at: Option<Epoch>,
    #[serde(rename = "nbf")]
    _not_before: Option<Epoch>,
    #[serde(rename = "iat")]
    _issued_at: Option<Epoch>,
    #[serde(rename = "jti")]
    _id: Option<String>,
    #[serde(rename = "aud")]
    _audience: Option<String>,
    #[serde(rename = "iss")]
    _issuer: Option<String>,
    #[serde(rename = "sub")]
    _subject: Option<String>,
}

impl RegisterdClaims {
    fn is_expired(&self) -> bool {
        let now = match instant::SystemTime::now().duration_since(instant::SystemTime::UNIX_EPOCH) {
            Ok(now) => now,
            Err(error) => {
                log::debug!("error={:#?}", error);
                return false;
            }
        };
        let Some(expires_at) = self.expires_at else {
            log::debug!("expires_at is None");
            return false;
        };
        return &now.as_secs() > &expires_at;
    }
}

#[cfg(test)]
mod tests {
    use crate::AccessToken;

    #[test]
    fn should_validate_jwt() {
        let access_token2 = AccessToken::from("");
        assert!(!access_token2.is_valid());
    }
}

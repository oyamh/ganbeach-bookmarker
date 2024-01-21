use crate::{gooscut::AccessTokenResponse as PbAccessTokenResponse, ServerError};
use domain::{AccessToken, AccessTokenValue, LoginUrl};

impl TryFrom<PbAccessTokenResponse> for AccessTokenValue {
    type Error = ServerError;
    fn try_from(response: PbAccessTokenResponse) -> Result<Self, Self::Error> {
        if response.access_token != "" {
            return Ok(Self::Token(AccessToken::new(&response.access_token)));
        }
        if response.login_url != "" {
            let login_url = LoginUrl::parse(response.login_url)
                .map_err(|error| ServerError::UnexpectedResponse(format!("login_url: {error}")))?;
            return Ok(Self::LoginUrl(login_url));
        }
        Err(ServerError::UnexpectedResponse(
            "empty PbAccessTokenResponse".to_string(),
        ))
    }
}

// impl From<PbAccessTokenResponse> for AccessTokenValue {
//     fn from(response: PbAccessTokenResponse) -> Self {
//         // match response {
//         //     response if response.access_token != "" => {
//         //         Self::Token(AccessToken::new(&response.access_token))
//         //     }
//         //     response if response.login_url != "" => {
//         //         Self::LoginUrl(LoginUrl::new(response.login_url))
//         //     }
//         //     _ => unreachable!(""),
//         // }

//         if response.access_token != "" {
//             return Self::Token(AccessToken::new(&response.access_token));
//         }
//         if response.login_url != "" {
//             let login_url = LoginUrl::parse(response.login_url).unwrap();
//             return Self::LoginUrl(login_url);
//         }
//         unreachable!("PbAccessTokenResponse => AccessTokenValue")
//     }
// }

// // use super::LoginUrl;
// use crate::gooscut::AccessTokenResponse as PbAccessTokenResponse;
// use domain::{AccessToken, AccessTokenValue, LoginUrl};
// // use serde::Serialize;
// // use std::fmt::Display;

// // #[derive(Debug)]
// // pub enum AccessTokenValue {
// //     Token(AccessToken),
// //     LoginUrl(LoginUrl),
// // }

// // impl AccessTokenValue {
// //     pub fn token(self) -> Option<AccessToken> {
// //         match self {
// //             Self::Token(access_token) => Some(access_token),
// //             _ => None,
// //         }
// //     }
// // }

// // #[derive(Debug, Default, Serialize)]
// // pub struct AccessToken(String);
// // // pub struct AccessToken(jwt::AccessToken);

// // impl AccessToken {
// //     pub fn new(token_str: &str) -> Self {
// //         Self(token_str.to_string())
// //         // Self(jwt::AccessToken::new(token_str))
// //     }

// //     pub fn is_valid(&self) -> bool {
// //         // self.0.is_valid()
// //         false
// //     }

// //     pub(crate) fn header(&self) -> String {
// //         format!("Bearer {}", self.0)
// //     }
// // }

// // impl Display for AccessToken {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         write!(f, "{}", self.0)
// //     }
// // }

// impl From<PbAccessTokenResponse> for AccessTokenValue {
//     fn from(response: PbAccessTokenResponse) -> Self {
//         match response {
//             response if response.access_token != "" => {
//                 Self::Token(AccessToken::new(&response.access_token))
//             }
//             response if response.login_url != "" => {
//                 Self::LoginUrl(LoginUrl::new(response.login_url))
//             }
//             _ => unreachable!(""),
//         }
//     }
// }

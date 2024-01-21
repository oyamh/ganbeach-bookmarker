use crate::{gooscut::LoginUrlResponse as PbLoginUrlResponse, ServerError};
use domain::LoginUrl;

impl TryFrom<PbLoginUrlResponse> for LoginUrl {
    type Error = ServerError;
    fn try_from(response: PbLoginUrlResponse) -> Result<Self, Self::Error> {
        Ok(LoginUrl::parse(response.url)
            .map_err(|error| ServerError::UnexpectedResponse(format!("url: {error}")))?)
    }
}

// impl From<PbLoginUrlResponse> for LoginUrl {
//     fn from(response: PbLoginUrlResponse) -> LoginUrl {
//         LoginUrl::new(response.url)
//     }
// }

// use crate::gooscut::LoginUrlResponse as PbLoginUrlResponse;
// use domain::LoginUrl;

// // #[derive(Debug)]
// // pub struct LoginUrl(String);

// // impl LoginUrl {
// //     pub fn new(url: String) -> Self {
// //         Self { 0: url }
// //     }
// // }

// impl From<PbLoginUrlResponse> for LoginUrl {
//     fn from(response: PbLoginUrlResponse) -> LoginUrl {
//         LoginUrl(response.url)
//     }
// }

use std::fmt::Display;

use domain::DomainError;
use http::header::{InvalidHeaderName, InvalidHeaderValue, ToStrError};
use thiserror::Error;
use tonic_types::{ErrorDetails, StatusExt};

impl From<ServerError> for DomainError {
    fn from(error: ServerError) -> Self {
        match error {
            ServerError::TonicStatusError(detail) => detail.into(),
            _ => Self::Server(error.to_string()),
        }
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("http error: {0}")]
    HttpError(#[from] http::Error),

    #[error("base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("failed to parse headers")]
    HeaderParsingError,

    #[error("failed to convert header value to string: {0}")]
    HeaderValueError(#[from] ToStrError),

    #[error("missing content-type header in grpc response")]
    MissingContentTypeHeader,

    #[error("invalid header name: {0}")]
    InvalidHeaderName(#[from] InvalidHeaderName),

    #[error("invalid header value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    // #[error("grpc error: {0}")]
    // TonicStatusError(#[from] tonic::Status),
    #[error("{0}")]
    TonicStatusError(ErrorDetail),

    #[error("unexpected response: {0}")]
    UnexpectedResponse(String),

    #[error("unreachable")]
    Unreachable,

    #[error("todo")]
    Todo,
}

#[derive(Debug, Default)]
pub struct ErrorDetail {
    reason: String,
    req_id: String,
    time: String,
}

impl Display for ErrorDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "reason={} reqID={} time={}",
            &self.reason, &self.req_id, &self.time
        )
    }
}

impl Into<String> for ErrorDetail {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Into<DomainError> for ErrorDetail {
    fn into(self) -> DomainError {
        match self.reason.as_str() {
            "US1003" => DomainError::GetAccessToken(self.into()),
            "US1004" => DomainError::ParseAuthProvider(self.into()),
            _ => DomainError::Server(self.into()),
        }
    }
}

impl From<ErrorDetails> for ErrorDetail {
    fn from(error_details: ErrorDetails) -> Self {
        let mut detail = ErrorDetail::default();
        if let Some(error_info) = error_details.error_info() {
            log::debug!("{:#?}", error_info);
            detail.reason = error_info.reason.to_owned();
            let metadata = &error_info.metadata;
            if let Some(req_id) = metadata.get("reqID") {
                detail.req_id = req_id.to_owned();
            };
            if let Some(time) = metadata.get("time") {
                detail.time = time.to_owned();
            };
        };
        detail
    }
}

impl From<tonic::Status> for ServerError {
    fn from(status: tonic::Status) -> Self {
        log::debug!("{:#?}", status);
        Self::TonicStatusError(ErrorDetail::from(status.get_error_details()))
    }
}

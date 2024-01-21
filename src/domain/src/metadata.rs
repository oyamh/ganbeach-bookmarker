use crate::{AccessToken, CommandGetter, GrpcTimeout, RefreshToken};

pub trait MetadataHeaders {
    fn headers(&self) -> Vec<(&str, String)>;
}

pub trait MetadataHeader {
    fn as_header(&self) -> (&str, String);
}

#[derive(Debug, Default)]
pub struct Metadata {
    pub timeout: Option<GrpcTimeout>,
    pub access_token: Option<AccessToken>,
    pub refresh_token: Option<RefreshToken>,
}

impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_timeout(mut self, timeout: GrpcTimeout) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_access_token(mut self, access_token: AccessToken) -> Self {
        self.access_token = Some(access_token);
        self
    }

    pub fn with_refresh_token(mut self, refresh_token: RefreshToken) -> Self {
        self.refresh_token = Some(refresh_token);
        self
    }
}

impl MetadataHeaders for Metadata {
    fn headers(&self) -> Vec<(&str, String)> {
        vec![
            self.timeout.as_ref().map(|v| v.as_header()),
            self.access_token.as_ref().map(|v| v.as_header()),
            self.refresh_token.as_ref().map(|v| v.as_header()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl CommandGetter for Metadata {
    fn timeout(&self) -> Option<&GrpcTimeout> {
        self.timeout.as_ref()
    }

    fn access_token(&self) -> Option<&AccessToken> {
        self.access_token.as_ref()
    }

    fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }
}

impl CommandGetter for &Metadata {
    fn timeout(&self) -> Option<&GrpcTimeout> {
        self.timeout.as_ref()
    }

    fn access_token(&self) -> Option<&AccessToken> {
        self.access_token.as_ref()
    }

    fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }
}

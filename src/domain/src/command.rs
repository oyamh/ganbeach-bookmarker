use serde::{Deserialize, Serialize};

use crate::{
    AccessToken, AccessTokenValue, Accounter, AccounterProvider, Annotation, BookmarkCreator,
    BookmarkCreatorProvider, BookmarkId, BookmarkReader, BookmarkReaderProvider,
    CreateBookmarkResult, DomainError, FolderTitle, Lists, Metadata, MetadataHeader, PageUrl,
    RefreshToken, TagIds, Title, Titles, TypeCode,
};
use std::{ops::Deref, time::Duration};

#[derive(Debug, Default, Clone, Copy)]
pub struct GrpcTimeout(Duration);

impl GrpcTimeout {
    pub fn new(timeout: Duration) -> Self {
        Self(timeout)
    }
}

impl MetadataHeader for GrpcTimeout {
    fn as_header(&self) -> (&str, String) {
        ("grpc-timeout", self.0.as_millis().to_string())
    }
}

pub trait CommandGetter {
    fn timeout(&self) -> Option<&GrpcTimeout>;
    fn access_token(&self) -> Option<&AccessToken>;
    fn refresh_token(&self) -> Option<&RefreshToken>;
}

pub trait CommandSetter {
    fn with_timeout(self, timeout: Option<Duration>) -> Self;
    fn with_access_token(self, access_token: Option<String>) -> Self;
    fn with_refresh_token(self, refresh_token: Option<String>) -> Self;
}

pub struct CommandBuilder<T> {
    inner: Option<T>,
    timeout: Option<GrpcTimeout>,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl<T> CommandBuilder<T> {
    pub(crate) fn new() -> Self {
        Self {
            inner: None,
            timeout: None,
            access_token: None,
            refresh_token: None,
        }
    }

    pub fn with_inner(mut self, inner: T) -> Self {
        self.inner = Some(inner);
        self
    }

    pub fn with_timeout(mut self, deadline: Duration) -> Self {
        self.timeout = Some(GrpcTimeout::new(deadline));
        self
    }

    pub fn with_access_token<S>(mut self, access_token: S) -> Self
    where
        S: AsRef<str>,
    {
        self.access_token = Some(access_token.as_ref().to_string());
        self
    }

    pub fn with_refresh_token<S>(mut self, refresh_token: S) -> Self
    where
        S: AsRef<str>,
    {
        self.refresh_token = Some(refresh_token.as_ref().to_string());
        self
    }

    pub fn build(self) -> Command<T> {
        self.into()
    }
}

impl<T> From<CommandBuilder<T>> for Command<T> {
    fn from(builder: CommandBuilder<T>) -> Self {
        let CommandBuilder {
            mut inner,
            mut timeout,
            mut access_token,
            mut refresh_token,
        } = builder;
        Self {
            inner: inner.take(),
            metadata: Metadata {
                timeout: timeout.take(),
                access_token: access_token.take().map(|token| token.into()),
                refresh_token: refresh_token.take().map(|token| token.into()),
            },
        }
    }
}

#[derive(Debug)]
pub struct Command<T> {
    pub inner: Option<T>,
    pub metadata: Metadata,
}

impl<T> Command<T> {
    pub fn builder() -> CommandBuilder<T> {
        CommandBuilder::new()
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl<T> CommandGetter for Command<T> {
    fn timeout(&self) -> Option<&GrpcTimeout> {
        self.metadata.timeout()
    }

    fn access_token(&self) -> Option<&AccessToken> {
        self.metadata.access_token()
    }

    fn refresh_token(&self) -> Option<&RefreshToken> {
        self.metadata.refresh_token()
    }
}

#[derive(Debug, Default)]
pub struct AccessTokenCommand {
    pub old_access_token: String,
}

impl AccessTokenCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_old_access_token(mut self, old_access_token: String) -> Self {
        self.old_access_token = old_access_token;
        self
    }
}

impl AccessTokenCommand {
    pub async fn request<T>(
        self,
        ctx: &T,
        metadata: Metadata,
    ) -> Result<AccessTokenValue, DomainError>
    where
        T: AccounterProvider,
    {
        let accounter = AccounterProvider::provide(ctx);
        accounter.generate_access_token(self, metadata).await
    }
}

#[derive(Debug)]
pub struct LoginUrlCommand {
    pub provider: String,
}

#[derive(Debug, Default)]
pub struct GetAllListsCommand {}

impl GetAllListsCommand {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GetAllListsCommand {
    pub async fn request<T>(self, ctx: &T, metadata: Metadata) -> Result<Lists, DomainError>
    where
        T: BookmarkReaderProvider,
    {
        let reader = BookmarkReaderProvider::provide(ctx);
        reader.fetch_all_lists(self, metadata).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NewBookmarkData {
    pub name: Title,
    pub url: PageUrl,
    pub type_code: TypeCode,
    pub annotation: Annotation,
}

impl Default for FolderTitle {
    fn default() -> Self {
        Self::Old(Title::default())
    }
}

impl Into<Title> for FolderTitle {
    fn into(self) -> Title {
        match self {
            Self::New(title) => title,
            Self::Old(title) => title,
        }
    }
}

impl Deref for FolderTitle {
    type Target = Title;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::New(title) => title,
            Self::Old(title) => title,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBookmarkCommand {
    pub folder_id: Option<BookmarkId>,
    pub new_bookmarks: Vec<NewBookmarkData>,
    pub tag_ids: TagIds,
    pub new_tags: Titles,
    pub folder_title: FolderTitle,
}

impl CreateBookmarkCommand {
    pub fn new(
        name: Title,
        url: PageUrl,
        type_code: TypeCode,
        folder_id: Option<BookmarkId>,
        tag_ids: TagIds,
        unknown_tag_names: Titles,
        folder_title: FolderTitle,
        annotation: Annotation,
    ) -> Self {
        let new_bookmarks = vec![NewBookmarkData {
            name,
            url,
            type_code,
            annotation,
        }];
        Self {
            folder_id,
            new_bookmarks,
            tag_ids,
            new_tags: unknown_tag_names,
            folder_title,
        }
    }

    pub fn first(&self) -> Option<&NewBookmarkData> {
        self.new_bookmarks.get(0)
    }

    pub fn push(&mut self, name: Title, url: PageUrl, type_code: TypeCode, annotation: Annotation) {
        self.new_bookmarks.push(NewBookmarkData {
            name,
            url,
            type_code,
            annotation,
        })
    }
}

impl CreateBookmarkCommand {
    pub async fn request<T>(
        self,
        ctx: &T,
        metadata: Metadata,
    ) -> Result<CreateBookmarkResult, DomainError>
    where
        T: BookmarkCreatorProvider,
    {
        let client = BookmarkCreatorProvider::provide(ctx);
        Ok(client.create_bookmarks(self, metadata).await?)
    }
}

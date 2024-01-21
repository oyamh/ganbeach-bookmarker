#![allow(async_fn_in_trait)]
use crate::{
    AccessTokenCommand, AccessTokenValue, Command, CreateBookmarkCommand, CreateBookmarkResult,
    DatabaseConfig, DomainError, GetAllListsCommand, Lists, LoginUrl, LoginUrlCommand,
    MessageToBackground, Metadata, NotificationId, PageUrl, PopupOrigin, SenderInfo, Tab, Title,
    Url,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Window;

pub trait Accounter {
    async fn generate_access_token(
        &self,
        command: AccessTokenCommand,
        metadata: Metadata,
    ) -> Result<AccessTokenValue, DomainError>;
    async fn generate_login_url(
        &self,
        command: Command<LoginUrlCommand>,
    ) -> Result<LoginUrl, DomainError>;
}

pub trait AccounterProvider {
    fn provide(&self) -> &impl Accounter;
}

pub trait BookmarkCreator {
    async fn create_bookmark(
        &self,
        command: Command<CreateBookmarkCommand>,
    ) -> Result<CreateBookmarkResult, DomainError>;

    async fn create_bookmarks(
        &self,
        command: CreateBookmarkCommand,
        metadata: Metadata,
    ) -> Result<CreateBookmarkResult, DomainError>;
}

pub trait BookmarkCreatorProvider {
    fn provide(&self) -> &impl BookmarkCreator;
}

pub trait BookmarkReader {
    async fn fetch_all_lists(
        &self,
        command: GetAllListsCommand,
        metadata: Metadata,
    ) -> Result<Lists, DomainError>;
}

pub trait BookmarkReaderProvider {
    fn provide(&self) -> &impl BookmarkReader;
}

// Command: Post,Put,Delete
// Query: Get
pub trait RequestSender {
    type Output;
    type Error;
    async fn send(&self, metadata: Metadata) -> Result<Self::Output, Self::Error>;
}

pub trait SecretAccessor {
    type Error;
    async fn get<T>(&self, details: T) -> Result<String, Self::Error>
    where
        T: Serialize;
    async fn delete<T>(&self, details: T) -> Result<(), Self::Error>
    where
        T: Serialize;
}

pub trait SecretAccessorProvider {
    type Error;
    fn provide(&self) -> &impl SecretAccessor<Error = Self::Error>;
}

// pub trait RefreshTokenObserver {
//     fn on_changed(&self, callback: impl Fn(Option<RefreshToken>) + 'static);
// }

// pub trait RefreshTokenObserverProvider {
//     type Observer: RefreshTokenObserver;
//     fn provide(&self) -> &Self::Observer;
// }

pub trait DatabaseInitializer {
    async fn initialize(&self, config: DatabaseConfig) -> Result<(), DomainError>;
}

pub trait DatabaseInitializerProvider {
    fn provide(&self) -> &impl DatabaseInitializer;
}

pub trait DatabaseAccessor {
    type Error;
    async fn put(&self, value: impl Into<JsValue>) -> Result<(), Self::Error>;
    async fn put_iter<I: IntoIterator>(&self, values: I) -> Result<(), Self::Error>
    where
        // I: IntoIterator<Item = JsValue>;
        <I as IntoIterator>::Item: JsCast;
    async fn get(&self, key: impl AsRef<str>) -> Result<JsValue, Self::Error>;
    async fn get_all(&self) -> Result<JsValue, Self::Error>;
    async fn delete(&self, key: impl AsRef<str>) -> Result<(), Self::Error>;
    async fn clear(&self) -> Result<(), Self::Error>;
}

pub trait DatabaseAccessorProvider {
    type Error;
    fn provide(&self) -> &impl DatabaseAccessor<Error = Self::Error>;
}

pub trait MessageSender {
    fn send_to_parent_frame<T>(&self, message: T) -> Result<(), DomainError>
    where
        T: Serialize;
    fn send_to_child_frame<T>(
        &self,
        message: T,
        content_window: &Window,
        target_uri: impl AsRef<str>,
    ) -> Result<(), DomainError>
    where
        T: Serialize;
    async fn send_to_tab<T>(&self, message: T, sender_info: SenderInfo) -> Result<(), DomainError>
    where
        T: Serialize;
    async fn send_to_background(&self, message: MessageToBackground) -> Result<(), DomainError>;
}

pub trait MessageSenderProvider {
    fn provide(&self) -> &impl MessageSender;
}

pub trait MessageReceiver {
    /// background <=> popupをlistenする。
    /// backgroundからのmessageもlistenする。
    fn on_extension_message<M, M2, T>(&self, callback: T)
    where
        M: for<'de> Deserialize<'de>,
        M2: for<'de> Deserialize<'de>,
        T: Fn(M, M2) + 'static;
    // /// content <=> popupをlistenする。
    // fn on_window_message<M, T>(&self, callback: T) -> EventListener
    // where
    //     M: for<'de> Deserialize<'de>,
    //     T: Fn(M) + 'static;
}

pub trait MessageReceiverProvider {
    fn provide(&self) -> &impl MessageReceiver;
}

pub trait HotkeysListener {
    fn on_hotkeys<T>(&self, callback: T)
    where
        T: Fn(String) + 'static;
}

pub trait HotkeysListenerProvider {
    fn provide(&self) -> &impl HotkeysListener;
}

pub trait ErrorMessenger {
    fn error_message(&self, error: &DomainError) -> &'static str;
    fn error_title(&self, error: &DomainError) -> &'static str;
}

pub trait ErrorMessengerProvider {
    fn provide(&self) -> &impl ErrorMessenger;
}

pub trait NotificationEmitter {
    async fn notify(
        &self,
        id: NotificationId,
        message: impl AsRef<str>,
        title: impl AsRef<str>,
    ) -> Result<NotificationId, DomainError>;
}

pub trait NotificationEmitterProvider {
    fn provide(&self) -> &impl NotificationEmitter;
}

// #[async_trait(?Send)]
// pub trait NotificationGetter {
//     async fn get_notification_by_id(id: NotificationId)
//         -> Result<NotificationOptions, DomainError>;
// }

// pub trait NotificationGetterProvider {
//     type Getter: NotificationGetter;
//     fn provide(&self) -> &Self::Getter;
// }

pub trait AuthAgent {
    async fn login(&self, login_url: &LoginUrl) -> Result<(), DomainError>;
}

pub trait AuthAgentProvider {
    fn provide(&self) -> &impl AuthAgent;
}

pub trait LinkOpener {
    async fn open(&self, url: &Url) -> Result<(), DomainError>;
}

pub trait LinkOpenerProvider {
    fn provide(&self) -> &impl LinkOpener;
}

pub trait ToolbarListener {
    fn on_toolbar<V, C>(&self, callback: C)
    where
        V: for<'de> Deserialize<'de>,
        C: Fn(V) + 'static;
}

pub trait ToolbarListenerProvider {
    fn provide(&self) -> &impl ToolbarListener;
}

pub trait BrowserBookmarker {
    async fn create(
        &self,
        title: Title,
        url: PageUrl,
        parent_title: Option<Title>,
    ) -> Result<(), DomainError>;
}

pub trait BrowserBookmarkerProvider {
    fn provide(&self) -> &impl BrowserBookmarker;
}

pub trait PopupOpener {
    async fn open_popup(&self, tab: Option<Tab>) -> Result<(), DomainError>;
}

pub trait PopupOpenerProvider {
    fn provide(&self) -> &impl PopupOpener;
}

pub trait PopupOriginExtractor {
    fn extract_popup_origin(&self) -> Result<PopupOrigin, DomainError>;
}

pub trait PopupOriginExtractorProvider {
    fn provide(&self) -> &impl PopupOriginExtractor;
}

pub trait UrlQueryBuilder {
    fn append_pairs(&self, base_url: Url) -> Result<Url, DomainError>;
}

pub trait HistoryCleaner {
    fn register(&self);
}

pub trait HistoryCleanerProvider {
    fn provide(&self) -> &impl HistoryCleaner;
}

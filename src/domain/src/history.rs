use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::{
    BookmarkId, DatabaseAccessor, DatabaseAccessorProvider, DomainError, NameIndexer, TagIds,
};

/// 履歴として保管できる値の型。
#[derive(Debug, Serialize, Deserialize)]
pub enum History {
    RecentFolderId(BookmarkId),
    RecentTagIds(TagIds),
}

impl History {
    pub fn as_key(&self) -> HistoryKey {
        match self {
            Self::RecentFolderId(_) => HistoryKey::RecentFolderId,
            Self::RecentTagIds(_) => HistoryKey::RecentTagIds,
        }
    }

    pub fn from_value(&self, value: String) -> Result<Self, DomainError> {
        Ok(match self {
            Self::RecentFolderId(_) => Self::RecentFolderId(BookmarkId::try_from(value)?),
            Self::RecentTagIds(_) => Self::RecentTagIds(TagIds::try_from(value)?),
        })
    }

    pub fn into_history_with_key(key: &HistoryKey, value: String) -> Result<Self, DomainError> {
        Ok(match key {
            HistoryKey::RecentFolderId => Self::RecentFolderId(BookmarkId::try_from(value)?),
            HistoryKey::RecentTagIds => Self::RecentTagIds(TagIds::try_from(value)?),
        })
    }

    pub async fn save<T>(&self, ctx: &T) -> Result<(), DomainError>
    where
        T: DatabaseAccessorProvider<Error = DomainError>,
    {
        let accessor = DatabaseAccessorProvider::provide(ctx);
        let user_data = NameIndexer::new(self.as_key(), self);
        let js_value: JsValue = user_data.into();
        accessor.put(js_value).await?;
        Ok(())
    }

    pub async fn load<T>(ctx: &T, key: HistoryKey) -> Result<History, DomainError>
    where
        T: DatabaseAccessorProvider<Error = DomainError>,
    {
        let accessor = DatabaseAccessorProvider::provide(ctx);
        let js_value = accessor.get(key).await?;
        let history_data = TryInto::<NameIndexer<History>>::try_into(js_value)?;
        Ok(history_data.to_value())
    }
}

impl From<History> for JsValue {
    fn from(value: History) -> Self {
        serde_wasm_bindgen::to_value(&value).expect("serialize History")
    }
}

/// 履歴をIndexedDBで扱うときに使うKey。
#[derive(Debug, Serialize, Deserialize)]
pub enum HistoryKey {
    RecentFolderId,
    RecentTagIds,
}

const KEY_RECENT_FOLDER_ID: &'static str = "RecentFolderId";
const KEY_RECENT_TAG_IDS: &'static str = "RecentTagIds";

impl HistoryKey {
    pub fn into_history(&self, value: String) -> Result<History, DomainError> {
        History::into_history_with_key(&self, value)
    }
}

impl AsRef<str> for HistoryKey {
    fn as_ref(&self) -> &str {
        match self {
            Self::RecentFolderId => KEY_RECENT_FOLDER_ID,
            Self::RecentTagIds => KEY_RECENT_TAG_IDS,
        }
    }
}

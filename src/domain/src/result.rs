use crate::{BookmarkId, List, Title, TypeCode};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct CreateBookmarkResult {
    pub code: u32,
    pub status: String,
    pub message: String,
    pub results: Vec<BookmarkResult>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct BookmarkResult {
    pub name: Title,
    pub bookmark_id: BookmarkId,
    pub type_code: TypeCode,
}

impl From<BookmarkResult> for List {
    fn from(result: BookmarkResult) -> Self {
        let BookmarkResult {
            name,
            bookmark_id,
            type_code,
        } = result;
        Self {
            bookmark_id,
            type_code,
            title: name,
            ..Default::default()
        }
    }
}

impl From<&BookmarkResult> for List {
    fn from(result: &BookmarkResult) -> Self {
        let BookmarkResult {
            name,
            bookmark_id,
            type_code,
        } = result;
        Self {
            bookmark_id: *bookmark_id,
            type_code: *type_code,
            title: name.clone(),
            ..Default::default()
        }
    }
}

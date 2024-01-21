use domain::{BookmarkResult, CreateBookmarkResult};

use crate::gooscut::{
    AddBookmarkResponse as PbAddBookmarkResponse, AddBookmarkResult as PbAddBookmarkResult,
};

impl From<PbAddBookmarkResponse> for CreateBookmarkResult {
    fn from(response: PbAddBookmarkResponse) -> Self {
        let PbAddBookmarkResponse {
            code,
            status,
            message,
            results,
        } = response;
        Self {
            code,
            status,
            message,
            results: results
                .into_iter()
                .map(|list_result| list_result.into())
                .collect(),
        }
    }
}

impl From<PbAddBookmarkResult> for BookmarkResult {
    fn from(result: PbAddBookmarkResult) -> Self {
        let PbAddBookmarkResult {
            name,
            bookmark_id,
            type_code,
        } = result;
        Self {
            name: name.into(),
            bookmark_id: bookmark_id.into(),
            type_code: type_code.into(),
        }
    }
}

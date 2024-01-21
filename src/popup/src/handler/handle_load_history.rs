use domain::{BookmarkId, DomainError, History, HistoryKey, TagIds};
use load_history_context::LoadHistoryContext;

pub(crate) async fn handle_recent_metadata(
) -> Result<(Option<BookmarkId>, Option<TagIds>), DomainError> {
    let ctx = LoadHistoryContext::new();
    let folder_id = match History::load(&ctx, HistoryKey::RecentFolderId).await? {
        History::RecentFolderId(folder_id) => Some(folder_id),
        _ => None,
    };
    let tag_ids = match History::load(&ctx, HistoryKey::RecentTagIds).await? {
        History::RecentTagIds(tag_ids) => Some(tag_ids),
        _ => None,
    };
    Ok((folder_id, tag_ids))
}

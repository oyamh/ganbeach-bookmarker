use domain::{AccessToken, DomainError, GetAllListsCommand, Lists, Metadata};
use fetch_lists_context::FetchListsContext;

pub async fn handle_fetch_lists(access_token: AccessToken) -> Result<Lists, DomainError> {
    let ctx = FetchListsContext::new();

    let command = GetAllListsCommand::new();
    let metadata = Metadata::new().with_access_token(access_token);
    let all_lists = command.request(&ctx, metadata).await?;

    all_lists.put_all(&ctx).await?;

    Ok(all_lists)
}

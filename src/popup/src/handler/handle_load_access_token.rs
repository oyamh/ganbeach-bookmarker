use domain::{AccessToken, AccessTokenCommand, DomainError, Metadata, RefreshToken};
use load_access_token_context::LoadAccessTokenContext;

pub(crate) async fn handle_load_access_token() -> Result<AccessToken, DomainError> {
    log::debug!("handle_load_access_token");
    let ctx = LoadAccessTokenContext::new();

    if let Ok(access_token) = AccessToken::load(&ctx).await {
        log::debug!("access_token: {access_token:?}");
        if access_token.is_valid() {
            return Ok(access_token);
        }
    };
    log::debug!("access_token is not exist");

    let refresh_token = match RefreshToken::load(&ctx).await {
        Ok(refresh_token) => refresh_token,
        Err(error) => {
            log::debug!("RefreshToken::load error={:?}", error);
            RefreshToken::default()
        }
    };
    log::debug!("refresh_token: {refresh_token:?}");

    let command = AccessTokenCommand::new();
    let metadata = Metadata::new().with_refresh_token(refresh_token);
    let access_token = command.request(&ctx, metadata).await?.token()?;

    access_token.store(&ctx).await?;

    log::debug!("handle_load_access_token succeed");
    Ok(access_token)
}

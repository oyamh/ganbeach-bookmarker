use background_error_context::BackgroundErrorContext;
use domain::{DomainError, MessageToPopup, NotificationId, SenderInfo, ServerUrl};

use super::handle_open_tab;

pub(crate) async fn handle_notify_error(error: DomainError, sender: SenderInfo) {
    log::debug!("handle_background_error");
    log::debug!("error: {error:#?}");
    let ctx = BackgroundErrorContext::new();
    let error = match error {
        DomainError::LoginUrlInstaedOfToken(ref login_url) => {
            login_url.login(&ctx).await.map(|_| {
                log::debug!(
                    "re-login succeeded"
                );
                return;
            }).map_err(|error| {
                log::error!(
                    "failed to open login_url from DomainError::GetAccessToken login_url={login_url} error={error:?}"
                );
                error
            }).unwrap_err()
        }
        DomainError::GetAccessToken(ref _message) => {
            if let Err(background_error) = handle_open_tab(ServerUrl::Home.into()).await {
                log::error!("failed to open tab: {background_error:?}");
            }
            error
        }
        DomainError::NotFoundNewFolder => {
            DomainError::CreateBookmarks("Not found new folder id.".to_string())
        }
        DomainError::NotFoundParentId => {
            DomainError::CreateBookmarks("Not found parent folder id.".to_string())
        }
        _ => {
            error
        }
    };
    let error_title = usecase::error_title(&ctx, &error);

    let message = usecase::error_message(&ctx, &error);
    log::error!("message: {message:#?}");

    if let Err(error) = usecase::notify(
        &ctx,
        NotificationId::String(error.to_string()),
        message.to_string(),
        error_title.to_string(),
    )
    .await
    {
        log::error!("handle_background_error usecase::notify error: {}", error);
    }

    if let Err(error) =
        usecase::send_outer_message(&ctx, sender, MessageToPopup::Error(message.to_string())).await
    {
        log::error!(
            "handle_background_error usecase::send_outer_message error: {}",
            error
        );
    };
}

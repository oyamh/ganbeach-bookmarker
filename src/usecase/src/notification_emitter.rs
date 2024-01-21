use domain::{DomainError, NotificationEmitter, NotificationEmitterProvider, NotificationId};

pub async fn notify<T>(
    ctx: &T,
    id: NotificationId,
    message: String,
    title: String,
) -> Result<NotificationId, DomainError>
where
    T: NotificationEmitterProvider,
{
    let browser_notifier = NotificationEmitterProvider::provide(ctx);
    Ok(browser_notifier.notify(id, message, title).await?)
}

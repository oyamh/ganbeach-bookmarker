use domain::{DomainError, PopupOpener, PopupOpenerProvider};

pub async fn open_popup<T>(ctx: &T, tab: Option<domain::Tab>) -> Result<(), DomainError>
where
    T: PopupOpenerProvider,
{
    let opener = PopupOpenerProvider::provide(ctx);
    opener.open_popup(tab).await
}

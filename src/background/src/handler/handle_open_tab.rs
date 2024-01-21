use domain::{DomainError, LinkOpener, LinkOpenerProvider, Url};
use open_tab_context::OpenTabContext;

pub(crate) async fn handle_open_tab(url: Url) -> Result<(), DomainError> {
    let ctx = OpenTabContext::new();

    let link_opener = LinkOpenerProvider::provide(&ctx);
    Ok(link_opener.open(&url).await?)
}

use domain::{DomainError, LinkOpener, LinkOpenerProvider, Url};

pub async fn open_link<T>(ctx: &T, url: &Url) -> Result<(), DomainError>
where
    T: LinkOpenerProvider,
{
    let link_opener = LinkOpenerProvider::provide(ctx);
    Ok(link_opener.open(url).await?)
}

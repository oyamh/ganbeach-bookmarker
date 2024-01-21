use domain::{BrowserBookmarker, BrowserBookmarkerProvider, DomainError, PageUrl, Title};

pub async fn create_browser_bookmark<T>(
    ctx: &T,
    title: Title,
    url: PageUrl,
    parent_title: Option<Title>,
) -> Result<(), DomainError>
where
    T: BrowserBookmarkerProvider,
{
    let bookmarker = BrowserBookmarkerProvider::provide(ctx);
    Ok(bookmarker.create(title, url, parent_title).await?)
}

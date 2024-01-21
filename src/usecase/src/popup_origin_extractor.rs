use domain::{PopupOriginExtractor, PopupOriginExtractorProvider};

pub fn extract_popup_origin<T>(ctx: &T) -> Result<domain::PopupOrigin, domain::DomainError>
where
    T: PopupOriginExtractorProvider,
{
    let extractor = PopupOriginExtractorProvider::provide(ctx);
    extractor.extract_popup_origin()
}

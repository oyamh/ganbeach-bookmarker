#![allow(refining_impl_trait)]
use domain::PopupOriginExtractorProvider;
use web_interface::PageDataGetter;

#[derive(Debug, Default)]
pub struct ExtractPopupOriginContext {
    extractor: PageDataGetter,
}

impl ExtractPopupOriginContext {
    pub fn new() -> Self {
        Self {
            extractor: PageDataGetter,
        }
    }
}

impl PopupOriginExtractorProvider for ExtractPopupOriginContext {
    fn provide(&self) -> &PageDataGetter {
        &self.extractor
    }
}

use domain::{DomainError, PopupOrigin};
use extract_popup_origin_context::ExtractPopupOriginContext;

pub(crate) fn handle_extract_popup_origin() -> Result<PopupOrigin, DomainError> {
    let ctx = ExtractPopupOriginContext::new();
    usecase::extract_popup_origin(&ctx)
}

use domain::{DomainError, PopupOrigin, PopupOriginExtractor, Url};
use web_sys::{window, Location};

use crate::{FromJsError, PageDataError};

#[derive(Debug, Default)]
pub struct PageDataGetter;

impl PageDataGetter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PopupOriginExtractor for PageDataGetter {
    fn extract_popup_origin(&self) -> Result<PopupOrigin, DomainError> {
        let url = extract_url().map_err(|error| DomainError::WebInterface(error.to_string()))?;
        let query_params = PopupOrigin::from(url);
        Ok(query_params)
    }
}

fn get_location() -> Option<Location> {
    window()?.document()?.location()
}

fn extract_url() -> Result<Url, PageDataError> {
    let href = get_location()
        .ok_or(PageDataError::NoLocation)?
        .href()
        .map_err(Into::<FromJsError>::into)?;
    Ok(Url::try_from(href).map_err(|error| PageDataError::ParseUrl(error.to_string()))?)
}

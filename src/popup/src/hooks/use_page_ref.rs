use crate::handler::handle_extract_popup_origin;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[hook]
pub fn use_page_ref() -> PageRef {
    log::debug!("use_page_ref");
    let page_ref = PageRef {
        name_ref: use_node_ref(),
        url_state: use_state_eq(|| String::default()),
    };

    {
        let page_ref = page_ref.clone();
        use_effect_with_deps(
            move |_e| match handle_extract_popup_origin() {
                Ok(popup_origin) => {
                    log::debug!("title={} url={}", &popup_origin.title, &popup_origin.url);
                    page_ref.set_page_data(popup_origin.title, popup_origin.url)
                }
                Err(error) => {
                    log::error!("handle_extract_popup_origin error={}", error.to_string())
                }
            },
            (),
        );
    }
    page_ref
}

pub struct PageRef {
    name_ref: NodeRef,
    url_state: UseStateHandle<String>,
}

impl PageRef {
    pub fn set_page_data(&self, name: impl AsRef<str>, url: impl AsRef<str>) {
        if let Some(name_ref) = self.name_ref.cast::<HtmlInputElement>() {
            name_ref.set_value(name.as_ref());
        }
        self.url_state.set(url.as_ref().to_string());
    }

    pub fn value(&self) -> (String, String) {
        let name = self
            .name_ref
            .get()
            .unwrap()
            .unchecked_ref::<HtmlInputElement>()
            .value();
        let url = (*self.url_state).clone();
        (name, url)
    }

    pub fn name_ref(&self) -> NodeRef {
        self.name_ref.clone()
    }

    pub fn has_url(&self) -> bool {
        self.url_state.is_empty()
    }
}

impl Clone for PageRef {
    fn clone(&self) -> Self {
        Self {
            name_ref: self.name_ref.clone(),
            url_state: self.url_state.clone(),
        }
    }
}

impl PartialEq for PageRef {
    fn eq(&self, other: &Self) -> bool {
        *self.url_state == *other.url_state
    }
}

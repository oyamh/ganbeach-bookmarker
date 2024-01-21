use crate::{
    handler::handle_request_open_tab,
    icons::{FolderIcon, TagIcon},
};
use config::SERVER_URL_BASE;
use domain::{BookmarkId, Title, TypeCode, Url};
use gloo_utils::window;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StoredListProps {
    pub list_id: BookmarkId,
    pub type_code: TypeCode,
    pub title: Title,
}

const CSS_STORED_LIST: &'static str = include_str!("../css/stored_list.css");

#[function_component]
pub fn StoredList(props: &StoredListProps) -> Html {
    let type_code = props.type_code;
    let list_id = props.list_id;
    let onclick = move |_| {
        let url_string = format!(
            "{}/{}/{}",
            SERVER_URL_BASE,
            Into::<&str>::into(type_code),
            &list_id,
        );
        if let Err(error) = window().open_with_url_and_target_and_features(
            &url_string,
            "_blank",
            "noreferrer,noopener",
        ) {
            log::error!("error: {error:?}");
        }
    };

    let onkeydown = {
        move |e: KeyboardEvent| match e.key().as_str() {
            "Enter" => {
                // spawn_local(async move {
                //     match window().open_with_url_and_target_and_features(
                //         &url_string,
                //         "_blank",
                //         "noreferrer,noopener",
                //     ) {
                //     }
                // });

                let url_string = format!(
                    "{}/{}/{}",
                    SERVER_URL_BASE,
                    Into::<&str>::into(type_code),
                    &list_id,
                );
                spawn_local(async move {
                    let url = Url::try_from(url_string).unwrap();
                    handle_request_open_tab(url).await;
                });
            }
            _ => {}
        }
    };

    let icon_elem = match props.type_code {
        TypeCode::Folder => html! {<FolderIcon />},
        TypeCode::Tag => html! {<TagIcon />},
        _ => html! {<FolderIcon />},
    };

    html! {
        <>
        <style>
            {CSS_STORED_LIST}
        </style>

        <div class="stored-list-container">
            <div class="stored-list-wrapper" onclick={onclick} onkeydown={onkeydown} tabindex="0">
                <div class="stored-list-icon">{icon_elem}</div>
                <div class="stored-list-title">{&props.title}</div>
            </div>
        </div>
        </>
    }
}

use crate::{components::stored_list::StoredList, hooks::use_lists_context::use_lists_context};
use domain::{BookmarkId, TagIds, Title, TypeCode};
use yew::prelude::*;
// use gloo_timers::future::TimeoutFuture;
use gloo_utils::window;
// use wasm_bindgen_futures::spawn_local;

// const CREATED_RESULT_TIMEOUT: u32 = 6000;

#[derive(Properties, PartialEq)]
pub(crate) struct CreatedResultProps {
    pub folder_id: BookmarkId,
    pub folder_title: Title,
    pub tag_ids: TagIds,
    // #[prop_or_default]
    // pub children: Children,
}

const CSS_CREATED_RESULT: &'static str = include_str!("../css/created_result.css");

#[function_component(CreatedResult)]
pub(crate) fn created_result(props: &CreatedResultProps) -> Html {
    let CreatedResultProps {
        folder_id,
        folder_title,
        tag_ids,
    } = props;

    let lists_ctx = use_lists_context();

    // let folder_title = lists_ctx
    //     .list_title_by_id(folder_id)
    //     // .or(Some(Title::from(format!("Folder:{}", folder_id))))
    //     .or(Some(Title::from(folder_id.to_string())))
    //     .unwrap()
    //     .to_owned();

    let folder_link = html! {
        <StoredList list_id={*folder_id} type_code={TypeCode::Folder} title={folder_title.to_owned()}/>
    };

    let tag_links = tag_ids
        .iter()
        .map(|tag_id| {
            let tag_title = &lists_ctx
                .list_title_by_id(tag_id)
                // .or(Some(Title::from(format!("Tag:{}", tag_id))))
                .or(Some(Title::from(tag_id.to_string())))
                .unwrap()
                .to_owned();
            html! {
                <StoredList list_id={*tag_id} type_code={TypeCode::Tag} title={tag_title.clone()} />
            }
        })
        .collect::<Html>();

    let close = |_e| {
        let result = window().close();
        log::debug!("result={result:?}");
    };

    html! {
        <>

        <style>
            {CSS_CREATED_RESULT}
        </style>

        <div id="header">
            {"Bookmarked"}
        </div>

        <div id="main">
            {folder_link}
            {tag_links}
        </div>

        <div id="sub">
            <div class={classes!("button-container")} title={"Close"}>
                <div class={classes!("button-wrapper", "cancel-button")} onclick={close}>
                    <button class={classes!("button", "close-created")} onclick={close} type="button">{"CLOSE"}</button>
                </div>
            </div>
        </div>

        </>
    }
}

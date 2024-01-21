use crate::{
    components::{
        annotation::Annotation, dialog::Dialog, input_field::InputField,
        suggest_panel::SuggestPanel,
    },
    handler::handle_send_creating_bookmark,
    hooks::{
        use_annotation::use_annotation, use_dialog_ref::use_dialog_ref,
        use_lists_context::use_lists_context, use_page_ref::use_page_ref,
        use_restore_metadata::use_restore_metadata, use_scene_context::use_scene_context,
    },
};
use domain::{BookmarkId, FolderTitle, Scene, TypeCode};
use domain::{CreateBookmarkCommand, PageUrl, Title};
use gloo_events::EventListener;
use gloo_utils::window;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlButtonElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

pub const NAME_ELEM_ID: &'static str = "name-input";
pub const FOLDER_ELEM_ID: &'static str = "folder-input";
pub const TAGS_ELEM_ID: &'static str = "tags-input";
pub const SUBMIT_BUTTON_ID: &'static str = "submit-button";

const CSS_INPUT: &'static str = include_str!("../css/input.css");

#[function_component(InputFields)]
pub fn input_fields(_props: &Props) -> Html {
    log::debug!("InputFields::input_fields");
    let lists_ctx = use_lists_context();
    let scene_ctx = use_scene_context();
    let bookmark_ref = use_page_ref();
    let folder_ref = use_node_ref();
    let tags_ref = use_node_ref();
    let annotation_ref = use_annotation();
    let (dialog_ref, dialog_state) = use_dialog_ref();
    use_restore_metadata(&folder_ref, &tags_ref);

    let send_callback = Callback::from({
        let lists_ctx = lists_ctx.clone();
        log::debug!("send_callback before");
        // log::debug!("lists_ctx: {:#?}", &lists_ctx);
        let bookmark_ref = bookmark_ref.clone();
        let folder_ref = folder_ref.clone();
        let tags_ref = tags_ref.clone();
        let dialog_state = dialog_state.clone();

        move |_| {
            log::debug!("send_callback");
            let (name_value, url_value) = bookmark_ref.value();
            let folder_value = get_ref_value(&folder_ref);
            let tags_value = get_ref_value(&tags_ref);
            log::debug!("name: {}", &name_value);
            log::debug!("url: {:#?}", &url_value);
            log::debug!("folder: {}", &folder_value);
            log::debug!("tags: {}", &tags_value);

            let annotation_value = annotation_ref.value().unwrap_or_default();
            log::debug!("annotation_value: {}", &annotation_value);

            let title = Title::new(&name_value);

            let folder_result = lists_ctx.folder_id_by_name(&folder_value);

            let (tags, unknown_tags): (
                Vec<(Title, Option<BookmarkId>)>,
                Vec<(Title, Option<BookmarkId>)>,
            ) = tags_value
                .split(',')
                .filter(|tag_title| !tag_title.trim().is_empty())
                .map(|tag_title| {
                    let tag_id = &lists_ctx.tag_id_by_name(&tag_title);
                    (tag_title.into(), *tag_id)
                })
                .partition(|(_tag_title, tag_id)| tag_id.is_some());

            let tag_ids = tags
                .into_iter()
                .map(|(_title, id)| id.unwrap())
                .collect::<Vec<BookmarkId>>();

            let unknown_tag_titles = unknown_tags
                .into_iter()
                .map(|(title, _id)| title)
                .collect::<Vec<Title>>();

            let url = PageUrl::from(url_value);

            let scene_ctx = scene_ctx.clone();
            let mut dialog_state = dialog_state.clone();
            spawn_local(async move {
                let (folder_id, folder_title) = match folder_result {
                    Some(folder_id) => (Some(folder_id), FolderTitle::Old(folder_value.into())),
                    None => {
                        log::debug!("unknown folder folder_title={}", &folder_value);
                        dialog_state.show_modal();
                        if !dialog_state.await.is_ok() {
                            return;
                        }
                        (None, FolderTitle::New(folder_value.into()))
                    }
                };

                if folder_title.is_empty() {
                    scene_ctx.set(Scene::Error {
                        message: "Folder is empty.".to_string(),
                    });
                    return;
                }

                let command = CreateBookmarkCommand::new(
                    title,
                    url,
                    TypeCode::Link,
                    folder_id,
                    tag_ids.into(),
                    unknown_tag_titles.into(),
                    folder_title,
                    annotation_value.into(),
                );
                log::debug!("command: {command:?}");

                scene_ctx.set(Scene::Creating);

                match handle_send_creating_bookmark(command).await {
                    Ok(scene) => match scene {
                        Scene::Created {
                            folder_id,
                            folder_title,
                            tag_ids,
                        } => scene_ctx.set(Scene::Created {
                            folder_id,
                            folder_title,
                            tag_ids,
                        }),
                        Scene::Error { message } => scene_ctx.set(Scene::Error { message }),
                        _ => scene_ctx.set(Scene::Error {
                            message: "Unexpected created result.".to_string(),
                        }),
                    },
                    Err(error) => {
                        scene_ctx.set(Scene::Error {
                            message: error.to_string(),
                        });
                    }
                }
            });
        }
    });

    let onclick_send = {
        let send_callback = send_callback.clone();
        move |_e| {
            send_callback.emit(());
        }
    };

    let cancel = move |_e: MouseEvent| {
        log::debug!("cancel");
        let result = window().close();
        log::debug!("result={result:?}");
    };

    {
        let lists_ctx = lists_ctx.clone();
        let send_callback = send_callback.clone();
        let bookmark_ref = bookmark_ref.clone();
        use_effect_with_deps(
            move |_e| {
                let send_callback = send_callback.clone();
                let listener = EventListener::new(&window(), "keydown", move |e| {
                    e.stop_propagation();
                    e.stop_immediate_propagation();
                    if e.target()
                        .is_some_and(|target| target.dyn_ref::<HtmlButtonElement>().is_some())
                    {
                        log::debug!("Ctrl + Enter on button element");
                        return;
                    }
                    let key_event = e.unchecked_ref::<web_sys::KeyboardEvent>();
                    if key_event.ctrl_key() && key_event.code() == "Enter" {
                        log::debug!("Ctrl + Enter");
                        send_callback.emit(());
                    }
                });
                || drop(listener)
            },
            (lists_ctx, bookmark_ref),
        );
    }

    html! {
        <>
            <style>
                {CSS_INPUT}
            </style>

            <div id="header">
                {"Create Bookmark"}
            </div>

            <div id="main">

                <InputField label={"Name"} label_id={NAME_ELEM_ID} input_ref={bookmark_ref.name_ref()} />
                <InputField label={"Folder"} label_id={FOLDER_ELEM_ID} input_ref={folder_ref.clone()} placeholder={"required"}>
                    <SuggestPanel input_ref={folder_ref.clone()} type_code={TypeCode::Folder} />
                </InputField>
                <InputField label={"Tags"} label_id={TAGS_ELEM_ID} input_ref={tags_ref.clone()} autofocus={true}>
                    <SuggestPanel input_ref={tags_ref.clone()} type_code={TypeCode::Tag} />
                </InputField>

                <Dialog dialog_ref={dialog_ref} />

                <div class={classes!("buttons-container")}>
                    <div class={classes!("buttons-wrapper")}>

                        <div class={classes!("button-container")} title={"Send (Ctrl+Enter)"}>
                            <div class={classes!("button-wrapper", "send-button")}>
                                <button id={SUBMIT_BUTTON_ID} class={classes!("button")} onclick={onclick_send} type="button">{"SEND (Ctrl+Enter)"}</button>
                            </div>
                        </div>

                        <div class={classes!("button-container")} title={"Cancel"}>
                            <div class={classes!("button-wrapper", "cancel-button")}>
                                <button class={classes!("button")} onclick={cancel} type="button">{"CANCEL"}</button>
                            </div>
                        </div>

                    </div>
                </div>

            </div>

            <div id="sub">
                <Annotation />
            </div>
        </>
    }
}

fn get_ref_value(node_ref: &NodeRef) -> String {
    node_ref
        .get()
        .unwrap()
        .unchecked_ref::<HtmlInputElement>()
        .value()
}

// // #![cfg(target_arch = "wasm32")]
// #[test]
// fn should_send_bookmark() {
//     use wasm_bindgen_test::*;

//     wasm_bindgen_test_configure!(run_in_browser);

//     todo!()
// }

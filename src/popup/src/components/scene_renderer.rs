use super::created_result::CreatedResult;
use super::creating::Creating;
use super::error_result::ErrorResult;
use super::input_fields::InputFields;
use crate::hooks::use_scene_context::use_scene_context;
use domain::Scene;
use gloo_utils::document_element;
use yew::prelude::*;

pub const HTML_ATTRIBUTE_NAME_SCENE: &'static str = "data-scene";

#[function_component(SceneRenderer)]
pub fn scene_renderer() -> Html {
    log::debug!("SceneRenderer");
    let scene_ctx = use_scene_context();
    let scene_str: &str = scene_ctx.as_ref();
    if let Err(error) = document_element().set_attribute(HTML_ATTRIBUTE_NAME_SCENE, scene_str) {
        log::error!("document_element.set_attribute error: {error:?}");
    }
    log::debug!("scene_str={scene_str}");
    match &*scene_ctx {
        Scene::Input => html! {
            <InputFields />
        },
        Scene::Created {
            folder_id,
            folder_title,
            tag_ids,
        } => html! {
            <CreatedResult folder_id={*folder_id} folder_title={folder_title.to_owned()} tag_ids={tag_ids.to_owned()} />
        },
        Scene::Creating => html! {
            <Creating />
        },
        Scene::Error { message } => html! {
            <ErrorResult message={message.clone()} />
        },
    }
}

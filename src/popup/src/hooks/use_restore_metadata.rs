use crate::{handler::handle_recent_metadata, hooks::use_lists_context::use_lists_context};
use domain::{BookmarkId, DomainError, TagIds};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[hook]
pub fn use_restore_metadata(folder_ref: &NodeRef, tags_ref: &NodeRef) {
    let lists_ctx = use_lists_context();

    use_effect_with_deps(
        {
            let lists_ctx = lists_ctx.clone();
            // log::debug!("lists_ctx: {lists_ctx:?}");
            let folder_ref = folder_ref.clone();
            let tags_ref = tags_ref.clone();
            move |_| {
                let lists_ctx = lists_ctx.clone();
                // log::debug!("lists_ctx: {lists_ctx:?}");
                let folder_ref = folder_ref.clone();
                let tags_ref = tags_ref.clone();
                spawn_local(async move {
                    let get_titles =
                        |(folder_id, tag_ids): (Option<BookmarkId>, Option<TagIds>)| {
                            log::debug!("handle_recent_metadata folder_id={folder_id:?}");
                            let folder_title = folder_id
                                .and_then(|folder_id| lists_ctx.list_title_by_id(&folder_id))
                                .or(lists_ctx.latest_folder_title())
                                .unwrap_or_default();

                            let tag_titles = tag_ids
                                .unwrap_or_default()
                                .iter()
                                .map(|tag_id| {
                                    lists_ctx
                                        .list_title_by_id(tag_id)
                                        .unwrap_or_default()
                                        .to_string()
                                })
                                .filter(|title| !title.is_empty())
                                .collect::<Vec<String>>()
                                .join(",");
                            (folder_title, tag_titles)
                        };

                    let result: Result<(), DomainError> = handle_recent_metadata()
                        .await
                        .map(get_titles)
                        .or_else(|error| {
                            log::debug!("handle_recent_metadata error={error:?}");
                            // log::debug!("lists_ctx: {lists_ctx:?}");
                            Ok((
                                lists_ctx.latest_folder_title().unwrap_or_default(),
                                String::default(),
                            ))
                        })
                        .and_then(|(folder_title, tag_titles)| {
                            folder_ref
                                .get()
                                .unwrap()
                                .unchecked_ref::<HtmlInputElement>()
                                .set_value(folder_title.as_ref());

                            tags_ref
                                .get()
                                .unwrap()
                                .unchecked_ref::<HtmlInputElement>()
                                .set_value(&tag_titles);

                            Ok(())
                        });
                    if let Err(error) = result {
                        log::debug!("handle_recent_metadata error={error:#?}");
                    }
                });
            }
        },
        lists_ctx.clone(),
    );
}

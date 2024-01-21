use crate::{
    handler::handle_request_lists,
    hooks::use_lists_context::{ListsAction, ListsContext, ListsState},
    hooks::use_scene_context::use_scene_context,
};
use domain::Lists;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[hook]
fn use_lists_fetch(ctx: ListsContext) {
    let scene_ctx = use_scene_context();
    use_effect_with_deps(
        move |_e| {
            spawn_local(async move {
                let ctx2 = ctx.clone();
                let lists_callback = move |lists: Lists| {
                    log::debug!("lists_callback!! lists.len: {:?}", &lists.len());
                    // log::debug!("lists: {:#?}", &lists);
                    ctx.dispatch(ListsAction::Set(lists));
                };
                let stored_lists_callback = move |lists: Lists| {
                    log::debug!("stored_lists_callback!! lists.len: {:?}", &lists.len());
                    ctx2.dispatch(ListsAction::Set(lists));
                };
                let error_callback = move |error| {
                    log::error!("error_callback!!: {error}");
                    scene_ctx.set(domain::Scene::Error { message: error });
                };
                handle_request_lists(lists_callback, stored_lists_callback, error_callback).await;
            });
            || {}
        },
        (),
    );
}

#[function_component(ListsContextProvider)]
pub fn lists_context_provider(props: &Props) -> Html {
    let ctx = use_reducer(|| ListsState::default());
    {
        let ctx = ctx.clone();
        use_lists_fetch(ctx);
    }

    html! {
        <ContextProvider<ListsContext> context={ctx}>
            {for props.children.iter()}
        </ContextProvider<ListsContext>>
    }
}

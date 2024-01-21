use crate::hooks::use_scene_context::SceneContext;
use domain::Scene;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(SceneContextProvider)]
pub fn scene_context_provider(props: &Props) -> Html {
    let state = use_state(|| Scene::default());
    html! {
        <ContextProvider<SceneContext> context={state}>
            {for props.children.iter()}
        </ContextProvider<SceneContext>>
    }
}

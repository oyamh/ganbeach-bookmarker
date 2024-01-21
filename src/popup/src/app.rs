use crate::components::{
    lists_context_provider::ListsContextProvider, scene_context_provider::SceneContextProvider,
    scene_renderer::SceneRenderer,
};
use yew::prelude::*;

const CSS_INDEX: &'static str = include_str!("./css/index.css");

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <style>
            {CSS_INDEX}
        </style>
        <div id="app-container">
            <div id="app-wrapper">

                <SceneContextProvider>
                    <ListsContextProvider>
                        <SceneRenderer />
                    </ListsContextProvider>
                </SceneContextProvider>

            </div>
        </div>
        </>
    }
}

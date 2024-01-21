use yew::prelude::*;

// #[derive(Properties, PartialEq)]
// pub struct Props {}

#[function_component(Annotation)]
pub fn annotation() -> Html {
    //_props: &Props
    // let easy_mde = use_state(|| None);

    // {
    //     let easy_mde = easy_mde.clone();
    //     use_effect_with_deps(
    //         move |_| {
    //             easy_mde.set(Some(EasyMDEWrapper::new("annotation-editor".to_string())));
    //             // easy_mde.as_ref().map(|mde| {
    //             //     mde.set_value("test".to_string());
    //             // });
    //             move || easy_mde.set(None)
    //         },
    //         (),
    //     );
    // }

    html! {
        <div class={classes!("annotation-container")}>
            <div class={classes!("annotation-wrapper")}>
                <div class={classes!("annotation-label")}>{"Annotation"}</div>
                <textarea name="annotation-editor" id="annotation-editor"></textarea>
            </div>
        </div>
    }
}

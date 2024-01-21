use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: String,
    // #[prop_or_default]
    // pub children: Children,
}

#[function_component(ErrorResult)]
pub fn error_result(props: &Props) -> Html {
    html! {
        <>
            <div id="header">
                {"Error"}
            </div>
            <div id="main">
                {props.message.clone()}
                // {for props.children.iter()}
            </div>
            <div id="sub">
            </div>
        </>
    }
}

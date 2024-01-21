use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CreatingProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Creating(_props: &CreatingProps) -> Html {
    // let CreatingProps {} = props;
    html! {
        <div>
            // {for props.children.iter()}
            {"Creating Now..."}
        </div>
    }
}

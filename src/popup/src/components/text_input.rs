use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: String,
    pub placeholder: String,
    //pub on_change: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props { id, placeholder } = props.clone();
    html! {
        <div class={classes!("input-container")}>
            <div class={classes!("input-wrapper")}>
                <input type="text" id={id} placeholder={placeholder} />
            </div>
        </div>
    }
}

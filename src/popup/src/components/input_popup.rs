use gloo_events::EventListener;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub input_ref: NodeRef,
    pub class_name: &'static str,
    // #[prop_or_default]
    pub children: Children,
}

#[function_component(InputPopup)]
pub fn input_popup(props: &Props) -> Html {
    let Props {
        input_ref,
        class_name,
        children,
    } = props.clone();
    let details_ref = use_node_ref();

    {
        let input_ref = input_ref.clone();
        let details_ref = details_ref.clone();
        use_effect_with_deps(
            move |_| {
                set_details_open(&details_ref);
                let listener = if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let on_focus = {
                        EventListener::new(&input, "focus", move |_e| {
                            set_details_open(&details_ref);
                        })
                    };
                    Some(on_focus)
                } else {
                    None
                };
                || drop(listener)
            },
            (),
        );
    }

    html! {
        <details ref={details_ref.clone()} class={class_name} open={true}>
            <summary />
            {for children.iter()}
        </details>
    }
}

fn set_details_open(details_ref: &NodeRef) {
    if let Some(details) = details_ref.cast::<HtmlElement>() {
        let _result = details.set_attribute("open", "true");
    }
}

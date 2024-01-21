use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub input_ref: NodeRef,
    pub label: &'static str,
    pub label_id: &'static str,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub placeholder: &'static str,
    #[prop_or_default]
    pub children: Children,
    // pub children: ChildrenWithProps<HtmlInputElement>,
}

#[function_component(InputField)]
pub fn input_field(props: &Props) -> Html {
    let Props {
        input_ref,
        label,
        label_id,
        autofocus,
        placeholder,
        children,
    } = props.clone();

    {
        let input_ref = input_ref.clone();
        use_effect_with_deps(
            move |_| {
                if autofocus {
                    if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                        input.focus().unwrap();
                    }
                }
                || ()
            },
            (),
        );
    }

    let focus_input = {
        let input_ref = input_ref.clone();
        move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.focus().expect("focus should be success");
            }
        }
    };

    html! {
        <div class={classes!("field-container")}>
            <div class={classes!("field-wrapper")}>
                <label for={label_id}>
                    {label}
                </label>
                <div class={classes!("input-container")}>
                    <div class={classes!("input-wrapper")} onclick={focus_input}>
                        <input ref={input_ref.clone()} name={label_id} id={label_id} placeholder={placeholder} type="text" />
                    </div>
                    {for children.iter()}
                </div>
            </div>
        </div>
    }
}

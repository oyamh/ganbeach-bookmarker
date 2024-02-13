use std::rc::Rc;

use domain::{SuggestResult, TypeCode};
use yew::{prelude::*, virtual_dom::AttrValue};

use super::suggest_panel::set_selected_value;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_ref: NodeRef,
    pub result: Rc<SuggestResult>,
    pub type_code: TypeCode,
}

#[function_component(SuggestRow)]
pub fn suggest_row(props: &Props) -> Html {
    let Props {
        input_ref,
        result,
        type_code,
    } = props;

    let on_click_row = {
        let input_ref = input_ref.clone();
        let result = result.clone();
        let type_code = type_code.clone();
        Callback::from(move |_e: MouseEvent| {
            set_selected_value(&input_ref, &result.title, type_code);
        })
    };

    html! {
        <div class={classes!("suggest-row-container")} onclick={on_click_row} data-suggest-title={Some(AttrValue::from(result.title.to_string()))}>
            <div class={classes!("suggest-row-wrapper")}>
                <div class={classes!("suggest-row-title")}>
                    {&result.title}
                </div>
                <div class={classes!("suggest-row-count")}>
                    {&result.count}
                </div>
            </div>
        </div>
    }
}

use crate::hooks::use_suggest::use_suggest;
use crate::{
    components::{input_popup::InputPopup, suggest_row::SuggestRow},
    hooks::use_suggest::SuggestResults,
};
use domain::TypeCode;
use gloo_events::{EventListener, EventListenerOptions};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement, ScrollIntoViewOptions, ScrollLogicalPosition};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_ref: NodeRef,
    pub type_code: TypeCode,
}

enum KeyAction {
    Up,
    Down,
    Enter,
    Nop,
}

#[function_component(SuggestPanel)]
pub fn suggest_panel(props: &Props) -> Html {
    let Props {
        input_ref,
        type_code,
    } = props;
    let rows_parent_ref = use_node_ref();

    let suggested_results = use_suggest(input_ref, *type_code);

    {
        let input_ref = input_ref.clone();
        let rows_parent_ref = rows_parent_ref.clone();
        let suggested_results = suggested_results.clone();
        let suggested_results_2 = suggested_results.clone();
        let type_code = type_code.clone();
        use_effect_with_deps(
            move |_| {
                let listener = if suggested_results.len() == 0 {
                    None
                } else if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let on_input = {
                        let options = EventListenerOptions::enable_prevent_default();
                        EventListener::new_with_options(&input, "keydown", options, move |e| {
                            log::debug!("keydown");
                            let event = e.dyn_ref::<web_sys::KeyboardEvent>();
                            // let key_event = e.unchecked_ref::<web_sys::KeyboardEvent>();
                            event.map(|key_event| {
                                //NOTE: KeyNとKeyPにしたかったが、Ctrl + KeyNは仕様で上書きできなかった。
                                let code = key_event.code();
                                let key_action = if key_event.ctrl_key() {
                                    match code.as_str() {
                                        "KeyJ" => KeyAction::Down,
                                        "KeyK" => KeyAction::Up,
                                        _ => KeyAction::Nop,
                                    }
                                } else if code == "Tab" {
                                    match key_event.shift_key() {
                                        true => KeyAction::Up,
                                        false => KeyAction::Down,
                                    }
                                } else if code == "ArrowDown" {
                                    KeyAction::Down
                                } else if code == "ArrowUp" {
                                    KeyAction::Up
                                } else if code == "Enter" {
                                    KeyAction::Enter
                                } else if code == "Escape" {
                                    e.target().map(|target| {
                                        target.unchecked_ref::<HtmlInputElement>().blur()
                                    });
                                    KeyAction::Nop
                                } else {
                                    KeyAction::Nop
                                };

                                match key_action {
                                    KeyAction::Nop => {}
                                    KeyAction::Enter => {
                                        key_event.prevent_default();
                                        if let Some(value) =
                                            extract_selected_value(&rows_parent_ref)
                                        {
                                            set_selected_value(
                                                &input_ref,
                                                value.as_str(),
                                                type_code,
                                            );
                                            // append_selected_value(&input_ref, value.as_str());
                                            suggested_results.set(SuggestResults::default());
                                        };
                                    }
                                    _ => {
                                        key_event.prevent_default();
                                        move_cursor(&rows_parent_ref, key_action);
                                    }
                                }
                            });
                        })
                    };
                    Some(on_input)
                } else {
                    None
                };
                || drop(listener)
            },
            suggested_results_2.len() > 0,
        );
    }

    let stop_propagation = |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        e.stop_immediate_propagation();
    };

    let on_click_rows = {
        let input_ref = input_ref.clone();
        let suggested_results = suggested_results.clone();
        Callback::from(move |_e: MouseEvent| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                suggested_results.set(SuggestResults::default());
                input.focus().expect("focus should be success");
            }
        })
    };

    if suggested_results.len() > 0 {
        html! {
            <InputPopup input_ref={input_ref.clone()} class_name={"suggest-panel-popup"}>
                <div onclick={stop_propagation} class={classes!("suggest-panel-container")}>
                    <div ref={rows_parent_ref.clone()} onclick={on_click_rows} class={classes!("suggest-panel-wrapper")}>
                        {for suggested_results.iter().map(|result| {
                            html! {
                                <SuggestRow input_ref={input_ref.clone()} result={result} type_code={*type_code} />
                            }
                        })}
                    </div>
                </div>
            </InputPopup>
        }
    } else {
        html! {}
    }
}

const SELECTED_ROW_ID_NAME: &'static str = "suggest-cursor";
const SELECTED_ROW_ID: &'static str = "#suggest-cursor";

fn extract_selected_value(rows_parent_ref: &NodeRef) -> Option<String> {
    rows_parent_ref.cast::<Element>().and_then(|parent| {
        let elem = parent.query_selector(SELECTED_ROW_ID).ok().flatten()?;
        elem.get_attribute("data-suggest-title")
    })
}

pub fn set_selected_value(input_ref: &NodeRef, selected_value: &str, type_code: TypeCode) {
    let split_pattern = match type_code {
        TypeCode::Folder => None,
        TypeCode::Tag => Some(","),
        _ => None,
    };
    let Some(input) = input_ref.cast::<HtmlInputElement>() else {
        return;
    };
    let Some(pattern) = split_pattern else {
        input.set_value(selected_value);
        return;
    };
    let value = input.value();
    let value: &str = &value;
    let Some((another_values, _)) = value.rsplit_once(pattern) else {
        input.set_value(&format!("{selected_value}{pattern}"));
        return;
    };
    input.set_value(&format!(
        "{another_values}{pattern}{selected_value}{pattern}"
    ));
}

// pub fn append_selected_value(input_ref: &NodeRef, selected_value: &str) {
//     if let Some(input) = input_ref.cast::<HtmlInputElement>() {
//         let value = input.value();
//         let value: &str = &value;
//         // let another_values = value.rsplit_once(",").map_or(value, |tupple| tupple.0);
//         // input.set_value(&format!("{another_values},{},", selected_value));
//         match value.rsplit_once(",") {
//             Some((another_values, _)) => {
//                 input.set_value(&format!("{another_values},{},", selected_value));
//             }
//             None => input.set_value(&format!("{selected_value},")),
//         }
//     }
// }

fn move_cursor(rows_parent_ref: &NodeRef, key_action: KeyAction) {
    rows_parent_ref.cast::<Element>().map(|parent| {
        let alter_child = match key_action {
            KeyAction::Down => parent.first_element_child(),
            KeyAction::Up => parent.last_element_child(),
            _ => None,
        };
        match parent.query_selector(SELECTED_ROW_ID).ok().flatten() {
            Some(elem) => {
                let target_child = match key_action {
                    KeyAction::Down => elem.next_element_sibling(),
                    KeyAction::Up => elem.previous_element_sibling(),
                    _ => None,
                };
                match target_child {
                    Some(target) => {
                        target.set_id(SELECTED_ROW_ID_NAME);
                        // target.scroll_into_view_with_scroll_into_view_options(
                        //     &create_scroll_options(&parent, &target),
                        // );
                        scroll_into_view(&parent, &target);
                    }
                    None => {
                        alter_child.map(|alter| {
                            alter.set_id(SELECTED_ROW_ID_NAME);
                            // alter.scroll_into_view_with_scroll_into_view_options(
                            //     &create_scroll_options(&parent, &alter),
                            // );
                            scroll_into_view(&parent, &alter);
                        });
                    }
                };
                let _result = elem.remove_attribute("id");
            }
            None => {
                alter_child.map(|alter| {
                    alter.set_id(SELECTED_ROW_ID_NAME);
                    // alter.scroll_into_view_with_scroll_into_view_options(&create_scroll_options(
                    //     &parent, &alter,
                    // ));
                    scroll_into_view(&parent, &alter);
                });
            }
        };
    });
}

fn scroll_into_view(parent: &Element, child: &Element) {
    child.scroll_into_view_with_scroll_into_view_options(&create_scroll_options(parent, child));
}

fn create_scroll_options(parent: &Element, child: &Element) -> ScrollIntoViewOptions {
    let mut options = ScrollIntoViewOptions::new();
    if &parent.first_element_child().unwrap() == child {
        options.block(ScrollLogicalPosition::Center);
    } else {
        options.block(ScrollLogicalPosition::Nearest);
    }
    options
}

// let calc_selected_index = {
//     let suggested_results = suggested_results.clone();
//     let selected_index = selected_index.clone();
//     move |key_action: KeyAction| {
//         let new_index = match key_action {
//             KeyAction::Up => {
//                 log::debug!("Up");
//                 if *selected_index - 1 < 0 {
//                     suggested_results.len() as i32
//                 } else {
//                     *selected_index - 1
//                 }
//             }
//             KeyAction::Down => {
//                 log::debug!("Down");
//                 log::debug!("results: {:?}", *suggested_results);
//                 log::debug!("len1: {}", (*suggested_results).len());
//                 log::debug!("len3: {}", suggested_results.len());
//                 log::debug!("output: {}", *selected_index + 1);
//                 if *selected_index + 1 >= suggested_results.len() as i32 {
//                     0
//                 } else {
//                     *selected_index + 1
//                 }
//             }
//             _ => *selected_index,
//         };
//         log::debug!("new_index: {new_index:?}");
//         selected_index.set(new_index);
//     }
// };

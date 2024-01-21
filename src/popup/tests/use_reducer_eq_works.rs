#![cfg(target_arch = "wasm32")]

use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;

use gloo_utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;
// use gloo::timers::future::sleep;
use yew::platform::time::sleep;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Clone, PartialEq)]
struct ContentState {
    content: HashSet<String>,
}

impl Reducible for ContentState {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut self_: Self = (*self).clone();
        self_.content.insert(action);
        self_.into()
    }
}

//wasm-pack test --firefox --headless -- --test use_reducer_eq_works
//[引用](https://github.com/YoungHaKim7/todomvc/blob/b6c6c98df3d32665d8079c249b15b47b1a389e02/packages/yew/tests/use_reducer.rs)
#[wasm_bindgen_test]
async fn use_reducer_eq_works() {
    #[function_component(UseReducerComponent)]
    fn use_reducer_comp() -> Html {
        let content = use_reducer_eq(|| ContentState {
            content: HashSet::default(),
        });

        let render_count = use_mut_ref(|| 0);

        let render_count = {
            let mut render_count = render_count.borrow_mut();
            *render_count += 1;

            *render_count
        };

        let add_content_a = {
            let content = content.clone();
            Callback::from(move |_| content.dispatch("A".to_string()))
        };

        let add_content_b = Callback::from(move |_| content.dispatch("B".to_string()));

        html! {
            <>
                <div>
                    {"This component has been rendered: "}<span id="result">{render_count}</span>{" Time(s)."}
                </div>
                <button onclick={add_content_a} id="add-a">{"Add A to Content"}</button>
                <button onclick={add_content_b} id="add-b">{"Add B to Content"}</button>
            </>
        }
    }

    yew::Renderer::<UseReducerComponent>::with_root(
        document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "1");

    document()
        .get_element_by_id("add-a")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "2");

    document()
        .get_element_by_id("add-a")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "2");

    document()
        .get_element_by_id("add-b")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "3");

    document()
        .get_element_by_id("add-b")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "3");
}

pub fn obtain_result() -> String {
    gloo_utils::document()
        .get_element_by_id("result")
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html()
}

#![cfg(target_arch = "wasm32")]

use std::{rc::Rc, time::Duration};
use yew::platform::time::sleep;

// use popup::hooks::use_lists_context::ListsState;
use wasm_bindgen_test::*;
use yew::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

//wasm-pack test --firefox --headless -- --test yew_tests

// //[参考](https://github.com/yewstack/yew/blob/de613fa832139f96ebe030bfc93bb6982bd14b67/packages/yew/tests/use_context.rs#L15)
// #[wasm_bindgen_test]
// async fn use_lists_context_works() {
//     #[function_component]
//     fn UseContextComponent() -> Html {
//         type ListsContextProvider = ContextProvider<String>;
//         html! {
//             <div>
//                 <ListsContextProvider context={"-test state-".to_string()}>
//                     <div>{"ignored"}</div>
//                     <UseContextComponentInner />
//                 </ListsContextProvider>
//             </div>
//         }
//     }

//     #[function_component]
//     fn UseContextComponentInner() -> Html {
//         let context = use_context::<String>();
//         html! {
//             <div id="result">{ &context.unwrap() }</div>
//         }
//     }

//     yew::Renderer::<UseContextComponent>::with_root(
//         gloo_utils::document().get_element_by_id("output").unwrap(),
//     )
//     .render();

//     sleep(Duration::ZERO).await;

//     let result: String = obtain_result_by_id("result");
//     assert_eq!("-test state-", result);
// }

// fn obtain_result() -> String {
//     gloo_utils::document()
//         .get_element_by_id("result")
//         .expect("No result found. Most likely, the application crashed and burned")
//         .inner_html()
// }

fn obtain_result_by_id(id: &str) -> String {
    gloo_utils::document()
        .get_element_by_id(id)
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html()
}

// fn output_element() -> web_sys::Element {
//     gloo_utils::document().get_element_by_id("output").unwrap()
// }

#[wasm_bindgen_test]
async fn use_context_update_works() {
    #[derive(Clone, Debug, PartialEq)]
    struct MyContext(String);

    #[derive(Clone, Debug, PartialEq, Properties)]
    struct RenderCounterProps {
        id: String,
        children: Children,
    }

    /// レンダリング回数の表示。
    #[function_component]
    fn RenderCounter(props: &RenderCounterProps) -> Html {
        let counter = use_mut_ref(|| 0);
        *counter.borrow_mut() += 1;
        html! {
            <>
                <div id={props.id.clone()}>
                    { format!("total: {}", counter.borrow()) }
                </div>
                { props.children.clone() }
            </>
        }
    }

    #[derive(Clone, Debug, PartialEq, Properties)]
    struct ContextOutletProps {
        id: String,
        #[prop_or_default]
        magic: usize,
    }

    /// Contextの実数をチェックするための表示。
    /// 作られる度に+1される？Contextじゃないからされずに1のまま？
    #[function_component]
    fn ContextOutlet(props: &ContextOutletProps) -> Html {
        let counter = use_mut_ref(|| 0);
        *counter.borrow_mut() += 1;

        // ここでuse_contextを実行して使う(use)側としてContextを取得している。
        let ctx = use_context::<Rc<MyContext>>().expect("context not passed down");

        html! {
            <>
                <div>{ format!("magic: {}\n", props.magic) }</div>
                <div id={props.id.clone()}>
                    { format!("current: {}, total: {}", ctx.0, counter.borrow()) }
                </div>
            </>
        }
    }

    ///
    #[function_component]
    fn TestComponent() -> Html {
        type MyContextProvider = ContextProvider<Rc<MyContext>>;

        // ここでcontextの実数を定義している。
        let ctx = use_state(|| MyContext("hello".into()));
        let rendered = use_mut_ref(|| 0);

        // this is used to force an update specific to test-2
        let magic_rc = use_state(|| 0);
        let magic: usize = *magic_rc;
        {
            let ctx = ctx.clone();
            use_effect(move || {
                let count = *rendered.borrow();
                match count {
                    0 => {
                        ctx.set(MyContext("world".into()));
                        *rendered.borrow_mut() += 1;
                    }
                    1 => {
                        // force test-2 to re-render.
                        magic_rc.set(1);
                        *rendered.borrow_mut() += 1;
                    }
                    2 => {
                        ctx.set(MyContext("hello world!".into()));
                        *rendered.borrow_mut() += 1;
                    }
                    _ => (),
                };
                || {}
            });
        }
        html! {
            <MyContextProvider context={Rc::new((*ctx).clone())}>
                <RenderCounter id="test-0">
                    <ContextOutlet id="test-1"/>
                    <ContextOutlet id="test-2" {magic}/>
                </RenderCounter>
            </MyContextProvider>
        }
    }

    yew::Renderer::<TestComponent>::with_root(
        gloo_utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    sleep(Duration::ZERO).await;

    // 1 initial render + 3 update steps
    assert_eq!(obtain_result_by_id("test-0"), "total: 4");

    // 1 initial + 2 context update
    assert_eq!(
        obtain_result_by_id("test-1"),
        "current: hello world!, total: 3"
    );

    // 1 initial + 1 context update + 1 magic update + 1 context update
    assert_eq!(
        obtain_result_by_id("test-2"),
        "current: hello world!, total: 4"
    );
}

#![cfg(target_arch = "wasm32")]

use std::time::Duration;
use yew::platform::time::sleep;

use domain::{BookmarkId, List, Lists};
// use popup::hooks::use_lists_context::ListsState;
use popup::hooks::use_lists_context::{ListsContext, ListsState};
use wasm_bindgen_test::*;
use yew::prelude::*;

//wasm-pack test --firefox --headless -- --test use_lists_context

// fn obtain_result() -> String {
//     gloo_utils::document()
//         .get_element_by_id("result")
//         // .expect("No result found. Most likely, the application crashed and burned")
//         .expect("no result found")
//         .inner_html()
// }

fn obtain_result_by_id(id: &str) -> String {
    gloo_utils::document()
        .get_element_by_id(id)
        // .expect("No result found. Most likely, the application crashed and burned")
        .expect("no result found")
        .inner_html()
}

// fn output_element() -> web_sys::Element {
//     gloo_utils::document().get_element_by_id("output").unwrap()
// }

#[wasm_bindgen_test]
async fn use_lists_context_update_works() {
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone, Debug, PartialEq, Properties)]
    struct ContextOutletProps {
        // id: String,
        id: BookmarkId,
    }

    #[function_component]
    fn ContextOutlet(props: &ContextOutletProps) -> Html {
        let ctx = use_context::<ListsContext>().unwrap();
        let lists = ctx.inner();
        let list = lists.list_by_id(&props.id);
        let inner_html = match list {
            Some(list) => list.title.to_owned(),
            None => "not found list".into(),
        };
        html! {
            <>
                <div id={props.id.clone().to_string()}>
                    // {for lists.into_iter().map(|list| {
                    //     html! {
                    //         <div>{&list.title}</div>
                    //     }
                    // })}
                    <div>{inner_html}</div>
                </div>
            </>
        }
    }

    #[function_component]
    fn TestComponent() -> Html {
        let bookmark_id_1 = 1;
        let bookmark_id_2 = 2;
        let bookmark_id_3 = 3;
        let list_1 = List::builder()
            .set_bookmark_id(bookmark_id_1)
            .set_title("list1".to_owned())
            .build();
        let list_2 = List::builder()
            .set_bookmark_id(bookmark_id_2)
            .set_title("list2".to_owned())
            .build();
        let list_3 = List::builder()
            .set_bookmark_id(bookmark_id_3)
            .set_title("list3".to_owned())
            .build();
        let list_vec = vec![list_1, list_2, list_3];
        let lists = Lists::from(list_vec);
        let ctx = use_reducer(|| ListsState::from(lists));
        html! {
            <ContextProvider<ListsContext> context={ctx}>
                <ContextOutlet id={Into::<BookmarkId>::into(bookmark_id_1)}/>
                // <ContextOutlet id={<u64 as Into<BookmarkId>>::into(bookmark_id_1)}/>
                <ContextOutlet id={Into::<BookmarkId>::into(bookmark_id_2)}/>
                <ContextOutlet id={Into::<BookmarkId>::into(bookmark_id_3)}/>
            </ContextProvider<ListsContext>>
        }
    }

    yew::Renderer::<TestComponent>::with_root(
        gloo_utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    sleep(Duration::ZERO).await;

    assert_eq!(obtain_result_by_id("1"), "<div>list1</div>");
    assert_eq!(obtain_result_by_id("2"), "<div>list2</div>");
    assert_eq!(obtain_result_by_id("3"), "<div>list3</div>");
}

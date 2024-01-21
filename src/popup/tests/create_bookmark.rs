#![cfg(target_arch = "wasm32")]

use domain::{BookmarkId, Scene, TagIds, Title};
use gloo_utils::document;
use popup::components::input_fields::{
    FOLDER_ELEM_ID, NAME_ELEM_ID, SUBMIT_BUTTON_ID, TAGS_ELEM_ID,
};

use popup::components::{
    lists_context_provider::ListsContextProvider, scene_context_provider::SceneContextProvider,
    scene_renderer::SceneRenderer,
};
use std::time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::platform::time::sleep;
use yew::prelude::*;

// fn obtain_result_by_id(id: &str) -> String {
//     document()
//         .get_element_by_id(id)
//         // .expect("No result found. Most likely, the application crashed and burned")
//         .expect("no result found")
//         .inner_html()
// }

const OUTPUT_ID: &'static str = "scene-output";

//wasm-pack test --firefox --headless -- --test create_bookmark
#[wasm_bindgen_test]
async fn create_bookmark() {
    wasm_bindgen_test_configure!(run_in_browser);

    #[function_component]
    fn TestComponent() -> Html {
        html! {
            <SceneContextProvider>
                <ListsContextProvider>
                    <div id={OUTPUT_ID}>
                        <SceneRenderer />
                    </div>
                </ListsContextProvider>
            </SceneContextProvider>
        }
    }

    yew::Renderer::<TestComponent>::with_root(document().get_element_by_id("output").unwrap())
        .render();
    sleep(Duration::ZERO).await;

    // assert_eq!(
    //     document().get_element_by_id("output").unwrap().inner_html(),
    //     "<div>fail pattern</div>"
    // );

    document()
        .get_element_by_id(NAME_ELEM_ID)
        .unwrap()
        // .dyn_into::<HtmlInputElement>()
        // .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .set_value("testname");
    sleep(Duration::ZERO).await;

    document()
        .get_element_by_id(FOLDER_ELEM_ID)
        .unwrap()
        // .dyn_into::<HtmlInputElement>()
        // .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .set_value("testfolder");
    sleep(Duration::ZERO).await;

    document()
        .get_element_by_id(TAGS_ELEM_ID)
        .unwrap()
        // .dyn_into::<HtmlInputElement>()
        // .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .set_value("testtag");
    sleep(Duration::ZERO).await;

    let button = document().get_element_by_id(SUBMIT_BUTTON_ID).unwrap();
    // assert_eq!(button.inner_html(), "<div>button</div>");

    //FIXME: なぜかclickされない。もしくは、clickはされているけど、その後の処理に失敗している？
    // button.dyn_into::<HtmlElement>().unwrap().click();
    button.unchecked_into::<HtmlElement>().click();
    sleep(Duration::ZERO).await;
    // sleep(Duration::from_secs(10)).await;

    // HTML出力表示用。わざとfailさせて、エラーメッセージでHTML出力を確認する。
    assert_eq!(document_element().inner_html(), "<div>fail pattern</div>");
    // assert_eq!(obtain_result_by_id(OUTPUT_ID), "<div>fail pattern</div>");
    // assert_eq!(document_element().inner_html(), "<div>fail pattern</div>");

    // Createdページへ遷移するかどうか。
    use gloo_utils::document_element;
    use popup::components::scene_renderer::HTML_ATTRIBUTE_NAME_SCENE;
    let scene_attr = document_element().get_attribute(HTML_ATTRIBUTE_NAME_SCENE);
    assert!(scene_attr.is_some());
    let scene_created = Scene::Created {
        folder_id: BookmarkId::default(),
        folder_title: Title::new("TestFolder"),
        tag_ids: TagIds::default(),
    };
    assert_eq!(scene_created.as_ref(), scene_attr.unwrap());

    // folder, tagの表示titleが正しいかどうか。
    let titles = document().get_elements_by_class_name("stored-list-title");

    let folder_title = titles.item(0);
    assert!(folder_title.is_some());
    assert_eq!(folder_title.unwrap().inner_html(), "testfolder");

    let tag_title = titles.item(1);
    assert!(tag_title.is_some());
    assert_eq!(tag_title.unwrap().inner_html(), "testtag");
}

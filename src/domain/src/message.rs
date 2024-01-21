use crate::{BookmarkId, Lists, PageUrl, TagIds, Title, Url};
use serde::{Deserialize, Serialize};

///popup宛てのmessage。
///content => popup
///background => popup
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageToPopup {
    Page {
        name: Title,
        url: PageUrl,
    },
    CreatedLists {
        folder_id: BookmarkId,
        folder_title: Title,
        tag_ids: TagIds,
        lists: Lists,
    },
    Lists(Lists),
    Error(String),
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod wasm_serde_lists_test {
    use super::*;
    use crate::{List, TypeCode};

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    //wasm-pack test --firefox --headless --lib -- wasm_serde_lists_test::should_serialize_lists
    //wasm-pack test --chrome --lib -- wasm_serde_lists_test::should_serialize_lists
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn should_serialize_lists() {
        let list_1 = List::builder()
            .set_bookmark_id(1)
            .set_title("list1".to_owned())
            .set_type_code(TypeCode::Link.into())
            .build();
        let list_2 = List::builder()
            .set_bookmark_id(2)
            .set_title("list2".to_owned())
            .set_type_code(TypeCode::Folder.into())
            .build();
        let list_3 = List::builder()
            .set_bookmark_id(3)
            .set_title("list3".to_owned())
            .set_type_code(TypeCode::Tag.into())
            .build();
        let list_4 = List::builder()
            .set_bookmark_id(4)
            .set_title("list4".to_owned())
            .set_type_code(TypeCode::Tag.into())
            .build();
        let list_vec = vec![list_1, list_2, list_3, list_4];
        let lists = Lists::from(list_vec);
        println!("{:#?}", &lists);
        // assert!(serde_wasm_bindgen::to_value(&lists).is_ok());
        // let message = serde_wasm_bindgen::to_value(&MessageToPopup::Lists(lists)).unwrap();
        // let lists_2: MessageToPopup::Lists(Lists) =
        //     serde_wasm_bindgen::from_value(message).unwrap();
        // assert_eq!(4, lists_2.len());
    }
}

///background宛てのmessage。
///popup => background
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageToBackground {
    RequestLists,
    OpenTab { url: Url },
}

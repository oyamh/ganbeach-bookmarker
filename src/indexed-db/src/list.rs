use domain::List;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::JsValue;

pub fn js_to_list(js_value: JsValue) -> Result<List, serde_wasm_bindgen::Error> {
    serde_wasm_bindgen::from_value(js_value)
}

pub fn list_to_js(list: &List) -> Result<JsValue, serde_wasm_bindgen::Error> {
    // serde_wasm_bindgen::to_value(&list).expect("convert list to js")
    // NOTE: u64はそのままではJavascriptで処理できないので、serialize_large_number_types_as_bigints(true)はu64をBigIntに変換させるオプションを使う。
    let serializer = Serializer::new().serialize_large_number_types_as_bigints(true);
    list.serialize(&serializer)
}

// エディタ上で薄く見えているのは、VSCodeでのテストができないことを表しているだけ。(WASMだから？)
// 適切なコマンドを使えば正常にテストできる。
#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod serde_test {
    use super::*;

    use crate::list::List;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    //wasm-pack test --firefox --headless --lib -- serde_test::should_serde_wasm_value --nocapture <= --nocaptureはsupportされていないらしい。
    //wasm-pack test --firefox --headless --lib -- serde_test::should_serde_wasm_value
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn should_serde_wasm_value() {
        let list = List::builder()
            .set_now_updated()
            .set_bookmark_id(623677211081183232)
            .build();
        let js_value = list_to_js(&list);
        assert!(js_value.is_ok());
        let de_value = js_to_list(js_value.expect("unwrap js_value"));
        assert!(de_value.is_ok());
    }
}

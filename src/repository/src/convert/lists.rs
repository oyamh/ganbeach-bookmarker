use domain::Lists;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub struct ListsConverter(Lists);

impl From<JsValue> for ListsConverter {
    fn from(value: JsValue) -> Self {
        // serde_wasm_bindgen::from_value::<ListsConverter>(value).unwrap()
        Self(serde_wasm_bindgen::from_value::<Lists>(value).unwrap())
    }
}

impl From<ListsConverter> for Lists {
    fn from(converter: ListsConverter) -> Self {
        converter.0
    }
}

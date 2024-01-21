use domain::SenderInfo;
use wasm_bindgen::JsValue;

pub struct SenderInfoConverter(SenderInfo);

impl From<JsValue> for SenderInfoConverter {
    fn from(js_value: JsValue) -> Self {
        SenderInfoConverter(serde_wasm_bindgen::from_value::<SenderInfo>(js_value).unwrap())
    }
}

impl From<SenderInfoConverter> for SenderInfo {
    fn from(wrapper: SenderInfoConverter) -> Self {
        wrapper.0
    }
}

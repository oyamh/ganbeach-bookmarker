use std::hash::{Hash, Hasher};

use wasm_bindgen::{JsCast, JsValue};

pub struct IdbKeyPath {
    inner: JsValue,
}

impl IdbKeyPath {
    pub fn str(key_path: &str) -> Self {
        Self::new(key_path.into())
    }

    pub fn new(inner: JsValue) -> Self {
        Self { inner }
    }

    pub fn as_js_value(&self) -> &JsValue {
        &self.inner
    }
}

impl AsRef<JsValue> for IdbKeyPath {
    fn as_ref(&self) -> &JsValue {
        self.as_js_value()
    }
}

impl From<IdbKeyPath> for JsValue {
    fn from(path: IdbKeyPath) -> Self {
        path.inner
    }
}

impl From<&str> for IdbKeyPath {
    fn from(key_path: &str) -> Self {
        Self::str(key_path)
    }
}

impl Hash for IdbKeyPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(v) = self.as_js_value().as_string() {
            state.write_u8(1);
            v.hash(state);
        } else {
            state.write_u8(1);
            hash_array(self.as_js_value().unchecked_ref(), state);
        }
    }
}

fn hash_array<H: Hasher>(arr: &js_sys::Array, h: &mut H) {
    let len = arr.length() as u32;

    h.write_u32(len);

    for i in 0..len {
        if let Some(v) = arr.get(i).as_string() {
            h.write_u8(1);
            v.hash(h);
        } else {
            h.write(&[0, 0]);
        }
    }
}

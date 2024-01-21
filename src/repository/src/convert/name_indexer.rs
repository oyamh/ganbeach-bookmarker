use indexed_db::DatabaseError;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// user_dataにIndexedDBインデックスnameを付与するためだけの構造体。
#[derive(Debug, Serialize, Deserialize)]
pub struct NameIndexer<T> {
    name: String,
    value: T,
}

impl<T> NameIndexer<T>
where
    T: Serialize,
{
    pub fn new(name: impl AsRef<str>, value: T) -> Self {
        Self {
            name: name.as_ref().to_string(),
            value,
        }
    }

    pub fn to_value(self) -> T {
        self.value
    }
}

impl<T> TryFrom<JsValue> for NameIndexer<T>
where
    T: for<'a> Deserialize<'a>,
{
    type Error = DatabaseError;
    fn try_from(js_value: JsValue) -> Result<Self, Self::Error> {
        log::debug!("from {js_value:?}");
        if js_value.is_null() {
            return Err(DatabaseError::EmptyValue("null"));
        }
        if js_value.is_undefined() {
            return Err(DatabaseError::EmptyValue("undefined"));
        }
        serde_wasm_bindgen::from_value::<NameIndexer<T>>(js_value)
            .map_err(Into::<DatabaseError>::into)
    }
}

// impl<T> From<JsValue> for UserData<T>
// where
//     T: for<'a> Deserialize<'a>,
// {
//     fn from(js_value: JsValue) -> Self {
//         log::debug!("from {js_value:?}");
//         if js_value.is_null() || js_value.is_undefined() {}
//         serde_wasm_bindgen::from_value::<UserData<T>>(js_value).expect("deserialize UserData")
//     }
// }

impl<T> From<NameIndexer<T>> for JsValue
where
    T: Serialize,
{
    fn from(value: NameIndexer<T>) -> Self {
        serde_wasm_bindgen::to_value(&value).expect("serialize UserData")
    }
}

// impl<T> UserData<T>
// where
//     T: Serialize,
// {
//     pub fn to_js(name: impl AsRef<str>, value: T) -> Result<JsValue, serde_wasm_bindgen::Error> {
//         let user_data = Self {
//             name: name.as_ref().to_string(),
//             value,
//         };
//         serde_wasm_bindgen::to_value(&user_data)
//     }
// }

// impl<T> UserData<T>
// where
//     T: DeserializeOwned,
// {
//     pub fn from_js(js_value: JsValue) -> Result<T, serde_wasm_bindgen::Error> {
//         let user_data = serde_wasm_bindgen::from_value::<UserData<T>>(js_value)?;
//         Ok(user_data.value)
//     }
// }

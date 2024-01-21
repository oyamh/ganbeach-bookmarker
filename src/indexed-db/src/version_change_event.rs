use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::IdbOpenDbRequest;

use super::database::IdbDatabase;

pub struct IdbVersionChangeEvent {
    _event: web_sys::IdbVersionChangeEvent,
    database: IdbDatabase,
}

pub type IdbVersionChangeCallback =
    Closure<dyn FnMut(web_sys::IdbVersionChangeEvent) -> Result<(), JsValue> + 'static>;

impl IdbVersionChangeEvent {
    fn new(e: web_sys::IdbVersionChangeEvent) -> Self {
        let request = e
            .target()
            .expect("failed to unwrap IdbOpenDbRequest event target")
            .unchecked_into::<IdbOpenDbRequest>();
        let database = request
            .result()
            .expect("failed to unwrap IdbOpenDbRequest result")
            .unchecked_into::<web_sys::IdbDatabase>();
        Self {
            _event: e,
            database: IdbDatabase::new(database),
        }
    }

    pub fn wrap_callback<F>(cb: F) -> IdbVersionChangeCallback
    where
        F: Fn(&Self) -> Result<(), JsValue> + 'static,
    {
        let f_box = Box::new(move |e: web_sys::IdbVersionChangeEvent| cb(&Self::new(e)));
        Closure::wrap(f_box)
    }

    pub fn db(&self) -> &IdbDatabase {
        &self.database
    }
}

impl AsRef<IdbDatabase> for IdbVersionChangeEvent {
    fn as_ref(&self) -> &IdbDatabase {
        &self.db()
    }
}

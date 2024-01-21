use domain::ToolbarListener;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_interface::on_clicked_toolbar;

#[derive(Debug, Default)]
pub struct PopupToolbar;

impl ToolbarListener for PopupToolbar {
    fn on_toolbar<V, C>(&self, callback: C)
    where
        V: for<'de> serde::Deserialize<'de>,
        C: Fn(V) + 'static,
    {
        let closure = Closure::new(move |js_value| {
            if let Ok(value) = serde_wasm_bindgen::from_value::<V>(js_value) {
                callback(value);
            }
        }) as Closure<dyn Fn(JsValue)>;
        on_clicked_toolbar(closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

impl PopupToolbar {
    pub fn new() -> Self {
        Self::default()
    }
}

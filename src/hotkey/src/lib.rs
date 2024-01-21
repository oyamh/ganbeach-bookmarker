use domain::HotkeysListener;
use wasm_bindgen::{prelude::*, JsCast};
use web_interface::on_command;

#[derive(Debug, Default)]
pub struct Listener;

impl HotkeysListener for Listener {
    fn on_hotkeys<T>(&self, callback: T)
    where
        T: Fn(String) + 'static,
    {
        log::debug!("hotkey::Listener.on_hotkeys");
        let closure = Closure::new(Box::new(move |command| {
            callback(command);
        })) as Closure<dyn Fn(String)>;
        on_command(closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

use crate::app::App;
use wasm_bindgen::prelude::*;

mod app;
pub mod components;
mod handler;
pub mod hooks;
mod icons;

#[wasm_bindgen(start)]
pub async fn main_start() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::debug!("Hello World from Popup Script");

    yew::Renderer::<App>::new().render();
}

#![allow(refining_impl_trait)]
use domain::HotkeysListenerProvider;
use hotkey::Listener;

#[derive(Debug, Default)]
pub struct ListenHotkeysContext {
    listener: Listener,
}

impl ListenHotkeysContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HotkeysListenerProvider for ListenHotkeysContext {
    fn provide(&self) -> &Listener {
        &self.listener
    }
}

#![allow(refining_impl_trait)]
use domain::ToolbarListenerProvider;
use popup_toolbar::PopupToolbar;

#[derive(Debug, Default)]
pub struct ToolbarContext {
    toolbar: PopupToolbar,
}

impl ToolbarContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ToolbarListenerProvider for ToolbarContext {
    fn provide(&self) -> &PopupToolbar {
        &self.toolbar
    }
}

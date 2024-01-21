#![allow(refining_impl_trait)]
use domain::PopupOpenerProvider;
use web_interface::WindowsController;

#[derive(Debug, Default)]
pub struct OpenWindowContext {
    opener: WindowsController,
}

impl OpenWindowContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PopupOpenerProvider for OpenWindowContext {
    fn provide(&self) -> &WindowsController {
        &self.opener
    }
}

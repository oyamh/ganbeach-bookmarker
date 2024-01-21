#![allow(refining_impl_trait)]
use domain::LinkOpenerProvider;
use web_interface::TabController;

#[derive(Default)]
pub struct OpenTabContext {
    link_opener: TabController,
}

impl OpenTabContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LinkOpenerProvider for OpenTabContext {
    fn provide(&self) -> &TabController {
        &self.link_opener
    }
}

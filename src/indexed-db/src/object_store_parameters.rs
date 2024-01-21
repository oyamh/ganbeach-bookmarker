use super::key_path::IdbKeyPath;

pub struct IdbObjectStoreParameters {
    inner: web_sys::IdbObjectStoreParameters,
}

impl IdbObjectStoreParameters {
    pub fn new() -> Self {
        Self {
            inner: web_sys::IdbObjectStoreParameters::new(),
        }
    }

    pub fn key_path(&mut self, val: Option<&IdbKeyPath>) -> &mut Self {
        self.inner.key_path(val.map(|v| v.as_js_value()));
        self
    }

    pub fn auto_increment(&mut self, val: bool) -> &mut Self {
        self.inner.auto_increment(val);
        self
    }

    pub fn as_js_value(&self) -> &web_sys::IdbObjectStoreParameters {
        self.inner.as_ref()
    }
}

impl AsRef<web_sys::IdbObjectStoreParameters> for IdbObjectStoreParameters {
    fn as_ref(&self) -> &web_sys::IdbObjectStoreParameters {
        self.as_js_value()
    }
}

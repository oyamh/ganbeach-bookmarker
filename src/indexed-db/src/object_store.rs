#![allow(dead_code)]

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{DomException, IdbIndexParameters};

use super::request::IdbRequest;

#[derive(Debug)]
pub struct IdbObjectStore {
    inner: web_sys::IdbObjectStore,
}

impl IdbObjectStore {
    pub fn create_index_with_str_and_optional_parameters(
        &self,
        name: &str,
        key_path: &str,
        params: &IdbIndexParameters,
    ) -> Result<(), DomException> {
        self.inner
            .create_index_with_str_and_optional_parameters(name, key_path, params)?;
        Ok(())
    }
}

impl IdbObjectStore {
    pub fn from_db(inner: web_sys::IdbObjectStore) -> Self {
        Self { inner }
    }

    pub fn from_tx(inner: web_sys::IdbObjectStore) -> Self {
        Self { inner }
    }

    pub fn get<K>(&self, key: K) -> Result<IdbRequest, DomException>
    where
        K: Into<JsValue>,
    {
        let req = self.inner.get(&key.into().unchecked_ref())?;
        Ok(IdbRequest::new(req))
    }

    pub fn get_all(&self) -> Result<IdbRequest, DomException> {
        let req = self.inner.get_all()?;
        Ok(IdbRequest::new(req))
    }

    pub fn put<V>(&self, value: &V) -> Result<IdbRequest, DomException>
    where
        V: JsCast,
    {
        let v = value.unchecked_ref();
        let req = self.inner.put(v)?;
        Ok(IdbRequest::new(req))
    }

    pub fn put_with_key<K, V>(&self, key: K, value: &V) -> Result<IdbRequest, DomException>
    where
        K: Into<JsValue>,
        V: JsCast,
    {
        let req = self
            .inner
            .put_with_key(value.unchecked_ref(), &key.into().unchecked_ref())?;
        Ok(IdbRequest::new(req))
    }

    pub fn delete<K>(&self, key: K) -> Result<IdbRequest, DomException>
    where
        K: Into<JsValue>,
    {
        let req = self.inner.delete(&key.into().unchecked_ref())?;
        Ok(IdbRequest::new(req))
    }

    pub fn clear(&self) -> Result<IdbRequest, DomException> {
        let req = self.inner.clear()?;
        Ok(IdbRequest::new(req))
    }
}

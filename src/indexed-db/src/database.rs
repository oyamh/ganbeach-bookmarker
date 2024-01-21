use wasm_bindgen::JsCast;
use web_sys::WorkerGlobalScope;
use web_sys::{window, DomException, IdbFactory, IdbTransactionMode};

use super::{
    dom_string_iterator::DomStringIterator, object_store::IdbObjectStore,
    object_store_parameters::IdbObjectStoreParameters, open_db_request::IdbOpenDbRequest,
    transaction::IdbTransaction,
};

/// openでIdbOpenDbRequestを生成する。
/// 実体を生成するにはfutureを使わなければならない。
#[derive(Debug)]
pub struct IdbDatabase {
    inner: web_sys::IdbDatabase,
}

impl IdbDatabase {
    pub fn new(inner: web_sys::IdbDatabase) -> Self {
        Self { inner }
    }

    pub fn open_with_u32(name: &str, version: u32) -> Result<IdbOpenDbRequest, DomException> {
        Ok(IdbOpenDbRequest::new(
            factory()?.open_with_u32(name, version)?, //.map_err(|err| Error::DatabaseOpenFailed(err))?,
        ))
    }

    pub fn object_store_names(&self) -> impl Iterator<Item = String> + 'static {
        DomStringIterator::from(self.inner.object_store_names())
    }

    pub fn create_object_store(&self, name: &str) -> Result<IdbObjectStore, DomException> {
        let object_store = self.inner.create_object_store(name)?;
        Ok(IdbObjectStore::from_db(object_store))
    }

    pub fn create_object_store_with_optional_parameters(
        &self,
        name: &str,
        params: &IdbObjectStoreParameters,
    ) -> Result<IdbObjectStore, DomException> {
        let object_store = self
            .inner
            .create_object_store_with_optional_parameters(name, params.as_js_value())?;
        Ok(IdbObjectStore::from_db(object_store))
    }

    pub fn tx_with_name_and_mode(
        &self,
        name: &str,
        mode: IdbTransactionMode,
    ) -> Result<IdbTransaction, DomException> {
        let inner_tx = self.inner.transaction_with_str_and_mode(name, mode)?;
        Ok(IdbTransaction::new(inner_tx))
    }
}

fn factory() -> Result<IdbFactory, DomException> {
    let factory = if let Some(window) = window() {
        // firefox
        window.indexed_db()
    } else {
        // chrome
        let worker_global_scope = js_sys::global().dyn_into::<WorkerGlobalScope>().ok();
        worker_global_scope
            .expect("get worker_global_scope object")
            .indexed_db()
    };

    factory
        .and_then(|result| Ok(result.unwrap()))
        .map_err(|error| {
            DomException::new_with_message(&format!("failed to create factory: {error:?}")).unwrap()
        })
}

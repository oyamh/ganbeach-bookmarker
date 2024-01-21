use crate::{DatabaseError, IdbDatabase, IdbObjectStore, IdbTransaction};
use web_sys::{DomException, IdbTransactionMode};

#[derive(Debug, Default)]
pub struct Accessor {}

impl Accessor {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn open(db_name: &'static str, version: u32) -> Result<StoreAccessor, DatabaseError> {
        let db = IdbDatabase::open_with_u32(db_name, version)?.await?;
        Ok(StoreAccessor { db_name, db })
    }
}

pub struct StoreAccessor {
    db_name: &'static str,
    db: IdbDatabase,
}

impl StoreAccessor {
    fn tx_and_store(
        &self,
        mode: IdbTransactionMode,
    ) -> Result<(IdbTransaction, IdbObjectStore), DomException> {
        let tx = self.db.tx_with_name_and_mode(self.db_name, mode)?;
        let store = tx.object_store(self.db_name)?;
        Ok((tx, store))
    }

    pub fn as_reader(&self) -> Result<(IdbTransaction, IdbObjectStore), DomException> {
        self.tx_and_store(IdbTransactionMode::Readonly)
    }

    pub fn as_writer(&self) -> Result<(IdbTransaction, IdbObjectStore), DomException> {
        self.tx_and_store(IdbTransactionMode::Readwrite)
    }
}

// use futures::{Future, FutureExt};
// use wasm_bindgen::JsValue;
// pub struct Access {
//     tx: IdbTransaction,
//     store: IdbObjectStore,
//     mode: IdbTransactionMode,
// }

// impl Access {
//     async fn put(&self, value: impl Into<JsValue>) -> Result<(), DatabaseError> {
//         self.store.put(&value.into())?;
//         Ok(())
//     }
// }

// impl Future for Access {
//     type Output = Result<(), DomException>;
//     fn poll(
//         mut self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         self.tx.poll_unpin(cx)
//     }
// }

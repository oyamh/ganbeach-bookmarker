#![allow(dead_code)]

use super::{
    dom_string_iterator::DomStringIterator, object_store::IdbObjectStore,
    rewritable_result::RewritableResult, rewritable_waker::RewritableWaker,
};
use futures::Future;
use std::{rc::Rc, task::Poll};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{DomException, Event};

type SharedIdbTransaction = Rc<web_sys::IdbTransaction>;

#[derive(Debug)]
pub enum TxResult {
    Complete,
    Error(DomException),
    Abort,
}

impl Into<Result<(), DomException>> for TxResult {
    fn into(self) -> Result<(), DomException> {
        use TxResult::*;
        match self {
            Complete => Ok(()),
            Error(dom_exception) => Err(dom_exception),
            Abort => Err(DomException::new_with_message("transaction aborted")
                .expect("failed to create dom exception for abort result")),
        }
    }
}

type TxCallback = Closure<dyn FnMut(Event) + 'static>;

#[derive(Debug)]
pub struct IdbTransaction {
    inner: SharedIdbTransaction,
    result: RewritableResult<TxResult>,
    waker: RewritableWaker,
    on_complete: Option<TxCallback>,
    on_error: Option<TxCallback>,
    on_abort: Option<TxCallback>,
}

impl IdbTransaction {
    pub fn object_store_names(&self) -> impl Iterator<Item = String> {
        DomStringIterator::from(self.inner.object_store_names())
    }

    pub fn error(&self) -> Option<DomException> {
        self.inner.error()
    }

    pub fn abort(&self) -> Result<(), DomException> {
        Ok(self.inner.abort()?)
    }
}

impl IdbTransaction {
    pub fn new(inner: web_sys::IdbTransaction) -> Self {
        let inner = Rc::new(inner);
        let result = RewritableResult::default();
        let waker = RewritableWaker::new();

        let on_complete = {
            let tx = inner.clone();
            let result = result.clone();
            let waker = waker.clone();
            complete_callback(tx, result, waker)
        };

        let on_error = {
            let tx = inner.clone();
            let result = result.clone();
            let waker = waker.clone();
            error_callback(tx, result, waker)
        };

        let on_abort = {
            let tx = inner.clone();
            let result = result.clone();
            let waker = waker.clone();
            abort_callback(tx, result, waker)
        };

        Self {
            inner,
            result,
            waker,
            on_complete: Some(on_complete),
            on_error: Some(on_error),
            on_abort: Some(on_abort),
        }
    }

    pub fn object_store(&self, name: &str) -> Result<IdbObjectStore, DomException> {
        let object_store = self.inner.object_store(name)?;
        Ok(IdbObjectStore::from_tx(object_store))
    }

    fn take_result(&self) -> Option<TxResult> {
        self.result.clone().replace(None)
    }

    fn register_waker(&mut self, ctx: &mut std::task::Context<'_>) {
        self.waker.replace(ctx.waker().clone());
    }
}

impl Future for IdbTransaction {
    type Output = Result<(), DomException>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        log::debug!("IdbTransaction::poll");
        if self.result.is_some() {
            log::debug!("result.is_some == true");
            // result.is_some()がtrueなので.unwrapが使える。
            Poll::Ready(self.take_result().unwrap().into())
        } else {
            log::debug!("result.is_some == false");
            // self.waker.replace(ctx.waker().clone());
            self.register_waker(ctx);
            Poll::Pending
        }
    }
}

// #[cfg(test)]
// #[cfg(target_arch = "wasm32")]
// mod tx {
//     async fn open_any_db() -> (crate::IdbDatabase, String) {
//         let db_name = uuid::Uuid::new_v4().to_string();
//         let store_name = uuid::Uuid::new_v4().to_string();
//         let mut request =
//             crate::IdbDatabase::open_with_u32(&db_name, 1).expect("IdbDatabase::open_with_u32");
//         let store_name_cloned = store_name.clone();
//         request.set_on_update_needed(move |e: &crate::IdbVersionChangeEvent| {
//             e.db().create_object_store_with_optional_parameters(
//                 &store_name_cloned,
//                 // crate::IdbObjectStoreParameters::new().key_path(Some(&PRIMARY_KEY.into())),
//                 &crate::IdbObjectStoreParameters::new(),
//             )?;
//             Ok(())
//         });
//         (request.await.expect("request.await"), store_name)
//     }

//     // #[tokio::test]
//     #[wasm_bindgen_test::wasm_bindgen_test]
//     async fn should_await() {
//         let (db, store_name) = open_any_db().await;
//         let tx = db
//             .tx_with_str_and_mode(&store_name, web_sys::IdbTransactionMode::Readwrite)
//             .expect("tx");
//         tx.await.expect("await");
//     }
// }

fn complete_callback(
    tx: SharedIdbTransaction,
    result: RewritableResult<TxResult>,
    waker: RewritableWaker,
) -> TxCallback {
    let complete_callback = {
        let mut result = result.clone();
        let waker = waker.clone();
        waker.as_callback(move |_e: Event| {
            let _ = result.replace(Some(TxResult::Complete));
        })
    };
    tx.set_oncomplete(Some(complete_callback.as_ref().unchecked_ref()));
    complete_callback
}

fn error_callback(
    tx: SharedIdbTransaction,
    result: RewritableResult<TxResult>,
    waker: RewritableWaker,
) -> TxCallback {
    let error_callback = {
        let mut result = result.clone();
        let waker = waker.clone();
        waker.as_callback(move |e: Event| {
            let error = if let Some(target) = e.target() {
                target
                    .unchecked_into::<web_sys::IdbRequest>()
                    .error()
                    .unwrap_or_else(|js_error| {
                        DomException::new_with_message(&format!(
                            "failed to extract transaction error: {js_error:?}"
                        ))
                        .ok()
                    })
                    .unwrap()
            } else {
                DomException::new_with_message("failed to extract target from web_sys::Event")
                    .unwrap()
            };
            let _ = result.replace(Some(TxResult::Error(error)));
        })
    };
    tx.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    error_callback
}

fn abort_callback(
    tx: SharedIdbTransaction,
    result: RewritableResult<TxResult>,
    waker: RewritableWaker,
) -> TxCallback {
    let abbort_callback = {
        let mut result = result.clone();
        let waker = waker.clone();
        waker.as_callback(move |_e: Event| {
            let _ = result.replace(Some(TxResult::Abort));
        })
    };
    tx.set_onabort(Some(abbort_callback.as_ref().unchecked_ref()));
    abbort_callback
}

impl Drop for IdbTransaction {
    fn drop(&mut self) {
        if self.on_complete.is_some() {
            self.inner.set_oncomplete(None);
        }
        if self.on_error.is_some() {
            self.inner.set_onerror(None);
        }
        if self.on_abort.is_some() {
            self.inner.set_onabort(None);
        }
    }
}

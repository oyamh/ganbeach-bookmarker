use std::{
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use futures::Future;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::DomException;

use super::{
    database::IdbDatabase,
    request::IdbRequest,
    version_change_event::{IdbVersionChangeCallback, IdbVersionChangeEvent},
};

/// IdbDatabaseを生成するためのリクエスト。
/// factory().open()で生成される。
/// 内部でweb_sysのIdbOpenRequestをIdbRequestに変換してから保管して運用する。
pub struct IdbOpenDbRequest {
    inner: Rc<IdbRequest>,
    on_upgrade_needed: Option<IdbVersionChangeCallback>,
}

impl IdbOpenDbRequest {
    pub fn new(base: web_sys::IdbOpenDbRequest) -> Self {
        let request = IdbRequest::new(base.unchecked_into::<web_sys::IdbRequest>());
        let inner = Rc::new(request);
        Self {
            inner,
            on_upgrade_needed: None,
        }
    }

    pub fn set_on_update_needed<F>(&mut self, callback: F)
    where
        F: Fn(&IdbVersionChangeEvent) -> Result<(), JsValue> + 'static,
    {
        let req = self.inner.clone();
        self.on_upgrade_needed = {
            let callback = IdbVersionChangeEvent::wrap_callback(callback);
            req.unchecked_ref::<web_sys::IdbOpenDbRequest>()
                .set_onupgradeneeded(Some(callback.as_ref().unchecked_ref()));
            Some(callback)
        }
    }
}

impl Future for IdbOpenDbRequest {
    type Output = Result<IdbDatabase, DomException>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let output = self.inner.pub_poll(ctx);
        match output {
            Poll::Ready(result) => Poll::Ready(Ok(IdbDatabase::new(result?.unchecked_into()))),
            Poll::Pending => Poll::Pending,
        }
    }
}

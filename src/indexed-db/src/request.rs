use futures::Future;
use std::{
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{DomException, Event, IdbRequestReadyState};

use super::{
    rewritable_result::RewritableResult,
    rewritable_waker::{IdbCallback, RewritableWaker},
};

type IdbResult = Result<JsValue, DomException>;
type SharedIdbRequest = Rc<web_sys::IdbRequest>;

//TODO: TryFromを実装する。
impl From<SharedIdbRequest> for RewritableResult<IdbResult> {
    fn from(request: SharedIdbRequest) -> Self {
        if request.ready_state() == IdbRequestReadyState::Done {
            let result = request.result().map_err(|err| {
                // request
                //     .error()
                //     .ok()
                //     .unwrap()
                //     .unwrap_or_else(move || err.unchecked_into())
                request
                    .error()
                    .map_err(move |_js_error| err.unchecked_into::<DomException>())
                    .unwrap()
                    .unwrap()
            });
            Self::new(Some(result))
        } else {
            Self::default()
        }
    }
}

/// IdbRequestのWrapper。Futureトレイトを実装している。
#[derive(Debug)]
pub struct IdbRequest {
    //TODO: innerをGenerics型にしてテストしやすくする。
    inner: SharedIdbRequest,
    result: RewritableResult<IdbResult>,
    //listener: Option<IdbCallbackListener>,
    waker: Option<RewritableWaker>,
    on_success: Option<IdbCallback>, // .forget()を使わずに保管する。
    on_error: Option<IdbCallback>,   // .forget()を使わずに保管する。
}

impl IdbRequest {
    pub fn new(inner: web_sys::IdbRequest) -> Self {
        let inner = Rc::new(inner);

        let result = RewritableResult::from(inner.clone());

        //let listener = if inner.ready_state() == IdbRequestReadyState::Pending {
        //    let inner = inner.clone();
        //    let result = result.clone();
        //    Some(IdbCallbackListener::new(inner, result))
        //} else {
        //    // IdbRequestReadyState::Done
        //    None
        //};

        let (waker, on_success, on_error) = if inner.ready_state() == IdbRequestReadyState::Pending
        {
            let waker = RewritableWaker::new();

            let on_success = success_callback(inner.clone(), result.clone(), waker.clone());

            let on_error = error_callback(inner.clone(), result.clone(), waker.clone());

            (Some(waker), Some(on_success), Some(on_error))
        } else {
            (None, None, None)
        };

        Self {
            inner,
            result,
            //listener,
            waker,
            on_success,
            on_error,
        }
    }

    // fn error(&self) -> Option<DomException> {
    //     self.inner.error().ok()?
    // }

    // pub fn result(&self) -> Result<JsValue, DomException> {
    //     self.inner
    //         .result()
    //         .map_err(|err| self.error().unwrap_or_else(move || err.unchecked_into()))
    // }

    // pub fn ready_state(&self) -> web_sys::IdbRequestReadyState {
    //     self.inner.ready_state()
    // }

    pub fn unchecked_ref<T>(&self) -> &T
    where
        T: JsCast,
    {
        self.inner.unchecked_ref::<T>()
    }

    fn take_result(&self) -> Option<IdbResult> {
        self.result.clone().replace(None)
    }

    fn register_waker(&self, ctx: &Context<'_>) {
        self.waker
            .as_ref()
            .unwrap()
            .clone()
            .replace(ctx.waker().clone());
    }

    pub fn pub_poll(&self, ctx: &mut Context<'_>) -> Poll<IdbResult> {
        if self.result.is_some() {
            // result.is_some()がtrueなので.unwrapが使える。
            Poll::Ready(self.take_result().unwrap())
        } else {
            self.register_waker(ctx);
            Poll::Pending
        }
    }
}

impl Future for IdbRequest {
    type Output = IdbResult;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        self.pub_poll(ctx)
    }
}

fn success_callback(
    request: SharedIdbRequest,
    mut result: RewritableResult<IdbResult>,
    waker: RewritableWaker,
) -> IdbCallback {
    let on_success = {
        let request = request.clone();
        // log::debug!("on_success => waker.as_callback");
        waker.as_callback(move |_e: Event| {
            let new_result = request.result().unwrap();
            let _ = result.replace(Some(Ok(new_result)));
            // log::debug!("END waker.as_callback");
        })
    };
    request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
    on_success
}

fn error_callback(
    request: SharedIdbRequest,
    mut result: RewritableResult<IdbResult>,
    waker: RewritableWaker,
) -> IdbCallback {
    let on_error = {
        let request = request.clone();
        // log::debug!("on_error => waker.as_callback");
        waker.as_callback(move |_e: Event| {
            let error = request.error().unwrap().unwrap();
            let _ = result.replace(Some(Err(error)));
        })
    };
    request.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    on_error
}

impl Drop for IdbRequest {
    fn drop(&mut self) {
        if self.on_success.is_some() {
            self.inner.set_onsuccess(None);
        }
        if self.on_error.is_some() {
            self.inner.set_onerror(None);
        }
    }
}

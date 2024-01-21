use std::{cell::RefCell, ops::Deref, rc::Rc, task::Waker};
use wasm_bindgen::prelude::Closure;
use web_sys::Event;

pub type IdbCallback = Closure<dyn FnMut(Event) + 'static>;

/// callback後のresult取得の準備ができたことをFutureに知らせるwake関数(wake_by_ref)を実行する。
#[derive(Debug)]
pub struct RewritableWaker(Rc<RefCell<Option<Waker>>>);

impl RewritableWaker {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }

    pub fn as_callback<F>(&self, mut f: F) -> IdbCallback
    where
        F: FnMut(Event) + 'static,
    {
        let waker = self.0.clone();
        Closure::wrap(Box::new(move |e: Event| {
            f(e);
            if let Some(w) = waker.borrow_mut().deref() {
                w.wake_by_ref();
            }
        }))
    }

    pub fn replace(&mut self, waker: Waker) {
        self.0.replace(Some(waker));
    }
}

impl Clone for RewritableWaker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

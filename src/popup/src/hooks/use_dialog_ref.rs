use std::{
    cell::RefCell,
    fmt::Display,
    future::Future,
    rc::Rc,
    // sync::{Arc, Mutex},
    task::{Poll, Waker},
};

use gloo_events::EventListener;
use web_sys::HtmlDialogElement;
use yew::prelude::*;

#[hook]
pub fn use_dialog_ref() -> (NodeRef, DialogState) {
    let dialog_ref = use_node_ref();

    let state = DialogState::new(dialog_ref.clone());

    (dialog_ref, state)
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum DialogValue {
    OK,
    #[default]
    Cancel,
}

impl DialogValue {
    pub fn is_ok(&self) -> bool {
        *self == Self::OK
    }
}

impl From<&str> for DialogValue {
    fn from(src: &str) -> Self {
        match src {
            "ok" => Self::OK,
            "cancel" => Self::Cancel,
            _ => Self::Cancel,
        }
    }
}

impl Into<&'static str> for DialogValue {
    fn into(self) -> &'static str {
        match self {
            Self::OK => "ok",
            Self::Cancel => "cancel",
        }
    }
}

impl Display for DialogValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OK => write!(f, "ok"),
            Self::Cancel => write!(f, "cancel"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DialogState {
    future_state: Rc<RefCell<FutureState>>,
    dialog_ref: NodeRef,
    listener: Rc<RefCell<Option<EventListener>>>,
}

#[derive(Debug, Default)]
struct FutureState {
    value: Option<DialogValue>,
    waker: Option<Waker>,
}

impl DialogState {
    pub fn new(dialog_ref: NodeRef) -> Self {
        let future_state = Rc::new(RefCell::new(FutureState {
            value: None,
            waker: None,
        }));

        Self {
            future_state,
            dialog_ref,
            listener: Rc::new(RefCell::new(None)),
        }
    }

    pub fn show_modal(&mut self) {
        if let Some(dialog) = self.dialog_ref.cast::<HtmlDialogElement>() {
            let new_listener = EventListener::new(&dialog, "close", {
                let future_state = self.future_state.clone();
                let dialog_ref = self.dialog_ref.clone();
                move |_e| {
                    let mut future_state = future_state.try_borrow_mut().unwrap();
                    if let Some(dialog) = dialog_ref.cast::<HtmlDialogElement>() {
                        future_state.value = Some(dialog.return_value().as_str().into());
                    }
                    if let Some(waker) = future_state.waker.take() {
                        waker.wake()
                    }
                }
            });
            let mut listener = self.listener.try_borrow_mut().unwrap();
            *listener = Some(new_listener);

            let _result = dialog.show_modal();
        }
    }
}

impl Future for DialogState {
    type Output = DialogValue;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut future_state = self.future_state.try_borrow_mut().unwrap();
        match future_state.value.take() {
            Some(value) => Poll::Ready(value),
            None => {
                future_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

// #[derive(Debug, Default, Clone)]
// pub struct DialogState {
//     future_state: Arc<Mutex<SharedState>>,
// }

// #[derive(Debug, Default)]
// struct SharedState {
//     completed: bool,
//     waker: Option<Waker>,
// }

// impl DialogState {
//     pub fn new() -> Self {
//         let future_state = Arc::new(Mutex::new(SharedState {
//             completed: false,
//             waker: None,
//         }));

//         // let thread_future_state = future_state.clone();
//         // spawn_local(async move {
//         //     sleep(Duration::from_secs(3)).await;
//         //     let mut future_state = thread_future_state.lock().unwrap();
//         //     future_state.completed = true;
//         //     if let Some(waker) = future_state.waker.take() {
//         //         waker.wake()
//         //     }
//         // });

//         Self { future_state }
//     }

//     pub fn complete(&self) {
//         let mut future_state = self.future_state.lock().unwrap();
//         future_state.completed = true;
//         if let Some(waker) = future_state.waker.take() {
//             waker.wake()
//         }
//     }
// }

// impl Future for DialogState {
//     type Output = ();
//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         let mut future_state = self.future_state.lock().unwrap();
//         match future_state.completed {
//             true => Poll::Ready(()),
//             false => {
//                 future_state.waker = Some(cx.waker().clone());
//                 Poll::Pending
//             }
//         }
//     }
// }

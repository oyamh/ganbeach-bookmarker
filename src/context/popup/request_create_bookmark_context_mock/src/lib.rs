#![allow(refining_impl_trait)]
use domain::{
    DomainError, MessageReceiver, MessageReceiverProvider, MessageSender, MessageToBackground,
};
use domain::{MessageSenderProvider, SenderInfo};
use serde::Deserialize;
use serde::Serialize;
use web_sys::Window;

#[derive(Default)]
pub struct RequestCreateBookmarkContext {
    message_sender: SenderMock,
    message_receiver: ReceiverMock,
    // history_repo: RepoMock,
}

impl RequestCreateBookmarkContext {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default)]
pub struct SenderMock;

impl MessageSender for SenderMock {
    fn send_to_parent_frame<T>(&self, _message: T) -> Result<(), DomainError>
    where
        T: Serialize,
    {
        Ok(())
    }
    fn send_to_child_frame<T>(
        &self,
        _message: T,
        _content_window: &Window,
        _target_uri: impl AsRef<str>,
    ) -> Result<(), DomainError>
    where
        T: Serialize,
    {
        Ok(())
    }
    async fn send_to_tab<T>(
        &self,
        _message: T,
        _sender_infoo: SenderInfo,
    ) -> Result<(), DomainError>
    where
        T: Serialize,
    {
        Ok(())
    }
    async fn send_to_background(&self, _message: MessageToBackground) -> Result<(), DomainError> {
        Ok(())
    }
}

#[derive(Default)]
pub struct ReceiverMock;

impl MessageReceiver for ReceiverMock {
    // fn on_window_message<M, T>(&self, callback: T) -> EventListener
    // where
    //     M: for<'de> Deserialize<'de>,
    //     T: Fn(M) + 'static,
    // {
    //     EventListener::new(&window().unwrap(), "message", move |_| {
    //         let message = MessageToPopup::CreatedLists {
    //             folder_id: BookmarkId(1111),
    //             folder_title: Title::from("MockFolder"),
    //             tag_ids: TagIds::from(vec![222, 333]),
    //             lists: Lists::new(vec![List::builder().set_bookmark_id(1).build()]),
    //         };
    //         let m = serde_wasm_bindgen::from_value(serde_wasm_bindgen::to_value(&message).unwrap())
    //             .unwrap();
    //         callback(m);
    //     })
    // }

    fn on_extension_message<M, M2, T>(&self, _callback: T)
    where
        M: for<'de> Deserialize<'de>,
        M2: for<'de> Deserialize<'de>,
        T: Fn(M, M2) + 'static,
    {
        unimplemented!()
    }
}

// #[derive(Default)]
// pub struct RepoMock;

// impl HistoryStorager for RepoMock {
//     async fn save_history(&self, _history: History) -> Result<(), DomainError> {
//         unimplemented!()
//     }
//     async fn load_history(&self, _history_key: HistoryKey) -> Result<History, DomainError> {
//         unimplemented!()
//     }
//     async fn clear_history(&self) -> Result<(), DomainError> {
//         unimplemented!()
//     }
// }

impl MessageSenderProvider for RequestCreateBookmarkContext {
    fn provide(&self) -> &SenderMock {
        &self.message_sender
    }
}

impl MessageReceiverProvider for RequestCreateBookmarkContext {
    fn provide(&self) -> &ReceiverMock {
        &self.message_receiver
    }
}

// impl HistoryStoragerProvider for RequestCreateBookmarkContext {
//     fn provide(&self) -> &RepoMock {
//         &self.history_repo
//     }
// }

#![allow(refining_impl_trait)]
use domain::MessageSenderProvider;
use message::Sender;

/// 拡張機能内部で全ListsデータをPopupに受け渡すためのContext。
pub struct LoadAllListsContext {
    pub message_sender: Sender,
}

impl LoadAllListsContext {
    pub fn new() -> Self {
        Self {
            message_sender: Sender::new(),
        }
    }
}

impl MessageSenderProvider for LoadAllListsContext {
    fn provide(&self) -> &Sender {
        &self.message_sender
    }
}

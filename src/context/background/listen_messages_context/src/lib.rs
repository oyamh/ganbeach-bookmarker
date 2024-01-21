#![allow(refining_impl_trait)]
use domain::MessageReceiverProvider;
use message::Receiver;

#[derive(Default)]
pub struct ListenMessagesContext {
    listener: Receiver,
}

impl ListenMessagesContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MessageReceiverProvider for ListenMessagesContext {
    fn provide(&self) -> &Receiver {
        &self.listener
    }
}

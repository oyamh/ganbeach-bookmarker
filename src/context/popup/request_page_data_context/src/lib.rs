#![allow(refining_impl_trait)]
use domain::{MessageReceiverProvider, MessageSenderProvider};
use message::{Receiver, Sender};

#[derive(Default)]
pub struct RequestPageDataContext {
    message_sender: Sender,
    message_receiver: Receiver,
}

impl RequestPageDataContext {
    pub fn new() -> Self {
        Default::default()
    }
}

impl MessageSenderProvider for RequestPageDataContext {
    fn provide(&self) -> &Sender {
        &self.message_sender
    }
}

impl MessageReceiverProvider for RequestPageDataContext {
    fn provide(&self) -> &Receiver {
        &self.message_receiver
    }
}

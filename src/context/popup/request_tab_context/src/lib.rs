#![allow(refining_impl_trait)]
use domain::MessageSenderProvider;
use message::Sender;

#[derive(Default)]
pub struct RequestTabContext {
    message_sender: Sender,
}

impl RequestTabContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MessageSenderProvider for RequestTabContext {
    fn provide(&self) -> &Sender {
        &self.message_sender
    }
}

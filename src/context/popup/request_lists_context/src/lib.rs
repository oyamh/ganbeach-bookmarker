#![allow(refining_impl_trait)]
use config::{DATABASE_VERSION, LISTS_DATABASE_NAME};
use domain::{
    DatabaseAccessorProvider, DomainError, MessageReceiverProvider, MessageSenderProvider,
};
use message::{Receiver, Sender};
use repository::Repository;

pub struct RequestListsContext {
    message_sender: Sender,
    message_receiver: Receiver,
    lists_repository: Repository,
}

impl RequestListsContext {
    pub fn new() -> Self {
        Self {
            message_sender: Sender::default(),
            message_receiver: Receiver::default(),
            lists_repository: Repository::new(LISTS_DATABASE_NAME, DATABASE_VERSION),
        }
    }
}

impl MessageSenderProvider for RequestListsContext {
    fn provide(&self) -> &Sender {
        &self.message_sender
    }
}

impl MessageReceiverProvider for RequestListsContext {
    fn provide(&self) -> &Receiver {
        &self.message_receiver
    }
}

impl DatabaseAccessorProvider for RequestListsContext {
    type Error = DomainError;
    fn provide(&self) -> &Repository {
        &self.lists_repository
    }
}

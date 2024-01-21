#![allow(refining_impl_trait)]
use domain::{
    AuthAgentProvider, ErrorMessengerProvider, MessageSenderProvider, NotificationEmitterProvider,
};
use message::Sender;
use public_messenger::Messenger;
use web_interface::{NotificationSender, TabController};

#[derive(Default)]
pub struct BackgroundErrorContext {
    sender: Sender,
    messenger: Messenger,
    notifier: NotificationSender,
    auth_agent: TabController,
}

impl BackgroundErrorContext {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl MessageSenderProvider for BackgroundErrorContext {
    fn provide(&self) -> &Sender {
        &self.sender
    }
}

impl ErrorMessengerProvider for BackgroundErrorContext {
    fn provide(&self) -> &Messenger {
        &self.messenger
    }
}

impl NotificationEmitterProvider for BackgroundErrorContext {
    fn provide(&self) -> &NotificationSender {
        &self.notifier
    }
}

impl AuthAgentProvider for BackgroundErrorContext {
    fn provide(&self) -> &TabController {
        &self.auth_agent
    }
}

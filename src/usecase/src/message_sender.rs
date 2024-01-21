use domain::{DomainError, MessageSender, MessageSenderProvider, MessageToBackground, SenderInfo};
use serde::Serialize;
// use web_sys::Window;

// pub fn send_message_to_popup<T, M>(
//     ctx: &T,
//     message: M,
//     content_window: &Window,
// ) -> Result<(), DomainError>
// where
//     T: MessageSenderProvider,
//     M: Serialize,
// {
//     let message_sender = MessageSenderProvider::provide(ctx);
//     Ok(message_sender.send_to_child_frame(message, content_window, "popup.html")?)
// }

// pub fn send_message_to_content<T, M>(ctx: &T, message: M) -> Result<(), DomainError>
// where
//     T: MessageSenderProvider,
//     M: Serialize,
// {
//     let message_sender = MessageSenderProvider::provide(ctx);
//     Ok(message_sender.send_to_parent_frame(message)?)
// }

pub async fn send_inner_message<T>(ctx: &T, message: MessageToBackground) -> Result<(), DomainError>
where
    T: MessageSenderProvider,
{
    let message_sender = MessageSenderProvider::provide(ctx);
    Ok(message_sender.send_to_background(message).await?)
}

pub async fn send_outer_message<T, M>(
    ctx: &T,
    sender_info: SenderInfo,
    message: M,
) -> Result<(), DomainError>
where
    T: MessageSenderProvider,
    M: Serialize,
{
    let message_sender = MessageSenderProvider::provide(ctx);
    Ok(message_sender.send_to_tab(message, sender_info).await?)
}

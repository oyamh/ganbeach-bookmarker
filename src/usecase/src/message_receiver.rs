use domain::{MessageReceiver, MessageReceiverProvider};
use serde::Deserialize;

pub fn on_extension_message<T, M, M2, C>(ctx: &T, callback: C)
where
    T: MessageReceiverProvider,
    M: for<'de> Deserialize<'de>,
    M2: for<'de> Deserialize<'de>,
    C: Fn(M, M2) + 'static,
{
    let receiver = MessageReceiverProvider::provide(ctx);
    receiver.on_extension_message(callback);
}

// pub fn on_window_message<T, M, C>(ctx: &T, callback: C) -> EventListener
// where
//     T: MessageReceiverProvider,
//     M: for<'de> Deserialize<'de>,
//     C: Fn(M) + 'static,
// {
//     let receiver = MessageReceiverProvider::provide(ctx);
//     receiver.on_window_message(callback)
// }

// pub fn once_window_message<T, M, C>(
//     ctx: &T,
//     callback: WindowOnceMessageCallback<M, C>,
// ) -> EventListener
// where
//     T: MessageReceiverProvider,
//     M: for<'de> Deserialize<'de> + 'static,
//     C: FnOnce(M) + Clone + Copy + 'static,
// {
//     let receiver = MessageReceiverProvider::provide(ctx);
//     receiver.once_window_message(callback)
// }

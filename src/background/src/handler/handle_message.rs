use domain::{DomainError, MessageToBackground, SenderInfo};
use listen_messages_context::ListenMessagesContext;
use wasm_bindgen_futures::spawn_local;

use super::{handle_fetch_lists, handle_response_lists};
use super::{handle_load_access_token, handle_notify_error, handle_open_tab};

pub(crate) fn handle_messages(
    message: MessageToBackground,
    sender_info: SenderInfo,
) -> Result<(), DomainError> {
    log::debug!("handle_messages");
    match message {
        MessageToBackground::RequestLists => {
            log::debug!("got MessageToBackground::RequestLists");
            spawn_local(async move {
                let access_token = match handle_load_access_token().await {
                    Ok(access_token) => access_token,
                    Err(error) => {
                        handle_notify_error(error, sender_info).await;
                        return;
                    }
                };

                let lists = match handle_fetch_lists(access_token).await {
                    Ok(lists) => lists,
                    Err(error) => {
                        handle_notify_error(error, sender_info).await;
                        return;
                    }
                };
                log::debug!("lists: {:?}", &lists);

                let sender_info_clone = sender_info.clone();
                {
                    if let Err(error) = handle_response_lists(sender_info, lists).await {
                        handle_notify_error(error, sender_info_clone).await;
                        return;
                    };
                }
            });
            Ok(())
        }
        MessageToBackground::OpenTab { url } => {
            log::debug!("got MessageToBackground::OpenTab");
            spawn_local(async move {
                if let Err(error) = handle_open_tab(url).await {
                    handle_notify_error(error, sender_info).await;
                    return;
                }
            });
            Ok(())
        }
    }
}

pub(crate) fn handle_listen_messages() {
    let ctx = ListenMessagesContext::new();

    let closure = |message: MessageToBackground, sender_info: SenderInfo| {
        log::debug!("!!! on message in rust !!!");
        if let Err(error) = handle_messages(message, sender_info) {
            log::error!("handle_listen_message error: {error}");
        }
    };

    usecase::on_extension_message(&ctx, closure);
}

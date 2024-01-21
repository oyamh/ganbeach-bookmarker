use domain::{Lists, MessageToBackground, MessageToPopup, SenderInfo};
use request_lists_context::RequestListsContext;

//TODO: callbackではなく、Futureトレイトを使ってResultを返す。
pub(crate) async fn handle_request_lists<T1, T2, T3>(
    lists_callback: T1,
    stored_lists_callback: T2,
    error_callback: T3,
) where
    T1: Fn(Lists) + 'static,
    T2: Fn(Lists) + 'static,
    T3: Fn(String) + 'static,
{
    log::debug!("handle_request_lists");
    let ctx = RequestListsContext::new();

    match Lists::get_all(&ctx).await {
        Ok(all_lists) => stored_lists_callback(all_lists),
        Err(error) => log::debug!("error={error}"),
    }

    let callback = move |message: MessageToPopup, _sender: SenderInfo| match message {
        MessageToPopup::Lists(lists) => lists_callback(lists),
        MessageToPopup::Error(error) => error_callback(error),
        _ => {}
    };

    usecase::on_extension_message(&ctx, callback);

    if let Err(error) = usecase::send_inner_message(&ctx, MessageToBackground::RequestLists).await {
        log::error!("send_inner_message error: {error:?}");
    }
}

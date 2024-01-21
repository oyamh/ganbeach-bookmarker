use domain::{MessageToBackground, Url};
use request_tab_context::RequestTabContext;

pub(crate) async fn handle_request_open_tab(url: Url) {
    let ctx = RequestTabContext::new();

    if let Err(error) =
        usecase::send_inner_message(&ctx, MessageToBackground::OpenTab { url }).await
    {
        log::error!("send_inner_message error: {error:?}");
    }
}

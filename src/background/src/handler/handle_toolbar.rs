use domain::Tab;
use open_window_context::OpenWindowContext;
use toolbar_context::ToolbarContext;
use wasm_bindgen_futures::spawn_local;

pub(crate) fn handle_listen_toolbar() {
    let ctx = ToolbarContext::new();
    let closure = |tab: Tab| {
        let ctx = OpenWindowContext::new();

        spawn_local(async move {
            let result = usecase::open_popup(&ctx, Some(tab)).await;
            log::debug!("usecase::open_popup result={result:#?}");
        });
    };
    usecase::on_toolbar_clicked(&ctx, closure);
}

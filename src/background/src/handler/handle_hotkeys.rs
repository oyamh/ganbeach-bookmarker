use domain::HotKey;
use listen_hotkeys_context::ListenHotkeysContext;
use open_window_context::OpenWindowContext;
use wasm_bindgen_futures::spawn_local;

pub(crate) fn handle_hotkeys(hotkey: HotKey) {
    log::debug!("hotkey={hotkey:?}");
    match hotkey {
        HotKey::OpenPopup => {
            let ctx = OpenWindowContext::new();

            spawn_local(async move {
                let result = usecase::open_popup(&ctx, None).await;
                log::debug!("usecase::open_popup result={result:#?}");
            });
        }
        _ => todo!(),
    }
}

pub(crate) fn handle_listen_hotkeys() {
    log::debug!("handle_listen_hotkeys");

    let ctx = ListenHotkeysContext::new();

    let closure = |hotkey: String| {
        handle_hotkeys(hotkey.into());
    };

    usecase::on_hotkeys(&ctx, closure);
}

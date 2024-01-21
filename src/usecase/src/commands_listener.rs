use domain::{HotkeysListener, HotkeysListenerProvider};

pub fn on_hotkeys<T, C>(ctx: &T, callback: C)
where
    T: HotkeysListenerProvider,
    C: Fn(String) + 'static,
{
    let listener = HotkeysListenerProvider::provide(ctx);
    listener.on_hotkeys(callback);
}

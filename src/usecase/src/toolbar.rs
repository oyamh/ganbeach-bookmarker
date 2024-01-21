use domain::{ToolbarListener, ToolbarListenerProvider};
use serde::Deserialize;

pub fn on_toolbar_clicked<T, M, C>(ctx: &T, callback: C)
where
    T: ToolbarListenerProvider,
    M: for<'de> Deserialize<'de>,
    C: Fn(M) + 'static,
{
    let listener = ToolbarListenerProvider::provide(ctx);
    listener.on_toolbar(callback);
}

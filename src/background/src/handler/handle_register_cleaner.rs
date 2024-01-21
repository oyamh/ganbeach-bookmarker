use clean_history_context::CleanHistoryContext;

pub(crate) fn handle_register_cleaner() {
    log::debug!("handle_register_cleaner");
    let ctx = CleanHistoryContext::new();
    usecase::register_cleaner(&ctx);
}

use domain::HistoryCleaner;
use domain::HistoryCleanerProvider;

pub fn register_cleaner<T>(ctx: &T)
where
    T: HistoryCleanerProvider,
{
    let cleaner = HistoryCleanerProvider::provide(ctx);
    cleaner.register();
}

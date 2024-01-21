use domain::{DomainError, ErrorMessenger, ErrorMessengerProvider};

pub fn error_message<T>(ctx: &T, error: &DomainError) -> &'static str
where
    T: ErrorMessengerProvider,
{
    let messenger = ErrorMessengerProvider::provide(ctx);
    messenger.error_message(error)
}

pub fn error_title<T>(ctx: &T, error: &DomainError) -> &'static str
where
    T: ErrorMessengerProvider,
{
    let messenger = ErrorMessengerProvider::provide(ctx);
    messenger.error_title(error)
}

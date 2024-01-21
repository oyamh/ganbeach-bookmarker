use domain::{DatabaseConfig, DatabaseInitializer, DatabaseInitializerProvider, DomainError};

pub async fn initialize_database<T>(ctx: &T, config: DatabaseConfig) -> Result<(), DomainError>
where
    T: DatabaseInitializerProvider,
{
    let initializer = DatabaseInitializerProvider::provide(ctx);
    Ok(initializer.initialize(config).await?)
}

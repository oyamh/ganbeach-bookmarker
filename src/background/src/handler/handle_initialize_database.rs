use domain::{DatabaseConfig, DomainError};
use initialize_database_context::InitializeDatabaseContext;

pub(crate) async fn handle_initialize_database(config: DatabaseConfig) -> Result<(), DomainError> {
    let ctx = InitializeDatabaseContext::new();
    Ok(usecase::initialize_database(&ctx, config).await?)
}

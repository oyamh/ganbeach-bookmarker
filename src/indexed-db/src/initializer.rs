use crate::{DatabaseError, IdbDatabase, IdbObjectStoreParameters, IdbVersionChangeEvent};
use domain::{DatabaseConfig, DatabaseInitializer, DomainError};
use futures::future::join_all;
use web_sys::IdbIndexParameters;

#[derive(Debug, Default)]
pub struct Initializer {}

impl Initializer {
    pub fn new() -> Self {
        Default::default()
    }

    async fn initialize(&self, db_config: DatabaseConfig) -> Result<(), DatabaseError> {
        let initialize_futures = db_config.into_iter().map(|config| async move {
            self.create_object_store(
                config.version(),
                config.name(),
                config.primary_key(),
                config.index_keys(),
            )
            .await
        });
        let results = join_all(initialize_futures).await;
        results
            .into_iter()
            .filter(|result| result.is_err())
            .for_each(|error| log::error!("initialise error: {:?}", error.err().unwrap()));
        Ok(())
    }

    /// pubで公開しているが、使うのはテスト用途でのみ。
    pub async fn create_object_store(
        &self,
        version: u32,
        db_name: &'static str,
        primary_key: Option<&'static str>,
        index_keys: Vec<&'static str>,
    ) -> Result<(), DatabaseError> {
        let mut request_open_db = IdbDatabase::open_with_u32(db_name, version)?;
        let initialize_object_store = move |e: &IdbVersionChangeEvent| {
            let store = if let Some(primary_key) = primary_key {
                e.db().create_object_store_with_optional_parameters(
                    db_name,
                    IdbObjectStoreParameters::new()
                        .key_path(Some(&primary_key.into()))
                        .auto_increment(false),
                )?
            } else {
                e.db().create_object_store_with_optional_parameters(
                    db_name,
                    IdbObjectStoreParameters::new().auto_increment(true),
                )?
                // e.db().create_object_store(db_name)?
                // return Ok(());
            };

            index_keys
                .clone()
                .into_iter()
                .map(|index_key| {
                    store.create_index_with_str_and_optional_parameters(
                        index_key,
                        index_key,
                        &IdbIndexParameters::new(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;

            // if let Some(index_key) = index_key {
            //     store.create_index_with_str_and_optional_parameters(
            //         index_key,
            //         index_key,
            //         // IdbIndexParameters::new().unique(true),
            //         &IdbIndexParameters::new(),
            //     )?;
            // }
            Ok(())
        };
        request_open_db.set_on_update_needed(initialize_object_store);
        let db = request_open_db.await?;
        let names = db.object_store_names().collect::<Vec<String>>();
        log::debug!("names: {names:?}");
        Ok(())
    }
}

impl DatabaseInitializer for Initializer {
    async fn initialize(&self, db_config: DatabaseConfig) -> Result<(), DomainError> {
        self.initialize(db_config).await.map_err(Into::into)
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod initializer_test {
    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    //wasm-pack test --firefox --headless --lib -- initializer_test::should_create_object_store
    //wasm-pack test --chrome --lib -- initializer_test::should_create_object_store
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn should_create_object_store() {
        let initializer = Initializer::new();
        let result = initializer
            .create_object_store(1, "TestDatabase", Some("title"), vec!["key"])
            .await;
        if let Err(error) = result {
            println!("error: {error}");
        } else {
            println!("result: ok");
        }
    }
}

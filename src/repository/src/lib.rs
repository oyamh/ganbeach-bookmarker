mod convert;

use domain::{DatabaseAccessor, DomainError};
use wasm_bindgen::{JsCast, JsValue};

use indexed_db::Accessor;
use web_sys::DomException;

#[derive(Debug)]
pub struct Repository {
    db_name: &'static str,
    version: u32,
}

impl Repository {
    pub fn new(db_name: &'static str, version: u32) -> Self {
        // log::debug!(
        //     "Repository::new db_name={} version={}",
        //     db_name,
        //     version
        // );
        Self { db_name, version }
    }
}

impl DatabaseAccessor for Repository {
    type Error = DomainError;
    async fn put(&self, value: impl Into<JsValue>) -> Result<(), Self::Error> {
        let accessor = Accessor::open(self.db_name, self.version).await?;
        let (tx, store) = accessor.as_writer()?;
        store.put(&value.into())?;
        tx.await?;
        Ok(())
    }

    async fn put_iter<I: IntoIterator>(&self, values: I) -> Result<(), Self::Error>
    where
        // I: IntoIterator<Item = JsValue>;
        <I as IntoIterator>::Item: JsCast,
    {
        let accessor = Accessor::open(self.db_name, self.version).await?;
        let (tx, store) = accessor.as_writer()?;
        values
            .into_iter()
            .map(|value| store.put(&value).map(|_| ()))
            .collect::<Result<(), DomException>>()?;
        tx.await?;
        Ok(())
    }

    async fn get(&self, key: impl AsRef<str>) -> Result<JsValue, Self::Error> {
        let accessor = Accessor::open(self.db_name, self.version).await?;
        // let (tx, store) = accessor.as_reader()?;
        // let js_value = store.get(key.as_ref())?.await?;
        // let _ = tx;
        // Ok(js_value)
        let (_, store) = accessor.as_reader()?;
        Ok(store.get(key.as_ref())?.await?)
    }

    async fn get_all(&self) -> Result<JsValue, Self::Error> {
        let accessor = Accessor::open(self.db_name, self.version).await?;
        let (_, store) = accessor.as_reader()?;
        Ok(store.get_all()?.await?)
    }

    async fn delete(&self, key: impl AsRef<str>) -> Result<(), Self::Error> {
        let accessor = Accessor::open(self.db_name, self.version).await?;
        let (tx, store) = accessor.as_writer()?;
        store.delete(key.as_ref())?;
        tx.await?;
        Ok(())
    }

    async fn clear(&self) -> Result<(), Self::Error> {
        let accessor = Accessor::open(self.db_name, self.version).await?;
        let (tx, store) = accessor.as_writer()?;
        store.clear()?;
        tx.await?;
        Ok(())
    }
}

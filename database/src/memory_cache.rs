use failure::Error;
use crate::Cache;
use slog::Logger;
use pretty_bytes::converter::convert;
use failure::_core::pin::Pin;
use crate::Schema;
use failure::_core::future::Future;
use futures::FutureExt;
use std::sync::{RwLock, Arc};
use std::mem;
use uuid::Uuid;

pub struct MemoryCache {
    schemas: Arc<RwLock<Vec<Schema>>>
}

impl MemoryCache {
    pub async fn new(logger: &Logger) -> Result<Self, Error> {
        info!(logger, "Starting memory cache");

        let info = sys_info::mem_info().unwrap();


        info!(logger, "Current free ram is: {}", convert((info.avail * 1000) as f64));

        Ok(Self {
            schemas: Arc::new(RwLock::new(Vec::new()))
        })
    }
}

impl Cache for MemoryCache {
    fn store_schemas(&self, logger: &Logger, schemas: Vec<Schema>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {

        let cache_schemas = Arc::clone(&self.schemas);
        let logger = logger.clone();

        async move {
            let mut lock = cache_schemas.write().unwrap();
            *lock = schemas;

            info!(logger, "Saved schemas into memory cache");

            Ok(())
        }.boxed()
    }

    fn schemas(&self, logger: &Logger) -> Pin<Box<dyn Future<Output=Result<Vec<Schema>, Error>> + Send>> {
        let cache_schemas = Arc::clone(&self.schemas);
        let logger = logger.clone();
        async move {
            let lock = cache_schemas.read().unwrap();
            let data = lock.clone();

            info!(logger, "Fetched {} schemas from cache", data.len());

            Ok(data)
        }.boxed()
    }

    fn schema(&self, logger: &Logger, id: &Uuid) -> Pin<Box<dyn Future<Output=Result<Schema, Error>> + Send>> {
        let cache_schemas = Arc::clone(&self.schemas);
        let logger = logger.clone();
        let id = id.clone();
        async move {
            let lock = cache_schemas.read().unwrap();
            match lock.iter().find(|i| i.id == id) {
                Some(v) => {
                    info!(logger, "Fetched schema {} from cache, id was {}", v.name, v.id);
                    Ok(v.clone())
                },
                None => {
                    bail!("Schema not found");
                }
            }
        }.boxed()
    }

    fn set_schema(&self, logger: &Logger, schema: Schema) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        let cache_schemas = Arc::clone(&self.schemas);
        let logger = logger.clone();
        async move {
            let mut lock = cache_schemas.write().unwrap();
            let id = schema.id.to_owned();

            // TODO: Validate constraints before insert

            lock.push(schema);

            info!(logger, "Inserted schema {} into cache", id);

            Ok(())
        }.boxed()
    }

    fn cache_size(&self) -> usize {
        let lock = self.schemas.read().unwrap();

        let mut size = 0;

        size += lock.len() * mem::size_of::<Schema>();

        size += mem::size_of_val(&self);

        size
    }
}
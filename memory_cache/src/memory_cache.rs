use failure::Error;
use shelf_database::{Cache, Store};
use slog::{Logger, SendSyncRefUnwindSafeDrain};
use pretty_bytes::converter::convert;
use failure::_core::pin::Pin;
use shelf_database::Schema;
use failure::_core::future::Future;
use futures::FutureExt;
use std::sync::{RwLock, Arc, RwLockReadGuard};
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
    fn load(&self, logger: &Logger, store: &Arc<dyn Store>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        let logger = logger.clone();
        let store = Arc::clone(&store);
        let schemas_store = Arc::clone(&self.schemas);
        async move {
            info!(logger, "Fetching schemas from store");
            let schemas = store.get_schemas().await?;
            {
                for schema in &schemas {
                    let logger = logger.new(o!("schema" => schema.name.to_string()));
                    schema.validate_definition(&logger)?;
                }
                let mut lock = schemas_store.write().unwrap();
                *lock = schemas;
            }
            info!(logger, "All schemas fetched and added to cache! 😎");
            Ok(())
        }.boxed()
    }

    fn save(&self, logger: &Logger, store: &Arc<dyn Store>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        let logger = logger.clone();
        let schemas = Arc::clone(&self.schemas);
        let store = Arc::clone(&store);

        async move  {
            let data = {
                let lock = schemas.read().unwrap();
                lock.clone()
            };
            for schema in data.iter() {
                store.save_schema(schema).await?;
            }
            Ok(())
        }.boxed()
    }

    fn schemas(&self) -> RwLockReadGuard<Vec<Schema>> {
        let lock = self.schemas.read().unwrap();
        lock
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

    fn add_schema(&self, logger: &Logger, schema: Schema) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        let cache_schemas = Arc::clone(&self.schemas);
        let logger = logger.clone();
        async move {
            let mut lock = cache_schemas.write().unwrap();
            let id = schema.id.to_owned();
            let name = schema.name.to_owned();

            schema.validate_definition(&logger)?;

            lock.push(schema);

            info!(logger, "Inserted schema {} into cache", name; "schema_id" => id.to_string());

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

    fn is_empty(&self) -> bool {
        let lock = self.schemas.read().unwrap();
        return lock.is_empty()
    }
}
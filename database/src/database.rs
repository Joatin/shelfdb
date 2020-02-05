use failure::Error;
use slog::Logger;
use crate::cache::Cache;
use crate::store::Store;
use crate::{Schema, Collection};
use uuid::Uuid;
use colored::*;
use std::sync::{Arc, RwLockReadGuard};

#[derive(Clone)]
pub struct Database {
    store: Arc<dyn Store + Send + Sync + 'static>,
    cache: Arc<dyn Cache + Send + Sync + 'static>
}

impl Database {

    pub async fn new<S: Store + Send + Sync + 'static, C: Cache + Send + Sync + 'static>(logger: &Logger, store: S, cache: C) -> Result<Self, Error> {
        let arc_store = Arc::new(store);

        cache.load(&logger, &(Arc::clone(&arc_store) as Arc<dyn Store>)).await?;

        if cache.is_empty() {
            warn!(logger, "No schemas found, creating initial setup...");
            cache.add_schema(&logger, Schema::get_system_schema()).await?;
            cache.add_schema(&logger, Schema::get_default_schema()).await?;
            cache.save(&logger, &(Arc::clone(&arc_store) as Arc<dyn Store>)).await?;
        }

        info!(logger, "Current cache size is: {}", pretty_bytes::converter::convert(cache.cache_size() as f64));

        Ok(Self {
            store: arc_store,
            cache: Arc::new(cache)
        })
    }

    // FETCH FUNCTIONS //

    pub fn schemas(&self) -> RwLockReadGuard<Vec<Schema>> {
        self.cache.schemas()
    }

    pub async fn schema(&self, logger: &Logger, id: &Uuid) -> Result<Schema, Error> {
        trace!(logger, "Fetching schema {}", id);
        Ok(self.cache.schema(&logger, &id).await?)
    }

    pub async fn set_schema(&self, logger: &Logger, schema: Schema) -> Result<(), Error> {
        trace!(logger, "Adding schema {} to cache", schema.id);
        Ok(self.cache.add_schema(&logger, schema).await?)
    }
}
use failure::Error;
use slog::Logger;
use crate::cache::Cache;
use crate::store::Store;
use crate::{Schema, Collection};
use uuid::Uuid;
use colored::*;


pub struct Database {
    store: Box<dyn Store + Send + Sync + 'static>,
    cache: Box<dyn Cache + Send + Sync + 'static>
}

impl Database {

    pub async fn new<S: Store + Send + Sync + 'static, C: Cache + Send + Sync + 'static>(logger: &Logger, store: S, cache: C) -> Result<Self, Error> {

        let mut schemas = store.get_schemas().await?;

        if schemas.is_empty() {
            schemas = Database::initial_setup(&logger, &store).await?;
        }

        for schema in &schemas {
            let collections = store.get_collections(&schema).await?;
            info!(logger, "Available schema {}", format!("\"{}\"", schema.name).yellow(); "collection_count" => collections.len(), "created_at" => schema.created_at.to_string(), "description" => &schema.description, "id" => schema.id.to_string(), "name" => &schema.name);

            for collection in collections {
                info!(logger, "Available database.collection \"{}\" in schema \"{}\"", collection.name, schema.name; "schema_id" => schema.id.to_string(), "schema_name" => &schema.name, "created_at" => collection.created_at.to_string(), "description" => &collection.description, "id" => collection.id.to_string(), "name" => &collection.name);
            }
        }

        cache.store_schemas(&logger, schemas).await?;

        info!(logger, "Current cache size is: {}", pretty_bytes::converter::convert(cache.cache_size() as f64));

        Ok(Self {
            store: Box::new(store),
            cache: Box::new(cache)
        })
    }

    async fn initial_setup<S: Store + Send + Sync + 'static>(logger: &Logger, store: &S) -> Result<Vec<Schema>, Error> {
        warn!(logger, "Database has no schemas, creating the initial one");
        let system_schema = Schema::get_system_schema();
        store.save_schema(&system_schema).await?;

        let default_schema = Schema::get_default_schema();
        store.save_schema(&default_schema).await?;

        let schema_version_collection = Collection::get_schema_version_collection();

        store.save_collection(&system_schema, &schema_version_collection).await?;

        let schemas = store.get_schemas().await?;

        Ok(schemas)
    }

    // FETCH FUNCTIONS //

    pub async fn schemas(&self, logger: &Logger) -> Result<Vec<Schema>, Error> {
        trace!(logger, "Fetching list of schemas");
        Ok(self.cache.schemas(&logger).await?)
    }

    pub async fn schema(&self, logger: &Logger, id: &Uuid) -> Result<Schema, Error> {
        trace!(logger, "Fetching schema {}", id);
        Ok(self.cache.schema(&logger, &id).await?)
    }

    pub async fn set_schema(&self, logger: &Logger, schema: Schema) -> Result<(), Error> {
        trace!(logger, "Adding schema {} to cache", schema.id);
        Ok(self.cache.set_schema(&logger, schema).await?)
    }
}
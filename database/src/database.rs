use failure::Error;
use slog::Logger;
use crate::cache::Cache;
use crate::store::Store;
use crate::{Schema, Document};
use std::ops::Deref;
use uuid::Uuid;
use std::collections::HashMap;
use crate::CacheSchema;
use crate::CacheCollection;


pub struct Database<C: Cache, S: Store> {
    cache: C,
    store: S,
}

impl<C: Cache, S: Store> Database<C, S> {

    pub async fn new(logger: &Logger, store: S, mut cache: C) -> Result<Self, Error> {

        cache.load(&logger, &store).await?;

        if cache.is_empty() {
            warn!(logger, "No schemas found, creating initial setup...");
            let schema_id = Uuid::new_v4();
            cache.set_schema(&logger, Schema::new(schema_id, "shelf", None), include_str!("shelf_base_schema.graphql"))?;

            let schema_lock = cache.schema(&logger, schema_id).unwrap();
            {
                let schema = schema_lock.read().unwrap();
                let collection_lock = schema.collection_by_name("Car").unwrap();
                let mut collection = collection_lock.write().unwrap();

                info!(logger, "Creating one million ids");
                let docs: Vec<_> = vec![0;100_000].into_iter().map(|_| Document {
                    id: Uuid::new_v4(),
                    fields: HashMap::new()
                }).collect();


                info!(logger, "Creating crazy amount of documents");

                for doc in docs {
                    collection.set_document(doc);
                }
                info!(logger, "Done creating fake data");
            }


            cache.save(&logger, &store).await?;
        }

        info!(logger, "Current cache size is: {}", pretty_bytes::converter::convert(cache.cache_size() as f64));

        Ok(Self {
            cache,
            store,
        })
    }

    pub fn cache(&self) -> &C {
        &self.cache
    }
    pub fn cache_mut(&mut self) -> &mut C {
        &mut self.cache
    }

    pub async fn save(&self, logger: &Logger) -> Result<(), Error> {
        self.cache.save(&logger, &self.store).await?;
        Ok(())
    }
}

impl<C: Cache, S: Store> Deref for Database<C, S> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}
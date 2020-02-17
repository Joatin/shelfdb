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

                let mut model_s = HashMap::new();
                model_s.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_s.insert("make".to_string(), serde_json::to_value("Model S").unwrap());
                collection.set_document(Document {
                    id: Uuid::new_v4(),
                    fields: model_s
                });

                let mut model_x = HashMap::new();
                model_x.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_x.insert("make".to_string(), serde_json::to_value("Model X").unwrap());
                collection.set_document(Document {
                    id: Uuid::new_v4(),
                    fields: model_x
                });

                let mut model_3 = HashMap::new();
                model_3.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_3.insert("make".to_string(), serde_json::to_value("Model 3").unwrap());
                collection.set_document(Document {
                    id: Uuid::new_v4(),
                    fields: model_3
                });

                let mut model_y = HashMap::new();
                model_y.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_y.insert("make".to_string(), serde_json::to_value("Model Y").unwrap());
                collection.set_document(Document {
                    id: Uuid::new_v4(),
                    fields: model_y
                });
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
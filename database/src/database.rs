use crate::{
    cache::Cache,
    store::Store,
    CacheCollection,
    CacheSchema,
    Document,
    Schema,
};
use failure::Error;
use slog::Logger;
use std::{
    collections::HashMap,
    ops::Deref,
};
use uuid::Uuid;

pub struct Database<C: Cache, S: Store> {
    cache: C,
    store: S,
}

impl<C: Cache, S: Store> Database<C, S> {
    pub async fn new(logger: &Logger, store: S, cache: C) -> Result<Self, Error> {
        cache.load(&logger, &store).await?;

        if cache.is_empty().await {
            warn!(logger, "No schemas found, creating initial setup...");
            let schema_id = Uuid::new_v4();
            cache
                .insert_schema(
                    &logger,
                    Schema::new(schema_id, "shelf", None),
                    include_str!("shelf_base_schema.graphql"),
                )
                .await?;

            let schema = cache.schema(schema_id).await.unwrap();
            {
                let collection = schema.collection_by_name("Car").await.unwrap();

                let mut model_s = HashMap::new();
                model_s.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_s.insert("make".to_string(), serde_json::to_value("Model S").unwrap());
                collection.set_document(Document {
                    id: Uuid::new_v4(),
                    fields: model_s,
                });

                let mut model_x = HashMap::new();
                model_x.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_x.insert("make".to_string(), serde_json::to_value("Model X").unwrap());
                collection.set_document(Document {
                    id: Uuid::new_v4(),
                    fields: model_x,
                });

                let mut model_3 = HashMap::new();
                model_3.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_3.insert("make".to_string(), serde_json::to_value("Model 3").unwrap());
                collection.set_document(Document {
                    id: Uuid::new_v4(),
                    fields: model_3,
                });

                let mut model_y = HashMap::new();
                model_y.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_y.insert("make".to_string(), serde_json::to_value("Model Y").unwrap());
                collection.set_document(Document {
                    id: Uuid::new_v4(),
                    fields: model_y,
                });
            }

            cache.save(&logger, &store).await?;
        }

        info!(
            logger,
            "Current cache size is: {}",
            pretty_bytes::converter::convert(cache.cache_size().await as f64)
        );

        Ok(Self { cache, store })
    }

    pub fn cache(&self) -> &C {
        &self.cache
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

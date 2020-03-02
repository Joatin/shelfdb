use crate::{
    cache::Cache,
    store::Store,
    CacheCollection,
    CacheSchema,
    Document,
    Schema,
};
use failure::Error;
use shelf_config::Config;
use slog::Logger;
use std::{
    collections::HashMap,
    ops::{
        Add,
        Deref,
    },
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
    },
    time::Duration,
};
use tokio::{
    task::JoinHandle,
    time::{
        interval_at,
        Instant,
    },
};
use uuid::Uuid;

pub struct Database<C: Cache, S: Store> {
    cache: Arc<C>,
    store: Arc<S>,
    run_save: Arc<AtomicBool>,
}

impl<C: Cache, S: Store> Database<C, S> {
    pub async fn new(logger: &Logger, config: &Config, store: S, cache: C) -> Result<Self, Error> {
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
                model_s.insert(
                    "model".to_string(),
                    serde_json::to_value("Model S").unwrap(),
                );
                collection
                    .set_document(Document {
                        id: Uuid::new_v4(),
                        fields: model_s,
                    })
                    .await;

                let mut model_x = HashMap::new();
                model_x.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_x.insert(
                    "model".to_string(),
                    serde_json::to_value("Model X").unwrap(),
                );
                collection
                    .set_document(Document {
                        id: Uuid::new_v4(),
                        fields: model_x,
                    })
                    .await;

                let mut model_3 = HashMap::new();
                model_3.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_3.insert(
                    "model".to_string(),
                    serde_json::to_value("Model 3").unwrap(),
                );
                collection
                    .set_document(Document {
                        id: Uuid::new_v4(),
                        fields: model_3,
                    })
                    .await;

                let mut model_y = HashMap::new();
                model_y.insert("brand".to_string(), serde_json::to_value("Tesla").unwrap());
                model_y.insert(
                    "model".to_string(),
                    serde_json::to_value("Model Y").unwrap(),
                );
                collection
                    .set_document(Document {
                        id: Uuid::new_v4(),
                        fields: model_y,
                    })
                    .await;
            }

            cache.save(&logger, &store).await?;
        }

        info!(
            logger,
            "Current cache size is: {}",
            pretty_bytes::converter::convert(cache.cache_size().await as f64)
        );

        let cache = Arc::new(cache);
        let store = Arc::new(store);

        let run_save = Arc::new(AtomicBool::new(true));

        Self::start_save_loop(
            &logger,
            Arc::clone(&run_save),
            &cache,
            &store,
            config.save_interval,
        );

        Ok(Self {
            cache,
            store,
            run_save,
        })
    }

    pub fn cache(&self) -> &C {
        &self.cache
    }

    pub async fn save(&self, logger: &Logger) -> Result<(), Error> {
        self.cache.save(&logger, self.store.deref()).await?;
        Ok(())
    }

    pub fn start_save_loop(
        logger: &Logger,
        run_save: Arc<AtomicBool>,
        cache: &Arc<C>,
        store: &Arc<S>,
        duration: Duration,
    ) -> JoinHandle<()> {
        let logger = logger.new(o!("save_interval" => format!("{:#?}", duration)));
        let cache = Arc::clone(cache);
        let store = Arc::clone(store);

        info!(logger, "➿ Starting save loop");

        tokio::spawn(async move {
            let mut interval = interval_at(Instant::now().add(duration), duration);

            while run_save.load(Ordering::Relaxed) {
                interval.tick().await;
                debug!(logger, "Saving data...");
                if let Err(err) = cache.save(&logger, store.deref()).await {
                    error!(logger, "Failed to save data"; "error" => format!("{}", err));
                }
            }
        })
    }
}

impl<C: Cache, S: Store> Clone for Database<C, S> {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
            store: Arc::clone(&self.store),
            run_save: Arc::clone(&self.run_save)
        }
    }
}

impl<C: Cache, S: Store> Drop for Database<C, S> {
    fn drop(&mut self) {
        self.run_save.store(false, Ordering::Relaxed);
    }
}

impl<C: Cache, S: Store> Deref for Database<C, S> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

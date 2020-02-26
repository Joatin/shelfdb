use crate::{
    memory_cache_collection::MemoryCacheCollection,
    memory_cache_schema::MemoryCacheSchema,
};
use failure::Error;
use futures::{
    future::BoxFuture,
    stream,
    stream::BoxStream,
    FutureExt,
    StreamExt,
};
use pretty_bytes::converter::convert;
use shelf_database::{
    Cache,
    CacheCollection,
    CacheSchema,
    Schema,
    Store,
};
use slog::Logger;
use std::{
    collections::HashMap,
    mem,
    time::Instant,
};
use tokio::sync::{
    broadcast::{
        channel,
        Receiver,
        Sender,
    },
    RwLock,
};
use uuid::Uuid;

pub struct MemoryCache {
    schemas: RwLock<Vec<MemoryCacheSchema>>,
    on_schema_updates_sender: Sender<()>,
}

impl MemoryCache {
    pub async fn new(logger: &Logger) -> Result<Self, Error> {
        info!(logger, "Starting memory cache");

        let info = sys_info::mem_info().unwrap();
        info!(
            logger,
            "Current free ram is: {}",
            convert((info.avail * 1000) as f64)
        );

        let (sender, _) = channel(1);

        Ok(Self {
            schemas: RwLock::new(Vec::new()),
            on_schema_updates_sender: sender,
        })
    }

    async fn do_insert_schema(&self, schema: MemoryCacheSchema) {
        let mut lock = self.schemas.write().await;
        lock.push(schema);
    }
}

impl Cache for MemoryCache
where
    MemoryCache: Send,
{
    type CacheSchema = MemoryCacheSchema;

    fn load<'a, S: Store>(
        &'a self,
        logger: &'a Logger,
        store: &'a S,
    ) -> BoxFuture<Result<(), Error>> {
        async move {
            let start_time = Instant::now();
            info!(logger, "Fetching schemas from store");
            let schemas = store.get_schemas(&logger).await?;
            info!(logger, "Info found {} schemas", schemas.len());

            for schema in schemas {
                info!(logger, "Fetching collection for schema {}", schema.name);
                let collections = store.get_collections(&logger, &schema).await?;
                info!(logger, "Info found {} collections for schema {}", collections.len(), schema.name);


                let mut mapped_collections = HashMap::new();
                for collection in collections {
                    let documents = store.get_documents(&logger, &schema, &collection).await?;
                    mapped_collections.insert(collection.id, MemoryCacheCollection::new(collection, documents));
                }

                self.do_insert_schema(MemoryCacheSchema::new(schema, mapped_collections)).await;
            }

            info!(logger, "All schemas fetched and added to cache! 😎"; "load_time" => format!("{}ms", Instant::now().duration_since(start_time).as_millis()));
            Ok(())
        }.boxed()
    }

    fn save<'a, S: Store>(
        &'a self,
        logger: &'a Logger,
        store: &'a S,
    ) -> BoxFuture<Result<(), Error>> {
        async move {
            self.schemas().for_each_concurrent(None, |schema: Self::CacheSchema| {
                let schema = schema;
                async move {
                    let inner_schema = schema.inner_schema().await;
                    if let Err(err) = store.save_schema(&logger, &inner_schema).await {
                        error!(logger, "Failed to save schema"; "error" => format!("{}", err));
                    }
                    schema.collections().for_each_concurrent(None, move |collection| {
                        let inner_schema = inner_schema.clone();
                        async move {
                            let inner_collection = collection.inner_collection().await;
                            if let Err(err) = store.save_collection(&logger, &inner_schema, &inner_collection).await {
                                error!(logger, "Failed to save collection"; "error" => format!("{}", err));
                            }

                            collection.documents().for_each_concurrent(None, |document| async {
                                if let Err(err) = store.save_document(&logger, &inner_schema, &inner_collection, document).await {
                                    error!(logger, "Failed to save document"; "error" => format!("{}", err));
                                }
                            }).await;
                        }
                    }).await;
                }
            }).await;

            store.flush(&logger).await?;
            Ok(())
        }
        .boxed()
    }

    fn schemas(&self) -> BoxStream<Self::CacheSchema> {
        stream::once(self.schemas.read())
            .map(|i| stream::iter(i.clone().into_iter()))
            .flatten()
            .then(|i| async move { Self::CacheSchema::clone(&i) })
            .boxed()
    }

    fn schema(&self, id: Uuid) -> BoxFuture<Option<Self::CacheSchema>> {
        self.schemas()
            .filter_map(move |i| {
                async move {
                    if i.inner_schema().await.id == id {
                        return Some(i);
                    }
                    return None;
                }
                .boxed()
            })
            .into_future()
            .map(|(next, _)| next)
            .boxed()
    }

    fn schema_by_name<'a>(&'a self, name: &'a str) -> BoxFuture<'a, Option<Self::CacheSchema>> {
        self.schemas()
            .filter_map(move |i| {
                async move {
                    if i.inner_schema().await.name == name {
                        return Some(i);
                    }
                    return None;
                }
                .boxed()
            })
            .into_future()
            .map(|(next, _)| next)
            .boxed()
    }

    fn insert_schema<'a>(
        &'a self,
        logger: &'a Logger,
        schema: Schema,
        new_graphql_schema: &'a str,
    ) -> BoxFuture<'a, Result<(), Error>> {
        async move {
            let mem_schema = MemoryCacheSchema::new(schema, HashMap::new());
            mem_schema.migrate(&logger, new_graphql_schema).await?;
            self.do_insert_schema(mem_schema).await;
            Ok(())
        }
        .boxed()
    }

    fn cache_size(&self) -> BoxFuture<usize> {
        async move {
            let mut size = 0;
            size += mem::size_of_val(&self);

            for schema in self.schemas.read().await.iter() {
                size += schema.get_size().await;
            }

            size
        }
        .boxed()
    }

    fn is_empty(&self) -> BoxFuture<bool> {
        async move { self.schemas.read().await.is_empty() }.boxed()
    }

    fn on_schema_updates(&self) -> Receiver<()> {
        self.on_schema_updates_sender.subscribe()
    }
}

use failure::Error;
use shelf_database::{Cache, Store, CacheSchema};
use slog::Logger;
use pretty_bytes::converter::convert;
use failure::_core::pin::Pin;
use shelf_database::Schema;
use failure::_core::future::Future;
use futures::FutureExt;
use std::sync::RwLock;
use std::mem;
use uuid::Uuid;
use tokio::sync::broadcast::{channel, Sender, Receiver};
use crate::memory_cache_schema::MemoryCacheSchema;
use futures::future::join_all;
use crate::memory_cache_collection::MemoryCacheCollection;
use std::time::Instant;


pub struct MemoryCache {
    schemas: Vec<RwLock<MemoryCacheSchema>>,
    on_schema_updates_sender: Sender<()>
}

impl MemoryCache {
    pub async fn new(logger: &Logger) -> Result<Self, Error> {
        info!(logger, "Starting memory cache");

        let info = sys_info::mem_info().unwrap();
        info!(logger, "Current free ram is: {}", convert((info.avail * 1000) as f64));


        let (sender, _) = channel(1);

        Ok(Self {
            schemas: Vec::new(),
            on_schema_updates_sender: sender
        })
    }
}

impl Cache for MemoryCache
    where MemoryCache: Send
{
    type CacheSchema = MemoryCacheSchema;

    fn load<'a, S: Store>(&'a mut self, logger: &'a Logger, store: &'a S) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> {

        async move {
            let start_time = Instant::now();
            info!(logger, "Fetching schemas from store");
            let schemas = store.get_schemas(&logger).await?;
            info!(logger, "Info found {} schemas", schemas.len());

            for schema in schemas {
                info!(logger, "Fetching collection for schema {}", schema.name);
                let collections = store.get_collections(&logger, &schema).await?;
                info!(logger, "Info found {} collections for schema {}", collections.len(), schema.name);


                let mut mapped_collections = vec![];
                for collection in collections {
                    let documents = store.get_documents(&logger, &schema, &collection).await?;
                    mapped_collections.push(RwLock::new(MemoryCacheCollection::new(collection, documents)))
                }

                self.schemas.push(RwLock::new(MemoryCacheSchema::new(schema, mapped_collections)));
            }

            info!(logger, "All schemas fetched and added to cache! 😎"; "load_time" => format!("{}ms", Instant::now().duration_since(start_time).as_millis()));
            Ok(())
        }.boxed()
    }

    fn save<'a, S: Store>(&'a self, logger: &'a Logger, store: &'a S) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> {
        async move {
            for schema_lock in self.schemas.iter() {
                let (schema, collection) = {
                    let lock = schema_lock.read().unwrap();
                    lock.get_data_cloned()
                };

                store.save_schema(&logger, &schema).await?;

                for (collection, documents) in collection {
                    store.save_collection(&logger, &schema, &collection).await?;

                    let futs: Vec<_> = documents.into_iter().map(|doc| {
                        store.save_document(&logger, &schema, &collection, doc)
                    }).collect();

                    join_all(futs).await.into_iter().collect::<Result<_, _>>()?;
                }

            }

            store.flush(&logger).await?;
            Ok(())
        }.boxed()
    }

    fn schemas(&self) -> &Vec<RwLock<Self::CacheSchema>> {
        &self.schemas
    }

    fn schema(&self, _logger: &Logger, id: Uuid) -> Option<&RwLock<Self::CacheSchema>> {
        self.schemas.iter().find(|i| {
            let lock = i.read().unwrap();
            lock.id.eq(&id)
        })
    }

    fn schema_by_name(&self, _logger: &Logger, name: &str) -> Option<&RwLock<Self::CacheSchema>> {
        self.schemas.iter().find(|i| {
            let lock = i.read().unwrap();
            lock.name.eq(name)
        })
    }

    fn set_schema(&mut self, logger: &Logger, schema: Schema, new_graphql_schema: &str) -> Result<(), Error> {
        let mut mem_schema = MemoryCacheSchema::new(schema, vec![]);
        mem_schema.migrate(&logger, new_graphql_schema)?;
        self.schemas.push(RwLock::new(mem_schema));
        Ok(())
    }

    fn cache_size(&self) -> usize {
        let mut size = 0;
        size += mem::size_of_val(&self);

        for schema in &self.schemas {
            let lock = schema.read().unwrap();
            size += lock.get_size();
        }

        size
    }

    fn is_empty(&self) -> bool {
        self.schemas.is_empty()
    }

    fn on_schema_updates(&self) -> Receiver<()> {
        self.on_schema_updates_sender.subscribe()
    }
}
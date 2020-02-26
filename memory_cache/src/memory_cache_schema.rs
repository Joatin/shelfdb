use crate::memory_cache_collection::MemoryCacheCollection;
use failure::Error;
use futures::{
    future::BoxFuture,
    stream,
    stream::BoxStream,
    FutureExt,
    StreamExt,
};
use shelf_database::{
    CacheCollection,
    CacheSchema,
    Collection,
    Schema,
};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct MemoryCacheSchema {
    schema: Arc<RwLock<Schema>>,
    collections: Arc<RwLock<HashMap<Uuid, MemoryCacheCollection>>>,
}

impl MemoryCacheSchema {
    pub fn new(schema: Schema, collections: HashMap<Uuid, MemoryCacheCollection>) -> Self {
        Self {
            schema: Arc::new(RwLock::new(schema)),
            collections: Arc::new(RwLock::new(collections)),
        }
    }

    pub async fn get_size(&self) -> usize {
        self.collections()
            .fold(0, |acc, val| async move { acc + val.get_size().await })
            .await
    }
}

impl CacheSchema for MemoryCacheSchema {
    type CacheCollection = MemoryCacheCollection;

    fn inner_schema(&self) -> BoxFuture<Schema> {
        async move { self.schema.read().await.clone() }.boxed()
    }

    fn set_schema(&self, schema: Schema) -> BoxFuture<()> {
        async move {
            let mut lock = self.schema.write().await;
            *lock = schema
        }
        .boxed()
    }

    fn collections(&self) -> BoxStream<<MemoryCacheSchema as CacheSchema>::CacheCollection> {
        stream::once(self.collections.read())
            .map(|i| stream::iter(i.clone().into_iter()))
            .flatten()
            .map(|(_key, val)| val)
            .boxed()
    }

    fn insert_collection(&self, collection: Collection) -> BoxFuture<Result<(), Error>> {
        async move {
            // CHECK NAME
            if let Some(coll) = self.collection_by_name(&collection.name).await {
                if coll.inner_collection().await.id != collection.id {
                    bail!("A collection with this name does already exist")
                }
            }

            // DO THE INSERT
            let mut lock = self.collections.write().await;
            lock.insert(
                collection.id,
                MemoryCacheCollection::new(collection, vec![]),
            );

            Ok(())
        }
        .boxed()
    }

    fn collection(&self, id: Uuid) -> BoxFuture<Option<Self::CacheCollection>> {
        async move {
            let stream = self.collections();
            let mut mapped = stream
                .filter_map(move |i| async move {
                    if i.inner_collection().await.id.eq(&id) {
                        return Some(i);
                    }
                    return None;
                })
                .boxed();
            mapped.next().await
        }
        .boxed()
    }

    fn collection_by_name<'a>(
        &'a self,
        name: &'a str,
    ) -> BoxFuture<'a, Option<Self::CacheCollection>> {
        async move {
            let stream = self.collections();
            let mut mapped = stream
                .filter_map(move |i| {
                    let name = name.to_string();
                    async move {
                        if i.inner_collection().await.name == name {
                            return Some(i);
                        }
                        return None;
                    }
                })
                .boxed();
            mapped.next().await
        }
        .boxed()
    }
}

#[cfg(test)]
mod test {
    use crate::memory_cache_schema::MemoryCacheSchema;
    use shelf_database::{
        CacheSchema,
        Schema,
    };
    use std::collections::HashMap;
    use uuid::Uuid;

    #[tokio::test]
    async fn inner_schema_should_return_the_inner_schema() {
        let id = Uuid::new_v4();
        let mem_schema = MemoryCacheSchema::new(Schema::new(id, "TEST", None), HashMap::new());
        assert_eq!(
            mem_schema.inner_schema().await.id,
            id,
            "The schemas are not the same"
        );
    }
}

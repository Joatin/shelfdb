use shelf_database::{CacheSchema, Schema, Collection, CacheCollection, Document};
use uuid::Uuid;
use std::sync::{RwLock};
use std::ops::Deref;
use crate::memory_cache_collection::MemoryCacheCollection;
use failure::Error;

pub struct MemoryCacheSchema {
    schema: Schema,
    collections: Vec<RwLock<MemoryCacheCollection>>
}

impl MemoryCacheSchema {
    pub fn new(schema: Schema, collections: Vec<RwLock<MemoryCacheCollection>>) -> Self {
        Self {
            schema,
            collections
        }
    }

    pub fn get_data_cloned(&self) -> (Schema, Vec<(Collection, Vec<Document>)>) {
        let collection: Vec<_> = self.collections().iter().map(|lock| {
            let coll = lock.read().unwrap();
            coll.get_data_cloned()
        }).collect();
        (self.schema.clone(), collection)
    }

    pub fn get_size(&self) -> usize {
        let mut size = 0;

        for coll in &self.collections {
            let lock = coll.read().unwrap();
            size += lock.get_size();
        }

        size
    }
}

impl CacheSchema for MemoryCacheSchema {
    type CacheCollection = MemoryCacheCollection;

    fn inner_schema(&self) -> &Schema {
        &self.schema
    }

    fn inner_schema_mut(&mut self) -> &mut Schema {
        &mut self.schema
    }

    fn collections(&self) -> &[RwLock<Self::CacheCollection>] {
        &self.collections
    }

    fn set_collection(&mut self, collection: Collection) -> Result<(), Error> {
        let same_name = self.collections.iter().find(|i| {
            let lock = i.read().unwrap();
            lock.inner_collection().name == collection.name
        });

        if let Some(val) = same_name {
            let id = {
                let lock = val.read().unwrap();
                lock.inner_collection().id
            };
            if id == collection.id {
                // Nice found same item, lets replace it
                let index = self.collections.iter().position(|i| {
                    let lock = i.read().unwrap();
                    lock.inner_collection().id == collection.id
                }).unwrap();
                self.collections.remove(index);
                self.collections.push(RwLock::new(MemoryCacheCollection::new(collection, vec![])));
                Ok(())
            } else {
                bail!("Another collection with the same name does already exist")
            }
        } else {
            self.collections.push(RwLock::new(MemoryCacheCollection::new(collection, vec![])));
            Ok(())
        }
    }

    fn collection(&self, id: Uuid) -> Option<&RwLock<Self::CacheCollection>> {
        self.collections.iter().find(|i| {
            let lock = i.read().unwrap();
            lock.inner_collection().id.eq(&id)
        })
    }

    fn collection_by_name(&self, name: &str) -> Option<&RwLock<Self::CacheCollection>> {
        self.collections.iter().find(|i| {
            let lock = i.read().unwrap();
            lock.inner_collection().name == name
        })
    }
}

impl Deref for MemoryCacheSchema {
    type Target = Schema;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}

#[cfg(test)]
mod test {
    use crate::memory_cache_schema::MemoryCacheSchema;
    use shelf_database::{Schema, CacheSchema};
    use uuid::Uuid;

    #[test]
    fn inner_schema_should_return_the_inner_schema() {
        let id = Uuid::new_v4();
        let mem_schema = MemoryCacheSchema::new(Schema::new(id, "TEST", None), vec![]);
        assert_eq!(mem_schema.inner_schema().id, id, "The schemas are not the same");
    }

}
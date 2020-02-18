use crate::{Cache, Schema, Store, CacheSchema, Collection, CacheCollection, Document};
use slog::{Logger};
use std::sync::{RwLock};
use uuid::Uuid;
use failure::Error;
use std::pin::Pin;
use std::future::Future;
use tokio::sync::broadcast::Receiver;

pub struct TestCache;
pub struct TestCacheSchema;
pub struct TestCacheCollection;

impl Cache for TestCache {
    type CacheSchema = TestCacheSchema;

    fn load<'a, S: Store>(&'a mut self, logger: &'a Logger, store: &'a S) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        unimplemented!()
    }

    fn save<'a, S: Store>(&'a self, logger: &'a Logger, store: &'a S) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        unimplemented!()
    }

    fn schemas(&self) -> &Vec<RwLock<Self::CacheSchema>> {
        unimplemented!()
    }

    fn schema(&self, logger: &Logger, id: Uuid) -> Option<&RwLock<Self::CacheSchema>> {
        unimplemented!()
    }

    fn schema_by_name(&self, logger: &Logger, name: &str) -> Option<&RwLock<Self::CacheSchema>> {
        unimplemented!()
    }

    fn set_schema(&mut self, logger: &Logger, schema: Schema, new_graphql_schema: &str) -> Result<(), Error> {
        unimplemented!()
    }

    fn cache_size(&self) -> usize {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        unimplemented!()
    }

    fn on_schema_updates(&self) -> Receiver<()> {
        unimplemented!()
    }
}

impl CacheSchema for TestCacheSchema {
    type CacheCollection = TestCacheCollection;

    fn inner_schema(&self) -> &Schema {
        unimplemented!()
    }

    fn inner_schema_mut(&mut self) -> &mut Schema {
        unimplemented!()
    }

    fn collections(&self) -> &[RwLock<Self::CacheCollection>] {
        unimplemented!()
    }

    fn set_collection(&mut self, collection: Collection) -> Result<(), Error> {
        unimplemented!()
    }

    fn collection(&self, id: Uuid) -> Option<&RwLock<Self::CacheCollection>> {
        unimplemented!()
    }

    fn collection_by_name(&self, name: &str) -> Option<&RwLock<Self::CacheCollection>> {
        unimplemented!()
    }
}

impl CacheCollection for TestCacheCollection {
    fn set_document(&mut self, document: Document) {
        unimplemented!()
    }

    fn inner_collection(&self) -> &Collection {
        unimplemented!()
    }

    fn inner_collection_mut(&mut self) -> &mut Collection {
        unimplemented!()
    }

    fn documents(&self) -> &[RwLock<Document>] {
        unimplemented!()
    }

    fn document(&self, id: Uuid) -> Option<&RwLock<Document>> {
        unimplemented!()
    }

    fn find_first_by_field(&self, field_name: &str, field_value: &str) -> Option<&RwLock<Document>> {
        unimplemented!()
    }

    fn find_by_field(&self, field_name: &str, field_value: &str) -> Vec<&RwLock<Document>> {
        unimplemented!()
    }
}
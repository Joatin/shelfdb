use crate::{Cache, CacheCollection, CacheSchema, Collection, Document, Schema, Store};
use failure::Error;
use slog::Logger;
use std::future::Future;
use std::pin::Pin;
use std::sync::RwLock;
use tokio::sync::broadcast::Receiver;
use uuid::Uuid;

pub struct TestCache;
pub struct TestCacheSchema;
pub struct TestCacheCollection;

impl Cache for TestCache {
    type CacheSchema = TestCacheSchema;

    fn load<'a, S: Store>(
        &'a mut self,
        _logger: &'a Logger,
        _store: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        unimplemented!()
    }

    fn save<'a, S: Store>(
        &'a self,
        _logger: &'a Logger,
        _store: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        unimplemented!()
    }

    fn schemas(&self) -> &Vec<RwLock<Self::CacheSchema>> {
        unimplemented!()
    }

    fn schema(&self, _logger: &Logger, _id: Uuid) -> Option<&RwLock<Self::CacheSchema>> {
        unimplemented!()
    }

    fn schema_by_name(&self, _logger: &Logger, _name: &str) -> Option<&RwLock<Self::CacheSchema>> {
        unimplemented!()
    }

    fn set_schema(
        &mut self,
        _logger: &Logger,
        _schema: Schema,
        _new_graphql_schema: &str,
    ) -> Result<(), Error> {
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

    fn set_collection(&mut self, _collection: Collection) -> Result<(), Error> {
        unimplemented!()
    }

    fn collection(&self, _id: Uuid) -> Option<&RwLock<Self::CacheCollection>> {
        unimplemented!()
    }

    fn collection_by_name(&self, _name: &str) -> Option<&RwLock<Self::CacheCollection>> {
        unimplemented!()
    }
}

impl CacheCollection for TestCacheCollection {
    fn set_document(&mut self, _document: Document) {
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

    fn document(&self, _id: Uuid) -> Option<&RwLock<Document>> {
        unimplemented!()
    }

    fn find_first_by_field(
        &self,
        _field_name: &str,
        _field_value: &str,
    ) -> Option<&RwLock<Document>> {
        unimplemented!()
    }

    fn find_by_field(&self, _field_name: &str, _field_value: &str) -> Vec<&RwLock<Document>> {
        unimplemented!()
    }
}

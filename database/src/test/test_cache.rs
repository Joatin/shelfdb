use crate::{
    Cache,
    CacheCollection,
    CacheSchema,
    Collection,
    Document,
    DocumentResult,
    Schema,
    Store,
};
use failure::Error;
use futures::{
    future::BoxFuture,
    stream::BoxStream,
};
use slog::Logger;
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
};
use tokio::sync::broadcast::Receiver;
use uuid::Uuid;

#[derive(Clone)]
pub struct TestCache;

#[derive(Clone)]
pub struct TestCacheSchema;

#[derive(Clone)]
pub struct TestCacheCollection;

impl Cache for TestCache {
    type CacheSchema = TestCacheSchema;

    fn load<'a, S: Store>(
        &'a self,
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

    fn schemas(&self) -> BoxStream<Self::CacheSchema> {
        unimplemented!()
    }

    fn schema(&self, _id: Uuid) -> BoxFuture<Option<Self::CacheSchema>> {
        unimplemented!()
    }

    fn schema_by_name(&self, _name: &str) -> BoxFuture<Option<Self::CacheSchema>> {
        unimplemented!()
    }

    fn insert_schema<'a>(
        &'a self,
        _logger: &'a Logger,
        _schema: Schema,
        _new_graphql_schema: &'a str,
    ) -> BoxFuture<'a, Result<(), Error>> {
        unimplemented!()
    }

    fn cache_size(&self) -> BoxFuture<usize> {
        unimplemented!()
    }

    fn is_empty(&self) -> BoxFuture<bool> {
        unimplemented!()
    }

    fn on_schema_updates(&self) -> Receiver<()> {
        unimplemented!()
    }
}

impl CacheSchema for TestCacheSchema {
    type CacheCollection = TestCacheCollection;

    fn inner_schema(&self) -> BoxFuture<Schema> {
        unimplemented!()
    }

    fn set_schema(&self, _schema: Schema) -> BoxFuture<()> {
        unimplemented!()
    }

    fn collections(&self) -> BoxStream<Self::CacheCollection> {
        unimplemented!()
    }

    fn insert_collection(&self, _collection: Collection) -> BoxFuture<Result<(), Error>> {
        unimplemented!()
    }

    fn collection(&self, _id: Uuid) -> BoxFuture<Option<Self::CacheCollection>> {
        unimplemented!()
    }

    fn collection_by_name(&self, _name: &str) -> BoxFuture<Option<Self::CacheCollection>> {
        unimplemented!()
    }
}

impl CacheCollection for TestCacheCollection {
    fn set_document(&self, _document: Document) -> BoxFuture<()> {
        unimplemented!()
    }

    fn inner_collection(&self) -> BoxFuture<Collection> {
        unimplemented!()
    }

    fn set_collection(&self, _collection: Collection) -> BoxFuture<()> {
        unimplemented!()
    }

    fn documents<'a>(&'a self) -> BoxFuture<'a, Box<dyn DocumentResult + 'a>> {
        unimplemented!()
    }

    fn document(&self, _id: Uuid) -> BoxFuture<Option<Arc<Document>>> {
        unimplemented!()
    }

    fn find_first_by_field(
        &self,
        _field_name: &str,
        _field_value: &str,
    ) -> BoxFuture<Option<Arc<Document>>> {
        unimplemented!()
    }

    fn find_by_field<'a>(
        &'a self,
        _field_name: &'a str,
        _field_value: &'a str,
    ) -> BoxFuture<'a, Box<dyn DocumentResult + 'a>> {
        unimplemented!()
    }
}

use crate::{
    Collection,
    Document,
    Schema,
    Store,
};
use failure::Error;
use futures::future::BoxFuture;
use slog::Logger;
use std::{
    collections::HashMap,
    sync::Arc,
};
use uuid::Uuid;

pub struct TestStore;

impl Store for TestStore {
    fn get_schemas(&self, _logger: &Logger) -> BoxFuture<Result<HashMap<Uuid, Schema>, Error>> {
        unimplemented!()
    }

    fn get_collections(
        &self,
        _logger: &Logger,
        _schema: &Schema,
    ) -> BoxFuture<Result<Vec<Collection>, Error>> {
        unimplemented!()
    }

    fn get_documents(
        &self,
        _logger: &Logger,
        _schema: &Schema,
        _collection: &Collection,
    ) -> BoxFuture<Result<Vec<Document>, Error>> {
        unimplemented!()
    }

    fn save_schema(&self, _logger: &Logger, _schema: &Schema) -> BoxFuture<Result<(), Error>> {
        unimplemented!()
    }

    fn save_collection<'a>(
        &'a self,
        _logger: &'a Logger,
        _schema: &'a Schema,
        _collection: &'a Collection,
    ) -> BoxFuture<Result<(), Error>> {
        unimplemented!()
    }

    fn save_document<'a>(
        &'a self,
        _logger: &'a Logger,
        _schema: &'a Schema,
        _collection: &'a Collection,
        _document: Arc<Document>,
    ) -> BoxFuture<Result<(), Error>> {
        unimplemented!()
    }

    fn flush<'a>(&'a self, _logger: &'a Logger) -> BoxFuture<Result<(), Error>> {
        unimplemented!()
    }
}

use crate::{Collection, Document, Schema, Store};
use failure::Error;
use slog::Logger;
use std::future::Future;
use std::pin::Pin;

pub struct TestStore;

impl Store for TestStore {
    fn get_schemas(
        &self,
        _logger: &Logger,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Schema>, Error>> + Send>> {
        unimplemented!()
    }

    fn get_collections(
        &self,
        _logger: &Logger,
        _schema: &Schema,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Collection>, Error>> + Send>> {
        unimplemented!()
    }

    fn get_documents(
        &self,
        _logger: &Logger,
        _schema: &Schema,
        _collection: &Collection,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Document>, Error>> + Send>> {
        unimplemented!()
    }

    fn save_schema(
        &self,
        _logger: &Logger,
        _schema: &Schema,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        unimplemented!()
    }

    fn save_collection<'a>(
        &'a self,
        _logger: &'a Logger,
        _schema: &'a Schema,
        _collection: &'a Collection,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        unimplemented!()
    }

    fn save_document<'a>(
        &'a self,
        _logger: &'a Logger,
        _schema: &'a Schema,
        _collection: &'a Collection,
        _document: Document,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        unimplemented!()
    }

    fn flush<'a>(
        &'a self,
        _logger: &'a Logger,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        unimplemented!()
    }
}

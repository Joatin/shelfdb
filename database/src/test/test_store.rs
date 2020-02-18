use crate::{Store, Schema, Document, Collection};
use slog::{Logger};
use failure::Error;
use std::pin::Pin;
use std::future::Future;

pub struct TestStore;

impl Store for TestStore {
    fn get_schemas(&self, logger: &Logger) -> Pin<Box<dyn Future<Output=Result<Vec<Schema>, Error>> + Send>> {
        unimplemented!()
    }

    fn get_collections(&self, logger: &Logger, schema: &Schema) -> Pin<Box<dyn Future<Output=Result<Vec<Collection>, Error>> + Send>> {
        unimplemented!()
    }

    fn get_documents(&self, logger: &Logger, schema: &Schema, collection: &Collection) -> Pin<Box<dyn Future<Output=Result<Vec<Document>, Error>> + Send>> {
        unimplemented!()
    }

    fn save_schema(&self, logger: &Logger, schema: &Schema) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        unimplemented!()
    }

    fn save_collection<'a>(&'a self, logger: &'a Logger, schema: &'a Schema, collection: &'a Collection) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        unimplemented!()
    }

    fn save_document<'a>(&'a self, logger: &'a Logger, schema: &'a Schema, collection: &'a Collection, document: Document) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        unimplemented!()
    }

    fn flush<'a>(&'a self, logger: &'a Logger) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
        unimplemented!()
    }
}
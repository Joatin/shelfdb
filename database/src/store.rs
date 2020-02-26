use crate::{
    Collection,
    Document,
    Schema,
};
use failure::Error;
use futures::Future;
use slog::Logger;
use std::pin::Pin;

pub trait Store: Sync + Send + 'static {
    fn get_schemas(
        &self,
        logger: &Logger,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Schema>, Error>> + Send>>;
    fn get_collections(
        &self,
        logger: &Logger,
        schema: &Schema,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Collection>, Error>> + Send>>;
    fn get_documents(
        &self,
        logger: &Logger,
        schema: &Schema,
        collection: &Collection,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Document>, Error>> + Send>>;
    fn save_schema(
        &self,
        logger: &Logger,
        schema: &Schema,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn save_collection<'a>(
        &'a self,
        logger: &'a Logger,
        schema: &'a Schema,
        collection: &'a Collection,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
    fn save_document<'a>(
        &'a self,
        logger: &'a Logger,
        schema: &'a Schema,
        collection: &'a Collection,
        document: Document,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
    fn flush<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
}

use crate::{
    Collection,
    Document,
    Schema,
};
use failure::Error;
use futures::{
    future::BoxFuture,
    Future,
};
use slog::Logger;
use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
};
use uuid::Uuid;

pub trait Store: Sync + Send + 'static {
    fn get_schemas<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> BoxFuture<'a, Result<HashMap<Uuid, Schema>, Error>>;
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
    fn save_schema<'a>(
        &'a self,
        logger: &'a Logger,
        schema: &'a Schema,
    ) -> BoxFuture<'a, Result<(), Error>>;
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
        document: Arc<Document>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
    fn flush<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
}

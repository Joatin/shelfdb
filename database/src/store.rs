use crate::{
    Collection,
    Document,
    Schema,
};
use failure::Error;
use futures::future::BoxFuture;
use slog::Logger;
use std::{
    collections::HashMap,
    sync::Arc,
};
use uuid::Uuid;

pub trait Store: Sync + Send + 'static {
    fn get_schemas<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> BoxFuture<'a, Result<HashMap<Uuid, Schema>, Error>>;
    fn get_collections<'a>(
        &'a self,
        logger: &'a Logger,
        schema: &'a Schema,
    ) -> BoxFuture<'a, Result<Vec<Collection>, Error>>;
    fn get_documents<'a>(
        &'a self,
        logger: &'a Logger,
        schema: &'a Schema,
        collection: &'a Collection,
    ) -> BoxFuture<'a, Result<Vec<Document>, Error>>;
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
    ) -> BoxFuture<'a, Result<(), Error>>;
    fn save_document<'a>(
        &'a self,
        logger: &'a Logger,
        schema: &'a Schema,
        collection: &'a Collection,
        document: Arc<Document>,
    ) -> BoxFuture<'a, Result<(), Error>>;
    fn flush<'a>(&'a self, logger: &'a Logger) -> BoxFuture<'a, Result<(), Error>>;
}

use futures::Future;
use std::pin::Pin;
use failure::Error;
use crate::collection::Schema;
use slog::Logger;
use uuid::Uuid;

pub trait Cache {

    fn store_schemas(&self, logger: &Logger, schemas: Vec<Schema>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;
    fn schemas(&self, logger: &Logger) -> Pin<Box<dyn Future<Output=Result<Vec<Schema>, Error>> + Send>>;
    fn schema(&self, logger: &Logger, id: &Uuid) -> Pin<Box<dyn Future<Output=Result<Schema, Error>> + Send>>;
    fn set_schema(&self, logger: &Logger, schema: Schema) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;

    fn cache_size(&self) -> usize;
}
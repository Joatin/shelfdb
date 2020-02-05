use futures::Future;
use std::pin::Pin;
use failure::Error;
use crate::{Schema, Store};
use slog::Logger;
use uuid::Uuid;
use std::sync::{Arc, RwLockReadGuard};

pub trait Cache {
    fn load(&self, logger: &Logger, store: &Arc<dyn Store>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;
    fn save(&self, logger: &Logger, store: &Arc<dyn Store>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;

    fn schemas(&self) -> RwLockReadGuard<Vec<Schema>>;
    fn schema(&self, logger: &Logger, id: &Uuid) -> Pin<Box<dyn Future<Output=Result<Schema, Error>> + Send>>;
    fn add_schema(&self, logger: &Logger, schema: Schema) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;

    fn cache_size(&self) -> usize;
    fn is_empty(&self) -> bool;
}
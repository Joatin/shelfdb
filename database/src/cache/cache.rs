use crate::{CacheSchema, Schema, Store};
use failure::Error;
use futures::Future;
use slog::Logger;
use std::pin::Pin;
use std::sync::RwLock;
use tokio::sync::broadcast::Receiver;
use uuid::Uuid;

/// A cache is mainly responsible for keeping all the indexes of objects
///
/// All methods except those that includes some sort of io should be strictly synchronous. Even
/// though async is nice, it does come with performance costs
pub trait Cache: Send + Sync + 'static {
    type CacheSchema: CacheSchema;

    fn load<'a, S: Store>(
        &'a mut self,
        logger: &'a Logger,
        store: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
    fn save<'a, S: Store>(
        &'a self,
        logger: &'a Logger,
        store: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;

    /// Retrieves all schemas
    fn schemas(&self) -> &Vec<RwLock<Self::CacheSchema>>;

    /// Retrieves a schema by it's id
    fn schema(&self, logger: &Logger, id: Uuid) -> Option<&RwLock<Self::CacheSchema>>;

    /// Retrieves a schema by it's name
    fn schema_by_name(&self, logger: &Logger, name: &str) -> Option<&RwLock<Self::CacheSchema>>;

    /// Adds or replaces a new schema to the cache
    fn set_schema(
        &mut self,
        logger: &Logger,
        schema: Schema,
        new_graphql_schema: &str,
    ) -> Result<(), Error>;

    /// Gets the current size in bytes from the cache
    fn cache_size(&self) -> usize;

    /// Tells if this cache is empty
    fn is_empty(&self) -> bool;

    fn on_schema_updates(&self) -> Receiver<()>;
}
